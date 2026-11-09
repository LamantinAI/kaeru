//! Cargo-only builds embed the checked-in bundle and never invoke Node.

use std::path::Path;

fn check(directory: &Path) {
    for entry in std::fs::read_dir(directory)
        .expect("Missing UI bundle; run npm run build:embedded in kaeru-viz")
    {
        let entry = entry.expect("Read embedded UI asset");
        let kind = entry.file_type().expect("Inspect embedded UI asset");
        assert!(!kind.is_symlink(), "Embedded assets must not be symlinks");
        let path = entry.path();
        if kind.is_dir() {
            check(&path);
            continue;
        }
        let name = entry.file_name();
        let allowed = name == "manifest.json"
            || matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("html" | "js" | "css" | "svg" | "woff2" | "png" | "ico")
            );
        assert!(allowed, "Unexpected embedded asset: {}", path.display());
    }
}

fn main() {
    println!("cargo:rerun-if-changed=ui-dist");
    let directory = Path::new("ui-dist");
    assert!(
        directory.join("index.html").is_file() && directory.join("manifest.json").is_file(),
        "Build the frontend with npm run build:embedded before packaging"
    );
    check(directory);
}
