# Extending Syntax (New Nodes)
How to add new AST node types to `bsharp_syntax`.

---

## 1. Define the Node

- Add a struct or enum in the relevant module under `src/bsharp_syntax/src/`.
- Derive `bsharp_syntax_derive::AstNode` so it participates in traversal and rendering.

```rust
#[derive(bsharp_syntax_derive::AstNode, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterpolatedString {
    pub parts: Vec<InterpolatedPart>,
}

#[derive(bsharp_syntax_derive::AstNode, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterpolatedPart {
    Text(String),
    Expr(Expression),
}
```

The derive implements `AstNode` and auto-generates `children()` that pushes nested nodes.

---

## 2. Implement Emit (Optional)

If the node needs to be formatted back to C#, implement `Emit` in `bsharp_syntax` emitters.

```rust
impl crate::emitters::emit_trait::Emit for InterpolatedString {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError> {
        cx.token(w, "$")?;
        cx.bracketed(w, '"', '"', || {
            for p in &self.parts { p.emit(w, cx)?; }
            Ok(())
        })
    }
}
```

Add per-part emitters in the same or nearby module (e.g., `emitters/expressions/...`).

---

## 3. Wire Up Parser (in `bsharp_parser`)

- Add a parser in `src/bsharp_parser/src/expressions/...` that constructs the new node.
- Use `Span`-based parsers (`bsharp_parser::syntax::span::Span`).
- On errors, rely on helpers and contexts so `format_error_tree()` is informative.

---

## 3a. Add Keywords & Tokens

- Define keyword helpers using `define_keyword_pair!` in `src/bsharp_parser/src/keywords/`.
- If a new reserved word, add it to `KEYWORDS` (identifier filtering).
- Use `kw_*()`/`peek_*()` in parsers, wrapped with `ws()` at boundaries, and insert `.cut()` after commitment.

See: `docs/parser/keywords-and-tokens.md` for the macro and examples.

---

## 3b. Use Syntax Parsers (Whitespace/Lists)

- Whitespace/comments: `syntax/comment_parser.rs` (`ws()`, `parse_whitespace_or_comments()`)
- Lists: `syntax/list_parser.rs` for delimited/separated lists
- Tokens: prefer `nom_supreme::tag::complete::tag()` and compose with `preceded/terminated/delimited` and `ws()`

Example token with trivia:
```rust
use nom::{combinator::map, sequence::delimited};
use nom_supreme::tag::complete::tag;
use crate::syntax::comment_parser::ws;

map(delimited(ws, tag(","), ws), |_| ())
```

---

## 4. Tests (`bsharp_tests`)

- Create tests under `src/bsharp_tests/src/parser/...` verifying the node appears in the AST.
- Add formatter round-trip tests if `Emit` is implemented.

```rust
#[test]
fn interpolated_string_ast() {
    let src = r#"class C { void M(){ var s = $"x={x}"; } }"#;
    let (cu, _spans) = bsharp_parser::facade::Parser::new().parse_with_spans(src).unwrap();
    // Use Query to find InterpolatedString once parser supports it
}
```

---

## 5. Visualization (Optional)

Graph views require no changes: `to_text`, `to_mermaid`, and `to_dot` use `AstNode` traversal.

---

## Tips

- **Box recursion**: Use `Box<T>` for recursive enum variants.
- **Keep primitives out**: Store `String`, `bool`, numbers as payload only; derive will skip them.
- **Naming**: Use PascalCase node names; no `Syntax` suffix.
