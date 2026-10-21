//! ZIP entry names derive only from record IDs, never names or filesystem paths.

use std::io::{self, Cursor, Seek, SeekFrom, Write};

use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use zip::{CompressionMethod, ZipWriter, write::SimpleFileOptions};

use crate::config::amg::AmgLimits;
use crate::models::amg::AmgDocument;
use crate::{Error, Result};

struct Bounded {
    cursor: Cursor<Vec<u8>>,
    maximum: usize,
}
impl Write for Bounded {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self
            .cursor
            .position()
            .checked_add(bytes.len() as u64)
            .is_none_or(|n| n > self.maximum as u64)
        {
            return Err(io::Error::other("archive exceeds configured byte limit"));
        }
        self.cursor.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Seek for Bounded {
    fn seek(&mut self, from: SeekFrom) -> io::Result<u64> {
        self.cursor.seek(from)
    }
}
fn invalid(error: impl std::fmt::Display) -> Error {
    Error::Invalid(format!("Markdown archive: {error}"))
}
fn filename(id: &str) -> String {
    format!("nodes/{:x}.md", Sha256::digest(id.as_bytes()))
}
fn label(text: &str) -> String {
    text.replace('\n', " ")
        .replace('\r', " ")
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn literal(text: &str) -> String {
    let fence = "`".repeat(
        text.split(|c| c != '`')
            .map(str::len)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
            .max(3),
    );
    format!("{fence}\n{text}\n{fence}\n")
}
fn value(value: impl Serialize) -> Result<String> {
    Ok(literal(
        &serde_json::to_string_pretty(&value).map_err(invalid)?,
    ))
}
fn entry(
    zip: &mut ZipWriter<Bounded>,
    path: &str,
    body: &str,
    total: &mut usize,
    limits: &AmgLimits,
) -> Result<()> {
    *total = total
        .checked_add(body.len())
        .filter(|n| *n <= limits.file_bytes)
        .ok_or_else(|| invalid("uncompressed pages exceed configured byte limit"))?;
    zip.start_file(
        path,
        SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o600),
    )
    .map_err(invalid)?;
    zip.write_all(body.as_bytes())?;
    Ok(())
}

pub(super) fn encode(doc: &AmgDocument, limits: &AmgLimits) -> Result<Vec<u8>> {
    let mut zip = ZipWriter::new(Bounded {
        cursor: Cursor::new(vec![]),
        maximum: limits.file_bytes,
    });
    let mut total = 0;
    let mut index = String::from(
        "# Kaeru knowledge archive\n\nCurrent knowledge only; source history and runtime configuration are not included.\n\n## Snapshot\n\n",
    );
    index.push_str(&value(&doc.metadata)?);
    index.push_str("\n## Records\n\n");
    for node in &doc.data.nodes {
        index.push_str(&format!(
            "- [{}]({}) — {}\n",
            label(&node.name),
            filename(&node.id),
            node.node_type
        ));
    }
    index.push_str("\n[Relationships](RELATIONS.md) · [Reasoning trails](TRAILS.md) · [Slots and verdicts](STATE.md)\n");
    entry(&mut zip, "README.md", &index, &mut total, limits)?;
    for node in &doc.data.nodes {
        let mut page = format!(
            "# {}\n\n[Archive index](../README.md)\n\n## Record\n\n",
            label(&node.name)
        );
        page.push_str(&value(json!({"id":node.id,"type":node.node_type,"tier":node.tier,"layer":node.layer,
            "visibility":node.visibility,"initiatives":node.initiatives,"tags":node.tags,"recorded_at":node.recorded_at,"created_at":node.created_at,"origins":node.origins}))?);
        if let Some(body) = &node.body {
            page.push_str("\n## Content\n\n");
            page.push_str(&literal(body));
        }
        if !node.properties.is_null() {
            page.push_str("\n## Properties\n\n");
            page.push_str(&value(&node.properties)?);
        }
        entry(&mut zip, &filename(&node.id), &page, &mut total, limits)?;
    }
    let mut edges = String::from("# Relationships\n\n[Archive index](README.md)\n\n");
    for edge in &doc.data.edges {
        edges.push_str(&format!(
            "## [{}]({}) → [{}]({})\n\n",
            label(&edge.src),
            filename(&edge.src),
            label(&edge.dst),
            filename(&edge.dst)
        ));
        edges.push_str(&value(edge)?);
        edges.push('\n');
    }
    entry(&mut zip, "RELATIONS.md", &edges, &mut total, limits)?;
    let mut trails = String::from("# Reasoning trails\n\n[Archive index](README.md)\n\n");
    for trail in &doc.data.trails {
        trails.push_str(&format!(
            "## [{}]({})\n\n",
            label(&trail.id),
            filename(&trail.id)
        ));
        for (index, id) in trail.members.iter().enumerate() {
            trails.push_str(&format!(
                "{}. [{}]({})\n",
                index + 1,
                label(id),
                filename(id)
            ));
        }
        trails.push('\n');
        trails.push_str(&value(trail)?);
        trails.push('\n');
    }
    entry(&mut zip, "TRAILS.md", &trails, &mut total, limits)?;
    let state = format!(
        "# Slots and verdicts\n\n[Archive index](README.md)\n\n## Current slots\n\n{}\n## Hypothesis decisions\n\n{}",
        value(&doc.data.slots)?,
        value(&doc.data.decisions)?
    );
    entry(&mut zip, "STATE.md", &state, &mut total, limits)?;
    Ok(zip.finish().map_err(invalid)?.cursor.into_inner())
}
