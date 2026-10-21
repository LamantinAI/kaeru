//! Live archive capture, host authorization and no-clobber artifact publication.

use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tempfile::NamedTempFile;

use super::super::{ApiOptions, ApiResult, Args, issue_from_error};
use crate::Store;
use crate::amg::snapshot;
use crate::archive::encode;
use crate::config::amg::AmgLimits;
use crate::guard::scan_public;
use crate::models::amg::AmgSelection;
use crate::models::{ArchiveFormat, Artifact, FailedStep, Issue, MutationData, Response, State};

fn io_issue(error: impl std::fmt::Display) -> Issue {
    Issue::new("IO_ERROR", error.to_string())
}
fn sensitive(value: &Value) -> bool {
    match value {
        Value::String(text) => !scan_public(text).is_empty(),
        Value::Array(values) => values.iter().any(sensitive),
        Value::Object(values) => values.iter().any(|(key, value)| {
            !scan_public(key).is_empty()
                || value
                    .as_str()
                    .is_some_and(|text| !scan_public(&format!("{key}={text}")).is_empty())
                || sensitive(value)
        }),
        _ => false,
    }
}
fn destination(root: &Path, name: &str, format: ArchiveFormat) -> ApiResult<PathBuf> {
    let mut path = PathBuf::from(name);
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(Issue::new(
            "INVALID_ARGUMENT",
            "destination cannot traverse parent directories",
        ));
    }
    match path.extension().and_then(|e| e.to_str()) {
        None => {
            path.set_extension(format.extension());
        }
        Some(extension) if extension == format.extension() => {}
        _ => {
            return Err(Issue::new(
                "INVALID_ARGUMENT",
                format!("destination must end in .{}", format.extension()),
            ));
        }
    }
    let path = if path.is_absolute() {
        path
    } else {
        root.join(path)
    };
    let parent = path
        .parent()
        .ok_or_else(|| Issue::new("INVALID_ARGUMENT", "destination needs a parent directory"))?;
    let parent = parent.canonicalize().map_err(|_| {
        Issue::new(
            "INVALID_ARGUMENT",
            "destination parent must exist in the artifact root",
        )
    })?;
    if !parent.starts_with(root) {
        return Err(Issue::new(
            "INVALID_ARGUMENT",
            "destination is outside artifact root",
        ));
    }
    let name = path
        .file_name()
        .ok_or_else(|| Issue::new("INVALID_ARGUMENT", "destination needs a filename"))?;
    Ok(parent.join(name))
}
fn publish(path: &Path, bytes: &[u8]) -> ApiResult<Option<Issue>> {
    let parent = path.parent().expect("validated parent");
    let mut file = NamedTempFile::new_in(parent).map_err(io_issue)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.as_file()
            .set_permissions(std::fs::Permissions::from_mode(0o600))
            .map_err(io_issue)?;
    }
    file.write_all(bytes).map_err(io_issue)?;
    file.as_file_mut().sync_all().map_err(io_issue)?;
    file.persist_noclobber(path).map_err(|e| {
        if e.error.kind() == std::io::ErrorKind::AlreadyExists {
            Issue::new("CONFLICT", "choose a new export destination")
        } else {
            io_issue(e.error)
        }
    })?;
    #[cfg(unix)]
    if let Err(error) = File::open(parent).and_then(|dir| dir.sync_all()) {
        return Ok(Some(io_issue(format!(
            "artifact published, but directory sync failed: {error}"
        ))));
    }
    Ok(None)
}

pub(super) fn run(store: &Store, args: &Args, options: &ApiOptions) -> ApiResult<Response> {
    let root = options
        .artifact_root
        .as_ref()
        .ok_or_else(|| Issue::new("UNSUPPORTED_SOURCE", "host has no authorized artifact root"))?;
    let selection = if let Some(name) = args.text("initiative") {
        AmgSelection::Initiatives(vec![name])
    } else if args.has("initiatives") {
        AmgSelection::Initiatives(args.parse("initiatives")?)
    } else {
        AmgSelection::All
    };
    let format: ArchiveFormat = args.parse("format")?;
    let limits = AmgLimits::default();
    let document=snapshot(store,&selection,&limits).map_err(|error| {
        if options.allowed_initiatives.is_some() {Issue::new("INVALID_ARGUMENT","cannot export a closed current-knowledge snapshot from the selected accessible initiatives")}
        else{issue_from_error(error)}
    })?;
    let bytes = encode(&document, format, &limits).map_err(issue_from_error)?;
    // Redacting hashes/original source text would create a misleading portable
    // archive. Restricted egress refuses it instead of silently changing meaning.
    if options.allowed_initiatives.is_some()
        && sensitive(&json!({"metadata":document.metadata,"data":document.data}))
    {
        return Err(Issue::new(
            "EXPORT_REQUIRES_REDACTION",
            "selected archive contains content requiring redaction; choose a narrower selection or use the trusted local owner",
        ));
    }
    // No final artifact or directory is created until capture/guard validation.
    create_dir_all(root).map_err(io_issue)?;
    let root = root.canonicalize().map_err(io_issue)?;
    let path = destination(&root, &args.required_text("destination")?, format)?;
    let warning = publish(&path, &bytes)?;
    let mut data = MutationData {
        artifacts: vec![Artifact {
            path: path.to_string_lossy().into(),
            format,
            bytes: bytes.len(),
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            nodes: document.data.nodes.len(),
            edges: document.data.edges.len(),
            initiatives: document.metadata.initiatives,
            omissions: document.metadata.omissions,
        }],
        ..Default::default()
    };
    if let Some(issue) = warning {
        data.failed_steps.push(FailedStep {
            step: "directory_sync".into(),
            code: issue.code.clone(),
            message: issue.message.clone(),
        });
        let mut response = Response::mutation("export", data);
        response.state = State::Partial;
        response.issues.push(issue);
        return Ok(response);
    }
    Ok(Response::mutation("export", data))
}
