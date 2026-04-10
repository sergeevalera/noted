use std::collections::HashMap;

use camino::{Utf8Path, Utf8PathBuf};

/// The full in-memory index of a vault (workspace directory).
#[derive(Debug, Default)]
pub struct VaultIndex {
    /// All indexed notes, keyed by absolute path.
    pub notes: HashMap<Utf8PathBuf, NoteEntry>,
    /// Reverse link map: for each note, which other notes link to it.
    pub backlinks: HashMap<Utf8PathBuf, Vec<LinkReference>>,
}

/// Parsed representation of a single Markdown note.
#[derive(Debug, Clone)]
pub struct NoteEntry {
    pub path: Utf8PathBuf,
    /// Display title: from frontmatter, or first H1, or filename stem.
    pub title: String,
    pub headings: Vec<Heading>,
    pub links: Vec<LinkReference>,
    pub tags: Vec<Tag>,
    #[allow(dead_code)]
    pub frontmatter: Option<Frontmatter>,
}

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub line: u32,
}

#[derive(Debug, Clone)]
pub struct LinkReference {
    /// The wikilink target, e.g. `"My Note"` from `[[My Note]]`.
    pub target: String,
    /// Optional heading anchor, e.g. `"section"` from `[[My Note#section]]`.
    pub anchor: Option<String>,
    /// Optional display alias, e.g. `"click here"` from `[[My Note|click here]]`.
    pub alias: Option<String>,
    pub line: u32,
    pub col: u32,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    #[allow(dead_code)]
    pub line: u32,
}

#[derive(Debug, Clone)]
pub struct Frontmatter {
    pub title: Option<String>,
    #[allow(dead_code)]
    pub tags: Vec<String>,
    /// Raw YAML text between the `---` markers.
    #[allow(dead_code)]
    pub raw: String,
}

// ── Index building ─────────────────────────────────────────────────────────

/// Build a `VaultIndex` from a list of parsed notes.
/// Populates both the notes map and the reverse backlinks map.
pub fn build_index(notes: Vec<NoteEntry>) -> VaultIndex {
    let mut index = VaultIndex::default();

    // First pass: insert all notes so resolve_wikilink can find them.
    for note in notes {
        index.notes.insert(note.path.clone(), note);
    }

    // Second pass: for every wikilink in every note, populate backlinks.
    let paths: Vec<Utf8PathBuf> = index.notes.keys().cloned().collect();
    for path in paths {
        let links: Vec<LinkReference> = index.notes[&path].links.clone();
        for link in links {
            if let Some(target_path) = resolve_wikilink(&index, &link.target) {
                index
                    .backlinks
                    .entry(target_path)
                    .or_default()
                    .push(LinkReference {
                        target: link.target,
                        anchor: link.anchor,
                        alias: link.alias,
                        line: link.line,
                        col: link.col,
                    });
            }
        }
    }

    index
}

/// Resolve a wikilink target string to an absolute path in the vault.
///
/// Matching rules (in order):
/// 1. Exact basename match (case-insensitive): `[[My Note]]` → `…/My Note.md`
/// 2. Path suffix match: `[[daily/2026-04-05]]` → `…/daily/2026-04-05.md`
///
/// Returns `None` if no note matches (broken link).
pub fn resolve_wikilink(index: &VaultIndex, target: &str) -> Option<Utf8PathBuf> {
    let target_lower = target.to_lowercase();

    // Try suffix match first (supports sub-path targets like "daily/note")
    let with_ext = format!("{}.md", target_lower);
    let with_ext_markdown = format!("{}.markdown", target_lower);

    for path in index.notes.keys() {
        let path_lower = path.as_str().to_lowercase();
        if path_lower.ends_with(&with_ext) || path_lower.ends_with(&with_ext_markdown) {
            return Some(path.clone());
        }
        // Basename-only match: [[Note]] matches …/any/path/Note.md
        if let Some(stem) = Utf8Path::new(&path_lower).file_stem() {
            if stem == target_lower {
                return Some(path.clone());
            }
        }
    }

    None
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::parser::parse_note;

    fn make_note(path: &str, content: &str) -> NoteEntry {
        parse_note(&Utf8PathBuf::from(path), content)
    }

    fn vault_with(notes: Vec<NoteEntry>) -> VaultIndex {
        build_index(notes)
    }

    #[test]
    fn test_build_index_populates_notes() {
        let vault = vault_with(vec![
            make_note("/vault/alice.md", "# Alice\n"),
            make_note("/vault/bob.md", "# Bob\n"),
            make_note("/vault/carol.md", "# Carol\n"),
        ]);
        assert_eq!(vault.notes.len(), 3);
    }

    #[test]
    fn test_resolve_simple_wikilink() {
        let vault = vault_with(vec![
            make_note("/vault/alice.md", "# Alice\n"),
            make_note("/vault/bob.md", "# Bob\n"),
        ]);
        let result = resolve_wikilink(&vault, "alice");
        assert!(result.is_some());
        assert_eq!(result.unwrap().file_name(), Some("alice.md"));
    }

    #[test]
    fn test_resolve_is_case_insensitive() {
        let vault = vault_with(vec![make_note("/vault/My Note.md", "# My Note\n")]);
        assert!(resolve_wikilink(&vault, "my note").is_some());
        assert!(resolve_wikilink(&vault, "MY NOTE").is_some());
        assert!(resolve_wikilink(&vault, "My Note").is_some());
    }

    #[test]
    fn test_resolve_broken_link_returns_none() {
        let vault = vault_with(vec![make_note("/vault/alice.md", "# Alice\n")]);
        assert!(resolve_wikilink(&vault, "nonexistent").is_none());
    }

    #[test]
    fn test_resolve_subpath_wikilink() {
        let vault = vault_with(vec![make_note("/vault/daily/2026-04-05.md", "# Daily\n")]);
        assert!(resolve_wikilink(&vault, "daily/2026-04-05").is_some());
    }

    #[test]
    fn test_backlinks_populated() {
        let vault = vault_with(vec![
            make_note("/vault/alice.md", "# Alice\n\nSee [[bob]].\n"),
            make_note("/vault/bob.md", "# Bob\n"),
            make_note("/vault/carol.md", "# Carol\n\nAlso links to [[bob]].\n"),
        ]);
        let bob_path = Utf8PathBuf::from("/vault/bob.md");
        let backlinks = vault.backlinks.get(&bob_path);
        assert!(backlinks.is_some());
        assert_eq!(backlinks.unwrap().len(), 2);
    }

    #[test]
    fn test_broken_links_not_in_backlinks() {
        let vault = vault_with(vec![make_note(
            "/vault/alice.md",
            "# Alice\n\nSee [[missing]].\n",
        )]);
        // No entry for "missing" since it doesn't exist
        assert_eq!(vault.backlinks.len(), 0);
    }
}
