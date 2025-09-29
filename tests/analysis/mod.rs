// Tests for code analysis functionality

pub mod control_flow;
pub mod metrics;
pub mod naming;
pub mod navigation;
pub mod quality;
pub mod types;
pub mod symbols_index;
pub mod cfg_pass;
pub mod deps_pass;

// These modules don't exist yet:
// pub mod security;
// pub mod performance;
// pub mod ai_analysis;
// pub mod rules;
// pub mod reports;
// pub mod fixes;
// pub mod diagnostics;

// Integration tests that test multiple analysis features together
// pub mod comprehensive_tests;  // File doesn't exist yet
