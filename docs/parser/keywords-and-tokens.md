# Keywords and Tokens

Keyword and token helpers used by the parser.

---

## Keyword Pairs Macro

- **Location:** `src/bsharp_parser/src/keywords/mod.rs`
- **Macro:** `define_keyword_pair!` (macro_rules)
- Generates two functions per keyword:
  - `kw_<name>()` – consumes the keyword with word boundary check
  - `peek_<name>()` – non-consuming peek with surrounding whitespace/comments tolerated

```rust
// Define a pair:
// define_keyword_pair!(kw_public, peek_public, "public");
#[macro_export]
macro_rules! define_keyword_pair {
    ($kw_fn:ident, $peek_fn:ident, $lit:literal) => {
        pub fn $kw_fn() -> impl FnMut($crate::syntax::span::Span) -> $crate::syntax::errors::BResult<&str> {
            use nom::Parser as _;
            (|i: $crate::syntax::span::Span| {
                nom::combinator::map(
                    nom::sequence::terminated(
                        nom_supreme::tag::complete::tag($lit),
                        nom::combinator::peek(nom::combinator::not(
                            nom::character::complete::satisfy(|c: char| c.is_alphanumeric() || c == '_'),
                        )),
                    ),
                    |s: $crate::syntax::span::Span| *s.fragment(),
                )
                .parse(i)
            })
        }
        pub fn $peek_fn() -> impl FnMut($crate::syntax::span::Span) -> $crate::syntax::errors::BResult<&str> {
            use nom::Parser as _;
            (|i: $crate::syntax::span::Span| {
                nom::combinator::peek(
                    nom::sequence::delimited(
                        $crate::syntax::comment_parser::ws,
                        nom::combinator::map(
                            nom::sequence::terminated(
                                nom_supreme::tag::complete::tag($lit),
                                nom::combinator::peek(nom::combinator::not(
                                    nom::character::complete::satisfy(|c: char| c.is_alphanumeric() || c == '_'),
                                )),
                            ),
                            |_| $lit,
                        ),
                        $crate::syntax::comment_parser::ws,
                    ),
                )
                .parse(i)
            })
        }
    };
}
```

- Keyword modules live under `src/bsharp_parser/src/keywords/` (e.g., `access_keywords.rs`, `declaration_keywords.rs`, `linq_query_keywords.rs`, `type_keywords.rs`).
- Central keyword set: `KEYWORDS` in `keywords/mod.rs` and check `is_keyword()`.

---

## Token and Whitespace Helpers

- **Whitespace/comments:** `src/bsharp_parser/src/syntax/comment_parser.rs`
  - `ws()` parses optional whitespace and comments
  - `parse_whitespace_or_comments()` returns the consumed span text
- **List parsing:** `src/bsharp_parser/src/syntax/list_parser.rs` provides helpers for delimited/separated lists
- **Punctuation/tokens:** Use `nom_supreme::tag::complete::tag("...")` with:
  - `peek(not(satisfy(|c| ...)))` for word boundaries on keywords
  - `preceded/terminated/delimited` and `ws()` to control surrounding trivia

Example token with trivia discipline:
```rust
use nom::{combinator::map, sequence::delimited};
use nom_supreme::tag::complete::tag;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::span::Span;

pub fn comma(i: Span) -> BResult<()> {
    map(delimited(ws, tag(","), ws), |_| ()).parse(i)
}
```

---

## Usage Patterns

- Prefer `peek_*()` when branching without consuming input (e.g., lookahead for statement kind).
- After consuming a keyword with `kw_*()`, use `cut()` to prevent backtracking past the commitment.
- Always wrap top-level file parser with `all_consuming`.
- Keep context labels short and specific.

---

## Adding a New Keyword

1. Pick the right module in `keywords/` and add a `define_keyword_pair!` entry.
2. If it's a reserved word, add it to `KEYWORDS` (for identifier filtering).
3. Use `kw_*()`/`peek_*()` in parsers with `ws()` at boundaries.
4. Add tests under `src/bsharp_tests/src/parser/...` for both positive and negative cases.

---

## References

- Keyword macro and modules: `src/bsharp_parser/src/keywords/`
- Whitespace/comment parser: `src/bsharp_parser/src/syntax/comment_parser.rs`
- Lists: `src/bsharp_parser/src/syntax/list_parser.rs`
- Error formatting: `src/bsharp_parser/src/syntax/errors.rs`
