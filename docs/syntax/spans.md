# Spans

This page explains how source spans are represented and returned during parsing.

---

## Span Type

- **Type:** `bsharp_parser::syntax::span::Span<'a>`
- **Alias:** `type Span<'a> = nom_locate::LocatedSpan<&'a str>;`
- Provides line/column offsets and byte positions for parser errors and mapping.

```rust
// src/bsharp_parser/src/syntax/span.rs
pub type Span<'a> = nom_locate::LocatedSpan<&'a str>;
```

---

## Parsing With Spans

Use the parser facade to parse and also get a span table for top-level declarations.

```rust
use bsharp_parser::facade::Parser;

let source = std::fs::read_to_string("Program.cs")?;
let (cu, spans) = Parser::new().parse_with_spans(&source)?;
```

- The return value is `(CompilationUnit, SpanTable)`.
- `SpanTable` maps top-level declarations to byte ranges for later mapping.

---

## Error Reporting

Pretty error formatting uses `Span` to print line/column with context:

```rust
use bsharp_parser::syntax::errors::format_error_tree;

let msg = format_error_tree(&source, &error_tree);
```

See: `docs/parser/error-handling.md` for details.
