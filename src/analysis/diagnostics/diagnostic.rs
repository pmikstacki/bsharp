use std::fmt;
use serde::{Deserialize, Serialize};
use crate::analysis::{DiagnosticCode, DiagnosticSeverity, SourceLocation};
use crate::analysis::diagnostics::diagnostic_category::DiagnosticCategory;
use crate::analysis::diagnostics::diagnostic_related_information::DiagnosticRelatedInformation;

/// A diagnostic message (error, warning, info, or hint)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: DiagnosticSeverity,
    pub category: DiagnosticCategory,
    pub message: String,
    pub location: Option<SourceLocation>,
    pub related_information: Vec<DiagnosticRelatedInformation>,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, message: String) -> Self {
        Self {
            severity: code.severity(),
            category: code.category(),
            code,
            message,
            location: None,
            related_information: vec![],
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_related(mut self, related: DiagnosticRelatedInformation) -> Self {
        self.related_information.push(related);
        self
    }

    /// Create a diagnostic with a computed location from a byte span via AnalysisContext
    pub fn from_span(
        ctx: &crate::analysis::context::AnalysisContext,
        code: DiagnosticCode,
        message: impl Into<String>,
        start: usize,
        length: usize,
    ) -> Self {
        let mut d = Diagnostic::new(code, message.into());
        let loc = ctx.location_from_span(start, length);
        d.location = Some(loc);
        d
    }

    /// Create a diagnostic with the default message and a computed location from span
    pub fn default_from_span(
        ctx: &crate::analysis::context::AnalysisContext,
        code: DiagnosticCode,
        start: usize,
        length: usize,
    ) -> Self {
        Self::from_span(ctx, code, code.default_message(), start, length)
    }

    pub fn error(code: DiagnosticCode, message: String) -> Self {
        debug_assert!(matches!(code.severity(), DiagnosticSeverity::Error));
        Self::new(code, message)
    }

    pub fn warning(code: DiagnosticCode, message: String) -> Self {
        debug_assert!(matches!(code.severity(), DiagnosticSeverity::Warning));
        Self::new(code, message)
    }

    /// Create diagnostic with default message
    pub fn with_default_message(code: DiagnosticCode) -> Self {
        Self::new(code, code.default_message().to_string())
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}: {}",
            self.code.as_str(),
            self.severity.to_string().to_uppercase(),
            self.message
        )
    }
}