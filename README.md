# noted

A Zed IDE extension that turns the editor into an Obsidian-like environment for Markdown — wikilinks, callouts, tags, smart navigation, and live preview.

> **Status:** Phase 1 complete. Phase 2 in progress — semantic tokens (full + delta) done. 90 tests passing.

---

## Features

| Feature | Status |
|---|---|
| Wikilink completion (`[[`) | ✅ Working |
| Go-to-definition on wikilinks | ✅ Working |
| Broken link diagnostics | ✅ Working |
| Hover preview (title + snippet + tags) | ✅ Working |
| Document symbols / outline | ✅ Working |
| Semantic tokens (H1–H6, bold, italic, strikethrough, code, wikilink, tag, callout, math, frontmatter) | ✅ Working |
| Semantic tokens delta (incremental updates) | ✅ Working |
| Inlay hints (checkbox ✓/○) | ✅ Working |
| Vault indexer (scan on open, reindex on save) | ✅ Working |
| Tree-sitter grammar | 🚧 Written, needs `tree-sitter generate` |
| Code actions | 📋 Planned (Phase 2) |
| Rename / refactor | 📋 Planned (Phase 2) |
| Live preview | 📋 Planned (Phase 3) |

---

## Trying the Plugin (Dev Mode)

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasip2
```

### 1. Build the LSP binary

```bash
cargo build --release -p noted-lsp
```

The binary will be at `target/release/noted-lsp`.

### 2. Set the LSP path

The extension finds the binary via an environment variable. Add to your shell profile (`.zshrc` / `.bashrc`):

```bash
export NOTED_LSP_PATH=/absolute/path/to/noted/target/release/noted-lsp
```

Replace `/absolute/path/to/noted` with the actual path to this repo. Then reload your shell or open a new terminal.

### 3. Install as a dev extension in Zed

1. Open Zed
2. Open the Command Palette (`Cmd+Shift+P`)
3. Run `zed: install dev extension`
4. Select this repository folder

The extension will appear in **Extensions** with a "Dev Extension" badge.

### 4. Open a Markdown vault

Open any directory that contains `.md` files as your workspace. You should see:

- **Wikilinks** `[[note-name]]` — type `[[` to get completions, hover for a preview, Cmd+click to jump
- **Broken links** underlined in red (after the vault finishes indexing)
- **Outline panel** (`Cmd+Shift+O`) — hierarchical heading tree
- **Inlay hints** — `- [x]` shows `✓`, `- [ ]` shows `○` after the bracket
- **Semantic tokens** — headings (H1–H6), bold, italic, strikethrough, wikilinks, tags, callouts, math, frontmatter (requires companion theme)

The vault is indexed on startup. Open the Zed log (`Cmd+Shift+P` → "Open Log") and look for:

```
noted-lsp started
Vault indexed: N notes in X.Xms
```

### 5. Rebuilding after code changes

```bash
cargo build --release -p noted-lsp
```

Then in Zed: **Cmd+Shift+P** → `zed: restart language server`.

---

## Semantic Token Styling

Semantic token visual styling is controlled by the **theme**, not `settings.json`.
To see heading and formatting colours, install the companion `noted-theme` extension
(Phase 2 — not yet published) or add `semantic_token_rules` to your custom Zed theme file.

Example rules for the Verdant Garden dark theme:

```json
"semantic_token_rules": [
  { "selector": "heading.h1", "style": { "color": "#8FBF6A", "font_weight": 800 } },
  { "selector": "heading.h2", "style": { "color": "#7AAD58", "font_weight": 700 } },
  { "selector": "heading.h3", "style": { "color": "#669B48", "font_weight": 600 } },
  { "selector": "heading.h4", "style": { "color": "#548938", "font_weight": 500 } },
  { "selector": "heading.h5", "style": { "color": "#447830", "font_weight": 500 } },
  { "selector": "heading.h6", "style": { "color": "#3A6828", "font_weight": 400 } },
  { "selector": "markup.bold", "style": { "font_weight": 700 } },
  { "selector": "markup.italic", "style": { "font_style": "italic" } },
  { "selector": "markup.strikethrough", "style": { "color": "#7A7C72" } },
  { "selector": "markup.code", "style": { "color": "#E0B460" } },
  { "selector": "markup.wikilink", "style": { "color": "#7CB5C4" } },
  { "selector": "markup.wikilink.broken", "style": { "color": "#CC4444" } },
  { "selector": "markup.tag", "style": { "color": "#D4A56A" } },
  { "selector": "markup.callout", "style": { "color": "#C47D8A", "font_style": "italic" } },
  { "selector": "markup.math", "style": { "color": "#B8DC94", "font_style": "italic" } },
  { "selector": "markup.checkbox_done", "style": { "color": "#7A7C72" } },
  { "selector": "comment.frontmatter", "style": { "color": "#545648", "font_style": "italic" } },
  { "selector": "punctuation.markup_punctuation", "style": { "color": "#4A4A40" } }
]
```

---

## Development

### Project structure

```
noted/
├── src/lib.rs                        # Zed extension (WASM) — locates and launches LSP
├── extension.toml                    # Extension manifest
├── Cargo.toml                        # Workspace root + extension crate
├── languages/noted/            # Language definition for Zed
│   ├── config.toml                   # File types, bracket pairs
│   ├── highlights.scm                # Tree-sitter highlight queries (inactive until grammar compiled)
│   ├── injections.scm                # Fenced code block language injection
│   ├── outline.scm                   # Outline panel node queries
│   └── folds.scm                     # Code folding queries
├── grammars/tree-sitter-noted/ # Custom Tree-sitter grammar
│   ├── grammar.js                    # Grammar rules (wikilinks, callouts, tags, …)
│   ├── src/                          # MISSING — run: tree-sitter generate
│   └── test/corpus/                  # Grammar test cases
└── crates/noted-lsp/                 # LSP server (native binary)
    └── src/
        ├── main.rs                   # Server state, handler dispatch
        ├── completion.rs             # Wikilink completion
        ├── definition.rs             # Go-to-definition
        ├── diagnostics.rs            # Broken link diagnostics
        ├── hover.rs                  # Note preview hover
        ├── inlay_hints.rs            # Checkbox inlay hints
        ├── semantic_tokens.rs        # Semantic token encoding
        ├── symbols.rs                # Document symbol / outline
        └── vault/                    # Vault index
            ├── index.rs              # VaultIndex, resolve_wikilink
            ├── parser.rs             # Note parsing (headings, links, tags)
            └── scanner.rs            # Vault directory scanning
```

### Build & check

```bash
# Run all tests (90 tests)
cargo test -p noted-lsp

# Build release binary
cargo build --release -p noted-lsp

# Lint
cargo clippy -p noted-lsp -- -D warnings

# Check extension WASM target compiles
cargo check -p noted --target wasm32-wasip2
```

### Tree-sitter grammar

The grammar is written (`grammar.js`) but the generated C parser is missing.
To activate it:

```bash
# Install tree-sitter CLI
cargo install tree-sitter-cli
# or: npm install -g tree-sitter-cli

cd grammars/tree-sitter-noted

# Generate C parser
tree-sitter generate

# Run corpus tests
tree-sitter test
```

After generating:
1. Restore `grammar = "noted"` in `languages/noted/config.toml`
2. Restore the `[grammars.noted]` block in `extension.toml`
3. Commit the generated `src/parser.c` and `src/tree_sitter/parser.h`

---

## Contributing

See [CLAUDE.md](./CLAUDE.md) for architecture details and coding conventions.
Full implementation plan: [zed-md-wysiwyg-spec.md](./zed-md-wysiwyg-spec.md).
