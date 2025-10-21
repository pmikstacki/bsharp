# Writing Parsers

Guidelines for implementing parsers in `bsharp_parser` using nom and spans.

---

## Spans & Result Type

- **Span:** `bsharp_parser::syntax::span::Span<'a>` (alias of `nom_locate::LocatedSpan<&'a str>`)
- **Error type:** `nom_supreme::error::ErrorTree<Span<'a>>`
- **Result alias:** `type BResult<'a, O> = IResult<Span<'a>, O, ErrorTree<Span<'a>>>` in `bsharp_parser::syntax::errors`

```rust
use bsharp_parser::syntax::errors::BResult;
use bsharp_parser::syntax::span::Span;
```

---

## Streaming vs Complete

nom supports streaming parsers by default. Use `nom::combinator::complete(parser)` to transform `Incomplete` into `Error` when you want a "complete input" behavior for a sub-parser (e.g., tokens, literals).

Example (from nom docs):
```rust
use nom::bytes::streaming::take;
use nom::combinator::complete;

let mut parser = complete(take(5u8));
assert_eq!(parser.parse("abcdefg"), Ok(("fg", "abcde")));
assert!(parser.parse("abcd").is_err());
```

At the top level, wrap file parsers with `nom::combinator::all_consuming` to ensure the entire input is consumed:

```rust
use nom::combinator::all_consuming;
let mut parser = all_consuming(file_parser);
```

---

## Error Contexts and Cuts

Use `nom_supreme` for structured errors and better messages:

- `context("label", p)` to push human-readable frames.
- `cut(p)` to prevent backtracking across critical boundaries and surface the right error.
- Our pretty-printer `format_error_tree(&source, &error_tree)` renders the tree with line/column and context stack.

```rust
use nom::{branch::alt, sequence::{preceded, terminated}};
use nom_supreme::context::ContextError;
use nom_supreme::ParserExt; // for .context(), .cut()

fn identifier(input: Span) -> BResult<String> { /* ... */ }
fn lbrace(input: Span) -> BResult<()> { /* ... */ }
fn rbrace(input: Span) -> BResult<()> { /* ... */ }

fn block(input: Span) -> BResult<Vec<Stmt>> {
    preceded(
        lbrace.context("block: '{'"),
        terminated(statements, rbrace.cut().context("block: '}'"))
    ).parse(input)
}
```

---

## Common Combinators

- `preceded(a, b)`, `terminated(a, b)`, `delimited(a, b, c)`
- `alt((p1, p2, ...))` for alternatives
- `tuple((p1, p2, ...))` to sequence
- `separated_list0(sep, item)` to parse comma-separated lists
- `map(p, f)` to build AST nodes

Prefer small, focused parsers composed with these combinators.

---

## Top-Level Entry Points

- Keep clear entry points for precedence chains (e.g., primary → postfix → binary → assignment).
- Use wrapper nodes for constructs like `New`, `Invocation`, `MemberAccess`, etc., to keep variants orthogonal in the AST (see `bsharp_syntax::expressions::expression.rs`).

---

## Testing Parsers

- Place tests in `src/bsharp_tests/src/parser/...`.
- Parse using `Parser::new().parse_with_spans(&source)` and assert expected AST shapes.
- On failure, pretty-print errors with `format_error_tree` to diagnose.

```rust
#[test]
fn parses_expression_statement() {
    let src = "class C { void M(){ Foo(1); } }";
    let (cu, _spans) = bsharp_parser::facade::Parser::new().parse_with_spans(src).unwrap();
    // Verify expected nodes using Query or pattern matching
}
```

---

## Tips

- **Return early with cut** after consuming a keyword to avoid misleading alternatives.
- **Use complete** for tokens/literals that must not be partial.
- **all_consuming** at file/compilation-unit to ban trailing garbage.
- **Context labels**: Be concise and specific; they surface in error messages and docs.

---

## References

- nom combinator `complete`: https://docs.rs/nom/8.0.0/nom/combinator/fn.complete.html
- nom combinator `all_consuming`: https://docs.rs/nom/8.0.0/nom/combinator/fn.all_consuming.html
