// Metrics analysis module - organized by functionality

pub mod core; // Core traits and data structures
pub mod shared; // Shared statement metrics helpers
// Comprehensive test suite -- Removing this as tests are in `tests/` dir.

// Re-export main types for easy access
pub use core::AstAnalysis;
