# Compliance Guide

```admonish warning
This Compliance section is a work in progress. Content, mappings, and assertions may evolve and change between versions.
```

This guide explains how to write custom asserts for Roslyn compliance tests using our `bsharp_compliance_testing` helpers. It focuses on structural checks and optional diagnostics checks.

## Where custom asserts live

- File: `roslyn_testing/bsharp_compliance_testing/src/custom_asserts/after_parse.rs`
- Entry points:
  - `after_parse(...)`: lightweight per-case hook for structural or source-based assertions.
  - `after_parse_with_expected(...)`: adds an optional diagnostics expectation integration.
- Helper macro:
  - `assert_when! { ... }` — enables concise, per-case matching on module/file/method/index.

## Using `assert_when!`

The macro lets you target a specific Roslyn case by module name, Roslyn filename, Roslyn test method name, and case index (0-based within the method).

Example for a Statement case:

```rust
use crate::custom_asserts::after_parse::{after_parse, CaseData};

pub fn after_parse(
    module: &str,
    roslyn_file: &str,
    roslyn_method: &str,
    idx: usize,
    case: CaseData<'_>,
) {
    assert_when!(
        module = "statement_parsing_tests",
        roslyn_file = "StatementParsingTests",
        roslyn_method = "TestSwitchStatementWithNullableTypeInPattern3",
        idx = 2,
        Statement(ast, src) {
            // Add your targeted assertions here
            assert!(src.contains("switch"));
            // Optional: pattern-match on `ast` when you need structure checks
            // match ast { /* ... */ }
        }
    );
}
```

Example for a File case (full `CompilationUnit` available):

```rust
use crate::custom_asserts::after_parse::{after_parse, CaseData};

pub fn after_parse(
    module: &str,
    roslyn_file: &str,
    roslyn_method: &str,
    idx: usize,
    case: CaseData<'_>,
) {
    assert_when!(
        module = "using_directive_parsing_tests",
        roslyn_file = "UsingDirectiveParsingTests",
        roslyn_method = "SimpleUsingDirectiveNamePointer",
        idx = 0,
        File(unit, src, original) {
            assert!(src.starts_with("using "));
            // `unit` is a &bsharp_syntax::ast::CompilationUnit
            // You can inspect its using directives or declarations if needed.
            assert!(unit.using_directives.len() >= 1);
            let _ = original; // original Roslyn text when provided
        }
    );
}
```

## Diagnostics integration

If the generator attaches expected diagnostics, use `after_parse_with_expected(...)` to compare counts when diagnostics are supported by the build:

```rust
use crate::custom_asserts::after_parse::{after_parse_with_expected, CaseData};

pub fn my_integration(
    module: &str,
    roslyn_file: &str,
    roslyn_method: &str,
    idx: usize,
    expected: Option<crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics>,
    case: CaseData<'_>,
) {
    // Runs custom case asserts and then asserts diagnostics count when available
    after_parse_with_expected(module, roslyn_file, roslyn_method, idx, expected, case);
}
```

Notes:
- When diagnostics support is disabled, the helper asserts with an explicit "unimplemented" fallback to avoid silent failures.
- Keep asserts precise and self-contained; prefer checking concrete substrings or specific AST facts.

## Best practices

- Keep assertions small and focused. Use `assert_when!` blocks per case.
- Avoid brittle assumptions: prefer checking presence/shape over exact token trivia.
- Match our naming convention in any structure references (PascalCase, no `Syntax` suffix).
- Fail fast with clear messages; do not silently swallow errors.
