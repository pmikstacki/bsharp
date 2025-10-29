# Spanned-first Parsers

This project follows a spanned-first policy for all parser entrypoints. Public parsers return `Spanned<T>` so every AST value carries precise source ranges for diagnostics, tooling, and downstream analysis.

---

## Rationale

- Rich diagnostics: precise byte and line/column ranges for errors and UI highlighting.
- Uniform contract: tools and tests can rely on span presence everywhere.
- Safer refactors: span plumbing is not an afterthought.

---

## Usage Patterns

### 1) Prefer spanned entrypoints

```rust
// Prefer spanned variants
let (rest, s_expr) = parse_expression_spanned(input)?;
// Use inner value if spans are not needed at the call site
let expr = s_expr.node;
```

### 2) Map lists to inner nodes

```rust
use nom::sequence::delimited;

let (rest, args) = parse_delimited_list0(
    |i| delimited(ws, tok_l_paren(), ws).parse(i),
    |i| delimited(ws, parse_expression_spanned, ws).map(|s| s.node).parse(i),
    |i| delimited(ws, tok_comma(), ws).parse(i),
    |i| delimited(ws, tok_r_paren(), ws).parse(i),
    false,
    true,
).parse(input)?;
```

### 3) Statements

```rust
let (rest, s_stmt) = parse_statement_ws_spanned(input)?;
let stmt = s_stmt.node;
```

---

## Implementing new parsers

- Return `Spanned<T>` from public entrypoints.
- Compose with existing spanned parsers to retain spans through transformations.
- For adapters that must return unspanned values (e.g., legacy APIs), `.map(|s| s.node)` at the last possible boundary.
- Use `cut()` after committing to a branch to produce focused errors.
- Add `context("...")` labels on user-facing constructs.

Example:

```rust
use nom::sequence::delimited;
use nom_supreme::ParserExt;

pub fn parse_lambda_body(input: Span) -> BResult<LambdaBody> {
    nom::branch::alt((
        // block
        nom::combinator::map(parse_lambda_block_body, LambdaBody::Block),
        // expression
        nom::combinator::map(
            delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
            LambdaBody::ExpressionSyntax,
        ),
    ))
    .context("lambda body")
    .parse(input)
}
```

---

## Testing

- Prefer helpers that accept/return `Spanned<T>` in new tests.
- When asserting only structure, map to `.node` before comparison.
- For diagnostics, use the existing pretty printers (see `bsharp_parser::errors::format_error_tree` and `to_miette_report`).

---

## Migration Notes

- Old unspanned entrypoints are deprecated; use their `_spanned` counterparts.
- If a caller previously depended on unspanned types, add `.map(|s| s.node)`.
- For bulk changes: search for `parse_expression(` and `parse_statement(` and replace with spanned + `.node` mapping.
