# Zed Markdown WYSIWYG Extension — Project Specification

## 1. Product Vision

**Goal:** Create a Zed IDE extension that turns the editor into an Obsidian-like environment for working with Markdown — an inline WYSIWYG mode (like Typora), where the user sees rendered results directly while typing.

**Name:** `noted`

**Target audience:** Zed users who take notes/write documentation in Markdown and want an experience close to Obsidian without switching to a separate application.

---

## 2. Critical Limitation: Zed Extension API State

> **Warning: This is the main project challenge that must be resolved BEFORE starting development.**

### 2.1. What the API Can Do Now (April 2026)

Zed Extension API (v0.8.0) supports:
- Language support (Tree-sitter grammars, syntax highlighting)
- Language Server Protocol (LSP) adapters
- Themes and icons
- Slash commands
- Snippets
- MCP servers
- Debug adapters
- Process launching and file downloading

### 2.2. What the API CANNOT Do

- **No custom panels/UI** — extensions cannot create their own panels, widgets, or modify editor rendering
- **No custom editor** — cannot replace the standard text editor with a custom one (unlike VS Code Custom Editors)
- **No rich text rendering** — extensions cannot control text display (fonts, sizes, inline images)
- **No webview** — Zed fundamentally does not use webviews in extensions (unlike VS Code)
- **No GPUI access** — extensions run in a WASM sandbox and have no access to Zed's native UI framework

The Zed team is aware of requests for custom rendering (Discussion #37270), but it is not implemented and not announced in the roadmap.

### 2.3. Conclusion

**Full inline WYSIWYG (like Typora) is impossible through the current Zed Extension API.** This is not a matter of implementation complexity — the API simply does not provide primitives for custom rendering.

---

## 3. Implementation Strategies (from realistic to ambitious)

### Strategy A: "Enhanced Markdown Experience" via Extension API — Feasible now

The maximum achievable within the current API without touching the Zed core:

| Component | Implementation |
|---|---|
| **Enhanced MD highlighting** | Tree-sitter grammar with extended Obsidian-flavored MD (wikilinks, callouts, math) |
| **LSP for Markdown** | Language Server with wikilink autocompletion (`[[`), note navigation, go-to-definition for links |
| **Slash commands** | `/note`, `/link`, `/callout` — quick template insertion |
| **Diagnostics** | Broken link detection, missing files, duplicate headings |
| **Outline** | Uses Zed's built-in outline panel (already works with MD headings) |
| **Preview** | Use Zed's built-in Markdown Preview (already supports mermaid since March 2026) |

**Pros:** Can start right away, publishes as a standard extension.
**Cons:** No inline WYSIWYG — only split view (editor + preview).

### Strategy B: "LSP-powered Pseudo-WYSIWYG" — Partially feasible

Built on top of Strategy A. Uses LSP and Tree-sitter mechanisms to create
the closest possible WYSIWYG-like visual experience within a text editor.

#### B.1. Semantic Tokens — visual styles for Markdown elements

The LSP returns semantic tokens — markup like "this text range is H1",
"this is bold", "this is a wikilink". Zed applies styles from the theme.

**Available styling properties in Zed (confirmed):**
- `foreground_color` — text color (hex)
- `background_color` — background color (hex)
- `font_weight` — weight (100–900 or `"bold"`)
- `font_style` — `"italic"`, `"oblique"`, `"normal"`
- `underline` — underline (bool or hex color)
- `strikethrough` — strikethrough (bool or hex color)

**Not available:** font_size, font_family, letter-spacing, line-height.
All elements render at the same monospace font size.

**Custom semantic token types returned by the LSP:**

| Token Type | Token Modifier | Purpose | Styling |
|---|---|---|---|
| `heading` | `h1` | Heading H1 | Bold 800 + bright color |
| `heading` | `h2` | Heading H2 | Bold 700 + slightly darker |
| `heading` | `h3` | Heading H3 | Bold 600 + even darker |
| `heading` | `h4`, `h5`, `h6` | Headings H4–H6 | Semibold 500 + muted tones |
| `markup` | `bold` | Bold text `**text**` | font_weight: 700 |
| `markup` | `italic` | Italic `*text*` | font_style: italic |
| `markup` | `strikethrough` | Strikethrough `~~text~~` | strikethrough: true |
| `markup` | `code` | Inline code `` `text` `` | background_color + mono color |
| `string` | `link` | URL links | underline + link color |
| `markup` | `wikilink` | Wikilinks `[[page]]` | underline + accent color |
| `markup` | `wikilink.broken` | Broken wikilinks | underline red + strikethrough |
| `markup` | `tag` | Tags `#tag` | background color (badge effect) |
| `markup` | `callout` | Callout marker `> [!note]` | italic + accent color |
| `markup` | `checkbox.done` | `- [x]` completed | strikethrough + muted color |
| `markup` | `checkbox.todo` | `- [ ]` uncompleted | normal style |
| `markup` | `math` | Math `$...$` | italic + special color |
| `comment` | `frontmatter` | YAML frontmatter | muted color, italic |
| `punctuation` | `markup` | MD syntax (`#`, `**`, `>`) | heavily muted color |

**Key technique:** syntax characters (`#`, `**`, `[[`, `]]`, `>`) are marked
as `punctuation.markup` with a heavily muted color — they are not hidden, but
visually "recede into the background", letting the content dominate.

#### B.2. Inlay Hints — virtual insertions

Inlay hints are "virtual text" visible in the editor but absent from the file.

| Context | Inlay Hint |
|---|---|
| `- [x] Task` | `✓` after checkbox |
| `- [ ] Task` | `○` after checkbox |
| `![[image.png]]` | `(exists, 450×300)` — confirmation and size |
| `![[missing.png]]` | `⚠ not found` — file not found |
| `[[My Note]]` | `(23 links)` — backlink count |
| `date: 2026-04-03` | `(Thursday)` — day of week in frontmatter |

**Limitation:** inlay hints are small gray text, not icons or rich content.

#### B.3. Code Folding — collapsing blocks

Foldable blocks are defined through Tree-sitter queries (`folds.scm`).
A folded block is replaced with `…`.

| What folds | Result |
|---|---|
| Section content by heading | `## Architecture …` (H2 + everything until next H2) |
| Callout body | `> [!note] Title …` |
| Long code block | `` ```python … `` |
| Frontmatter | `--- …` |
| Long table (>5 rows) | First row + `…` |

**Limitation:** fold is all-or-nothing. You cannot hide only the `**` around bold text
while keeping the text itself. Fold hides a contiguous block entirely.

#### B.4. Code Actions — quick semantic actions

LSP code actions (via `Ctrl+.` or lightbulb) — context actions that modify text.

| Cursor context | Action | Result |
|---|---|---|
| `- [ ] task` | Toggle Checkbox | `- [x] task` |
| `- [x] task` | Toggle Checkbox | `- [ ] task` |
| Selected text | Wrap in Bold | `**selected text**` |
| Selected text | Wrap in Italic | `*selected text*` |
| Selected text | Wrap in Strikethrough | `~~selected text~~` |
| Selected text | Wrap in Code | `` `selected text` `` |
| Selected text | Wrap in Wikilink | `[[selected text]]` |
| Empty line | Insert Table 3x3 | MD table with header |
| Empty line | Insert Callout | `> [!note]\n> ` |
| `> [!note]` | Change Callout Type | Choose: warning, tip, danger, etc. |
| `## Heading` | Increase Heading Level | `### Heading` |
| `## Heading` | Decrease Heading Level | `# Heading` |

#### B.5. Rename — link refactoring

LSP rename (`F2`): renaming a note file → automatically updates
all `[[old name]]` across all workspace files to `[[new name]]`.

#### B.6. Workspace Symbols — vault search

`Ctrl+T` / workspace symbols: search across headings of all MD files in the project.
User types part of a heading → LSP returns matching results
with file and position → Zed opens the file at the right line.

#### Summary: what B adds on top of A

**You see in the editor:** headings visually stand out with color and weight,
bold is actually bold, italic is actually italic, strikethrough text is struck through,
wikilinks are underlined like links, broken links are red, MD syntax (`#`, `**`, `>`)
is muted and "recedes into the background", status symbols appear next to checkboxes.

**You work through:** quick actions (Ctrl+.) instead of manually typing syntax,
F2 for renaming links, fold for hiding long blocks.

**What's still missing:** different font sizes for headings, inline images,
fully hiding individual syntax characters, visual callout blocks
with backgrounds and icons, rendering tables as grids, rendering formulas.

**Pros:** Noticeably closer to WYSIWYG experience without custom rendering, publishes as a standard extension.
**Cons:** Limited visual control, no inline images, no proportional fonts, no font size differences.

### Strategy C: "BAML Trick — LSP + localhost server" — Experimental

Inspired by the BAML extension for Zed, which embeds a web interface via LSP:

1. LSP server launches a localhost HTTP server
2. Renders full Markdown preview in the browser
3. Cursor and scroll sync between Zed and browser via LSP
4. Bidirectional live editing: changes in Zed → preview updates, click in preview → navigation in Zed

**Pros:** Full rich rendering, can implement wikilinks, mermaid, math, callouts.
**Cons:** Dependency on external browser, not true inline WYSIWYG, more complex architecture.

### Strategy D: "Fork Zed / Contribute to Core" — Maximum ambition

Implement WYSIWYG directly in the Zed core via GPUI:

1. Forked Zed or PR to upstream
2. New `MarkdownEditor` view using GPUI directly
3. Rich text rendering via GPUI primitives (text shaping, inline images, block elements)
4. Integration with existing Buffer/MultiBuffer to maintain compatibility

**Pros:** True inline WYSIWYG, native performance.
**Cons:** Huge amount of work, requires deep knowledge of GPUI and Zed internals, risk of merge conflicts with upstream.

---

## 4. Recommended Plan: phased approach

### Phase 0: Research and Prototyping (2–3 weeks)

**In parallel: basic Rust learning**
- [ ] Install Rust via `rustup`, set up Zed for Rust development
- [ ] Complete chapters 1–6 of The Rust Book (variables, ownership, structs, enums, pattern matching)
- [ ] Write a simple CLI utility in Rust (to get used to cargo, the compiler, borrow checker)

**Zed Extension API research:**
- [ ] Create a hello world dev extension (Rust → WASM → Zed)
- [ ] Study WASM extension lifecycle: how Zed compiles and loads extensions
- [ ] Test: semantic tokens from LSP → how Zed styles them (font_weight, italic, underline)
- [ ] Test: inlay hints from LSP → how they display in Zed
- [ ] Study the existing Markdown Tree-sitter grammar in Zed

**Prototypes:**
- [ ] Minimal LSP server on `tower-lsp`: hello world diagnostic + one semantic token type
- [ ] Connect LSP to Zed via dev extension, confirm the pipeline works
- [ ] **Make strategy decision (A+B, B+C, or D) based on test results**

### Phase 1: MVP — Obsidian-like Markdown LSP (4–6 weeks)

**Strategy A + start of B: extension + LSP**

> Timelines increased to account for parallel Rust learning.
> Claude Agent generates the main code, author reviews and tests.

Components:
1. **Tree-sitter grammar** for Obsidian-flavored Markdown
   - Wikilinks: `[[page]]`, `[[page|alias]]`
   - Callouts: `> [!note]`, `> [!warning]`
   - Tags: `#tag`
   - Embeds: `![[image.png]]`

2. **Language Server** (Rust, `tower-lsp` + `tokio`)
   - Vault indexer: workspace scanning, building file/link/heading index
   - Wikilink autocompletion by files in workspace
   - Go-to-definition for `[[links]]`
   - Hover preview for links
   - Diagnostics: broken links, missing files
   - Document symbols for outline (headings → nested structure)
   - Semantic tokens for visual differentiation of MD elements

3. **Slash commands** (via Extension API)
   - `/callout [type]` — insert callout block
   - `/table [rows] [cols]` — insert table
   - `/link` — interactive search and wikilink insertion

### Phase 2: Enhanced Visual Experience (2–3 weeks)

**Strategy B: maximizing LSP capabilities**

- Inlay hints for checkbox state preview
- Code folding for hiding MD syntax
- Code actions: toggle bold/italic/strikethrough, toggle checkbox
- Rename support for wikilinks (file rename → update all links)
- Workspace symbols for full-text search across notes

### Phase 3: Rich Preview Integration (3–4 weeks)

**Strategy C: LSP + localhost preview**

- Built-in HTTP server in LSP process (`axum`)
- Rendering MD → HTML via `pulldown-cmark` + JS injections (KaTeX, mermaid.js via CDN)
- WebSocket cursor sync Zed <-> Browser (`tokio-tungstenite`)
- Reverse navigation: click on element in preview → jump to line in Zed
- Hot reload on file save (via `notify` file watcher)

### Phase 4 (optional): Zed Core Contribution

- Proposal to Zed GitHub: Custom Editor API for extensions
- GPUI-based Markdown editor prototype
- Rich text rendering via display map / block decorations

---

## 5. Technical Stack

> **Decision:** the entire project is in Rust. Code is generated primarily by Claude Agent
> (Claude Code / Zed Agent), the project author reviews and directs.
> Rust experience will be built up over the course of the project.

### 5.1. Extension (WASM module, loaded into Zed)

| Component | Technology | Version / Note |
|---|---|---|
| Language | Rust | edition 2021 |
| Target | `wasm32-wasip2` | compiled by Zed on install dev extension |
| API | `zed_extension_api` | latest on crates.io (>=0.5.0) |
| Crate type | `cdylib` | required by Zed for WASM extensions |

**Extension Cargo.toml:**
```toml
[package]
name = "noted"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.5"
```

Minimal `lib.rs` — register language + specify LSP binary.
All logic lives in the LSP, not in the extension.

### 5.2. Language Server (separate binary)

| Component | Crate | Purpose |
|---|---|---|
| LSP framework | `tower-lsp` | Async LSP server on tokio |
| Async runtime | `tokio` | Async I/O, file watching |
| MD parsing | `pulldown-cmark` | Fast CommonMark parser |
| Frontmatter | `serde_yaml` + `serde` | YAML frontmatter parsing |
| File walking | `walkdir` | Vault indexing |
| File watching | `notify` | File change tracking |
| Fuzzy search | `nucleo` or `fuzzy-matcher` | Fuzzy match for autocompletion |
| Regex | `regex` | Parsing wikilinks, tags, callouts |
| Path handling | `camino` | UTF-8 paths (more ergonomic than std PathBuf) |
| Serialization | `serde_json` | LSP JSON-RPC |
| Logging | `tracing` + `tracing-subscriber` | Structured logging |

**LSP Cargo.toml:**
```toml
[package]
name = "noted-lsp"
version = "0.1.0"
edition = "2021"

[dependencies]
tower-lsp = "0.20"
tokio = { version = "1", features = ["full"] }
pulldown-cmark = "0.12"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
walkdir = "2"
notify = "7"
regex = "1"
camino = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

### 5.3. Tree-sitter Grammar

| Component | Technology |
|---|---|
| Grammar definition | JavaScript (`grammar.js`) |
| Base fork | `tree-sitter-markdown` (MDeiml) |
| Queries | Scheme (`.scm` files) |
| Compilation | tree-sitter CLI → C → WASM (via Zed build pipeline) |

Grammar.js is the only non-Rust component. This is the Tree-sitter standard.

### 5.4. Preview Server (Phase 3)

| Component | Crate / Technology |
|---|---|
| HTTP server | `axum` (embedded in LSP process) |
| WebSocket | `tokio-tungstenite` |
| HTML rendering | `pulldown-cmark` → HTML + `<script>` injections |
| Math | KaTeX (JS, loaded via CDN in preview HTML) |
| Diagrams | mermaid.js (JS, CDN) |
| Frontend | Vanilla HTML/JS (minimal weight, no frameworks) |

### 5.5. Companion Theme

| Component | Technology |
|---|---|
| Format | JSON (Zed Theme Schema v0.2.0) |
| Files | `noted-dark.json`, `noted-light.json` |
| Code | Not required — pure JSON |

### 5.6. Development Tooling

| Tool | Purpose |
|---|---|
| `rustup` | Rust toolchain management (required for Zed extensions) |
| `cargo` | Build, tests, dependencies |
| `tree-sitter-cli` | Grammar generation and testing |
| Zed IDE | Dev extension install, testing, Zed Agent for code generation |
| Claude Code | Rust code generation, refactoring, test writing |

### 5.7. Development Model: "AI-assisted Rust"

Since there is no Rust experience yet, the following model is adopted:

**Claude Agent generates:**
- Boilerplate (Cargo.toml, project structure, trait implementations)
- LSP handlers (completion, hover, diagnostics, semantic tokens)
- Vault parsing and indexing (walkdir + pulldown-cmark)
- Tests (unit + integration)

**Project author:**
- Formulates requirements and reviews code
- Makes architectural decisions
- Tests in Zed (dev extension install)
- Gradually learns Rust by reading and modifying generated code

**Recommended resources for parallel Rust learning:**
- [The Rust Book](https://doc.rust-lang.org/book/) — chapters 1–10 cover 90% of what's needed
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — practical patterns
- Key concepts for this project: ownership/borrowing, traits, enums/pattern matching, async/await (tokio), serde serialization

---

## 6. Companion Theme: "Noted"

Strategy B critically depends on the theme — standard Zed themes are optimized for code,
not for prose. A companion theme (or light/dark pair) is needed that:
- Maximizes visual differentiation for Markdown elements
- Mutes MD syntax, letting content dominate
- Ships as a separate theme extension (Zed registry requirement)

### 6.1. Theme Design Principles

**Muted syntax:** characters `#`, `**`, `>`, `[[`, `]]`, `---` render
in nearly invisible color (alpha ~30% of main text). The user sees them
when focusing on the line, but they don't interfere when reading.

**Heading hierarchy through color + weight:** since font_size is unavailable,
the only way to create visual hierarchy H1–H6 is a combination of color
and font_weight. H1 = brightest + boldest, H6 = most muted + thinnest.

**Prose-first:** background and main text are optimized for extended reading,
not for code contrast. Soft tones, sufficient contrast, no bright accent colors
in body text.

### 6.2. Semantic Token Rules (recommended settings)

The LSP returns custom token types. The theme + `semantic_token_rules` define styling.
These rules are included in the extension documentation as recommended user settings.

```jsonc
{
  "global_lsp_settings": {
    "semantic_token_rules": [
      // === HEADINGS: hierarchy through color + weight (Verdant Garden green ramp) ===
      { "token_type": "heading", "token_modifiers": ["h1"],
        "foreground_color": "#8FBF6A", "font_weight": 800 },
      { "token_type": "heading", "token_modifiers": ["h2"],
        "foreground_color": "#7AAD58", "font_weight": 700 },
      { "token_type": "heading", "token_modifiers": ["h3"],
        "foreground_color": "#669B48", "font_weight": 600 },
      { "token_type": "heading", "token_modifiers": ["h4"],
        "foreground_color": "#548938", "font_weight": 500 },
      { "token_type": "heading", "token_modifiers": ["h5"],
        "foreground_color": "#447830", "font_weight": 500 },
      { "token_type": "heading", "token_modifiers": ["h6"],
        "foreground_color": "#3A6828", "font_weight": 400 },

      // === INLINE FORMATTING ===
      { "token_type": "markup", "token_modifiers": ["bold"],
        "font_weight": 700 },
      { "token_type": "markup", "token_modifiers": ["italic"],
        "font_style": "italic" },
      { "token_type": "markup", "token_modifiers": ["strikethrough"],
        "strikethrough": true, "foreground_color": "#7A7C72" },
      { "token_type": "markup", "token_modifiers": ["code"],
        "foreground_color": "#E0B460", "background_color": "#181816" },

      // === LINKS ===
      { "token_type": "markup", "token_modifiers": ["wikilink"],
        "foreground_color": "#7CB5C4", "underline": true },
      { "token_type": "markup", "token_modifiers": ["wikilink", "broken"],
        "foreground_color": "#CC4444", "underline": "#CC4444", "strikethrough": true },
      { "token_type": "string", "token_modifiers": ["link"],
        "foreground_color": "#A0D8D8", "underline": true },

      // === TAGS & CALLOUTS ===
      { "token_type": "markup", "token_modifiers": ["tag"],
        "foreground_color": "#D4A56A", "background_color": "#1E1A10" },
      { "token_type": "markup", "token_modifiers": ["callout"],
        "foreground_color": "#C47D8A", "font_style": "italic" },

      // === CHECKBOXES ===
      { "token_type": "markup", "token_modifiers": ["checkbox_done"],
        "foreground_color": "#7A7C72", "strikethrough": true },
      { "token_type": "markup", "token_modifiers": ["checkbox_todo"],
        "foreground_color": "#C8CAC0" },

      // === MATH ===
      { "token_type": "markup", "token_modifiers": ["math"],
        "foreground_color": "#B8DC94", "font_style": "italic" },

      // === FRONTMATTER ===
      { "token_type": "comment", "token_modifiers": ["frontmatter"],
        "foreground_color": "#545648", "font_style": "italic" },

      // === MUTED MD SYNTAX (# ** [[ ]] > --- ~~) ===
      { "token_type": "punctuation", "token_modifiers": ["markup_punctuation"],
        "foreground_color": "#4A4A40" }
    ]
  }
}
```

### 6.3. Syntax Theme Overrides (in the theme JSON file)

In addition to semantic token rules, the theme defines styles for Tree-sitter captures
(these work even when the LSP is not running — fallback).

```jsonc
{
  "syntax": {
    // Standard MD captures from tree-sitter-markdown (dark theme — Verdant Garden)
    "emphasis": { "font_style": "italic" },
    "emphasis.strong": { "font_weight": 700 },
    "title": { "font_weight": 800, "color": "#8FBF6A" },
    "link_text": { "color": "#A0D8D8" },
    "link_uri": { "color": "#7CB5C4", "font_style": "italic" },
    "punctuation.bracket": { "color": "#4A4A40" },
    "punctuation.delimiter": { "color": "#4A4A40" },
    "punctuation.special": { "color": "#4A4A40" },

    // Custom captures from noted grammar
    "markup.wikilink": { "color": "#7CB5C4" },
    "markup.tag": { "color": "#D4A56A" },
    "markup.callout": { "color": "#C47D8A", "font_style": "italic" },

    // Standard code captures (for code blocks — Verdant Garden code palette)
    "string": { "color": "#E0B460" },
    "keyword": { "color": "#72AAD0", "font_weight": 600 },
    "function": { "color": "#B8DC94" },
    "comment": { "color": "#545648", "font_style": "italic" },
    "type": { "color": "#A898C4" },
    "variable": { "color": "#C8CAC0" }
  }
}
```

### 6.4. Theme Variants

| Variant | Description |
|---|---|
| **Verdant Dark** | Near-black warm background (`#0A0A08`), green heading ramp, teal links |
| **Verdant Light** | Near-white warm background (`#FCFCFA`), dark green headings, paper-like feel |

Both variants maintain full code highlighting in code blocks — the theme does not
sacrifice code experience for prose. Code blocks use the standard syntax color set.

### 6.5. Recommended Fonts

The theme does not control the font (that's a user setting), but the documentation
recommends using monospace fonts with good italic and variable weight support:

| Font | Why |
|---|---|
| **JetBrains Mono** | Excellent italic, good weight variations, ligatures |
| **Cascadia Code** | Cursive italic variant, good for prose |
| **Fira Code** | Wide range of weights, good readability |
| **Iosevka** | Most configurable, has quasi-proportional variants |
| **Victor Mono** | Especially beautiful cursive italic — ideal for emphasis |

### 6.6. Publishing

The theme is published as a **separate extension** (Zed registry requirement —
themes cannot be part of an extension with language support).

```
noted-theme/
├── extension.toml          # id = "noted-theme"
├── LICENSE
└── themes/
    ├── verdant-dark.json   # Verdant Garden dark
    └── verdant-light.json  # Verdant Garden light
```

---

## 7. Project Structure

```
noted/
├── extension.toml              # Extension manifest
├── Cargo.toml                  # Rust WASM extension
├── src/
│   └── lib.rs                  # Extension entry point
├── languages/
│   └── noted/
│       ├── config.toml         # Language config
│       ├── highlights.scm      # Syntax highlighting queries
│       ├── injections.scm      # Code block injection queries
│       ├── outline.scm         # Outline/symbols queries
│       └── folds.scm           # Folding queries
├── grammars/
│   └── tree-sitter-noted/  # Custom Tree-sitter grammar
│       ├── grammar.js
│       └── src/
├── lsp/                        # Language Server (separate binary)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── indexer.rs           # Vault file indexer
│       ├── completion.rs        # Wikilink autocomplete
│       ├── diagnostics.rs       # Broken link detection
│       ├── semantic_tokens.rs   # Visual MD tokens
│       └── hover.rs             # Link preview on hover
└── preview/                    # Preview server (Phase 3)
    ├── static/
    │   ├── index.html
    │   └── style.css
    └── src/
        └── server.rs

noted-theme/                # Companion theme (separate extension)
├── extension.toml                 # id = "noted-theme"
├── LICENSE
└── themes/
    ├── noted-dark.json         # Dark variant
    └── noted-light.json        # Light variant
```

---

## 8. Functional Requirements by Phase

### MVP (Phase 1)

| ID | Requirement | Priority |
|---|---|---|
| F1.1 | Syntax highlighting: wikilinks, callouts, tags, embeds | Must |
| F1.2 | Autocompletion `[[` → list of MD files in workspace | Must |
| F1.3 | Go-to-definition: Ctrl+Click on `[[link]]` → open file | Must |
| F1.4 | Diagnostics: underline broken wikilinks | Must |
| F1.5 | Document symbols: nested heading structure | Should |
| F1.6 | Hover: show first lines of linked file | Should |

### Phase 2

| ID | Requirement | Priority |
|---|---|---|
| F2.1 | Semantic tokens: visual styles for H1–H6, bold, italic, code | Must |
| F2.2 | Companion theme (dark + light) with optimized MD styling | Must |
| F2.3 | Code actions: toggle checkbox `[ ]` <-> `[x]` | Must |
| F2.4 | Code actions: wrap selection in bold/italic/strikethrough | Should |
| F2.5 | Rename: file rename → update all wikilinks | Should |
| F2.6 | Workspace symbols: search across headings of all MD files | Should |
| F2.7 | Folding: hide MD syntax (```, >, callout markers) | Could |

### Phase 3

| ID | Requirement | Priority |
|---|---|---|
| F3.1 | Live preview in browser with full rendering | Must |
| F3.2 | Scroll sync Zed <-> Browser | Must |
| F3.3 | Mermaid diagram support | Should |
| F3.4 | KaTeX/MathJax formula support | Should |
| F3.5 | Obsidian callouts with icons support | Should |
| F3.6 | Reverse navigation: click in preview → Zed | Could |

---

## 9. Non-functional Requirements

| Requirement | Criterion |
|---|---|
| **Performance** | LSP response < 100ms for autocompletion, < 500ms for indexing 1000 files |
| **Compatibility** | Zed stable + preview channels, macOS + Linux |
| **Note format** | Compatible with Obsidian vault (YAML frontmatter, wikilinks, callouts) |
| **License** | MIT or Apache 2.0 (Zed extension registry requirement) |
| **Size** | WASM binary < 5 MB |
| **Dependencies** | Minimal runtime dependencies, LSP downloaded automatically |

---

## 10. Risks and Mitigation

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| Zed API won't expand for custom UI | High | High | Strategy A+B+C covers the maximum without custom UI |
| Tree-sitter grammar for extended MD is complex | Medium | Medium | Start with fork of existing `tree-sitter-markdown`, add incrementally |
| Inlay hints / semantic tokens behave unexpectedly in Zed | Medium | Medium | Prototyping in Phase 0 |
| Competition with Zed's built-in Markdown Preview | Low | Medium | Position as complement, not replacement — focus on editing experience |
| BAML approach (localhost) is unstable | Medium | Medium | This is Phase 3; more info will be available by implementation time |

---

## 11. Success Metrics

| Metric | MVP Goal | v1.0 Goal |
|---|---|---|
| Wikilink autocompletion works | Yes | Yes |
| Go-to-definition for links | Yes | Yes |
| Broken link diagnostics | Yes | Yes |
| Semantic tokens for visual differentiation | — | Yes |
| Companion theme (dark + light) published | — | Yes |
| Live preview with sync | — | Yes |
| Indexing time for 1000 files | < 2s | < 500ms |
| Published in Zed extension registry | — | Yes |

---

## 12. Step-by-Step Execution Plan (for Claude Agent)

> Each step is a task for Claude Agent (Claude Code / Zed Agent).
> Prompts are written so they can be used as-is.
> The user copies a step → gives it to the agent → reviews the result → moves to the next.
>
> **Conventions:**
> - `[AGENT]` — step is performed by the agent
> - `[HUMAN]` — step is performed by the user manually (software installation, GUI testing)
> - `[VERIFY]` — acceptance criterion by which the user evaluates the result
> - Each `[AGENT]` step is self-contained: includes context, what to do, and where to put the result

---

### PHASE 0 — Scaffolding and Research

#### 0.1. Project Infrastructure

**0.1.1.** `[HUMAN]` Install Rust via `rustup`, add wasm target:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-wasip2
cargo install tree-sitter-cli
```

**0.1.2.** `[AGENT]` Create the Cargo workspace structure for the `noted` project.
The workspace should contain two crates:
- root crate (extension) — `crate-type = ["cdylib"]`, dependency `zed_extension_api = "0.5"`
- subcrate `crates/noted-lsp` — binary crate for the Language Server, dependencies:
  `tower-lsp = "0.20"`, `tokio = { version = "1", features = ["full"] }`,
  `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"`,
  `tracing = "0.1"`, `tracing-subscriber = "0.3"`

Create `extension.toml`:
```toml
id = "noted"
name = "Markdown Live"
version = "0.1.0"
schema_version = 1
authors = ["Your Name <your@email.com>"]
description = "Obsidian-like Markdown editing experience for Zed"
repository = "https://github.com/username/noted"
```

Create `.gitignore` for Rust, `LICENSE` (MIT), empty `README.md`.
`[VERIFY]` `cargo check` passes without errors for both crates.

**0.1.3.** `[AGENT]` Write a minimal `src/lib.rs` for the Zed extension:
- struct `MdLiveExtension`
- `impl zed::Extension for MdLiveExtension` with empty methods
- `zed::register_extension!(MdLiveExtension)`
- In `language_server_command()` return `Err` with message "LSP not configured yet" for now

`[VERIFY]` `cargo build --target wasm32-wasip2` compiles without errors.

**0.1.4.** `[HUMAN]` Install as dev extension in Zed:
Command Palette → `zed: install dev extension` → select the project folder.
`[VERIFY]` Extension is visible in Extensions list with "Dev Extension" label.

#### 0.2. Hello World LSP

**0.2.1.** `[AGENT]` Write a minimal LSP server in `crates/noted-lsp/src/main.rs`:
- Use `tower-lsp` with `tokio` runtime
- On `initialize` — return capabilities: `hover_provider: Some(true)`
- On `textDocument/hover` — return `Hover` with content `"Hello from noted LSP!"`
  for any position
- On `initialized` — log `"noted-lsp started successfully"` via `tracing::info!`
- Configure `tracing-subscriber` for stderr output (Zed redirects stderr to log)

`[VERIFY]` `cargo build -p noted-lsp` compiles. Binary runs
and responds correctly to stdin JSON-RPC `initialize` request.

**0.2.2.** `[AGENT]` Update `src/lib.rs` extension:
- In `language_server_command()` return the path to the compiled `noted-lsp` binary.
  For dev use an absolute path to `target/release/noted-lsp`
  (user will substitute their own path).
- Add a `[languages.markdown]` section to `extension.toml` or a separate
  `languages/noted/config.toml` with `.md` file binding.

Make sure extension.toml contains:
```toml
[language-servers.noted-lsp]
name = "Markdown Live LSP"
languages = ["Markdown"]
```

`[VERIFY]` After reinstalling dev extension: open a .md file in Zed →
hover over any word → popup shows "Hello from noted LSP!"

**0.2.3.** `[AGENT]` Add one diagnostic to the LSP:
- On `textDocument/didOpen` and `textDocument/didChange` — send `publishDiagnostics`
  with one Warning on the first line: "noted-lsp is connected and working"
- Severity: `DiagnosticSeverity::Information`

`[VERIFY]` Open a .md file → first line has an underline,
on hover — text "noted-lsp is connected and working".

#### 0.3. Semantic Tokens Verification

**0.3.1.** `[AGENT]` Add a semantic tokens provider to the LSP:
- Register capability `semantic_tokens_provider` with `full: true`
- Define token types: `["heading", "markup", "punctuation"]`
- Define token modifiers: `["h1", "h2", "h3", "bold", "italic"]`
- On `textDocument/semanticTokens/full`:
  - Find all lines starting with `# ` → mark text (without `# `) as `heading.h1`
  - Find all lines starting with `## ` → mark as `heading.h2`
  - Find all `**...**` → mark content as `markup.bold`
  - Find all `*...*` (not `**`) → mark content as `markup.italic`
  - Mark symbols `#`, `**`, `*` as `punctuation`

Use simple regex parsing, not pulldown-cmark (this is a prototype).

`[VERIFY]` `cargo build -p noted-lsp` compiles.

**0.3.2.** `[HUMAN]` Add the following rules to Zed `settings.json` and check the result:
```json
{
  "global_lsp_settings": {
    "semantic_token_rules": [
      { "token_type": "heading", "token_modifiers": ["h1"],
        "foreground_color": "#E8C56D", "font_weight": 800 },
      { "token_type": "heading", "token_modifiers": ["h2"],
        "foreground_color": "#D4A94E", "font_weight": 700 },
      { "token_type": "markup", "token_modifiers": ["bold"],
        "font_weight": 700 },
      { "token_type": "markup", "token_modifiers": ["italic"],
        "font_style": "italic" },
      { "token_type": "punctuation",
        "foreground_color": "#444444" }
    ]
  }
}
```
Open a .md file with headings, bold and italic text.
`[VERIFY]` Record which properties work and which don't:
- [ ] foreground_color applies?
- [ ] font_weight visually changes boldness?
- [ ] font_style: italic works?
- [ ] Muted color for punctuation is visible?
This result determines the viability of Strategy B.

**0.3.3.** `[AGENT]` Add an inlay hints provider to the LSP:
- Register capability `inlay_hint_provider: true`
- On `textDocument/inlayHint`: find lines with `- [x]` → return inlay hint
  with text `" ✓"` after `]`, kind: `InlayHintKind::Parameter`
- Find lines with `- [ ]` → return `" ○"`

`[VERIFY]` Open a .md with checkboxes → next to `[x]` you see `✓`, next to `[ ]` — `○`.

**0.3.4.** `[HUMAN]` **Checkpoint.** Based on results of 0.3.2 and 0.3.3, make a decision:
- If semantic tokens + inlay hints work → Strategy A+B confirmed, continue.
- If something critically doesn't work → adjust the specification, discuss with the agent.

---

### PHASE 1 — MVP: Vault Indexer + Core LSP

#### 1.1. Tree-sitter Grammar

**1.1.1.** `[AGENT]` Create directory `grammars/tree-sitter-noted/`.
Write `grammar.js` based on tree-sitter-markdown, adding rules for:
- Wikilinks: `[[page]]` and `[[page|alias]]` — new node type `wikilink`
  with children `wikilink_target` and optional `wikilink_alias`
- Obsidian embeds: `![[file]]` — node type `embed`
- Tags: `#tag-name` (but not inside headings and not `#` at start of line) — node type `tag`
- Callouts: `> [!type]` at start of blockquote — node type `callout` with `callout_type`

Write tests in `test/corpus/` for each new node type.

`[VERIFY]` `tree-sitter generate` passes. `tree-sitter test` — all tests green.
`tree-sitter parse test-file.md` — wikilinks, embeds, tags, callouts are recognized.

**1.1.2.** `[AGENT]` Write Tree-sitter query files for Zed:
- `languages/noted/highlights.scm`:
  Captures for wikilinks (`@markup.link`), embeds (`@markup.link`),
  tags (`@label`), callout type (`@keyword`), callout body (`@comment`),
  heading markers `#` (`@punctuation.special`).
  Use fallback captures: `@markup.link @string` for themes without `@markup.link`.
- `languages/noted/folds.scm`:
  Folding for: heading sections, callout blocks, fenced code blocks, frontmatter.
- `languages/noted/outline.scm`:
  Captures for heading nodes → Zed Outline panel.
- `languages/noted/injections.scm`:
  Injection for fenced code blocks (```lang → inject tree-sitter-lang).

`[VERIFY]` Files are syntactically correct for tree-sitter query parser.

**1.1.3.** `[AGENT]` Integrate grammar into extension:
- Update `extension.toml`: add `[grammars.noted]` with path to grammar.
- Update `languages/noted/config.toml`: `grammar = "noted"`,
  `path_suffixes = ["md", "markdown"]`.
- Make sure the root `Cargo.toml` doesn't conflict with the grammar.

`[VERIFY]` Reinstall dev extension → open .md → Status bar shows
"Noted Markdown" as language. Wikilinks and callouts are highlighted differently from plain text.

#### 1.2. Vault Indexer

**1.2.1.** `[AGENT]` Create module `crates/noted-lsp/src/vault/mod.rs` with submodules:
- `vault/index.rs` — data structures:
  ```rust
  pub struct VaultIndex {
      pub notes: HashMap<PathBuf, NoteEntry>,
      pub backlinks: HashMap<PathBuf, Vec<LinkReference>>,
  }
  pub struct NoteEntry {
      pub path: PathBuf,
      pub title: String,           // from frontmatter or first H1 or filename
      pub headings: Vec<Heading>,
      pub links: Vec<LinkReference>,
      pub tags: Vec<Tag>,
      pub frontmatter: Option<Frontmatter>,
  }
  pub struct Heading { pub level: u8, pub text: String, pub line: u32 }
  pub struct LinkReference { pub target: String, pub line: u32, pub col: u32, pub alias: Option<String> }
  pub struct Tag { pub name: String, pub line: u32 }
  pub struct Frontmatter { pub title: Option<String>, pub tags: Vec<String>, pub raw: String }
  ```
- `vault/scanner.rs` — function `scan_vault(root: &Path) -> Vec<PathBuf>`:
  recursive walk via `walkdir`, filter `.md`/`.markdown`, ignore `.obsidian/`, `node_modules/`.
- `vault/parser.rs` — function `parse_note(path: &Path, content: &str) -> NoteEntry`:
  uses `pulldown-cmark` for heading extraction,
  regex for wikilinks (`\[\[([^\]|]+)(?:\|([^\]]+))?\]\]`),
  regex for tags (`(?:^|\s)#([\w-/]+)`),
  manual frontmatter parsing (text between `---`).

Add dependencies to LSP `Cargo.toml`: `pulldown-cmark`, `walkdir`, `regex`.
Write unit tests for `parse_note` with examples: file with frontmatter, wikilinks, tags, headings.

`[VERIFY]` `cargo test -p noted-lsp` — all tests pass.

**1.2.2.** `[AGENT]` Add function `build_index(notes: Vec<NoteEntry>) -> VaultIndex`:
- Build `backlinks` map: for each wikilink in each note — add entry
  to target file's backlinks.
- Resolving wikilinks: `[[name]]` → search for file `name.md` in vault (case-insensitive,
  search by basename without extension).

Add function `resolve_wikilink(vault: &VaultIndex, target: &str) -> Option<PathBuf>`.
Write tests: vault of 5 files with cross-references, including broken ones.

`[VERIFY]` `cargo test -p noted-lsp` — tests for build_index and resolve_wikilink pass.

**1.2.3.** `[AGENT]` Integrate vault indexer into LSP server:
- On `initialize` — run `scan_vault` + `parse_note` for each file → `build_index`.
  Store `VaultIndex` in `Arc<RwLock<VaultIndex>>` in server state.
- On `textDocument/didSave` — reindex the changed file, update index.
- Log indexing time: `tracing::info!("Indexed {} notes in {:?}", count, elapsed)`.

`[VERIFY]` Run LSP with test vault → logs show "Indexed N notes in Xms".

#### 1.3. LSP — Completion

**1.3.1.** `[AGENT]` Implement `textDocument/completion`:
- Trigger character: `[`
- Logic: determine if cursor is inside `[[...` (parse line text up to cursor position).
  If yes — return completion items with all files from VaultIndex.
- Each CompletionItem:
  - `label`: filename without `.md`
  - `kind`: `CompletionItemKind::File`
  - `detail`: first line of file (or frontmatter title)
  - `insert_text`: `filename]]` (closing brackets)
  - `filter_text`: filename for fuzzy matching

Add dependency `fuzzy-matcher` or `nucleo` for sorting results by relevance.

`[VERIFY]` In Zed: type `[[` → file list appears. Type `[[bio` → filtering.
Select → `[[filename]]` is inserted.

#### 1.4. LSP — Go-to-Definition

**1.4.1.** `[AGENT]` Implement `textDocument/definition`:
- Determine if cursor is on a wikilink (regex find `[[...]]` around position).
- Extract target from wikilink.
- Resolve target via `resolve_wikilink`.
- If there's an anchor `[[file#heading]]` — find the heading line in target file.
- Return `LocationLink` with target file uri and heading range (or start of file).

`[VERIFY]` In Zed: Ctrl+Click on `[[note]]` → Zed opens note.md.
Ctrl+Click on `[[note#section]]` → opens note.md at the line with `## section`.

#### 1.5. LSP — Diagnostics

**1.5.1.** `[AGENT]` Implement diagnostics publishing:
- On `didOpen`/`didChange`: parse current file, find all wikilinks.
- For each wikilink check `resolve_wikilink`. If `None` →
  `Diagnostic` with severity Warning, message "Note not found: [[target]]", range — entire wikilink.
- For empty wikilinks `[[]]` → severity Error, message "Empty wikilink".
- Send via `client.publish_diagnostics()`.
- Clear diagnostics on `didClose`.

`[VERIFY]` In Zed: write `[[nonexistent]]` → wavy underline.
Problems Panel → diagnostic is visible. Fix it → underline disappears.

#### 1.6. LSP — Hover

**1.6.1.** `[AGENT]` Implement `textDocument/hover`:
- If cursor is on a wikilink: resolve → read first 10 lines of target file
  (or frontmatter title + description). Return as `MarkupContent::Markdown`.
- If cursor is on tag `#tagname`: show count of files with that tag.
- Otherwise: return `None`.

`[VERIFY]` In Zed: hover over `[[note]]` → popup with content preview of note.md.

#### 1.7. LSP — Document Symbols

**1.7.1.** `[AGENT]` Implement `textDocument/documentSymbol`:
- Parse current file, extract all headings.
- Build nested hierarchy: H1 contains H2, H2 contains H3, etc.
- Return as `Vec<DocumentSymbol>` with `kind: SymbolKind::String` (for headings),
  `children` for nested ones.

`[VERIFY]` In Zed: Outline Panel (toolbar button) → nested heading structure.

#### 1.8. Remove Hello World Code

**1.8.1.** `[AGENT]` Remove all hello world code from LSP:
- Remove hardcoded diagnostic on first line (from step 0.2.3).
- Remove hardcoded hover "Hello from noted LSP!" (from step 0.2.1) — hover now
  works through real logic from 1.6.
- Remove prototype semantic tokens from 0.3.1 — they will be replaced by full implementation in Phase 2.
- Keep inlay hints for checkboxes from 0.3.3 (they are already useful).
- Verify all `#[cfg(test)]` modules compile.

`[VERIFY]` `cargo build -p noted-lsp` clean. `cargo test -p noted-lsp` — all tests green.
In Zed: no fake diagnostics/hover, real features (completion, go-to-def, diagnostics,
hover, outline) work.

#### 1.9. MVP Integration Test

**1.9.1.** `[AGENT]` Create test vault in `tests/fixtures/vault/`:
- 15–20 .md files with cross-referencing wikilinks
- Files with frontmatter (title, tags, date)
- Files with callouts, code blocks, checklists
- Several broken wikilinks (links to nonexistent files)
- File with heading anchors (`[[file#heading]]`)
- `README.md` describing the test vault

**1.9.2.** `[HUMAN]` Open test vault in Zed with dev extension. Verify:
- [ ] Wikilinks are highlighted
- [ ] `[[` → autocomplete shows files
- [ ] Ctrl+Click on wikilink → file opens
- [ ] Broken wikilinks are underlined
- [ ] Hover on wikilink → preview
- [ ] Outline panel → heading structure
- [ ] Checkbox inlay hints are visible

`[VERIFY]` All items above work. Commit: tag `v0.1.0-mvp`.

---

### PHASE 2 — Semantic Tokens + Theme + Code Actions

#### 2.1. Semantic Tokens (full implementation)

**2.1.1.** `[AGENT]` Create module `crates/noted-lsp/src/semantic_tokens.rs`.
Implement `textDocument/semanticTokens/full`:

Register token types and modifiers:
```
Token types: heading, markup, string, comment, punctuation
Token modifiers: h1, h2, h3, h4, h5, h6, bold, italic, strikethrough,
                 code, link, wikilink, broken, tag, callout,
                 checkbox_done, checkbox_todo, math, frontmatter, markup_punctuation
```

Parse file via `pulldown-cmark` and collect semantic tokens:
- Headings H1–H6: heading text → `heading` + `hN` modifier.
  `#` symbols → `punctuation` + `markup_punctuation`.
- Strong (`**text**`): content → `markup` + `bold`.
  `**` symbols → `punctuation` + `markup_punctuation`.
- Emphasis (`*text*`): content → `markup` + `italic`.
- Strikethrough (`~~text~~`): content → `markup` + `strikethrough`.
- Inline code: content → `markup` + `code`.
- Links: text → `string` + `link`.
- Wikilinks (regex): content → `markup` + `wikilink`.
  If broken (per index) → add `broken` modifier. Brackets `[[]]` → `punctuation` + `markup_punctuation`.
- Tags (regex): `#tag` → `markup` + `tag`.
- Callouts (regex): `> [!type]` → `markup` + `callout`.
- Checkboxes: `[x]` → `markup` + `checkbox_done`. `[ ]` → `markup` + `checkbox_todo`.
- Math (regex): `$...$` → `markup` + `math`.
- Frontmatter (between `---`): all content → `comment` + `frontmatter`.

Encode result in LSP semantic tokens format (delta-encoded line/column/length/type/modifiers).

`[VERIFY]` `cargo test -p noted-lsp` — unit tests on test MD snippets pass.
Each token type is correctly encoded.

**2.1.2.** `[AGENT]` Add `textDocument/semanticTokens/full/delta`:
- Cache previous token result for each file.
- On `delta` request — compute and return only changed tokens.

`[VERIFY]` `cargo test` passes. Repeat request on unchanged file → empty delta.

#### 2.2. Companion Theme

**2.2.1.** `[AGENT]` Create directory `noted-theme/` (separate extension).
Create `extension.toml`:
```toml
id = "noted-theme"
name = "Noted Theme"
version = "0.1.0"
schema_version = 1
authors = ["Your Name"]
description = "Verdant Garden — a prose-optimized dark/light theme for Markdown editing in Zed"
```

Create `themes/verdant-dark.json` — full theme per Zed Theme Schema v0.2.0:
- Appearance: dark
- Editor background: `#0A0A08` (near-black warm)
- Foreground: `#C8CAC0`
- Syntax styles (Verdant Garden palette — see `noted-theme/verdant-garden-colors.md`):
  - `title`: `{ "color": "#8FBF6A", "font_weight": 800 }` (H1 heading)
  - `emphasis`: `{ "font_style": "italic" }`
  - `emphasis.strong`: `{ "font_weight": 700 }`
  - `link_text`: `{ "color": "#A0D8D8" }`
  - `link_uri`: `{ "color": "#7CB5C4", "font_style": "italic" }`
  - `punctuation.special`: `{ "color": "#4A4A40" }` (dimmed MD syntax)
  - `punctuation.delimiter`: `{ "color": "#4A4A40" }`
  - `string`: `{ "color": "#E0B460" }`
  - `keyword`: `{ "color": "#72AAD0", "font_weight": 600 }`
  - `function`: `{ "color": "#B8DC94" }`
  - `type`: `{ "color": "#A898C4" }`
  - `variable`: `{ "color": "#C8CAC0" }`
  - `comment`: `{ "color": "#545648", "font_style": "italic" }`
- Full set of UI colors using stone/green/teal palette from `noted-theme/verdant-garden-colors.md`
  optimized for extended prose reading.

Create `themes/verdant-light.json`:
- Appearance: light
- Editor background: `#FCFCFA` (near-white warm, paper-like)
- Foreground: `#2A2E22`
- Syntax: same semantic distinctions using light-theme stops from `noted-theme/verdant-garden-colors.md`.
  Headings: dark green ramp (`#3D7A1A` → `#8DCA64`). Links: teal (`#2E7E8C`).
  Punctuation: `#C4C6BC` (dimmed).

Add `LICENSE` (MIT).

`[VERIFY]` JSON files are valid. Install as dev extension → Zed shows theme in theme picker.

**2.2.2.** `[HUMAN]` Install theme, open test vault, visually verify:
- [ ] H1 headings — bright and bold
- [ ] H2–H6 — decreasing brightness
- [ ] Bold text is bold
- [ ] Italic text is italic
- [ ] Syntax `#`, `**`, `>` — nearly invisible
- [ ] Wikilinks — underlined, like links
- [ ] Code blocks — normal code highlighting (not ruined by prose styles)

#### 2.3. Code Actions

**2.3.1.** `[AGENT]` Create module `crates/noted-lsp/src/code_actions.rs`.
Implement `textDocument/codeAction`:

When cursor is on a line with a checkbox:
- `- [ ] text` → action "Toggle Checkbox" → edit: replace `[ ]` with `[x]`
- `- [x] text` → action "Toggle Checkbox" → edit: replace `[x]` with `[ ]`

When there is a selection:
- "Wrap in Bold" → wrap in `**...**`
- "Wrap in Italic" → wrap in `*...*`
- "Wrap in Strikethrough" → wrap in `~~...~~`
- "Wrap in Code" → wrap in `` `...` ``
- "Wrap in Wikilink" → wrap in `[[...]]`

When cursor is on a line with a heading:
- "Increase Heading Level" → `##` → `###` (up to `######`)
- "Decrease Heading Level" → `###` → `##` (down to `#`)

When cursor is on an empty line:
- "Insert Callout" → insert `> [!note]\n> `
- "Insert Table" → insert 3x3 table template

When cursor is on a callout `> [!type]`:
- "Change to Note/Warning/Tip/Important/Caution" → replace type

Each action is a `CodeAction` with `edit: WorkspaceEdit` containing `TextEdit`.

`[VERIFY]` `cargo test` passes. In Zed: cursor on `- [ ]` → Ctrl+. → "Toggle Checkbox" →
line changes to `- [x]`.

#### 2.4. Rename

**2.4.1.** `[AGENT]` Implement `textDocument/prepareRename` and `textDocument/rename`:
- `prepareRename`: if cursor is on a wikilink → return range of wikilink target (without `[[]]`).
  Otherwise → null (rename not available).
- `rename`: get new name. Find all files in vault containing `[[old_name]]`.
  Generate `WorkspaceEdit` with `TextEdit` for each occurrence in all files.
  Also: rename the target file itself (via `resource_operations` if LSP supports,
  or return a diagnostic "Please rename the file manually").

`[VERIFY]` In Zed: cursor on `[[old-note]]` → F2 → type "new-note" →
all files updated: `[[new-note]]`.

#### 2.5. Workspace Symbols

**2.5.1.** `[AGENT]` Implement `workspace/symbol`:
- On request with query string: find all headings in all vault files, filter
  by fuzzy match with query.
- Return as `Vec<SymbolInformation>` with name = heading text, kind = `SymbolKind::String`,
  location = file + heading line.

`[VERIFY]` In Zed: Ctrl+T (or workspace symbol command) → type part of heading →
results from all files. Select → jump to file and line.

#### 2.6. Phase 2 Integration Test

**2.6.1.** `[HUMAN]` Full end-to-end test with theme + all Phase 2 features:
- [ ] Semantic tokens correctly style all MD elements
- [ ] Theme visually improves prose experience
- [ ] Code actions work (checkbox toggle, wrap, heading level)
- [ ] Rename updates links
- [ ] Workspace symbols find headings
- [ ] Inlay hints for checkboxes and embeds
- [ ] Performance: vault of 100 files indexes < 1s

`[VERIFY]` All items passed. Commit: tag `v0.2.0-visual`.

---

### PHASE 3 — Preview Server

#### 3.1. HTTP Server

**3.1.1.** `[AGENT]` Add dependencies to LSP: `axum = "0.8"`, `tokio-tungstenite = "0.24"`,
`tower-http = { version = "0.6", features = ["fs"] }`.

Implement module `crates/noted-lsp/src/preview/mod.rs`:
- On LSP start → launch HTTP server on `127.0.0.1:0` (OS picks port) in a separate tokio task.
- Route `GET /` → serves `index.html` (embedded in binary via `include_str!`).
- Route `GET /style.css` → serves CSS.
- Route `GET /ws` → WebSocket endpoint.
- Log the chosen port: `tracing::info!("Preview server at http://127.0.0.1:{}", port)`.

`[VERIFY]` Run LSP → URL visible in logs → `curl http://127.0.0.1:<port>/` returns HTML.

**3.1.2.** `[AGENT]` Create `crates/noted-lsp/src/preview/static/index.html`:
- Minimal HTML: `<div id="content"></div>`, CSS and JS includes.
- JS: connect to WebSocket at `ws://127.0.0.1:<port>/ws`.
- On receiving message `{ "type": "content", "html": "..." }` → update `#content.innerHTML`.
- On receiving `{ "type": "scroll", "line": N }` → find element with `data-line="N"`,
  scrollIntoView smooth.
- Include mermaid.js and KaTeX via CDN `<script>` tags.
  After content update → `mermaid.run()` and `renderMathInElement()`.

`[VERIFY]` Open URL in browser → empty page, WebSocket connected (visible in DevTools).

#### 3.2. MD → HTML Rendering

**3.2.1.** `[AGENT]` Create module `crates/noted-lsp/src/preview/renderer.rs`:
- Function `render_to_html(content: &str, vault: &VaultIndex) -> String`:
  - Use `pulldown-cmark` with enabled extensions (tables, footnotes, strikethrough, tasklists).
  - Wrap each block-level element in `<div data-line="N">` where N is the source line number
    (for scroll sync).
  - Wikilinks: `[[page]]` → `<a class="wikilink" href="#" data-target="page">page</a>`.
  - Callouts: `> [!type] title` → `<div class="callout callout-type"><div class="callout-title">title</div>...`.
  - Math: `$...$` → `<span class="math">...</span>`, `$$...$$` → `<div class="math-block">...</div>`.
  - Mermaid: ` ```mermaid ` → `<div class="mermaid">...</div>`.

`[VERIFY]` Unit tests: render test MD → HTML contains data-line attributes,
wikilinks, callouts, math and mermaid blocks.

**3.2.2.** `[AGENT]` Create CSS styles `crates/noted-lsp/src/preview/static/style.css`:
- Prose-optimized styles: good typography, readable line-height.
- Callout block styles (by type: note, warning, tip, important, caution) —
  matching Obsidian visual style (colored left border, background, icon).
- Wikilink styles (blue, underlined).
- Checkbox styles (custom SVG checkmarks).
- Dark mode support via `prefers-color-scheme`.

`[VERIFY]` Visually — the HTML page looks like Obsidian reading view.

#### 3.3. Live Sync

**3.3.1.** `[AGENT]` Implement synchronization:
- On `textDocument/didChange` → re-render → send updated HTML via WebSocket
  to all connected clients.
- On cursor movement (use `textDocument/didChange` or custom notification
  if available) → send `{ "type": "scroll", "line": cursor_line }`.
- Debounce: no more than once every 100ms for content updates.

`[VERIFY]` Open MD in Zed + preview in browser → type → preview updates live.

#### 3.4. /preview Slash Command

**3.4.1.** `[AGENT]` Add `/preview` slash command in extension (via Extension API):
- On invocation — get preview server URL from LSP (via custom request).
- Output URL in chat/output. Or attempt to open via `process::Command`
  (`xdg-open` on Linux, `open` on macOS).

`[VERIFY]` In Zed: type `/preview` → browser opens with live preview of current file.

**3.4.2.** `[HUMAN]` Full Phase 3 testing. Commit: tag `v0.3.0-preview`.

---

### PHASE 4 — Publishing

#### 4.1. CI and Release Pipeline

**4.1.1.** `[AGENT]` Create `.github/workflows/ci.yml`:
- Trigger: push to main, pull requests.
- Jobs:
  - `check`: `cargo check --target wasm32-wasip2` (extension) + `cargo check -p noted-lsp`
  - `test`: `cargo test -p noted-lsp`
  - `clippy`: `cargo clippy -p noted-lsp -- -D warnings`

**4.1.2.** `[AGENT]` Create `.github/workflows/release.yml`:
- Trigger: push tag `v*`.
- Build matrix: `[macos-latest (x86_64, aarch64), ubuntu-latest (x86_64)]`.
- For each target: `cargo build --release -p noted-lsp`.
- Upload artifacts as GitHub Release assets with names:
  `noted-lsp-{os}-{arch}` (no extension for unix).

`[VERIFY]` Push tag → GitHub Actions builds release with 3 binaries.

#### 4.2. Auto-download LSP binary

**4.2.1.** `[AGENT]` Update `src/lib.rs`:
- In `language_server_command()`: check for LSP binary in extension work dir.
- If missing — download from GitHub Releases via `zed_extension_api::download_file()`.
  URL: `https://github.com/{user}/noted/releases/latest/download/noted-lsp-{os}-{arch}`.
- Determine OS and arch via `std::env::consts::{OS, ARCH}`.
- Make binary executable (`chmod +x` via `Command`).

`[VERIFY]` Delete LSP binary → reinstall extension → LSP automatically downloads and works.

#### 4.3. Documentation

**4.3.1.** `[AGENT]` Write `README.md` for the main extension:
- Description: what it does, screenshots (placeholder paths).
- Installation: from Zed Extensions panel.
- Features: feature list with brief descriptions.
- Recommended settings: semantic_token_rules JSON for settings.json.
- Recommended theme: noted-theme.
- Recommended fonts.
- Supported Obsidian-flavored Markdown: table of what's supported.
- Contributing: how to build, run tests, install dev extension.

**4.3.2.** `[AGENT]` Write `CHANGELOG.md` in Keep a Changelog format.

**4.3.3.** `[AGENT]` Write `README.md` for the theme extension:
- Description, screenshots (placeholder).
- How to install, how to switch.

#### 4.4. Publishing

**4.4.1.** `[HUMAN]` Fork `zed-industries/extensions`.
Add `noted` and `noted-theme` as git submodules.
Add entries to `extensions.toml`. Run `pnpm sort-extensions`.
Open PR. Pass review.

`[VERIFY]` Extension and theme are available through Zed Extensions panel.
Commit: tag `v1.0.0`.

---

### Summary Table

| Phase | [AGENT] Steps | [HUMAN] Steps | Key Deliverable |
|---|---|---|---|
| 0. Scaffolding | 8 | 4 | Dev extension + LSP hello world + semantic tokens check |
| 1. MVP | 10 | 1 | Completion + go-to-def + diagnostics + hover + outline |
| 2. Visual | 7 | 2 | Semantic tokens + theme + code actions + rename |
| 3. Preview | 5 | 1 | Live preview with mermaid/KaTeX |
| 4. Publish | 5 | 1 | Published on Zed registry |
| **Total** | **35** | **9** | **v1.0.0** |

---

## 13. References and Resources

- [Zed Extension API docs](https://zed.dev/docs/extensions)
- [Zed Extension API crate](https://crates.io/crates/zed_extension_api)
- [Developing Extensions guide](https://zed.dev/docs/extensions/developing-extensions)
- [Discussion #37270: Custom rendering](https://github.com/zed-industries/zed/discussions/37270)
- [Discussion #30275: Enhanced MD editing](https://github.com/zed-industries/zed/discussions/30275)
- [Discussion #23951: MD Preview tracking](https://github.com/zed-industries/zed/discussions/23951)
- [BAML Zed extension approach](https://boundaryml.com/blog/how-to-write-a-zed-extension-for-a-made-up-language)
- [Zed Decoded: Extensions](https://zed.dev/blog/zed-decoded-extensions)
- [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- [Obsidian Markdown reference](https://help.obsidian.md/Editing+and+formatting/Obsidian+Flavored+Markdown)
