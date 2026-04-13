# noted

A language server for Obsidian-flavored Markdown in Zed. Handles wikilink resolution, diagnostics, hover previews, rename refactoring, tag navigation, and live HTML preview via a local server.

For the best experience, enable semantic tokens and install the companion theme — see [Setup](#setup).

---

## Features

| Feature | Description |
|---|---|
| **Wikilink completion** | Type `[[` to autocomplete note names from your vault |
| **Go-to-definition** | Cmd+click on `[[wikilink]]` to jump to the target file |
| **Broken link diagnostics** | Unresolved wikilinks are underlined as errors |
| **Hover preview** | Hover over a wikilink to see title, snippet, tags, and backlink count |
| **Document symbols** | `Cmd+Shift+O` — hierarchical heading tree |
| **Workspace symbols** | `Cmd+T` — search headings across all vault files |
| **Rename** | F2 on a wikilink — renames the target across all files |
| **Code actions** | `Cmd+.` — toggle checkboxes, wrap in bold/italic/code/wikilink, change heading level, insert callouts/tables |
| **Semantic tokens** | Headings, bold, italic, strikethrough, wikilinks, tags, callouts, math, frontmatter |
| **Inlay hints** | `[x]` shows `✓`, `[ ]` shows `○` |
| **Live preview** | `Cmd+.` → "Open Preview" — browser preview with live WebSocket sync |
| **Vault indexer** | Scans on open, reindexes on save |
| **Tree-sitter grammar** | Wikilinks, callouts, tags, embeds, checkboxes |

---

## Installation

### From Zed Extensions (when published)

1. Open Zed → Extensions → search "Noted" → Install
2. The LSP binary downloads automatically on first use

### Dev Mode

```bash
# Build the LSP binary
cargo build --release -p noted-lsp

# Set the path (add to ~/.zshrc or ~/.bashrc)
export NOTED_LSP_PATH=/path/to/noted/target/release/noted-lsp

# Install as dev extension in Zed
# Cmd+Shift+P → "zed: install dev extension" → select this folder
```

---

## Setup

### Recommended settings

Add to your Zed `settings.json`:

```json
"languages": {
  "Noted Markdown": {
    "semantic_tokens": "combined",
    "soft_wrap": "editor_width"
  }
}
```

- **`semantic_tokens`** — enables per-element styling (headings, bold, italic, wikilinks, tags, etc.). Each element gets a distinct color and weight from the active theme.
- **`soft_wrap`** — wraps long lines at the editor width. Essential for comfortable prose editing. Use `"preferred_line_length"` if you prefer wrapping at a fixed column.

**Font tip:** Noted uses `font_weight` to distinguish heading levels and bold text. Avoid setting `buffer_font_weight` too high (e.g., 700+) — it compresses the visual range between normal text and headings. A weight of 400–500 works best.

### Companion theme

> **Full per-element styling requires a theme that defines Noted's custom syntax keys.**
> With a standard Zed theme, the extension still works (wikilinks, diagnostics, code actions, preview, etc.), but visual differentiation is limited — all headings share one color, all markup shares another.

For the complete experience install [Noted Theme](https://github.com/sergeevalera/noted-theme) — available in the Zed Extensions panel (search "Noted Theme"). Includes **Noted Verdant Garden Dark** and **Noted Verdant Garden Light** variants. It defines all custom syntax keys required by this extension: per-level heading colors and weights, distinct styles for bold, italic, wikilinks, broken links, tags, callouts, math, frontmatter, and muted MD punctuation.

**Building your own theme?** The full list of syntax keys the extension uses is in [`languages/noted/semantic_token_rules.json`](languages/noted/semantic_token_rules.json). The semantic specification (what each token represents and its intended styling) is documented in [`zed-md-wysiwyg-spec.md`](zed-md-wysiwyg-spec.md), section *Strategy B — B.7*.

---

## Usage

Open any directory with `.md` files as your workspace. The vault is indexed on startup.

### Wikilinks

- Type `[[` to get completions from the vault index
- Hover over `[[note]]` for a preview (title + first paragraph + tags + backlinks)
- Cmd+click to jump to the target file
- F2 to rename — updates all references across the vault

### Code actions (`Cmd+.`)

- Toggle checkboxes (`[x]` ↔ `[ ]`)
- Wrap selection in **bold**, *italic*, ~~strikethrough~~, `code`, `[[wikilink]]`
- Change heading level
- Insert callout or table
- Change callout type
- Open live preview

### Live preview

1. `Cmd+.` on any line → "Open Preview"
2. Copy the URL from the notification
3. Open in browser — updates live as you type

### Outline

- `Cmd+Shift+O` — document symbols (heading tree)
- `Cmd+T` — workspace symbols (search headings across all files)

---

## Development

### Project structure

```
noted/
├── src/lib.rs                        # Zed extension (WASM) — locates/downloads LSP
├── extension.toml                    # Extension manifest
├── languages/noted/                  # Language definition for Zed
│   ├── config.toml                   # File types, bracket pairs
│   ├── highlights.scm                # Tree-sitter highlight queries
│   ├── injections.scm                # Fenced code block language injection
│   ├── outline.scm                   # Outline panel queries
│   └── folds.scm                     # Code folding queries
├── grammars/noted/                   # Custom Tree-sitter grammar
│   ├── grammar.js                    # Grammar rules
│   ├── src/                          # Generated C parser
│   └── test/corpus/                  # Grammar test cases (24 tests)
└── crates/noted-lsp/                 # LSP server (native binary)
    └── src/
        ├── main.rs                   # Server state, handler dispatch
        ├── code_actions.rs           # Code actions
        ├── completion.rs             # Wikilink completion
        ├── definition.rs             # Go-to-definition
        ├── diagnostics.rs            # Broken link diagnostics
        ├── hover.rs                  # Note preview hover
        ├── inlay_hints.rs            # Checkbox inlay hints
        ├── preview.rs                # HTTP + WebSocket preview server
        ├── render.rs                 # MD → HTML renderer
        ├── rename.rs                 # Rename wikilinks across files
        ├── semantic_tokens.rs        # Semantic token encoding
        ├── symbols.rs                # Document symbols
        ├── workspace_symbols.rs      # Workspace symbols
        └── vault/                    # Vault index
            ├── index.rs              # VaultIndex, resolve_wikilink
            ├── parser.rs             # Note parsing
            └── scanner.rs            # Vault directory scanning
```

### Build & test

```bash
cargo test -p noted-lsp              # 142 unit tests
cargo build --release -p noted-lsp   # Build release binary
cargo clippy -p noted-lsp -- -D warnings
cargo check -p noted --target wasm32-wasip2  # Check WASM extension
```

### Tree-sitter grammar

After modifying `grammar.js`:

```bash
cd grammars/noted
tree-sitter generate
tree-sitter test    # 24 corpus tests
```

Commit the updated `src/parser.c` and `src/tree_sitter/parser.h`.

---

## License

[MIT](./LICENSE)

---

## Contributing

See [CLAUDE.md](./CLAUDE.md) for architecture details and coding conventions.
