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

#### B.8. Note-level hover + "Show All Links" — In/out link navigation

**Design decision (April 2026):** Obsidian shows incoming and outgoing links in
dedicated sidebar panels. Zed extensions cannot create custom panels, so a two-layer
approach is used instead:

**Layer 1 — Hover on line 1:** When the cursor is on the first line of a note (not
over a wikilink), hover shows a compact link summary:
- Note title
- Outgoing links (up to 5, with "…and N more" if larger)
- Incoming links (up to 5, sourced by scanning the vault index)

**Layer 2 — "Show All Links" code action:** `Cmd+.` on any line → "Show All Links"
triggers `noted.showLinks`. The LSP generates a full Markdown file listing all
outgoing and incoming links, writes it to `/tmp/noted-links-<stem>.md`, then opens
it in Zed via `window/showDocument`.

**Temp file lifecycle:** No cleanup. The file at `/tmp/noted-links-<stem>.md` persists
until OS clears temp or manual deletion. Each invocation overwrites the same path, so
files don't accumulate.

**Why not a clickable button in hover:** `command:` URI support in Zed hover popups
is unconfirmed. The code action is a reliable, already-working mechanism.

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

#### B.7. Extension-provided `semantic_token_rules.json` — Per-element styling

**Discovery (April 2026):** Zed supports an extension-provided file `semantic_token_rules.json`
placed alongside `config.toml` in the language directory. This file maps custom LSP semantic
token types to theme `syntax` keys, eliminating the earlier limitation where custom token type
names were silently ignored.

**How it works:**

1. The LSP declares a custom legend with descriptive token type names (`heading`, `markup`,
   `punctuation`) instead of standard LSP names (`keyword`, `variable`, `operator`).
2. The extension ships `languages/noted/semantic_token_rules.json` that tells Zed how to
   resolve each custom type + modifier combination to a theme syntax key.
3. The theme's `syntax` section provides the actual styling (color, weight, style).

**Format:**

```json
[
  { "token_type": "heading", "token_modifiers": ["h1"], "style": ["heading.h1", "heading", "title"] },
  { "token_type": "markup", "token_modifiers": ["bold"], "style": ["markup.bold", "emphasis.strong"] },
  { "token_type": "markup", "token_modifiers": ["wikilink"], "style": ["markup.wikilink", "link_text"] },
  { "token_type": "punctuation", "token_modifiers": ["markup_punctuation"], "style": ["punctuation.markup_punctuation", "punctuation.special"] }
]
```

The `style` array is a **fallback chain** — Zed tries each theme syntax key left-to-right,
using the first one the active theme defines. This means:
- With the companion Noted Verdant Garden theme: each element gets its dedicated style
  (e.g., `heading.h1` → bright green + font_weight 800)
- With any other Zed theme: falls back to standard captures
  (e.g., `title` → whatever the theme uses for titles)

**What this enables vs standard token mapping:**

| Element | Before (standard types) | After (custom types + rules) |
|---|---|---|
| H1 heading | `keyword` color (shared) | `heading.h1` — bright green, weight 800 |
| H3 heading | `keyword` color (shared) | `heading.h3` — medium green, weight 600 |
| Bold text | `variable` color (shared) | `markup.bold` — font_weight 700 |
| Italic text | `variable` color (shared) | `markup.italic` — font_style italic |
| Wikilink | `variable` color (shared) | `markup.wikilink` — cyan/teal |
| Broken link | `variable` color (shared) | `markup.wikilink.broken` — red |
| Tag | `variable` color (shared) | `markup.tag` — gold with background |
| MD punctuation | `operator` color (shared) | `punctuation.markup_punctuation` — very dimmed |

**Precedent:** The Rust extension uses this mechanism for the custom `lifetime` token type,
proving Zed correctly reads the LSP's declared legend and resolves custom types via the rules file.

**Known limitation:** `font_style: "italic"` does NOT carry through the `style` array mapping
in the extension rules file (Zed limitation as of April 2026). `font_weight` does work.
Italic styling must be defined directly in the theme's `syntax` section to take effect.

### Strategy C: "BAML Trick — LSP + localhost server" — Experimental

Inspired by the BAML extension for Zed, which embeds a web interface via LSP:

1. LSP server launches a localhost HTTP server
2. Renders full Markdown preview in the browser
3. Cursor and scroll sync between Zed and browser via LSP
4. Bidirectional live editing: changes in Zed → preview updates, click in preview → navigation in Zed

**Pros:** Full rich rendering, can implement wikilinks, mermaid, math, callouts.
**Cons:** Dependency on external browser, not true inline WYSIWYG, more complex architecture.

---

## 4. Technical Stack

> **Decision:** the entire project is in Rust. Rust experience will be built up
> over the course of the project.

### 4.1. Extension (WASM module, loaded into Zed)

| Component | Technology | Version / Note |
|---|---|---|
| Language | Rust | edition 2021 |
| Target | `wasm32-wasip2` | compiled by Zed on install dev extension |
| API | `zed_extension_api` | latest on crates.io (>=0.5.0) |
| Crate type | `cdylib` | required by Zed for WASM extensions |

Minimal `lib.rs` — register language + specify LSP binary.
All logic lives in the LSP, not in the extension.

### 4.2. Language Server (separate binary)

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

### 4.3. Tree-sitter Grammar

| Component | Technology |
|---|---|
| Grammar definition | JavaScript (`grammar.js`) |
| Base fork | `tree-sitter-markdown` (MDeiml) |
| Queries | Scheme (`.scm` files) |
| Compilation | tree-sitter CLI → C → WASM (via Zed build pipeline) |

Grammar.js is the only non-Rust component. This is the Tree-sitter standard.

### 4.4. Preview Server

| Component | Crate / Technology |
|---|---|
| HTTP server | `axum` (embedded in LSP process) |
| WebSocket | `tokio-tungstenite` |
| HTML rendering | `pulldown-cmark` → HTML + `<script>` injections |
| Math | KaTeX (JS, loaded via CDN in preview HTML) |
| Diagrams | mermaid.js (JS, CDN) |
| Frontend | Vanilla HTML/JS (minimal weight, no frameworks) |

### 4.5. Companion Theme

| Component | Technology |
|---|---|
| Format | JSON (Zed Theme Schema v0.2.0) |
| Files | `noted-verdant-garden-dark.json`, `noted-verdant-garden-light.json` |
| Code | Not required — pure JSON |

### 4.6. Development Tooling

| Tool | Purpose |
|---|---|
| `rustup` | Rust toolchain management (required for Zed extensions) |
| `cargo` | Build, tests, dependencies |
| `tree-sitter-cli` | Grammar generation and testing |
| Zed IDE | Dev extension install, testing, Zed Agent for code generation |
| Claude Code | Rust code generation, refactoring, test writing |

---

## 5. Companion Theme: "Noted"

Strategy B critically depends on the theme — standard Zed themes are optimized for code,
not for prose. A companion theme (or light/dark pair) is needed that:
- Maximizes visual differentiation for Markdown elements
- Mutes MD syntax, letting content dominate
- Ships as a separate theme extension (Zed registry requirement)

### 5.1. Theme Design Principles

**Muted syntax:** characters `#`, `**`, `>`, `[[`, `]]`, `---` render
in nearly invisible color (alpha ~30% of main text). The user sees them
when focusing on the line, but they don't interfere when reading.

**Heading hierarchy through color + weight:** since font_size is unavailable,
the only way to create visual hierarchy H1–H6 is a combination of color
and font_weight. H1 = brightest + boldest, H6 = most muted + thinnest.

**Prose-first:** background and main text are optimized for extended reading,
not for code contrast. Soft tones, sufficient contrast, no bright accent colors
in body text.

### 5.2. Semantic Token Styling

The LSP returns custom token types. Styling is handled through two layers:

1. **Extension-provided `semantic_token_rules.json`** — maps custom LSP token types
   to theme `syntax` keys via fallback chains (see B.7). No user configuration needed.
2. **Theme `syntax` section** — defines actual colors, weights, and styles for each key.

The companion Noted Verdant Garden theme defines all custom syntax keys used by
the extension. With any other theme, the fallback chains in the rules file ensure
basic styling through standard syntax keys.

**Verdant Garden dark theme heading ramp:**

| Level | Color | Weight |
|---|---|---|
| H1 | `#8FBF6A` | 800 |
| H2 | `#7AAD58` | 700 |
| H3 | `#669B48` | 600 |
| H4 | `#548938` | 500 |
| H5 | `#447830` | 500 |
| H6 | `#3A6828` | 400 |

**Other key styles:**

| Element | Color | Property |
|---|---|---|
| Bold | — | font_weight: 700 |
| Italic | — | font_style: italic |
| Strikethrough | `#7A7C72` | strikethrough: true |
| Inline code | `#E0B460` | background: `#181816` |
| Wikilink | `#7CB5C4` | underline: true |
| Broken wikilink | `#CC4444` | underline + strikethrough |
| Link | `#A0D8D8` | underline: true |
| Tag | `#D4A56A` | background: `#1E1A10` |
| Callout | `#C47D8A` | font_style: italic |
| Math | `#B8DC94` | font_style: italic |
| Frontmatter | `#545648` | font_style: italic |
| MD punctuation | `#4A4A40` | — |

### 5.3. Syntax Theme Overrides (in the theme JSON file)

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

### 5.4. Theme Variants

| Variant | Description |
|---|---|
| **Noted Verdant Garden Dark** | Near-black warm background (`#0A0A08`), green heading ramp, teal links |
| **Noted Verdant Garden Light** | Near-white warm background (`#FCFCFA`), dark green headings, paper-like feel |

Both variants maintain full code highlighting in code blocks — the theme does not
sacrifice code experience for prose. Code blocks use the standard syntax color set.

### 5.5. Recommended Fonts

The theme does not control the font (that's a user setting), but the documentation
recommends using monospace fonts with good italic and variable weight support:

| Font | Why |
|---|---|
| **JetBrains Mono** | Excellent italic, good weight variations, ligatures |
| **Cascadia Code** | Cursive italic variant, good for prose |
| **Fira Code** | Wide range of weights, good readability |
| **Iosevka** | Most configurable, has quasi-proportional variants |
| **Victor Mono** | Especially beautiful cursive italic — ideal for emphasis |

### 5.6. Publishing

The theme is published as a **separate extension** (Zed registry requirement —
themes cannot be part of an extension with language support).

```
noted-theme/
├── extension.toml                      # id = "noted-theme"
├── LICENSE
└── themes/
    ├── noted-verdant-garden-dark.json   # Noted Verdant Garden Dark
    └── noted-verdant-garden-light.json  # Noted Verdant Garden Light
```

---

## 6. Non-functional Requirements

| Requirement | Criterion |
|---|---|
| **Performance** | LSP response < 100ms for autocompletion, < 500ms for indexing 1000 files |
| **Compatibility** | Zed stable + preview channels, macOS + Linux |
| **Note format** | Compatible with Obsidian vault (YAML frontmatter, wikilinks, callouts) |
| **License** | MIT or Apache 2.0 (Zed extension registry requirement) |
| **Size** | WASM binary < 5 MB |
| **Dependencies** | Minimal runtime dependencies, LSP downloaded automatically |

---

## 7. References and Resources

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
