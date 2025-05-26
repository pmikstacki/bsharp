// Metrics analysis module - organized by functionality

pub mod core;            // Core traits and data structures
pub mod basic;           // Basic metrics (lines of code, etc.)
pub mod implementations; // AstAnalyze trait implementations
pub mod complexity;        // Advanced complexity metrics
pub mod maintainability;   // Maintainability index calculation
// Comprehensive test suite -- Removing this as tests are in `tests/` dir.

// Re-export main types for easy access
pub use core::{AstAnalyze, AstAnalysis, MetricCollector}; 