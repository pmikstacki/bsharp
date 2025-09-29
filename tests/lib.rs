// Include the CLI tests module
mod cli;

// Include the main syntax tests module
mod parser; // This will load tests/syntax/mod.rs

// Include the analysis tests module
mod analysis; // This will load tests/analysis/mod.rs

// Include navigation tests (moved to analysis)
// mod navigation; // Removed - moved to analysis/navigation

// Integration tests - properly structured
mod integration;
// Workspace readers/loader tests
mod workspace;
