use bsharp_syntax::statements::statement::Statement;
use crate::custom_asserts::roslyn_asserts::{ExpectedDiagnostics, assert_diagnostics_unimplemented};

pub enum CaseData<'a> {
    Statement {
        ast: &'a Statement,
        src: &'a str,
    },
    File {
        unit: &'a bsharp_syntax::ast::CompilationUnit,
        src: &'a str,
        original: Option<&'a str>,
    },
    /// Used when parsing failed and we still want to run diagnostics assertions.
    Empty,
}

// Macro helpers to write concise, per-case asserts.
// Usage inside `after_parse`: call `assert_when!(...)` for each case you want to customize.
macro_rules! assert_when {
    (
        module = $m:literal,
        roslyn_file = $rf:literal,
        roslyn_method = $rm:literal,
        idx = $i:expr,
        Statement($ast:ident, $src:ident) { $($body:tt)* }
    ) => {{
        if module == $m && roslyn_file == $rf && roslyn_method == $rm && idx == $i {
            if let CaseData::Statement { ast: $ast, src: $src } = case {
                $($body)*
            }
        }
    }};
    (
        module = $m:literal,
        roslyn_file = $rf:literal,
        roslyn_method = $rm:literal,
        idx = $i:expr,
        File($unit:ident, $src:ident, $orig:ident) { $($body:tt)* }
    ) => {{
        if module == $m && roslyn_file == $rf && roslyn_method == $rm && idx == $i {
            if let CaseData::File { unit: $unit, src: $src, original: $orig } = case {
                $($body)*
            }
        }
    }};
}

pub fn after_parse(
    module: &str,
    roslyn_file: &str,
    roslyn_method: &str,
    idx: usize,
    case: CaseData<'_>,
) {
    let _ = (module, roslyn_file, roslyn_method, idx);

    // Add your custom asserts below using the `assert_when!` macro.
    // Example:
    // assert_when!(
    //     module = "statement_parsing_tests",
    //     roslyn_file = "StatementParsingTests",
    //     roslyn_method = "TestSwitchStatementWithNullableTypeInPattern3",
    //     idx = 2,
    //     Statement(ast, src) {
    //         assert!(src.contains("switch"));
    //     }
    // );

    // Default: no-op
    let _ = case;
}

/// New entry that allows passing expected diagnostics gathered from Roslyn test helpers.
/// This keeps the legacy `after_parse` API intact while enabling progressive integration.
pub fn after_parse_with_expected(
    module: &str,
    roslyn_file: &str,
    roslyn_method: &str,
    idx: usize,
    expected: Option<ExpectedDiagnostics>,
    case: CaseData<'_>,
) {
    // First, run any custom per-case asserts (kept for developer convenience)
    after_parse(module, roslyn_file, roslyn_method, idx, case);
    // Then handle diagnostics expectation (stub for now)
    if let Some(exp) = expected.as_ref() {
        assert_diagnostics_unimplemented(exp);
    }
}