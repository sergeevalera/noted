# noted

A Zed IDE extension that turns the editor into an Obsidian-like environment for Markdown — wikilinks, callouts, tags, smart navigation, and live preview.

> **Status:** Phase 0 complete. LSP hello world with semantic tokens and inlay hints working. Phase 1 in progress (Tree-sitter grammar).

---

## Trying the Plugin (Dev Mode)

### Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasip2

# Tree-sitter CLI (for grammar work)
cargo install tree-sitter-cli
```

### 1. Build the LSP binary

```bash
cargo build -p noted-lsp
```

The binary will be at `target/debug/noted-lsp`.

### 2. Set the LSP path

The extension locates the binary via an environment variable. Add this to your shell profile (`.zshrc` / `.bashrc`) or set it before launching Zed:

```bash
export NOTED_LSP_PATH=/absolute/path/to/noted/target/debug/noted-lsp
```

Replace `/absolute/path/to/noted` with the actual path to this repo.

### 3. Install as a dev extension in Zed

1. Open Zed
2. Open the Command Palette (`Cmd+Shift+P`)
3. Run `zed: install dev extension`
4. Select this repository folder

The extension will appear in **Extensions** with a "Dev Extension" badge.

### 4. Open a Markdown file

Open any `.md` file. You should see:

- An information diagnostic on line 1: _"noted-lsp is connected and working"_
- Hover over any word → popup: _"Hello from noted LSP!"_
- Checkboxes `- [x]` and `- [ ]` get `✓` / `○` inlay hints

### 5. Enable semantic token styling (optional)

Add to your Zed `settings.json` to see heading and formatting highlights:

```json
{
  "global_lsp_settings": {
    "noted-lsp": {
      "semantic_token_rules": [
        { "token_type": "heading", "token_modifiers": ["h1"],
          "foreground_color": "#E8C56D", "font_weight": 800 },
        { "token_type": "heading", "token_modifiers": ["h2"],
          "foreground_color": "#D4A94E", "font_weight": 700 },
        { "token_type": "heading", "token_modifiers": ["h3"],
          "foreground_color": "#BF9040", "font_weight": 600 },
        { "token_type": "markup", "token_modifiers": ["bold"],
          "font_weight": 700 },
        { "token_type": "markup", "token_modifiers": ["italic"],
          "font_style": "italic" },
        { "token_type": "punctuation",
          "foreground_color": "#444444" }
      ]
    }
  }
}
```

### 6. Rebuilding after code changes

```bash
cargo build -p noted-lsp
```

Then in Zed: Command Palette → `zed: restart language server` (or close and reopen a `.md` file).

---

## Development

### Project structure

```
noted/
├── src/lib.rs                        # Zed extension (WASM) — registers language, launches LSP
├── extension.toml                    # Extension manifest
├── Cargo.toml                        # Workspace root + extension crate
├── crates/noted-lsp/                 # LSP server (native binary)
│   └── src/main.rs                   # tower-lsp server: hover, diagnostics, semantic tokens, inlay hints
└── grammars/tree-sitter-obsidian-md/ # Custom Tree-sitter grammar (WIP)
    ├── grammar.js
    └── test/corpus/
```

### Build & check

```bash
# Check both crates compile
cargo check

# Build the LSP binary
cargo build -p noted-lsp

# Run tests
cargo test -p noted-lsp

# Lint
cargo clippy -p noted-lsp -- -D warnings

# Check extension WASM target compiles
cargo check -p noted --target wasm32-wasip2
```

### Tree-sitter grammar

```bash
cd grammars/tree-sitter-obsidian-md

# Generate C parser from grammar.js (requires tree-sitter-cli)
tree-sitter generate

# Run corpus tests
tree-sitter test

# Parse a sample file
tree-sitter parse path/to/file.md
```

### What works now

| Feature | Status |
|---|---|
| LSP connects to Zed | Working |
| Info diagnostic on open/change | Working |
| Hover (placeholder) | Working |
| Semantic tokens: H1/H2/H3, bold, italic | Working |
| Inlay hints: checkbox status | Working |
| Tree-sitter grammar | WIP (grammar.js written, needs `tree-sitter generate`) |
| Wikilink completion | Planned (Phase 1) |
| Go-to-definition | Planned (Phase 1) |
| Broken link diagnostics | Planned (Phase 1) |
| Live preview | Planned (Phase 3) |

---

## Contributing

See [CLAUDE.md](./CLAUDE.md) for architecture details and the full implementation plan in [zed-md-wysiwyg-spec.md](./zed-md-wysiwyg-spec.md).
