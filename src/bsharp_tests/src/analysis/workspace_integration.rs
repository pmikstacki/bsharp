use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::report::AnalysisReport;
use analysis::workspace::model::{Language, Project, ProjectFile, ProjectFileKind, Workspace};
use std::fs;
use std::path::PathBuf;

fn write_file(path: &PathBuf, contents: &str) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, contents).expect("failed to write test file");
}

fn temp_dir(prefix: &str) -> PathBuf {
    let mut d = std::env::temp_dir();
    let unique = format!(
        "{}_{}",
        prefix,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    d.push(unique);
    fs::create_dir_all(&d).expect("failed to create temp dir");
    d
}

#[test]
fn run_workspace_aggregates_multiple_files() {
    // Prepare temp workspace with two source files
    let root = temp_dir("bsharp_ws");
    let file1 = root.join("Proj1").join("A.cs");
    let file2 = root.join("Proj1").join("B.cs");
    let src1 = r#"
public class A { public void M1() { if (true) { } } }
"#;
    let src2 = r#"
using System;
public class B { public void M2() { Console.WriteLine(1); } }
"#;
    write_file(&file1, src1);
    write_file(&file2, src2);

    let mut project = Project::default();
    project.name = "Proj1".into();
    project.path = root.join("Proj1");
    project
        .files
        .push(ProjectFile::new_source(file1.clone(), Language::CSharp));
    project
        .files
        .push(ProjectFile::new_source(file2.clone(), Language::CSharp));

    let workspace = Workspace {
        root: root.clone(),
        projects: vec![project],
        solution: None,
        ..Default::default()
    };

    let report: AnalysisReport = AnalyzerPipeline::run_workspace(&workspace);
    assert!(report.metrics.is_some(), "expected metrics in report");
    assert!(report.cfg.is_some(), "expected cfg summary in report");
    assert!(report.deps.is_some(), "expected deps summary in report");

    let cfg = report.cfg.unwrap();
    assert!(cfg.total_methods >= 2, "expected >= 2 methods aggregated");
}

#[test]
fn run_workspace_with_config_includes_and_excludes() {
    // Prepare temp workspace with three files; include only one
    let root = temp_dir("bsharp_ws_cfg");
    let file1 = root.join("Proj").join("one.cs");
    let file2 = root.join("Proj").join("two.cs");
    let file3 = root.join("Proj").join("three.cs");
    write_file(&file1, "public class C1 { public void M1() {} }");
    write_file(&file2, "public class C2 { public void M2() {} }");
    write_file(&file3, "public class C3 { public void M3() {} }");

    let mut project = Project::default();
    project.name = "Proj".into();
    project.path = root.join("Proj");
    project
        .files
        .push(ProjectFile::new_source(file1.clone(), Language::CSharp));
    project
        .files
        .push(ProjectFile::new_source(file2.clone(), Language::CSharp));
    project
        .files
        .push(ProjectFile::new_source(file3.clone(), Language::CSharp));

    let workspace = Workspace {
        root: root.clone(),
        projects: vec![project],
        solution: None,
        ..Default::default()
    };

    // Configure includes to pick only one.cs
    let mut config = analysis::context::AnalysisConfig::default();
    config.workspace.include = vec!["**/one.cs".to_string()];
    // run
    let report = AnalyzerPipeline::run_workspace_with_config(&workspace, config);

    let metrics = report.metrics.expect("metrics missing");
    assert_eq!(
        metrics.total_methods, 1,
        "include filter should restrict to one file"
    );
}
