#![allow(unused_variables)]
#![allow(unused_comparisons)]
#![allow(clippy::assertions_on_constants)]
#![allow(clippy::absurd_extreme_comparisons)]

use bsharp::analysis::artifacts::cfg::ControlFlowIndex;
use bsharp::analysis::context::AnalysisContext;
use bsharp::analysis::framework::pipeline::AnalyzerPipeline;
use bsharp::analysis::framework::session::AnalysisSession;
use bsharp::syntax::Parser;

#[test]
fn test_control_flow_pass_produces_stats() {
    let source = r#"
namespace N {
    public class C {
        public void M() { if (true) { } }
    }
}
"#;

    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);

    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let index = session
        .artifacts
        .get::<ControlFlowIndex>()
        .expect("ControlFlowIndex missing");

    // Key is fully-qualified: "N.C::M" for this test
    let stats = index.0.get("N.C::M").expect("missing method stats");
    assert!(stats.complexity >= 2);
}
