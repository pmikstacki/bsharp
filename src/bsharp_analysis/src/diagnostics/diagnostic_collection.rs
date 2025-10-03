use crate::diagnostics::diagnostic::Diagnostic;
use crate::{DiagnosticCode, DiagnosticSeverity};
use serde::{Deserialize, Serialize};

/// Collection of diagnostics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiagnosticCollection {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn add_error(&mut self, code: DiagnosticCode, message: String) {
        self.add(Diagnostic::error(code, message));
    }

    pub fn add_warning(&mut self, code: DiagnosticCode, message: String) {
        self.add(Diagnostic::warning(code, message));
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Error))
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Warning))
    }

    pub fn has_errors(&self) -> bool {
        self.errors().count() > 0
    }

    pub fn has_warnings(&self) -> bool {
        self.warnings().count() > 0
    }

    pub fn error_count(&self) -> usize {
        self.errors().count()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings().count()
    }

    pub fn extend(&mut self, other: DiagnosticCollection) {
        self.diagnostics.extend(other.diagnostics);
    }
}
