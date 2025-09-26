# B# Parser Grammar Snapshot

This document summarizes the grammar that B# currently supports, the precedence tiers, and the key entry points in the parser. It also lists the most important lookahead and disambiguation rules we enforce with references to the implementation.

## Entry points

- Expression entry: `parse_expression()`
  - Location: `src/parser/expressions/primary_expression_parser.rs` (re-exported by `src/parser/expressions/mod.rs`)
- Statement entry: `parse_statement()`
  - Location: `src/parser/statement_parser.rs`
- Type entry: `parse_type_expression()`
  - Location: `src/parser/types/type_parser.rs`
- Pattern entry: `parse_pattern()`
  - Location: `src/parser/expressions/pattern_parser.rs`
- Compilation unit entry: `Parser::parse(..)` façade
  - Location: `src/parser/facade.rs`

All entry points return `BResult<&str, T>` where `T` is the AST node type and the error type is `nom_supreme::error::ErrorTree<&str>` for rich, contextual diagnostics.

## Precedence tiers (expressions)

From lowest (binds last) to highest (binds first). Each tier consumes the next-higher tier and folds left-associatively unless specified.

1. Conditional (ternary) `?:`
2. Null-coalescing `??`
3. Logical OR `||`
4. Logical AND `&&`
5. Bitwise OR `|`
6. Bitwise XOR `^`
7. Bitwise AND `&`
8. Equality `== !=`
9. Relational and type checks `< <= > >= is as`
10. Shift `<< >>`
11. Additive `+ -`
12. Multiplicative `* / %`
13. Range `.. ..^` (see Range notes)
14. Unary prefix `++ -- + - ! ~ & *`
15. Postfix (member access `.`, index `[]`, invocation `()`, null-conditional `?.`, postfix `++ --`)
16. Primary (literals, identifiers, parenthesized/tuples, `this`, `new`, `nameof`, `default`, `throw` expression)

Implementation:
- Precedence chain modules under `src/parser/expressions/*_expression_parser.rs`.
- Left-associative folding follows Nom patterns (`many0` + fold) to avoid recursive right-branching.

## Disambiguation and lookahead rules

- Range vs dot vs float
  - Member access `.` must not be captured as range `..` or as a float literal dot.
  - Range starting with `..` is attempted first (see `parse_range_expression_or_higher`), then `operand .. operand`.
  - Tests: `tests/parser/expressions/lookahead_boundaries2_tests.rs`.

- Cast vs parenthesized expression
  - Cast parsing only commits when an operand after `)` parses successfully; otherwise the parser backtracks and treats `(expr)` as a parenthesized expression.
  - Implementation: `src/parser/expressions/unary_expression_parser.rs`.

- Prefix `++/--` vs single `+/-`
  - Multi-char prefix ops take priority over single-char versions.
  - Implementation: `src/parser/expressions/unary_expression_parser.rs`.

- Ternary `?:` vs `?.` and `??`
  - Boundaries and peek helpers ensure operators are not misinterpreted.
  - Tests: `tests/parser/expressions/ambiguity_tests.rs`, `lookahead_boundaries2_tests.rs`.

- Lambda vs generics vs tuple/paren
  - Ambiguity tests ensure `id<...>(...)` is parsed as generic invocation, while `x => x` remains a lambda; statement-like fragments are rejected at expression entry.
  - Tests: `tests/parser/expressions/lookahead_boundaries2_tests.rs`.

- New expressions
  - Supports typed `new T(...)`, object initializers, collection initializers, and target-typed `new(...)` (see `target_typed_new_tests.rs`).
  - Tests: `tests/parser/expressions/new_expression_tests.rs`, `tests/parser/expressions/target_typed_new_tests.rs`.

## Patterns

Supported pattern forms (see `src/parser/expressions/pattern_parser.rs`):
- Discard `_`
- `var` patterns (`var x`)
- Type patterns (`int x`, `Point p`)
- Relational patterns (`> 5`, `<= 10`, `== 0`, `!= null`)
- Logical patterns (`and`, `or`, `not`), left-associative folding
- Parenthesized and tuple patterns `(p1, p2, ...)`
- List patterns `[p1, .., pN]` with slice elements
- Property patterns `{ Name: p1, Age: p2 }` with optional type prefix `Type { ... }`
- Positional patterns `Type(p1, p2, ...)`

Integration points:
- `is` pattern expressions: `parse_is_pattern_expression` in `switch_expression_parser.rs`.
- `switch` expressions with pattern arms: `parse_switch_expression`.
- Tests: `tests/parser/expressions/pattern_matching_tests.rs`.

## Types

- Primitive, qualified reference, generic types
- Nullable `T?`, arrays `T[]`/multidimensional `T[,]`, pointers `T*`
- Function pointers `delegate*` and constraints
- Tests: `tests/parser/types/advanced_type_tests.rs`, `docs/parser/types.md`.

## Error handling and diagnostics

- All parsers return `BResult` with `ErrorTree<&str>`.
- Contexts added via `syntax::parser_helpers::context()` and helpers such as `bws`, `bdelimited`, `bpeek`, `peek_keyword`.
- Pretty printer for `ErrorTree`: `syntax::errors::format_error_tree`.
- Test helper `expect_ok(input, res)` to produce readable errors on failure.
- Snapshot style tests: `tests/parser/expressions/invalid_diagnostics_tests.rs`.

## Conventions

- Always parse through whitespace/comments using `bws(..)` combinator wrappers.
- Guard closing delimiters with `cut(..)` once committed (e.g., `)`, `]`, `}` on delimited constructs).
- Use `keyword(..)` for word-boundary-safe keyword parsing.
- Prefer small, specific parsers composed via helpers from `syntax::parser_helpers`.

## References

- Helpers: `src/syntax/parser_helpers.rs`
- Expressions: `src/parser/expressions/`
- Statements: `src/parser/expressions/statements/`
- Types: `src/parser/types/type_parser.rs`
- Patterns: `src/parser/expressions/pattern_parser.rs`
- Docs: `docs/parser/expressions.md`, `docs/parser/error-handling.md`, `docs/parser/types.md`, `docs/parser/declarations.md`, `docs/parser/statements.md`.
