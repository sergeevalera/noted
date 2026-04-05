; Inject the language declared after the opening fence into code block bodies.
; e.g. ```rust → highlight as Rust
(fenced_code_block
  fence_start: _ @injection.language
  code: _ @injection.content
  (#set! injection.include-children))
