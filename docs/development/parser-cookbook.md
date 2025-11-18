# Parser Cookbook

Practical recipes for nom-based parsers in `bsharp_parser`.

---

## Spanned-first policy

*   All public parser entrypoints return `Spanned<T>` so callers have precise source ranges for AST nodes.
*   Internals should prefer spanned parsers as well to preserve spans through transformations.
*   When you only need the inner value, map via `.node`.

Examples:

```
// Prefer the spanned variant and map to inner node when spans are not needed
let (rest, expr) = nom::sequence::delimited(ws, parse_expression_spanned, ws)
    .map(|s| s.node)
    .parse(input)?;

// Lists of expressions: collect inner nodes
let (rest, args) = parse_delimited_list0(
    |i| delimited(ws, tok_l_paren(), ws).parse(i),
    |i| delimited(ws, parse_expression_spanned, ws).map(|s| s.node).parse(i),
    |i| delimited(ws, tok_comma(), ws).parse(i),
    |i| delimited(ws, tok_r_paren(), ws).parse(i),
    false,
    true,
).parse(input)?;
```

### Parsable trait

*   For one-shot parsing of a type to `Spanned<Self>`, implement or use the crate’s `Parsable` abstraction (where available) instead of bespoke entrypoints.
*   This keeps a consistent contract across the parser and simplifies tests and tools that need spans.

---

## Conventions

*   Use `Span<'a>` and `BResult<'a, T>` from `bsharp_parser::syntax` modules.
*   Prefer small, composable parsers and add `context()` labels.
*   Use `cut()` to avoid misleading backtracking after committing to a branch.

```
use bsharp_parser::syntax::span::Span;
use bsharp_parser::syntax::errors::BResult;
use nom::{IResult, branch::alt, bytes::complete::tag, character::complete as cc, combinator::{all_consuming, complete, map}, sequence::{delimited, preceded, terminated, tuple}};
use nom_supreme::ParserExt; // for .context(), .cut()
```

---

## Identifier

```
fn identifier(input: Span) -> BResult<String> {
    // very simplified: letter (letter|digit|_)*
    map(
        tuple((cc::alpha1, cc::alphanumeric0)),
        |(h, t): (&str, &str)| format!("{}{}", h, t)
    ).context("identifier").parse(input)
}
```

---

## Comma-Separated List

```
use nom::multi::separated_list0;

fn comma_sep<T, F>(item: F) -> impl FnMut(Span) -> BResult<Vec<T>>
where F: Fn(Span) -> BResult<T> {
    separated_list0(cc::multispace0.and(tag(",")).and(cc::multispace0), item)
}
```

---

## Delimited Braces Block

```
fn lbrace(i: Span) -> BResult<()> { map(tag("{"), |_| ()).context("'{'").parse(i) }
fn rbrace(i: Span) -> BResult<()> { map(tag("}"), |_| ()).context("'}'").parse(i) }

fn block<T, F>(mut inner: F) -> impl FnMut(Span) -> BResult<Vec<T>>
where F: FnMut(Span) -> BResult<Vec<T>> {
    move |input| {
        delimited(
            lbrace.context("block start"),
            // prevent backtracking past '}' so the missing brace is reported
            inner.cut(),
            rbrace.cut().context("block end")
        ).parse(input)
    }
}
```

---

## Using complete() for Tokens

```
use nom::bytes::streaming::take;
use nom::combinator::complete;

fn exactly_n(n: u8) -> impl FnMut(Span) -> BResult<Span<'_>> {
    move |input| complete(take(n)).context("exactly_n").parse(input)
}
```

---

## all\_consuming at File Level

```
use nom::combinator::all_consuming;

fn parse_file(input: Span) -> BResult<File> {
    all_consuming(file_parser).parse(input)
}
```

---

## Precedence Chain Skeleton

```
fn primary(i: Span) -> BResult<Expr> { /* literals, names, parenthesized */ }
fn postfix(i: Span) -> BResult<Expr> { /* member access, invocation */ }
fn unary(i: Span) -> BResult<Expr> { /* + - ! ~ */ }
fn multiplicative(i: Span) -> BResult<Expr> { /* * / % */ }
fn additive(i: Span) -> BResult<Expr> { /* + - */ }
fn relational(i: Span) -> BResult<Expr> { /* < > <= >= */ }
fn equality(i: Span) -> BResult<Expr> { /* == != */ }
fn assignment(i: Span) -> BResult<Expr> { /* = += -= */ }

// Entry point used by statement parsers
fn expression(i: Span) -> BResult<Expr> { assignment(i) }
```

---

## Context Labels and Cuts

```
fn class_declaration(i: Span) -> BResult<ClassDecl> {
    preceded(
        tag("class").context("keyword 'class'"),
        tuple((
            identifier.cut().context("class name"),
            // ... type params, base list
        ))
    ).context("class declaration").map(|(name, ..)| ClassDecl { name }).parse(i)
}
```

---

## Tips

*   **Whitespace**: Prefer explicit `multispace0`/`multispace1` at boundaries to avoid accidental greedy matches.
*   **Error messages**: Keep `context()` labels concise and domain-specific (e.g., "parameter list").
*   **Backtracking**: Insert `cut()` after committing to a branch to stop alt from swallowing errors.