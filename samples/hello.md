# MD Converter smoke test

A small Markdown document used to verify DOCX and PDF output.

## Features

- Headings, **bold**, *italic*, `inline code`
- Lists and tables
- Code blocks with monospaced font

## Code

```python
def greet(name: str) -> str:
    return f"Hello, {name}!"

print(greet("MD Converter"))
```

## Table

| Format | Engine            | Notes                       |
|--------|-------------------|-----------------------------|
| DOCX   | pandoc            | Direct conversion           |
| PDF    | pandoc + WebKit   | HTML stylesheet, paginated  |

> A blockquote, just to round things out.
