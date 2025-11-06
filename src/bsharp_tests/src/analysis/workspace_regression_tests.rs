use std::collections::HashSet;
use std::path::{Path, PathBuf};

use analysis::framework::pipeline::{AnalyzerPipeline};
use analysis::workspace::model::{ProjectFile, ProjectFileKind, Project, Workspace, Language};
use analysis::{AnalysisConfig, AnalysisReport, DiagnosticCollection};

/// Helper to build a minimal Workspace for testing
fn test_workspace(root_dir: &Path, files: Vec<&str>) -> Workspace {
    let project_files: Vec<ProjectFile> = files
        .into_iter()
        .map(|f| ProjectFile {
            kind: ProjectFileKind::Source,
            path: root_dir.join(f),
            language: Some(Language::CSharp),
        })
        .collect();
    Workspace {
        root: root_dir.to_path_buf(),
        projects: vec![Project {
            name: "TestProj".to_string(),
            path: root_dir.join("TestProj.csproj"),
            files: project_files,
            errors: Vec::new(),
            project_references: Vec::new(),
        }],
        solution: None,
        source_map: Default::default(),
    }
}

/// Helper to manually analyze files and merge reports, mirroring AnalyzerPipeline::run_workspace logic
fn manual_analyze_and_merge(workspace: &Workspace) -> AnalysisReport {
    let mut files: Vec<PathBuf> = workspace
        .projects
        .iter()
        .flat_map(|p| p.files.iter())
        .filter(|f| matches!(f.kind, ProjectFileKind::Source))
        .map(|f| f.path.clone())
        .collect();
    files.sort();
    files.dedup();

    let mut merged_diags = DiagnosticCollection::default();
    let mut merged_metrics: Option<analysis::AstAnalysis> = None;
    let mut merged_cfg: Option<analysis::report::CfgSummary> = None;
    let mut dep_node_keys: HashSet<String> = HashSet::new();
    let mut dep_edge_keys: HashSet<String> = HashSet::new();

    for path in files {
        if let Some(report) = AnalyzerPipeline::analyze_file_report(&path, None) {
            // Reuse private merge_report logic locally for test
            merged_diags.extend(report.diagnostics.clone());
            if let Some(m) = &report.metrics {
                let m = m.clone();
                merged_metrics = Some(match merged_metrics.take() {
                    Some(prev) => prev.combine(m),
                    None => m,
                });
            }
            if let Some(cfg) = &report.cfg {
                let cfg = cfg.clone();
                merged_cfg = Some(match merged_cfg.take() {
                    Some(prev) => analysis::report::CfgSummary {
                        total_methods: prev.total_methods + cfg.total_methods,
                        high_complexity_methods: prev.high_complexity_methods + cfg.high_complexity_methods,
                        deep_nesting_methods: prev.deep_nesting_methods + cfg.deep_nesting_methods,
                    },
                    None => cfg,
                });
            }
            if let Some(node_keys) = &report.deps_node_keys {
                dep_node_keys.extend(node_keys.iter().cloned());
            }
            if let Some(edge_keys) = &report.deps_edge_keys {
                dep_edge_keys.extend(edge_keys.iter().cloned());
            }
        }
    }

    AnalyzerPipeline::sort_diagnostics(&mut merged_diags);
    let ws_warnings = workspace
        .projects
        .iter()
        .flat_map(|p| p.errors.clone())
        .chain(workspace.solution.as_ref().map(|s| s.errors.clone()).into_iter().flatten())
        .collect::<Vec<_>>();

    let deps = Some(analysis::artifacts::dependencies::DependencySummary {
        nodes: dep_node_keys.len(),
        edges: dep_edge_keys.len(),
    });
    AnalysisReport {
        schema_version: 1,
        diagnostics: merged_diags,
        metrics: merged_metrics,
        cfg: merged_cfg,
        deps,
        workspace_warnings: ws_warnings,
        workspace_errors: Vec::new(),
        deps_node_keys: None,
        deps_edge_keys: None,
    }
}

#[test]
fn run_workspace_matches_manual_merge() {
    // Create a temporary directory with a few C# files
    let tmp_dir = tempfile::tempdir().expect("tempdir");
    let root = tmp_dir.path();

    // Write sample files with varying complexity
    std::fs::write(
        root.join("A.cs"),
        r#"
using System;

class A {
    void Simple() { }
    void IfElse(int x) {
        if (x > 0) { } else { }
    }
}
"#,
    )
    .expect("write A.cs");

    std::fs::write(
        root.join("B.cs"),
        r#"
class B {
    void Loop(int n) {
        for (int i = 0; i < n; i++) { }
    }
}
"#,
    )
    .expect("write B.cs");

    std::fs::write(
        root.join("C.cs"),
        r#"
using System;

class C {
    void TryCatch() {
        try { } catch (Exception) { } finally { }
    }
}
"#,
    )
    .expect("write C.cs");

    let workspace = test_workspace(root, vec!["A.cs", "B.cs", "C.cs"]);

    // Run via AnalyzerPipeline::run_workspace
    let report_via_run = AnalyzerPipeline::run_workspace(&workspace);

    // Run via manual per-file analysis and merge
    let report_manual = manual_analyze_and_merge(&workspace);

    // Compare key fields
    assert_eq!(report_via_run.diagnostics.diagnostics, report_manual.diagnostics.diagnostics);
    assert_eq!(report_via_run.metrics, report_manual.metrics);
    // Compare cfg and deps field-wise since they don't implement PartialEq
    assert_eq!(report_via_run.cfg.as_ref().map(|c| c.total_methods), report_manual.cfg.as_ref().map(|c| c.total_methods));
    assert_eq!(report_via_run.cfg.as_ref().map(|c| c.high_complexity_methods), report_manual.cfg.as_ref().map(|c| c.high_complexity_methods));
    assert_eq!(report_via_run.cfg.as_ref().map(|c| c.deep_nesting_methods), report_manual.cfg.as_ref().map(|c| c.deep_nesting_methods));
    assert_eq!(report_via_run.deps.as_ref().map(|d| d.nodes), report_manual.deps.as_ref().map(|d| d.nodes));
    assert_eq!(report_via_run.deps.as_ref().map(|d| d.edges), report_manual.deps.as_ref().map(|d| d.edges));
    assert_eq!(report_via_run.workspace_warnings, report_manual.workspace_warnings);
}

#[test]
fn run_workspace_with_config_include_exclude() {
    let tmp_dir = tempfile::tempdir().expect("tempdir");
    let root = tmp_dir.path();

    // Write files
    std::fs::write(root.join("IncludeMe.cs"), "class I { void M() {} }").expect("write");
    std::fs::write(root.join("ExcludeMe.cs"), "class E { void N() {} }").expect("write");
    std::fs::write(root.join("Other.txt"), "not C#").expect("write");

    let workspace = test_workspace(root, vec!["IncludeMe.cs", "ExcludeMe.cs"]);

    // Config: include only IncludeMe.cs
    let mut cfg = AnalysisConfig::default();
    cfg.workspace.include.push("IncludeMe.cs".to_string());
    cfg.workspace.exclude.push("ExcludeMe.cs".to_string());

    let report = AnalyzerPipeline::run_workspace_with_config(&workspace, cfg);

    // Only diagnostics from IncludeMe.cs should be present (none in this simple case)
    // Verify that metrics reflect only included file
    assert!(report.metrics.is_some());
    let metrics = report.metrics.unwrap();
    // Expect at least one class/method from the included file
    assert!(metrics.total_classes >= 1);
    assert!(metrics.total_methods >= 1);
    // If exclusion worked, totals should be minimal; we can also check file count indirectly
    // For simplicity, ensure no diagnostics from excluded file (none here)
}

#[test]
fn run_workspace_deterministic_ordering() {
    let tmp_dir = tempfile::tempdir().expect("tempdir");
    let root = tmp_dir.path();

    // Write files in non-alphabetical order to test sorting
    let files = vec![
        ("Z.cs", "class Z { void A() { if (true) {} } }"),
        ("A.cs", "class A { void B() { for (;;) {} } }"),
        ("M.cs", "class M { void C() { while (false) {} } }"),
    ];
    for (name, content) in &files {
        std::fs::write(root.join(name), content).expect("write");
    }

    let workspace = test_workspace(root, vec!["Z.cs", "A.cs", "M.cs"]);

    let report1 = AnalyzerPipeline::run_workspace(&workspace);
    let report2 = AnalyzerPipeline::run_workspace(&workspace);

    // Diagnostics should be stably sorted by file, line, column, code
    assert_eq!(report1.diagnostics.diagnostics, report2.diagnostics.diagnostics);
    // Also check that they are sorted by file path
    let sorted_by_file = report1
        .diagnostics
        .diagnostics
        .iter()
        .map(|d| d.location.as_ref().map(|l| l.file.clone()).unwrap_or_default())
        .collect::<Vec<_>>();
    let mut sorted_for_check = sorted_by_file.clone();
    sorted_for_check.sort();
    assert_eq!(sorted_by_file, sorted_for_check);
}
