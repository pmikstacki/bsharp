use std::path::{Path, PathBuf};

use bsharp::analysis::framework::pipeline::AnalyzerPipeline;
use bsharp::analysis::context::{AnalysisConfig, WorkspaceConfig};
use bsharp::workspace::WorkspaceLoader;
use serde_json::Value;

fn normalize_paths_in_json(v: &mut Value, root: &str) {
    match v {
        Value::String(s) => {
            if s.contains(root) {
                *s = s.replace(root, "$ROOT");
            }
        }
        Value::Array(arr) => {
            for x in arr.iter_mut() {
                normalize_paths_in_json(x, root);
            }
        }
        Value::Object(map) => {
            for (_, x) in map.iter_mut() {
                normalize_paths_in_json(x, root);
            }
        }
        _ => {}
    }
}

#[test]
fn snapshot_workspace_analysis_include_only_program() {
    // Arrange
    let sln: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", "happy_path", "testSolution.sln"].iter().collect();
    let ws = WorkspaceLoader::from_path(&sln).expect("workspace from fixture sln");

    let mut cfg = AnalysisConfig::default();
    cfg.workspace = WorkspaceConfig { follow_refs: true, include: vec!["**/Program.cs".to_string()], exclude: vec![] };

    // Act
    let report = AnalyzerPipeline::run_workspace_with_config(&ws, cfg);
    let mut json = serde_json::to_value(&report).expect("serialize");

    // Normalize for snapshot stability
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/happy_path");
    let root_str = root.to_string_lossy().to_string();
    normalize_paths_in_json(&mut json, &root_str);

    // Assert snapshot
    insta::assert_json_snapshot!("workspace_include_only_program", json);
}

#[test]
fn snapshot_workspace_analysis_exclude_dependency_project() {
    // Arrange
    let sln: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", "happy_path", "testSolution.sln"].iter().collect();
    let ws = WorkspaceLoader::from_path(&sln).expect("workspace from fixture sln");

    let mut cfg = AnalysisConfig::default();
    cfg.workspace = WorkspaceConfig { follow_refs: true, include: vec![], exclude: vec!["**/testDependency/**".to_string()] };

    // Act
    let report = AnalyzerPipeline::run_workspace_with_config(&ws, cfg);
    let mut json = serde_json::to_value(&report).expect("serialize");

    // Normalize for snapshot stability
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/happy_path");
    let root_str = root.to_string_lossy().to_string();
    normalize_paths_in_json(&mut json, &root_str);

    // Assert snapshot
    insta::assert_json_snapshot!("workspace_exclude_dependency", json);
}
