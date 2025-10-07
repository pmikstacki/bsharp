// Metrics analysis module - organized by functionality

pub mod basic; // Basic metrics (lines of code, etc.)
pub mod complexity; // Advanced complexity metrics
pub mod core; // Core traits and data structures
pub mod maintainability; // Maintainability index calculation
pub mod visitor; // MetricsVisitor that collects basic counts
pub mod pass; // MetricPass trait
pub mod shared; // Shared statement metrics helpers
// Comprehensive test suite -- Removing this as tests are in `tests/` dir.

// Re-export main types for easy access
pub use core::AstAnalysis;
pub use pass::MetricPass;
pub use visitor::MetricsVisitor;
