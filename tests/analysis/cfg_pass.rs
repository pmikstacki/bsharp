use bsharp::analysis::framework::pipeline::AnalyzerPipeline;
use bsharp::analysis::framework::session::AnalysisSession;
use bsharp::analysis::artifacts::cfg::ControlFlowIndex;
use bsharp::analysis::context::AnalysisContext;
use bsharp::syntax::Parser;

#[test]
fn cfg_pass_computes_complexity_and_nesting() {
    let source = r#"
public class A {
  public void M() {
    if (true) { for (int i=0; i<10; i++) { if (false) { } } }
  }
}
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);

    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let cfg = session.artifacts.get::<ControlFlowIndex>().expect("control flow index missing");
    // Keys are fully-qualified only now; class without namespace uses just class name
    let stats = cfg.get("A::M").expect("stats for A::M");
    assert!(stats.complexity >= 3, "expected complexity >= 3");
    assert!(stats.max_nesting >= 2, "expected nesting >= 2");
}
