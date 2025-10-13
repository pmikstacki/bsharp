use analysis::artifacts::dependencies::DependencyGraph;
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn dependencies_graph_has_nodes_and_edges_for_types_and_calls() {
    let src = r#"
namespace N {
  public class B { public void M() {} }
  public class C { }
  public class A : C {
    private B fld;
    public void N(B p) { p.M(); }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graph = session
        .artifacts
        .get::<DependencyGraph>()
        .expect("DependencyGraph missing");

    assert!(
        graph.node_count() >= 3,
        "expected >= 3 nodes, got {}",
        graph.node_count()
    );
    assert!(
        graph.edge_count() >= 2,
        "expected >= 2 edges, got {}",
        graph.edge_count()
    );
}
