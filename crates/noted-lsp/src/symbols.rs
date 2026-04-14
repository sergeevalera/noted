use tower_lsp::lsp_types::*;

/// Compute document symbols (outline) from Markdown text.
///
/// Headings become `DocumentSymbol`s with proper nesting:
/// H2 headings are children of the preceding H1, H3 of H2, etc.
/// Each symbol's `range` covers its full section; `selection_range` is the heading line.
pub fn compute_document_symbols(text: &str) -> Vec<DocumentSymbol> {
    let headings = extract_headings(text);
    if headings.is_empty() {
        return vec![];
    }
    let total_lines = text.lines().count() as u32;
    let mut i = 0;
    build_children(&headings, &mut i, 0, total_lines.saturating_sub(1))
}

// ── Internal types ─────────────────────────────────────────────────────────

struct HeadingInfo {
    level: u8,
    text: String,
    line: u32,
    /// Byte length of the full heading line (for selection_range end character)
    line_len: u32,
}

// ── Tree building ──────────────────────────────────────────────────────────

/// Recursively collect symbols whose level is greater than `parent_level`
/// and whose line is within `[current_i .. until_line]`.
///
/// Advances `i` past all consumed headings and returns when it hits a heading
/// at `parent_level` or higher (which belongs to the caller's scope).
fn build_children(
    headings: &[HeadingInfo],
    i: &mut usize,
    parent_level: u8,
    until_line: u32,
) -> Vec<DocumentSymbol> {
    let mut result = Vec::new();

    while *i < headings.len() {
        let h = &headings[*i];

        if h.line > until_line {
            break;
        }
        // Stop if this heading belongs to the parent's scope (same or lower level number)
        if parent_level > 0 && h.level <= parent_level {
            break;
        }

        let level = h.level;
        *i += 1;

        // Section end: the line just before the next heading at same or higher level
        let section_end = headings[*i..]
            .iter()
            .find(|next| next.level <= level)
            .map(|next| next.line.saturating_sub(1))
            .unwrap_or(until_line);

        let children = build_children(headings, i, level, section_end);

        result.push(make_symbol(h, section_end, children));
    }

    result
}

fn make_symbol(h: &HeadingInfo, section_end: u32, children: Vec<DocumentSymbol>) -> DocumentSymbol {
    let heading_end_char = h.line_len;
    DocumentSymbol {
        name: h.text.clone(),
        detail: None,
        kind: level_to_kind(h.level),
        tags: None,
        #[allow(deprecated)]
        deprecated: None,
        // Full section range: from heading start to end of section
        range: Range {
            start: Position {
                line: h.line,
                character: 0,
            },
            end: Position {
                line: section_end,
                character: u32::MAX,
            },
        },
        // Selection: just the heading line itself
        selection_range: Range {
            start: Position {
                line: h.line,
                character: 0,
            },
            end: Position {
                line: h.line,
                character: heading_end_char,
            },
        },
        children: if children.is_empty() {
            None
        } else {
            Some(children)
        },
    }
}

fn level_to_kind(level: u8) -> SymbolKind {
    match level {
        1 => SymbolKind::MODULE,
        2 => SymbolKind::NAMESPACE,
        3 => SymbolKind::CLASS,
        _ => SymbolKind::FIELD,
    }
}

// ── Heading extraction ─────────────────────────────────────────────────────

fn extract_headings(text: &str) -> Vec<HeadingInfo> {
    text.lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let level = heading_level(line)?;
            // Heading text starts after "### " (level + space)
            let text = line[(level as usize + 1)..].trim().to_string();
            Some(HeadingInfo {
                level,
                text,
                line: i as u32,
                line_len: line.encode_utf16().count() as u32,
            })
        })
        .collect()
}

/// Returns the ATX heading level (1–6) if `line` starts with `# ` … `###### `,
/// or `None` otherwise.
fn heading_level(line: &str) -> Option<u8> {
    let hashes = line.bytes().take_while(|&b| b == b'#').count();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    // Must be followed by a space
    if line.as_bytes().get(hashes) == Some(&b' ') {
        Some(hashes as u8)
    } else {
        None
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn sym_names(syms: &[DocumentSymbol]) -> Vec<String> {
        syms.iter().map(|s| s.name.clone()).collect()
    }

    fn child_names(sym: &DocumentSymbol) -> Vec<String> {
        sym.children.as_deref().map(sym_names).unwrap_or_default()
    }

    #[test]
    fn test_empty_document() {
        assert!(compute_document_symbols("no headings here").is_empty());
    }

    #[test]
    fn test_flat_h1_headings() {
        let text = "# Alpha\n\ntext\n\n# Beta\n\nmore\n";
        let syms = compute_document_symbols(text);
        assert_eq!(sym_names(&syms), vec!["Alpha", "Beta"]);
        assert!(syms[0].children.is_none());
    }

    #[test]
    fn test_h1_with_h2_children() {
        let text = "# Parent\n\n## Child A\n\n## Child B\n\n# Next\n";
        let syms = compute_document_symbols(text);
        assert_eq!(sym_names(&syms), vec!["Parent", "Next"]);
        assert_eq!(child_names(&syms[0]), vec!["Child A", "Child B"]);
    }

    #[test]
    fn test_deep_nesting() {
        let text = "# H1\n\n## H2\n\n### H3\n\ntext\n";
        let syms = compute_document_symbols(text);
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name, "H1");
        let h2_syms = syms[0].children.as_ref().unwrap();
        assert_eq!(h2_syms[0].name, "H2");
        let h3_syms = h2_syms[0].children.as_ref().unwrap();
        assert_eq!(h3_syms[0].name, "H3");
    }

    #[test]
    fn test_orphan_h2_becomes_top_level() {
        // H2 without a preceding H1 — should still appear as top-level
        let text = "## Orphan\n\ntext\n";
        let syms = compute_document_symbols(text);
        assert_eq!(sym_names(&syms), vec!["Orphan"]);
    }

    #[test]
    fn test_heading_range() {
        let text = "# Title\n\n## Section\n\nContent.\n";
        let syms = compute_document_symbols(text);
        // H1 covers the whole doc
        assert_eq!(syms[0].range.start.line, 0);
        // H2 is a child
        let h2 = &syms[0].children.as_ref().unwrap()[0];
        assert_eq!(h2.selection_range.start.line, 2);
        assert_eq!(h2.selection_range.end.line, 2);
    }

    #[test]
    fn test_selection_range_is_heading_line() {
        let text = "# Hello World\n\ntext\n";
        let syms = compute_document_symbols(text);
        let sr = syms[0].selection_range;
        assert_eq!(
            sr.start,
            Position {
                line: 0,
                character: 0
            }
        );
        assert_eq!(sr.end.line, 0);
    }

    #[test]
    fn test_symbol_kind_by_level() {
        let text = "# H1\n\n## H2\n\n### H3\n\n#### H4\n";
        let syms = compute_document_symbols(text);
        assert_eq!(syms[0].kind, SymbolKind::MODULE);
        let h2 = &syms[0].children.as_ref().unwrap()[0];
        assert_eq!(h2.kind, SymbolKind::NAMESPACE);
        let h3 = &h2.children.as_ref().unwrap()[0];
        assert_eq!(h3.kind, SymbolKind::CLASS);
        let h4 = &h3.children.as_ref().unwrap()[0];
        assert_eq!(h4.kind, SymbolKind::FIELD);
    }

    #[test]
    fn test_skipped_level() {
        // H1 directly followed by H3 (no H2)
        let text = "# Top\n\n### Deep\n\ntext\n";
        let syms = compute_document_symbols(text);
        assert_eq!(syms.len(), 1);
        let children = syms[0].children.as_ref().unwrap();
        assert_eq!(children[0].name, "Deep");
    }

    #[test]
    fn test_non_heading_hash_ignored() {
        // `#tag` should not be treated as a heading
        let text = "text #tag more\n\n# Real Heading\n";
        let syms = compute_document_symbols(text);
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name, "Real Heading");
    }
}
