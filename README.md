# noted

A Zed IDE extension that turns the editor into an Obsidian-like environment for Markdown — wikilinks, callouts, tags, smart navigation, and live preview.

> **Status:** Phase 1 complete (LSP core). Phase 2 in progress — semantic tokens, delta, and code actions done. 104 tests passing. Next: companion theme, rename, workspace symbols.

---

## Features

| Feature | Status | Phase |
|---|---|---|
| Wikilink completion (`[[`) | ✅ Working | 1 |
| Go-to-definition on wikilinks | ✅ Working | 1 |
| Broken link diagnostics | ✅ Working | 1 |
| Hover preview (title + snippet + tags) | ✅ Working | 1 |
| Document symbols / outline | ✅ Working | 1 |
| Vault indexer (scan on open, reindex on save) | ✅ Working | 1 |
| Semantic tokens (H1–H6, bold, italic, strikethrough, code, wikilink, tag, callout, math, frontmatter) | ✅ Working | 2 |
| Semantic tokens delta (incremental updates) | ✅ Working | 2 |
| Inlay hints (checkbox ✓/○) | ✅ Working | 2 |
| Code actions (toggle checkbox, wrap bold/italic/strike/code/wikilink, heading level, insert callout/table, change callout type) | ✅ Working | 2 |
| Companion theme (Verdant Garden dark + light) | 📋 Planned | 2 |
| Rename (wikilink refactoring across files) | 📋 Planned | 2 |
| Workspace symbols (search headings across vault) | 📋 Planned | 2 |
| Tree-sitter grammar (wikilinks, callouts, tags, embeds, checkboxes) | ✅ Working | 1 |
| Test vault (integration fixtures) | 📋 Planned | 1 |
| Live preview (browser + WebSocket sync) | 📋 Planned | 3 |

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
- **Code actions** (`Ctrl+.`) — toggle checkboxes, wrap selection in bold/italic/strikethrough/code/wikilink, change heading level, insert callouts/tables, change callout type

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

Semantic tokens are styled through the active Zed **theme** (`syntax` section). The LSP
maps Markdown elements to standard LSP token types:

| Markdown element | LSP token type | Theme `syntax` key |
|---|---|---|
| Headings (H1–H6) | `keyword` | `keyword` |
| Bold, italic, wikilinks, tags, etc. | `variable` | `variable` |
| Frontmatter | `comment` | `comment` |
| Punctuation (`#`, `**`, `[[`, `]]`) | `operator` | `operator` |

To enable semantic tokens, add this to your Zed `settings.json`:

```json
"languages": {
  "Noted Markdown": {
    "semantic_tokens": "combined"
  }
}
```

The companion `noted-theme` extension (Verdant Garden) is optimized for these token
mappings but any Zed theme will work.

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
│   ├── highlights.scm                # Tree-sitter highlight queries
│   ├── injections.scm                # Fenced code block language injection
│   ├── outline.scm                   # Outline panel node queries
│   └── folds.scm                     # Code folding queries
├── grammars/noted/ # Custom Tree-sitter grammar
│   ├── grammar.js                    # Grammar rules (wikilinks, callouts, tags, …)
│   ├── src/                          # Generated C parser
│   └── test/corpus/                  # Grammar test cases
└── crates/noted-lsp/                 # LSP server (native binary)
    └── src/
        ├── main.rs                   # Server state, handler dispatch
        ├── code_actions.rs           # Code actions (checkbox, wrap, heading, callout)
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
# Run all tests (104 tests)
cargo test -p noted-lsp

# Build release binary
cargo build --release -p noted-lsp

# Lint
cargo clippy -p noted-lsp -- -D warnings

# Check extension WASM target compiles
cargo check -p noted --target wasm32-wasip2
```

### Tree-sitter grammar

The grammar (`grammar.js`) covers wikilinks, embeds, tags, callouts, checkboxes, headings, and fenced code blocks. The generated C parser is committed in `grammars/noted/src/`.

After modifying `grammar.js`:

```bash
cd grammars/noted
tree-sitter generate
tree-sitter test    # 24 corpus tests
```

Then commit the updated `src/parser.c` and `src/tree_sitter/parser.h`.

---

## Contributing

See [CLAUDE.md](./CLAUDE.md) for architecture details and coding conventions.
Full implementation plan: [zed-md-wysiwyg-spec.md](./zed-md-wysiwyg-spec.md).
