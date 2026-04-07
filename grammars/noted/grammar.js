/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

/**
 * Tree-sitter grammar for Noted — extended Markdown with knowledge-base features.
 *
 * Adds to standard Markdown:
 *   - Wikilinks:  [[target]]  [[target|alias]]
 *   - Embeds:     ![[target]]
 *   - Tags:       #tag-name
 *   - Callouts:   > [!type] title
 */
module.exports = grammar({
  name: "noted",

  // No implicit whitespace skipping — newlines are significant in Markdown
  extras: (_) => [],

  conflicts: ($) => [
    // bold (**) and italic (*) both start with *
    [$.bold, $.italic],
    // a line might look like both a paragraph and a list item until we see the marker
    [$.paragraph_line, $.list_item],
  ],

  rules: {
    // ─── Document ────────────────────────────────────────────────────────────

    document: ($) =>
      repeat(
        choice(
          $.heading,
          $.callout_block,
          $.fenced_code_block,
          $.list_item,
          $.thematic_break,
          $.blank_line,
          $.paragraph_line
        )
      ),

    // ─── Block elements ───────────────────────────────────────────────────────

    heading: ($) =>
      seq(
        field("marker", /#{1,6} /),
        repeat($._inline),
        /\n/
      ),

    // > [!type] optional title
    // followed by zero or more continuation lines starting with >
    callout_block: ($) =>
      seq(
        "> [!",
        field("callout_type", $.callout_type_name),
        "]",
        optional(seq(" ", repeat($._inline))),
        /\n/,
        repeat($.blockquote_continuation)
      ),

    callout_type_name: (_) => /[A-Za-z][A-Za-z0-9]*/,

    blockquote_continuation: (_) => /> [^\n]*\n/,

    // Fenced code block: ```lang\n...body...\n```
    fenced_code_block: ($) =>
      seq(
        field("fence_start", /`{3}[^\n]*/),
        /\n/,
        optional(field("code", /(?:[^`]|`{1,2}[^`])+/)),
        "```",
        /[^\n]*/,
        /\n?/
      ),

    // List item with optional checkbox
    list_item: ($) =>
      seq(
        /[ \t]*(?:[-*+]|\d+[.)]) /,
        optional(field("checkbox", $.checkbox)),
        repeat($._inline),
        /\n/
      ),

    // Checkbox markers (including trailing space that separates them from content)
    checkbox: (_) => token(choice("[ ] ", "[x] ", "[X] ")),

    paragraph_line: ($) => seq(repeat1($._inline), /\n/),

    blank_line: (_) => "\n",

    thematic_break: (_) => /(?:-{3,}|={3,}|_{3,}) *\n/,

    // ─── Inline elements ──────────────────────────────────────────────────────

    _inline: ($) =>
      choice(
        $.embed, // must precede wikilink — both start with [, embed with ![[
        $.wikilink,
        $.tag,
        $.bold,
        $.italic,
        $.inline_code,
        $.link,
        $.text
      ),

    // [[target]] or [[target|alias]]
    wikilink: ($) =>
      seq(
        "[[",
        field("target", $.wikilink_target),
        optional(seq("|", field("alias", $.wikilink_alias))),
        "]]"
      ),

    wikilink_target: (_) => /[^\]|\n]+/,
    wikilink_alias: (_) => /[^\]\n]+/,

    // ![[target]]
    embed: ($) =>
      seq("![[", field("target", $.embed_target), "]]"),

    embed_target: (_) => /[^\]\n]+/,

    // #tag  (must start with a letter to avoid matching # heading markers)
    tag: (_) => /#[A-Za-z][\w\-\/]*/,

    // **content**  (higher precedence than italic)
    bold: ($) =>
      prec(
        2,
        seq(
          token(prec(2, "**")),
          repeat1(choice($.inline_code, $.italic, $.bold_text)),
          token(prec(2, "**"))
        )
      ),

    bold_text: (_) => /[^*`\n]+/,

    // *content*
    italic: ($) =>
      prec(
        1,
        seq("*", repeat1(choice($.inline_code, $.italic_text)), "*")
      ),

    italic_text: (_) => /[^*`\n]+/,

    // `code`
    inline_code: (_) => seq("`", /[^`\n]+/, "`"),

    // [text](url)
    link: ($) =>
      seq(
        "[",
        field("text", /[^\]\n]+/),
        "](",
        field("url", /[^)\n]+/),
        ")"
      ),

    // Catch-all for any characters not matched by the above rules.
    // The alternation handles single special chars that appear outside
    // their structural context (e.g. a bare `!` or `#`).
    text: (_) => /[^\[\]!*`#\n]+|[!#]/,
  },
});
