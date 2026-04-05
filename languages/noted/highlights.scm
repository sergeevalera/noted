; Headings — marker and text styled separately
(heading marker: _ @punctuation.special)
(heading (text) @markup.heading)

; Heading levels via node test
(heading marker: _ @markup.heading.1
  (#match? @markup.heading.1 "^# "))
(heading marker: _ @markup.heading.2
  (#match? @markup.heading.2 "^## "))
(heading marker: _ @markup.heading.3
  (#match? @markup.heading.3 "^### "))

; Wikilinks
(wikilink
  "[[" @punctuation.bracket
  target: (wikilink_target) @markup.link
  "]]]" @punctuation.bracket)

(wikilink
  "[[" @punctuation.bracket
  target: (wikilink_target) @markup.link
  "|" @punctuation.delimiter
  alias: (wikilink_alias) @string
  "]]" @punctuation.bracket)

; Embeds  ![[file]]
(embed
  "![[" @punctuation.bracket
  target: (embed_target) @markup.link
  "]]" @punctuation.bracket)

; Tags  #tag
(tag) @label

; Callouts  > [!type]
(callout_block
  "> [!" @punctuation.special
  callout_type: (callout_type_name) @keyword
  "]" @punctuation.special)

(blockquote_continuation) @comment

; Checkboxes
(checkbox) @markup.list.checked

; Inline formatting
(bold
  "**" @punctuation.delimiter
  (bold_text) @markup.bold
  "**" @punctuation.delimiter)

(italic
  "*" @punctuation.delimiter
  (italic_text) @markup.italic
  "*" @punctuation.delimiter)

(inline_code
  "`" @punctuation.delimiter
  _ @string
  "`" @punctuation.delimiter)

; Fenced code blocks
(fenced_code_block
  fence_start: _ @punctuation.delimiter
  code: _ @string
  "```" @punctuation.delimiter)

; Links  [text](url)
(link
  "[" @punctuation.bracket
  text: _ @markup.link.text
  "](" @punctuation.bracket
  url: _ @markup.link.url
  ")" @punctuation.bracket)
