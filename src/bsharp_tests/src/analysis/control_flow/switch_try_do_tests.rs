use analysis::artifacts::control_flow_graph::edge::EdgeKind;
use analysis::artifacts::control_flow_graph::graph::{ControlFlowGraph, ControlFlowGraphs};
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn cfg_switch_has_switchcase_edges_and_reconverges() {
    let src = r#"
public class C {
  public void M(int x) {
    switch (x) {
      case 1: break;
      case 2: break;
      default: break;
    }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graphs = session
        .artifacts
        .get::<ControlFlowGraphs>()
        .expect("ControlFlowGraphs missing");
    let g: &ControlFlowGraph = graphs.0.get("C::M").expect("graph for C::M");

    let switch_edges = g
        .edges
        .iter()
        .filter(|(_, _, k)| matches!(k, EdgeKind::SwitchCase))
        .count();
    assert!(
        switch_edges >= 3,
        "expected >=3 SwitchCase edges, got {}",
        switch_edges
    );

    // Essential complexity should be <= cyclomatic complexity due to diamond/switch reductions
    let cc = g.cyclomatic_complexity();
    let ec = g.essential_complexity();
    assert!(
        ec <= cc,
        "essential complexity should not exceed cyclomatic: ec={} cc={}",
        ec,
        cc
    );
}

#[test]
fn cfg_try_catch_finally_has_exception_and_finally_edges() {
    let src = r#"
using System;
public class C {
  public void M() {
    try { int a = 1; }
    catch (Exception e) { int b = 2; }
    finally { int c = 3; }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graphs = session
        .artifacts
        .get::<ControlFlowGraphs>()
        .expect("ControlFlowGraphs missing");
    let g = graphs.0.get("C::M").expect("graph for C::M");

    let has_exception = g
        .edges
        .iter()
        .any(|(_, _, k)| matches!(k, EdgeKind::Exception));
    let has_finally = g
        .edges
        .iter()
        .any(|(_, _, k)| matches!(k, EdgeKind::Finally));
    assert!(has_exception, "expected at least one Exception edge");
    assert!(has_finally, "expected at least one Finally edge");
}

#[test]
fn cfg_do_while_forms_loop_and_increases_cc() {
    let src = r#"
public class C {
  public void M() {
    int i = 0; do { i++; } while (i < 1);
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graphs = session
        .artifacts
        .get::<ControlFlowGraphs>()
        .expect("ControlFlowGraphs missing");
    let g = graphs.0.get("C::M").expect("graph for C::M");

    let cc = g.cyclomatic_complexity();
    assert!(cc >= 2, "expected loop to increase CC, got {}", cc);
}
