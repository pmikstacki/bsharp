use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use parser::facade::Parser;

#[test]
fn reporting_pass_inserts_report_and_sorts_diagnostics() {
    // Two naming violations in predictable positions
    let src = r#"
public class bad_class { // line 2
  public void bad_method() {} // line 3
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);

    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    // Reporting pass should insert AnalysisReport artifact
    let report_art = session
        .artifacts
        .get::<AnalysisReport>()
        .expect("AnalysisReport artifact missing");

    assert_eq!(report_art.schema_version, 1);
    assert!(!report_art.diagnostics.diagnostics.is_empty());

    // Verify diagnostics are sorted by (file, line, column, code)
    let diags = &report_art.diagnostics.diagnostics;
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
