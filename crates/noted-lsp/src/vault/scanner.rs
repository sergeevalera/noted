use camino::Utf8PathBuf;
use walkdir::WalkDir;

/// Recursively scan `root` and return paths to all Markdown files.
/// Ignores `.obsidian/`, `node_modules/`, and hidden directories (`.git`, etc.).
pub fn scan_vault(root: &Utf8PathBuf) -> Vec<Utf8PathBuf> {
    WalkDir::new(root.as_std_path())
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            // Skip hidden dirs and common non-vault directories
            if e.file_type().is_dir() {
                return !matches!(
                    name.as_ref(),
                    ".git" | ".obsidian" | "node_modules" | "target" | ".cache"
                ) && !name.starts_with('.');
            }
            true
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            if !e.file_type().is_file() {
                return false;
            }
            matches!(
                path.extension().and_then(|s| s.to_str()),
                Some("md") | Some("markdown")
            )
        })
        .filter_map(|e| Utf8PathBuf::from_path_buf(e.into_path()).ok())
        .collect()
}
