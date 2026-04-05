use regex::Regex;
use tower_lsp::lsp_types::*;

// Token type indices — must match the legend order declared in `initialize`
pub const TYPE_HEADING: u32 = 0;
pub const TYPE_MARKUP: u32 = 1;
pub const TYPE_PUNCTUATION: u32 = 2;

// Token modifier bits (1 << legend index)
pub const MOD_H1: u32 = 1 << 0;
pub const MOD_H2: u32 = 1 << 1;
pub const MOD_H3: u32 = 1 << 2;
pub const MOD_BOLD: u32 = 1 << 3;
pub const MOD_ITALIC: u32 = 1 << 4;

/// The `SemanticTokensLegend` advertised in `initialize`.
/// Must stay in sync with the `TYPE_*` / `MOD_*` constants above.
pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            SemanticTokenType::new("heading"),
            SemanticTokenType::new("markup"),
            SemanticTokenType::new("punctuation"),
        ],
        token_modifiers: vec![
            SemanticTokenModifier::new("h1"),
            SemanticTokenModifier::new("h2"),
            SemanticTokenModifier::new("h3"),
            SemanticTokenModifier::new("bold"),
            SemanticTokenModifier::new("italic"),
        ],
    }
}

/// Compute semantic tokens for the given Markdown text.
/// Returns tokens sorted by position and encoded in the LSP delta format.
pub fn compute_semantic_tokens(text: &str) -> Vec<SemanticToken> {
    let bold_re = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    let italic_re = Regex::new(r"\*([^*\n]+?)\*").unwrap();

    // Collect raw tokens as (line, start_char, length, type, modifiers)
    let mut raw: Vec<(u32, u32, u32, u32, u32)> = Vec::new();

    for (line_idx, line) in text.lines().enumerate() {
        let ln = line_idx as u32;

        // Headings — emit punctuation token for the `#` markers, heading token for the text
        if line.starts_with("### ") {
            raw.push((ln, 0, 3, TYPE_PUNCTUATION, 0));
            let len = line.len().saturating_sub(4) as u32;
            if len > 0 {
                raw.push((ln, 4, len, TYPE_HEADING, MOD_H3));
            }
            continue;
        } else if line.starts_with("## ") {
            raw.push((ln, 0, 2, TYPE_PUNCTUATION, 0));
            let len = line.len().saturating_sub(3) as u32;
            if len > 0 {
                raw.push((ln, 3, len, TYPE_HEADING, MOD_H2));
            }
            continue;
        } else if line.starts_with("# ") {
            raw.push((ln, 0, 1, TYPE_PUNCTUATION, 0));
            let len = line.len().saturating_sub(2) as u32;
            if len > 0 {
                raw.push((ln, 2, len, TYPE_HEADING, MOD_H1));
            }
            continue;
        }

        // Track covered byte ranges to avoid italic matching inside bold
        let mut covered = vec![false; line.len()];

        // Bold: **content**
        for cap in bold_re.captures_iter(line) {
            let full = cap.get(0).unwrap();
            let content = cap.get(1).unwrap();
            for i in full.start()..full.end() {
                covered[i] = true;
            }
            raw.push((ln, full.start() as u32, 2, TYPE_PUNCTUATION, 0));
            raw.push((ln, content.start() as u32, content.len() as u32, TYPE_MARKUP, MOD_BOLD));
            raw.push((ln, content.end() as u32, 2, TYPE_PUNCTUATION, 0));
        }

        // Italic: *content* — skip ranges already covered by bold
        for cap in italic_re.captures_iter(line) {
            let full = cap.get(0).unwrap();
            if covered[full.start()] {
                continue;
            }
            let content = cap.get(1).unwrap();
            raw.push((ln, full.start() as u32, 1, TYPE_PUNCTUATION, 0));
            raw.push((ln, content.start() as u32, content.len() as u32, TYPE_MARKUP, MOD_ITALIC));
            raw.push((ln, content.end() as u32, 1, TYPE_PUNCTUATION, 0));
        }
    }

    // Sort by line then by start position
    raw.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    // Encode in LSP delta format (positions are relative to the previous token)
    let mut tokens = Vec::with_capacity(raw.len());
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;

    for (line, start, length, token_type, modifiers) in raw {
        let delta_line = line - prev_line;
        let delta_start = if delta_line == 0 { start - prev_start } else { start };
        tokens.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset: modifiers,
        });
        prev_line = line;
        prev_start = start;
    }

    tokens
}
