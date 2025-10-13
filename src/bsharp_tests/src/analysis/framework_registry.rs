use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use analysis::{context::AnalysisContext, metrics::AstAnalysis};
use parser::facade::Parser;

#[test]
fn pipeline_populates_core_artifacts_and_report() {
    let src = r#"
public class A {
  public void M() { if (true) { } }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);

    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    // Metrics artifact present
    let metrics = session.artifacts.get::<AstAnalysis>();
    assert!(metrics.is_some(), "AstAnalysis missing");

    // CFG summary present in report
    let report = AnalysisReport::from_session(&session);
    assert_eq!(report.schema_version, 1);
    assert!(report.metrics.is_some());
    assert!(report.cfg.is_some());
}
