use analysis::artifacts::control_flow_graph::index::ControlFlowIndex;
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn control_flow_stats_cover_complexity_and_nesting() {
    let src = r#"
public class A {
  public void M() {
    if (true) {
      for (int i=0; i<3; i++) {
        if (i % 2 == 0) { }
      }
    }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("test.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let cfg = session
        .artifacts
        .get::<ControlFlowIndex>()
        .expect("ControlFlowIndex missing");

    // Key without namespace is Type::Method
    let stats = cfg.get("A::M").expect("stats for A::M");
    assert!(
        stats.complexity >= 3,
        "expected complexity >= 3, got {}",
        stats.complexity
    );
    assert!(
        stats.max_nesting >= 2,
        "expected nesting >= 2, got {}",
        stats.max_nesting
    );
    assert!(stats.statement_count > 0);
}
