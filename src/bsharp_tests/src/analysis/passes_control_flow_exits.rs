use analysis::artifacts::control_flow_graph::index::ControlFlowIndex;
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn control_flow_counts_exit_points() {
    let src = r#"
public class A {
  public int M(int x) {
    if (x > 0) return 1;
    if (x == 0) return 0;
    return -1;
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

    let stats = cfg.get("A::M").expect("stats for A::M");
    assert!(
        stats.exit_points >= 3,
        "expected >= 3 exit points, got {}",
        stats.exit_points
    );
}
