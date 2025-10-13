use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use parser::facade::Parser;

#[test]
fn smell_rules_emit_high_complexity_and_deep_nesting() {
    let src = r#"
public class C {
  public void M() {
    if (true) { if (true) { if (true) { if (true) { } } } }
    for (int i=0; i<2; i++) { if (i>0) { } }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    // Lower thresholds to trigger
    session.config.cf_high_complexity_threshold = 1;
    session.config.cf_deep_nesting_threshold = 1;
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let report = AnalysisReport::from_session(&session);
    let codes: Vec<_> = report
        .diagnostics
        .diagnostics
        .iter()
        .map(|d| d.code.as_str())
        .collect();
    assert!(
        codes.contains(&"BSW01001"),
        "expected high complexity warning"
    );
    assert!(codes.contains(&"BSW01005"), "expected deep nesting warning");
}

#[test]
fn smell_rule_long_method_span_emits_bsw01002() {
    let body_lines = (0..60).map(|_| "    int x = 1;\n").collect::<String>();
    let src = format!(
        "public class C {{ public void L() {{\n{} }} }}\n",
        body_lines
    );
    let (cu, spans) = Parser::new().parse_with_spans(&src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src.clone()), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let report = AnalysisReport::from_session(&session);
    assert!(
        report
            .diagnostics
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "BSW01002"),
        "expected long method warning"
    );
}
