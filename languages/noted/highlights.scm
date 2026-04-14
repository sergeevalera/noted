; Headings
(heading (text) @markup.heading)

; Wikilinks
(wikilink target: (wikilink_target) @markup.link)
(wikilink alias: (wikilink_alias) @string)

; Embeds
(embed target: (embed_target) @markup.link)

; Tags
(tag) @label

; Callouts
(callout_block callout_type: (callout_type_name) @keyword)
(blockquote_continuation) @comment

; Checkboxes
(checkbox) @markup.list.checked

; Bold / Italic
(bold_text) @markup.bold
(italic_text) @markup.italic

; Code
(inline_code) @string
(fenced_code_block) @string
