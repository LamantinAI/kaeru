//! Bounded CBOR preflight before allocating decoded graphs.

use std::collections::BTreeSet;

use crate::config::amg::AmgLimits;
use crate::{Error, Result};

pub(super) fn preflight(bytes: &[u8], limits: &AmgLimits) -> Result<()> {
    let mut scanner = Scanner {
        bytes,
        position: 0,
        items: 0,
        limits,
    };
    scanner.item(0)?;
    if scanner.position != bytes.len() {
        return Err(invalid("trailing CBOR values"));
    }
    Ok(())
}

fn invalid(message: &str) -> Error {
    Error::Invalid(format!("AMG: {message}"))
}

struct Scanner<'a> {
    bytes: &'a [u8],
    position: usize,
    items: usize,
    limits: &'a AmgLimits,
}
impl<'a> Scanner<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8]> {
        let end = self
            .position
            .checked_add(count)
            .filter(|end| *end <= self.bytes.len())
            .ok_or_else(|| invalid("truncated CBOR value"))?;
        let out = &self.bytes[self.position..end];
        self.position = end;
        Ok(out)
    }
    fn header(&mut self) -> Result<(u8, u8, u64)> {
        self.items += 1;
        if self.items > self.limits.cbor_items {
            return Err(invalid("CBOR item limit exceeded"));
        }
        let head = self.take(1)?[0];
        let ai = head & 31;
        let argument = match ai {
            0..=23 => ai as u64,
            24 => self.take(1)?[0] as u64,
            25 => u16::from_be_bytes(self.take(2)?.try_into().unwrap()) as u64,
            26 => u32::from_be_bytes(self.take(4)?.try_into().unwrap()) as u64,
            27 => u64::from_be_bytes(self.take(8)?.try_into().unwrap()),
            _ => return Err(invalid("indefinite-length or reserved CBOR header")),
        };
        Ok((head >> 5, ai, argument))
    }
    fn text(&mut self, length: u64) -> Result<&'a str> {
        let length = usize::try_from(length).map_err(|_| invalid("CBOR length overflow"))?;
        std::str::from_utf8(self.take(length)?).map_err(|_| invalid("invalid UTF-8 key or text"))
    }
    fn item(&mut self, depth: usize) -> Result<()> {
        if depth > self.limits.nesting {
            return Err(invalid("CBOR nesting limit exceeded"));
        }
        let (major, ai, argument) = self.header()?;
        match major {
            0 | 1 => {}
            3 => {
                self.text(argument)?;
            }
            4 | 5 => {
                let count =
                    usize::try_from(argument).map_err(|_| invalid("CBOR count overflow"))?;
                let entries = count
                    .checked_mul(if major == 5 { 2 } else { 1 })
                    .ok_or_else(|| invalid("CBOR count overflow"))?;
                if entries > self.limits.cbor_items.saturating_sub(self.items)
                    || entries > self.bytes.len().saturating_sub(self.position)
                {
                    return Err(invalid("CBOR collection exceeds bounds"));
                }
                let mut keys = BTreeSet::new();
                for _ in 0..count {
                    if major == 5 {
                        let (kind, _, len) = self.header()?;
                        if kind != 3 {
                            return Err(invalid("CBOR map keys must be text"));
                        }
                        if !keys.insert(self.text(len)?) {
                            return Err(invalid("duplicate CBOR map key"));
                        }
                    }
                    self.item(depth + 1)?;
                }
            }
            7 => match ai {
                20..=22 => {}
                25 if argument & 0x7c00 != 0x7c00 => {}
                26 if f32::from_bits(argument as u32).is_finite() => {}
                27 if f64::from_bits(argument).is_finite() => {}
                _ => return Err(invalid("unsupported or non-finite CBOR value")),
            },
            _ => {
                return Err(invalid(
                    "CBOR tags, byte strings and unknown types are not in AMG v1",
                ));
            }
        }
        Ok(())
    }
}
