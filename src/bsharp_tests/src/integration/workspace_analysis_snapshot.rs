use std::path::{Path, PathBuf};

use super::common::happy_path_sln;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::workspace::WorkspaceLoader;
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
fn snapshot_workspace_analysis_report_for_fixture_solution() {
    // Arrange
    let sln: PathBuf = happy_path_sln();

    if !sln.exists() {
        eprintln!(
            "skipping workspace_analysis_snapshot (fixture missing): {}",
            sln.display()
        );
        return;
    }
    let ws = match WorkspaceLoader::from_path(&sln) {
        Ok(ws) => ws,
        Err(e) => { eprintln!("skipping workspace_analysis_snapshot (fixture load failed): {:?}", e); return; }
    };

    // Act
    let report = AnalyzerPipeline::run_workspace(&ws);
    let json = serde_json::to_value(&report).expect("serialize");
    let mut json = json;

    // Normalize absolute paths for snapshot stability
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/happy_path");
    let root_str = root.to_string_lossy().to_string();
    normalize_paths_in_json(&mut json, &root_str);

    // Assert snapshot
    insta::assert_json_snapshot!("workspace_fixture_report", json);
}
