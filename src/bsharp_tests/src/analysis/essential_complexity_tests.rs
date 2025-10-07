use analysis::artifacts::cfg::ControlFlowGraphs;
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn essential_complexity_reduces_diamond_to_one() {
    let source = r#"
public class A {
  public void M() {
    if (true) { int x = 1; } else { int x = 2; }
  }
}
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);

    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graphs = session
        .artifacts
        .get::<ControlFlowGraphs>()
        .expect("control flow graphs missing");

    let g = graphs.0.get("A::M").expect("graph for A::M");
    let ec = g.essential_complexity();
    assert!(ec <= g.cyclomatic_complexity());
    assert_eq!(ec, 1, "diamond should reduce to essential complexity 1");
}
