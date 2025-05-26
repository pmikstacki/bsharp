// Include the CLI tests module
mod cli;

// Include the main parser tests module
mod parser; // This will load tests/parser/mod.rs

// Include the analysis tests module
mod analysis; // This will load tests/analysis/mod.rs

// Include navigation tests (moved to analysis)
// mod navigation; // Removed - moved to analysis/navigation

// Integration tests - properly structured
mod integration;