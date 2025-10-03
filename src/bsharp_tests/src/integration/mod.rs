// Integration tests for B# analysis modules
// These tests verify that multiple analysis modules work together correctly

// Integration test modules
pub mod common;
pub mod complex_csproj_patterns;
pub mod fixtures_workspace_loader;
pub mod workspace_analysis_config_snapshots;
pub mod workspace_analysis_snapshot;

// Parser integration tests - moved from top level
pub mod parser_integration;
