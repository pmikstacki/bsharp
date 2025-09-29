use serde::{Deserialize, Serialize};
use crate::analysis::{AnalysisContext, SourceLocation};

/// Related information for a diagnostic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticRelatedInformation {
    pub location: SourceLocation,
    pub message: String,
}

impl DiagnosticRelatedInformation {
    /// Build a related information entry from a span using AnalysisContext
    pub fn from_span(ctx: &AnalysisContext, start: usize, length: usize, message: impl Into<String>) -> Self {
        Self {
            location: ctx.location_from_span(start, length),
            message: message.into(),
        }
    }
}