//! Human-readable Markdown ZIP and portable AMG from the same current snapshot.

mod markdown;

use self::markdown::encode as encode_markdown;
use crate::Result;
use crate::amg::encode as encode_amg;
use crate::config::amg::AmgLimits;
use crate::models::ArchiveFormat;
use crate::models::amg::AmgDocument;

pub fn encode(
    document: &AmgDocument,
    format: ArchiveFormat,
    limits: &AmgLimits,
) -> Result<Vec<u8>> {
    // Bound/validate the complete portable state before rendering either format.
    let encoded = encode_amg(document, limits)?;
    match format {
        ArchiveFormat::Amg => Ok(encoded),
        ArchiveFormat::Markdown => {
            drop(encoded);
            encode_markdown(document, limits)
        }
    }
}
