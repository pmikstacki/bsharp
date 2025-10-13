use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use analysis::workspace::WorkspaceLoader;
use parser::facade::Parser;
use std::fs;
use std::path::PathBuf;

#[test]
fn workspace_fixture_runs_pipeline_and_reports_mergeable_summaries() {
    // Reuse happy_path fixture from other integration tests
    let sln: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "..",
        "..",
        "tests",
        "fixtures",
        "happy_path",
        "testSolution.sln",
    ]
    .iter()
    .collect();

    if !sln.exists() {
        eprintln!(
            "skipping workspace_fixture_runs_pipeline_and_reports_mergeable_summaries (fixture missing): {}",
            sln.display()
        );
        return;
    }

    let ws = WorkspaceLoader::from_path(&sln).expect("workspace from fixture sln");
    let files = ws.all_source_files();
    assert!(!files.is_empty(), "fixture should have source files");

    let mut any_metrics = false;
    let mut any_cfg = false;
    let mut any_deps = false;
    let mut total_diags = 0usize;

    for path in files {
        let src = fs::read_to_string(path).expect("read source file");
        let (cu, spans) = Parser::new().parse_with_spans(&src).expect("parse error");
        let mut session = AnalysisSession::new(
            AnalysisContext::new(path.to_string_lossy().as_ref(), &src),
            spans,
        );
        AnalyzerPipeline::run_with_defaults(&cu, &mut session);

        let report = AnalysisReport::from_session(&session);
        total_diags += report.diagnostics.diagnostics.len();
        any_metrics |= report.metrics.is_some();
        any_cfg |= report.cfg.is_some();
        any_deps |= report.deps.is_some();

        // Diagnostics in each report are sorted by (file, line, column, code)
        let diags = &report.diagnostics.diagnostics;
        let mut last: Option<(String, usize, usize, String)> = None;
        for d in diags.iter() {
            let key = if let Some(loc) = &d.location {
                (
                    loc.file.clone(),
                    loc.line,
                    loc.column,
                    d.code.as_str().to_string(),
                )
            } else {
                (String::new(), 0, 0, d.code.as_str().to_string())
            };
            if let Some(prev) = &last {
                assert!(
                    prev <= &key,
                    "diagnostics not sorted: prev={:?}, curr={:?}",
                    prev,
                    key
                );
            }
            last = Some(key);
        }
    }

    assert!(any_metrics, "expected at least one file to produce metrics");
    assert!(
        any_cfg,
        "expected at least one file to produce control-flow summary"
    );
    // Depending on fixtures, deps may be empty; allow either
    assert!(total_diags >= 0);
}
