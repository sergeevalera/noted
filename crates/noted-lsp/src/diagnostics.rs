use regex::Regex;
use tower_lsp::lsp_types::*;

use crate::vault::{resolve_wikilink, VaultIndex};

/// Compute broken-link diagnostics for the given document text.
///
/// Scans all `[[...]]` wikilinks, resolves each against `index`,
/// and returns an ERROR diagnostic for every unresolvable target.
/// Returns an empty list when the index has not been built yet to avoid
/// false positives during startup.
pub fn compute_diagnostics(text: &str, index: &VaultIndex) -> Vec<Diagnostic> {
    if index.notes.is_empty() {
        return vec![];
    }

    let re = Regex::new(r"\[\[([^\]|#\n]+?)(?:#[^\]|\n]*?)?(?:\|[^\]\n]*?)?\]\]").unwrap();
    let line_starts = build_line_starts(text);
    let mut diagnostics = Vec::new();

    for cap in re.captures_iter(text) {
        let full = cap.get(0).unwrap();
        let target = cap[1].trim();

        if resolve_wikilink(index, target).is_none() {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: offset_to_position(&line_starts, full.start()),
                    end: offset_to_position(&line_starts, full.end()),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("noted-lsp".to_string()),
                message: format!("Broken link: '{}' not found in vault", target),
                ..Default::default()
            });
        }
    }

    diagnostics
}

fn build_line_starts(text: &str) -> Vec<usize> {
    let mut starts = vec![0];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

fn offset_to_position(line_starts: &[usize], offset: usize) -> Position {
    let line = line_starts.partition_point(|&s| s <= offset).saturating_sub(1);
    let character = offset - line_starts[line];
    Position { line: line as u32, character: character as u32 }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;
    use crate::vault::{build_index, parse_note, VaultIndex};

    fn make_index(notes: &[(&str, &str)]) -> VaultIndex {
        let entries = notes
            .iter()
            .map(|(path, content)| parse_note(&Utf8PathBuf::from(path), content))
            .collect();
        build_index(entries)
    }

    #[test]
    fn test_no_diagnostics_for_valid_link() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        assert!(compute_diagnostics("See [[alice]].", &index).is_empty());
    }

    #[test]
    fn test_broken_link_produces_error() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let diags = compute_diagnostics("See [[missing]].", &index);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Some(DiagnosticSeverity::ERROR));
        assert!(diags[0].message.contains("missing"));
    }

    #[test]
    fn test_multiple_broken_links() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let diags = compute_diagnostics("[[broken1]] and [[broken2]]", &index);
        assert_eq!(diags.len(), 2);
    }

    #[test]
    fn test_mixed_valid_and_broken() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let diags = compute_diagnostics("[[alice]] then [[broken]]", &index);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("broken"));
    }

    #[test]
    fn test_diagnostic_range() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let diags = compute_diagnostics("[[missing]]", &index);
        assert_eq!(diags.len(), 1);
        let r = diags[0].range;
        assert_eq!(r.start, Position { line: 0, character: 0 });
        assert_eq!(r.end, Position { line: 0, character: 11 }); // len("[[missing]]")
    }

    #[test]
    fn test_diagnostic_range_with_offset() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let diags = compute_diagnostics("text [[missing]] more", &index);
        assert_eq!(diags.len(), 1);
        let r = diags[0].range;
        assert_eq!(r.start, Position { line: 0, character: 5 });
        assert_eq!(r.end, Position { line: 0, character: 16 });
    }

    #[test]
    fn test_link_with_anchor_is_valid() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        assert!(compute_diagnostics("[[alice#section]]", &index).is_empty());
    }

    #[test]
    fn test_link_with_alias_is_valid() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        assert!(compute_diagnostics("[[alice|Alice In Wonderland]]", &index).is_empty());
    }

    #[test]
    fn test_empty_index_returns_no_diagnostics() {
        let index = VaultIndex::default();
        // Should not flag anything — index not ready yet
        assert!(compute_diagnostics("[[anything]]", &index).is_empty());
    }

    #[test]
    fn test_multiline_positions() {
        let index = make_index(&[("/vault/alice.md", "# Alice\n")]);
        let text = "line one\nline two [[missing]] end\n";
        let diags = compute_diagnostics(text, &index);
        assert_eq!(diags.len(), 1);
        let r = diags[0].range;
        assert_eq!(r.start.line, 1);
        assert_eq!(r.start.character, 9); // "line two " is 9 chars
    }
}
