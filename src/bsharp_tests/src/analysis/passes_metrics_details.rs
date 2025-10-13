use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::metrics::AstAnalysis;
use parser::facade::Parser;

#[test]
fn metrics_detailed_counts_and_loc() {
    let src = r#"
// comment-only line should be ignored for LOC

public class A {
  public void M() {
    if (true) { }
    for (int i=0; i<2; i++) { }
    foreach (var x in xs) { }
    while (false) { }
    do { } while (false);
    switch (1) { case 1: break; default: break; }
    try { } catch { } finally { }
    using (var d = Get()) { }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let a = session
        .artifacts
        .get::<AstAnalysis>()
        .expect("AstAnalysis missing");

    assert_eq!(a.total_classes, 1);
    assert_eq!(a.total_methods, 1);
    assert!(a.total_if_statements >= 1);
    assert!(
        a.total_for_loops >= 2,
        "for and foreach should both count in for_loops"
    );
    assert!(
        a.total_while_loops >= 2,
        "while and do-while should both count in while_loops"
    );
    assert!(a.total_switch_statements >= 1);
    assert!(a.total_try_statements >= 1);
    assert!(a.total_using_statements >= 1);
    assert!(
        a.cyclomatic_complexity
            >= 1 + 1 /*if*/ + 1 /*for*/ + 1 /*foreach*/ + 1 /*while*/ + 1 /*do*/ + 2 /*switch cases*/ + 1 /*try*/
    );
    assert!(a.max_nesting_depth >= 1);
    assert!(
        a.lines_of_code >= 8,
        "LOC should count significant non-empty, non-comment lines"
    );
}
