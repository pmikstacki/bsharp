use analysis::artifacts::dependencies::DependencyGraph;
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn deps_pass_builds_symbolid_graph() {
    let source = r#"
public class A { public void M() { B.N(); } }
public class B { public static void N() { } }
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);

    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graph = session
        .artifacts
        .get::<DependencyGraph>()
        .expect("dependency graph missing");
    assert!(
        graph.node_count() >= 2,
        "expected at least 2 nodes (A and B/N)"
    );
    assert!(
        graph.edge_count() >= 1,
        "expected at least 1 edge (A -> B.N method call)"
    );
}
