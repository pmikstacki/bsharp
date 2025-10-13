use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::metrics::AstAnalysis;
use parser::facade::Parser;

#[test]
fn metrics_counts_methods_loops_ifs_and_complexity() {
    let src = r#"
public class A {
  public void M() {
    if (true) { }
    for (int i=0; i<2; i++) { }
    while (false) { }
    try { } catch {} finally { }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("test.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let a = session
        .artifacts
        .get::<AstAnalysis>()
        .expect("AstAnalysis missing");

    assert_eq!(a.total_classes, 1);
    assert_eq!(a.total_methods, 1);
    assert!(a.total_if_statements >= 1);
    assert!(a.total_for_loops >= 1);
    assert!(a.total_while_loops >= 1);
    assert!(a.total_try_statements >= 1);
    assert!(a.cyclomatic_complexity >= 2); // baseline + try adds at least 1
    assert!(a.lines_of_code > 0);
}
