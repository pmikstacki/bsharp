# Writing Tests

How to write and organize tests for BSharp.

---

## Test Locations

- Parser and analysis tests live under `src/bsharp_tests/src/`.
- Prefer dedicated files per area, e.g.:
  - `src/bsharp_tests/src/parser/expressions/...`
  - `src/bsharp_tests/src/parser/statements/...`
  - `src/bsharp_tests/src/analysis/...`

---

## Parser Tests

- Use realistic C# snippets and assert AST shapes.
- Prefer external test helpers (avoid inline `#[cfg(test)]` in parser modules).

```rust
// Example skeleton
#[test]
fn parses_simple_invocation() {
    let source = "class C { void M() { Foo(1); } }";
    let (cu, _spans) = bsharp_parser::facade::Parser::new().parse_with_spans(source).unwrap();
    // Use Query or pattern matching to verify nodes
}
```

---

## Analysis Tests

- Run `AnalyzerPipeline::run_with_defaults` and inspect artifacts:
  - `AstAnalysis` metrics
  - CFG summary
  - Dependency summary

```rust
#[test]
fn counts_methods() {
    let src = "class C { void A(){} void B(){} }";
    let (cu, spans) = bsharp_parser::facade::Parser::new().parse_with_spans(src).unwrap();
    let mut session = bsharp_analysis::framework::AnalysisSession::new(
        bsharp_analysis::context::AnalysisContext::new("file.cs", src), spans);
    bsharp_analysis::framework::AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let metrics = session.artifacts.get::<bsharp_analysis::metrics::AstAnalysis>().unwrap();
    assert!(metrics.total_methods >= 2);
}
```

---

## Tips

- **Names**: Use descriptive test names; each file should focus on one area.
- **Fixtures**: Keep sources small and focused; add comments for intent.
- **Determinism**: Avoid relying on traversal order; query by type or match by name.
