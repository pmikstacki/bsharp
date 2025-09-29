use serde::{Deserialize, Serialize};

/// Diagnostic categories for organization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticCategory {
    Syntax,
    Semantic,
    Type,
    Style,
    Performance,
    Security,
    Maintainability,
}