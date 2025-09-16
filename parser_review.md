# Parser and Syntax Code Review – Granular Refactor Tasks

This plan focuses on deduplication, structure, clarity, and uniform error/whitespace handling for the parser and syntax layers. It references nom/nom-supreme best practices and targets concrete modules and functions in this repo.

References used (docs MCP):
- nom “cut” (commit parse branch): https://docs.rs/nom/8.0.0/nom/combinator/fn.cut.html
- nom many0/fold patterns for chains: https://docs.rs/nom/8.0.0/nom/multi/fn.many0.html and https://docs.rs/nom/8.0.0/nom/multi/fn.fold_many_m_n.html
- nom-supreme ErrorTree and ParserExt.context: https://docs.rs/nom-supreme/0.8.0/nom_supreme/error/type.ErrorTree.html

Note: We already use `ErrorTree` across `BResult` and `ParserExt.context`, but usage is inconsistent in places.


## Cross-cutting foundations
- [ ] Consolidate error handling to nom-supreme `ErrorTree` and `ParserExt.context` everywhere.
  - Files still importing/using `nom::error::context` (e.g., `src/parser/statements/block_statement_parser.rs`, `src/parser/statements/break_statement_parser.rs`) should switch to `crate::syntax::parser_helpers::context` which wraps `ErrorTree`.
  - Enforce rule of thumb: All public entry-points and significant sub-parsers should add context boundaries for better diagnostics.
- [ ] Normalize whitespace handling.
  - Prefer `bws(...)` from `src/syntax/parser_helpers.rs` to avoid ad-hoc `ws` and `trim_start` checks.
  - Replace direct `nom::character::complete::char` and `nom::bytes::complete::tag` with `bchar`/`btag` wrapped by `bws` or `bdelimited` when appropriate.
  - Use `comment_parser::ws` only when we truly need to peek or precede; otherwise, wrap with `bws` consistently.
- [ ] Unify keyword boundary semantics.
  - Today `parser_helpers::keyword()` enforces word boundaries on `[a-zA-Z0-9_]`, but some modules (e.g., `types/type_parser.rs` via local `word_boundary`) only check against `alpha1`. Replace ad-hoc boundary checks with `keyword()` usage to avoid partial matches like `int32`.
- [ ] Remove unused/legacy error type(s).
  - `BSharpParseError` in `src/syntax/errors.rs` appears unused. Either remove it or document why it remains. Keep `BResult = IResult<_, _, ErrorTree<_>>` as the only error channel.


## Expressions – dedup and precedence stack
- [ ] Introduce a generic left-associative chain builder to remove repeated patterns.
  - Targets: `parse_logical_or_expression_or_higher`, `parse_logical_and_expression_or_higher`, `parse_bitwise_or_expression_or_higher`, `parse_bitwise_xor_expression_or_higher`, `parse_bitwise_and_expression_or_higher`, `parse_relational_expression_or_higher`, `parse_shift_expression_or_higher`, `parse_additive_expression_or_higher`, `parse_multiplicative_expression_or_higher` in `src/parser/expressions/expression_parser.rs`.
  - Implement a helper like `left_chain(next: P, op: Op) -> P` using `many0`/`fold` patterns (see nom docs above). Replace while-loops with the helper to reduce boilerplate and bug surface.
- [ ] Eliminate duplicate primary expression implementations.
  - Currently we have:
    - A comprehensive `parse_primary_expression` inside `src/parser/expressions/expression_parser.rs` (pub(crate)).
    - A simplified `src/parser/expressions/primary_parser.rs::parse_primary_expression` that only handles literals/vars/paren.
  - `src/parser/expressions/mod.rs` re-exports the simple version, but `postfix_expression_parser.rs` explicitly uses the comprehensive one via a full path.
  - Action:
    - Choose a single canonical function (the comprehensive one in `expression_parser.rs`).
    - Update `expressions/mod.rs` to re-export the canonical one and remove or repurpose the simplified module (e.g., keep only leaf helpers or delete if unused).
    - Grep for all usages and adjust imports to the canonical export.
- [ ] Standardize `cut()` usage in expression delimiters for sharper errors.
  - Ensure closing tokens on delimited constructs are guarded by `cut(...)` to avoid misleading backtracking (see nom “cut” docs). Examples already present:
    - `interpolated_string` and `postfix` indexing/invocation use `cut(...)` correctly; verify and extend to ternary `? :`, tuples, and parens where missing.
- [ ] Normalize unary/postfix interplay.
  - `unary_expression_parser.rs` delegates to `postfix_expression_parser` when no prefix matches. Verify null-conditional operators and index-from-end `^` are handled consistently across both.
  - Confirm no ambiguity with `range` `..` and `^` index from end (ensure ordering and lookahead are correct and documented).


## Statements – structure and duplication
- [ ] Collapse `parse_group1_with_block` vs `parse_group1_without_block` into a single parametrized builder.
  - File: `src/parser/statement_parser.rs`.
  - Create a helper that takes a flag or two function references (with/without block) to generate the alt list; DRY the duplicate context strings.
  - Keep `parse_statement_for_block(_ws)` as a thin wrapper that calls the shared builder without block.
- [ ] Apply consistent `context` and `bws` across all statements.
  - Files with mixed usage:
    - `block_statement_parser.rs` uses `nom::error::context`; switch to helpers.
    - Verify `if_statement_parser.rs` and others consistently wrap inner delimited parts with `cut` on closing parens/braces.


## Types – consistency and boundaries
- [ ] Replace local `word_boundary` in `src/parser/types/type_parser.rs` with `keyword()`-backed checks or extend it to match the same `[A-Za-z0-9_]` rule as `parser_helpers::is_word_boundary_after`.
- [ ] Replace direct `tag(...)` with `btag(...)` when we also expect surrounding whitespace/comments; otherwise document why not.
- [ ] Review function-pointer and generic type parsing error messages.
  - Ensure expectations (e.g., “expected at least one type argument”) are expressed via `ErrorTree::Base` consistently, with contexts added at call sites.


## Declarations – structure and recovery
- [ ] Standardize whitespace and delimiter helpers in `src/parser/declarations/type_declaration_parser.rs`.
  - Replace explicit `ws` calls in the header parsing with `bws` around sub-parsers to centralize whitespace policy.
  - Ensure `parse_class_body` uses `bdelimited` with `cut` on the closing brace to commit once the opening brace is seen.
- [ ] Review error recovery in `skip_to_recovery_point`.
  - Audit brace-depth logic for correctness with nested generics/attributes where semicolons/braces may appear in expressions or attributes.
  - Document invariants and clearly scope where recovery can skip (top-level only) to avoid masking bugs.
- [ ] Validate interface-specific constraints.
  - We build `ErrorTree` manually for illegal interface members (e.g., accessors with bodies). Keep this, but add contexts on callers so the trace shows “interface member” -> “property” etc.


## Identifier and keywords
- [ ] Centralize the keyword list.
  - `src/parser/identifier_parser.rs` contains a static list. Ensure it feeds both the identifier verifier and `keyword()` to avoid drift.
  - Consider a `keywords.rs` module re-exported by `syntax` so both parser and future analysis can share the same source of truth.
- [ ] Improve qualified name parsing docs and contexts.
  - `parse_qualified_name` should add a higher-level context like “qualified name” everywhere it’s called to make errors actionable.


## Module structure and re-exports
- [ ] Make `src/syntax/mod.rs` purely API and re-exports (no logic), which it mostly is today.
  - Ensure any test-only helpers stay in `syntax/test_helpers.rs`.
- [ ] Remove/relocate outdated `src/syntax/navigation.rs`.
  - It references `crate::parser::ast::*` and appears superseded by `analysis` traits (already re-exported by `syntax/mod.rs`).
  - Either remove, or move under `analysis/` with a compatibility layer, or gate it behind a feature for legacy consumers.


## Top-level source parsing (`csharp.rs`)
- [ ] Replace ad-hoc loops over `using` and file-scoped namespace detection with combinators for consistency.
  - Convert the “while starts_with("using")” loop into a `many0` over `parse_using_directive` with `bws` and proper backtracking/commit semantics.
  - Use lookahead parsers instead of string peeks for file-scoped namespace detection; wrap with context and `cut` once committed.
- [ ] Ensure trailing input handling and logging are behind feature flags or debug-only to reduce noise in production.


## Testing and safety nets
- [ ] Add parser property tests for operator precedence chains.
  - Generate random sequences of operands/operators to assert associativity and precedence invariants (esp. `??`, `?:`, shifts, bitwise vs logical).
- [ ] Add ambiguity tests.
  - `(` disambiguation between tuple vs parenthesized; `^` vs XOR; `?.` vs `? :` ternary; range `..` vs slice.
  - Verify current ordering and peek logic is preserved after refactors.
- [ ] Add golden tests for error messages.
  - Validate that contexts from `ParserExt.context` and committed failures via `cut` produce clear `ErrorTree` structures (compare JSON snapshots in `debug_output/`).


## Performance and cleanliness
- [ ] Remove manual `.trim_start()` and `starts_with()` flows in favor of combinators to avoid accidental consumption or mis-detection.
- [ ] Prefer smaller local parsers and compose via helpers; avoid long functions (>200 LoC). Split `expression_parser.rs` into submodules if needed but keep a single public entry.
- [ ] Ensure no parser leaks non-deterministic behavior (all pure functions over input slices with no global state).


## Documentation and conventions
- [ ] Document parser invariants per module.
  - For each module (expressions/statements/types/declarations), list:
    - What whitespace policy it expects (`bws` everywhere unless stated).
    - Where `cut` is used and why.
    - Error contexts added.
    - Ordering constraints among alternatives.
- [ ] Add CONTRIBUTING notes for adding new syntax.
  - Require new parsers to use `context`, `bws`, `bdelimited`, and `bseparated_*` helpers; include examples.


## Acceptance checklist (definition of done)
- [ ] No duplicate `parse_primary_expression`; a single canonical export.
- [ ] All direct `nom::error::context` replaced with `syntax::parser_helpers::context`.
- [ ] Keyword boundaries unified via `keyword()`.
- [ ] Statement group duplication removed.
- [ ] Expression binary chain parsing implemented via a generic helper.
- [ ] `cut()` applied for all critical closing delimiters to prevent misleading backtracking.
- [ ] Dead/legacy code (e.g., `BSharpParseError`, outdated navigation) is removed or isolated.
- [ ] Tests added for precedence/ambiguities and error messages.
- [ ] Docs updated reflecting the conventions.
