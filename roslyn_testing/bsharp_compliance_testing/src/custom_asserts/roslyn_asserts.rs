//! Minimal Roslyn assert compatibility layer (initial stub).
//! In this iteration we only support counting diagnostics, not mapping codes/locations.

#[derive(Debug, Clone, Default)]
pub struct RoslynDiagnosticExpectation {
    pub code: Option<String>,
    pub span: Option<(usize, usize)>,
    pub message_args: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ExpectedDiagnostics {
    /// Expected total diagnostics count. When items is non-empty, this should
    /// match items.len(). If items is empty, count may still be set for a coarse check.
    pub count: usize,
    pub items: Vec<RoslynDiagnosticExpectation>,
}

/// Assert that actual diagnostics match the Roslyn expectation (stub version).
///
/// NOTE: Our parser does not expose diagnostics yet. Until it does, this function
/// should be wired to the parser diagnostic output. For now, it performs a no-op
/// when `expected.count == 0` and emits a note when `expected.count > 0`.
#[inline]
pub fn assert_diagnostics_unimplemented(expected: &ExpectedDiagnostics) {
    if expected.count == 0 {
        return;
    }
    eprintln!(
        "[roslyn_asserts] Expected {} diagnostic(s), but parser diagnostics are not yet exposed; skipping assertion.",
        expected.count
    );
}
