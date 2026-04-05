# Verdant Garden — Color Reference

## Dark Theme

### Backgrounds
```
background.primary:    #0A0A08   (editor, main bg — near-black warm)
background.secondary:  #111110   (surfaces, code blocks, cards)
background.tertiary:   #1A1A18   (sidebars, panels)
background.hover:      #222220   (hover states)
background.selected:   #2A2A26   (selected items, active line)
```

### Text
```
text.primary:          #C8CAC0   (body text)
text.secondary:        #8A8C82   (muted labels, descriptions)
text.tertiary:         #5A5C52   (hints, placeholders)
text.disabled:         #3A3C32   (disabled UI)
```

### Borders
```
border.primary:        #3A3C32   (strong borders)
border.secondary:      #2A2C24   (subtle borders)
border.tertiary:       #1E1E1A   (faint separators)
```

### Dimmed Syntax (MD punctuation: # ** [[ ]] > --- ~~)
```
dimmed.syntax:         #4A4A40
```

### Headings (green ramp, H1 brightest → H6 dimmest)
```
heading.h1:            #8FBF6A   font-weight: 800
heading.h2:            #7AAD58   font-weight: 700
heading.h3:            #669B48   font-weight: 600
heading.h4:            #548938   font-weight: 500
heading.h5:            #447830   font-weight: 500
heading.h6:            #3A6828   font-weight: 400
```

### Inline Formatting
```
markup.bold:           #C8CAC0   font-weight: 700 (same as body, weight does the work)
markup.italic:         #C8CAC0   font-style: italic
markup.strikethrough:  #7A7C72   strikethrough: true
markup.code.fg:        #E0B460   (inline code text)
markup.code.bg:        #181816   (inline code background)
markup.math:           #B8DC94   font-style: italic
```

### Links
```
link.wikilink:         #7CB5C4   underline: true
link.wikilink.broken:  #CC4444   underline: #CC4444, strikethrough: true
link.url:              #7CB5C4   underline: true
link.url.title:        #A0D8D8   (link text in [text](url))
```

### Tags
```
tag.fg:                #D4A56A
tag.bg:                #1E1A10
```

### Callouts
```
callout.marker:        #C47D8A   font-style: italic
callout.note:          #7CB5C4
callout.warning:       #E0B460
callout.danger:        #CC4444
callout.tip:           #6DA044
callout.info:          #72AAD0
```

### Checkboxes
```
checkbox.done.fg:      #7A7C72   strikethrough: true
checkbox.done.marker:  #6DA044   (the ✓ hint)
checkbox.todo.fg:      #C8CAC0
checkbox.todo.marker:  #545648   (the ○ hint)
```

### Frontmatter
```
frontmatter:           #545648   font-style: italic
```

### Code Syntax (for programming languages)
```
code.keyword:          #72AAD0   (fn, let, for, if, return, pub, use)
code.function:         #B8DC94   (function names, method calls)
code.type:             #A898C4   (types, traits, structs, enums)
code.string:           #E0B460   (string literals)
code.number:           #D4A56A   (numeric literals)
code.constant:         #E4CCA6   (constants, booleans, static)
code.variable:         #C8CAC0   (variables, parameters)
code.property:         #A0D8D8   (struct fields, properties)
code.operator:         #8A8C82   (=, +, -, ::, =>)
code.punctuation:      #7A7C72   (brackets, commas, semicolons)
code.comment:          #545648   font-style: italic
code.comment.doc:      #669B48   font-style: italic
code.attribute:        #C47D8A   (decorators, annotations, #[...])
code.tag:              #8FBF6A   (HTML/XML tags)
code.tag.attribute:    #D4A56A   (HTML/XML attributes)
code.embedded:         #E0B460   (template literals, interpolation)
code.regex:            #E4CCA6   (regular expressions)
code.escape:           #C47D8A   (escape sequences \n \t)
```

### Diagnostics & Status
```
error:                 #CC4444
warning:               #C89430
info:                  #72AAD0
success:               #6DA044
hint:                  #8A8C82
```

### UI Accents
```
accent.primary:        #6DA044   (cursor, active tab indicator)
accent.secondary:      #7CB5C4   (find matches, secondary highlights)
selection.bg:          #2A3A20   (text selection — green tint)
find.match.bg:         #3A3020   (find match highlight — amber tint)
bracket.match.bg:      #2A2C24   (matching bracket highlight)
git.added:             #6DA044
git.modified:          #72AAD0
git.deleted:           #CC4444
git.conflict:          #C89430
```

### Full Ramps (for reference, 50→900)
```
green:    #E8F5E0  #B8DC94  #8FBF6A  #6DA044  #4D8A28  #3D7A1A  #265010
teal:     #E0F4F4  #A0D8D8  #7CB5C4  #4A9AAE  #2E7E8C  #1E6472  #104448
earth:    #F6EDE0  #E4CCA6  #D4A56A  #B8843A  #8E6420  #6A4A14  #46300A
berry:    #F6E8EE  #E4B0C4  #C47D8A  #A85068  #8A3850  #6A2840  #461828
stone:    #F0F2EC  #D0D2C8  #A8AAA0  #7A7C72  #545648  #3A3C32  #222418
sky:      #E4F0FA  #A8CCE8  #72AAD0  #4888B8  #306A96  #204E72  #12344A
amber:    #FAF0DA  #F0D498  #E0B460  #C89430  #9A7018  #705010  #483408
red:      #FCE8E8  #F0B0B0  #E07878  #CC4444  #A03030  #782020  #501010
lavender: #F0ECF6  #CEC4E0  #A898C4  #8470A8  #645088  #483868  #302448
```

---

## Light Theme

### Backgrounds
```
background.primary:    #FCFCFA   (editor, main bg — near-white warm)
background.secondary:  #F6F7F2   (surfaces, code blocks, cards)
background.tertiary:   #EEEFE8   (sidebars, panels)
background.hover:      #E8EAE2   (hover states)
background.selected:   #E0E2DA   (selected items, active line)
```

### Text
```
text.primary:          #2A2E22   (body text)
text.secondary:        #6A6E62   (muted labels, descriptions)
text.tertiary:         #9A9E92   (hints, placeholders)
text.disabled:         #C0C2B8   (disabled UI)
```

### Borders
```
border.primary:        #C0C2B8   (strong borders)
border.secondary:      #D4D6CC   (subtle borders)
border.tertiary:       #E4E6DE   (faint separators)
```

### Dimmed Syntax (MD punctuation: # ** [[ ]] > --- ~~)
```
dimmed.syntax:         #C4C6BC
```

### Headings (green ramp, H1 darkest → H6 lightest)
```
heading.h1:            #3D7A1A   font-weight: 800
heading.h2:            #4D8A28   font-weight: 700
heading.h3:            #5D9A36   font-weight: 600
heading.h4:            #6DAA44   font-weight: 500
heading.h5:            #7DBA54   font-weight: 500
heading.h6:            #8DCA64   font-weight: 400
```

### Inline Formatting
```
markup.bold:           #2A2E22   font-weight: 700 (same as body, weight does the work)
markup.italic:         #2A2E22   font-style: italic
markup.strikethrough:  #A8AAA0   strikethrough: true
markup.code.fg:        #9A7018   (inline code text)
markup.code.bg:        #EEEFE8   (inline code background)
markup.math:           #4D8A28   font-style: italic
```

### Links
```
link.wikilink:         #2E7E8C   underline: true
link.wikilink.broken:  #A03030   underline: #A03030, strikethrough: true
link.url:              #2E7E8C   underline: true
link.url.title:        #1E6472   (link text in [text](url))
```

### Tags
```
tag.fg:                #8E6420
tag.bg:                #F4EDE2
```

### Callouts
```
callout.marker:        #8A3850   font-style: italic
callout.note:          #2E7E8C
callout.warning:       #9A7018
callout.danger:        #A03030
callout.tip:           #4D8A28
callout.info:          #306A96
```

### Checkboxes
```
checkbox.done.fg:      #A8AAA0   strikethrough: true
checkbox.done.marker:  #4D8A28   (the ✓ hint)
checkbox.todo.fg:      #2A2E22
checkbox.todo.marker:  #A8AAA0   (the ○ hint)
```

### Frontmatter
```
frontmatter:           #A8AAA0   font-style: italic
```

### Code Syntax (for programming languages)
```
code.keyword:          #306A96   (fn, let, for, if, return, pub, use)
code.function:         #3D7A1A   (function names, method calls)
code.type:             #645088   (types, traits, structs, enums)
code.string:           #9A7018   (string literals)
code.number:           #8E6420   (numeric literals)
code.constant:         #6A4A14   (constants, booleans, static)
code.variable:         #2A2E22   (variables, parameters)
code.property:         #1E6472   (struct fields, properties)
code.operator:         #6A6E62   (=, +, -, ::, =>)
code.punctuation:      #7A7C72   (brackets, commas, semicolons)
code.comment:          #A8AAA0   font-style: italic
code.comment.doc:      #5D9A36   font-style: italic
code.attribute:        #8A3850   (decorators, annotations, #[...])
code.tag:              #3D7A1A   (HTML/XML tags)
code.tag.attribute:    #8E6420   (HTML/XML attributes)
code.embedded:         #9A7018   (template literals, interpolation)
code.regex:            #6A4A14   (regular expressions)
code.escape:           #8A3850   (escape sequences \n \t)
```

### Diagnostics & Status
```
error:                 #A03030
warning:               #9A7018
info:                  #306A96
success:               #4D8A28
hint:                  #6A6E62
```

### UI Accents
```
accent.primary:        #4D8A28   (cursor, active tab indicator)
accent.secondary:      #2E7E8C   (find matches, secondary highlights)
selection.bg:          #DCE8D4   (text selection — green tint)
find.match.bg:         #F0E4D0   (find match highlight — amber tint)
bracket.match.bg:      #E0E2DA   (matching bracket highlight)
git.added:             #4D8A28
git.modified:          #306A96
git.deleted:           #A03030
git.conflict:          #9A7018
```

### Full Ramps (same as dark, used at different stops)
```
green:    #E8F5E0  #B8DC94  #8FBF6A  #6DA044  #4D8A28  #3D7A1A  #265010
teal:     #E0F4F4  #A0D8D8  #7CB5C4  #4A9AAE  #2E7E8C  #1E6472  #104448
earth:    #F6EDE0  #E4CCA6  #D4A56A  #B8843A  #8E6420  #6A4A14  #46300A
berry:    #F6E8EE  #E4B0C4  #C47D8A  #A85068  #8A3850  #6A2840  #461828
stone:    #F0F2EC  #D0D2C8  #A8AAA0  #7A7C72  #545648  #3A3C32  #222418
sky:      #E4F0FA  #A8CCE8  #72AAD0  #4888B8  #306A96  #204E72  #12344A
amber:    #FAF0DA  #F0D498  #E0B460  #C89430  #9A7018  #705010  #483408
red:      #FCE8E8  #F0B0B0  #E07878  #CC4444  #A03030  #782020  #501010
lavender: #F0ECF6  #CEC4E0  #A898C4  #8470A8  #645088  #483868  #302448
```
