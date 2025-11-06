// Tests for code analysis functionality

pub mod cfg_pass;
pub mod common;
pub mod control_flow;
pub mod control_flow_smells_tests;
pub mod deps_pass;
mod essential_complexity_tests;
pub mod framework_registry;
pub mod lookup_fqn;
pub mod naming;
pub mod passes_control_flow;
pub mod passes_dependencies;
pub mod passes_indexing;
pub mod passes_reporting;
pub mod pipeline_integration;
pub mod query_api;
pub mod symbols_index;
pub mod walker_traversal;
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

// New test modules added for expanded coverage
pub mod artifacts_tests;
pub mod deps_edges_tests;
pub mod diagnostics_collection_tests;
pub mod framework_pipeline_semantic_order_tests;
pub mod framework_registry_config;
pub mod passes_metrics_details;
pub mod query_api_filter_tests;
pub mod query_children_descendants_coverage;
pub mod report_aggregation_tests;
mod workspace_integration;
pub mod workspace_regression_tests;
