use analysis::artifacts::control_flow_graph::index::ControlFlowIndex;
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use parser::facade::Parser;

#[test]
fn config_can_disable_passes_and_rulesets() {
    let src = r#"
public class C {
  public void bad_method() { if (true) { } }
}
"#;
    let (cu, spans1) = Parser::new().parse_with_spans(src).expect("parse error");

    // Disable control_flow pass -> no ControlFlowIndex and no cfg summary
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans1);
    session
        .config
        .enable_passes
        .insert("passes.control_flow".into(), false);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let cfg_idx = session.artifacts.get::<ControlFlowIndex>();
    assert!(
        cfg_idx.is_none(),
        "ControlFlowIndex should be absent when pass disabled"
    );
    let report = AnalysisReport::from_session(&session);
    assert!(
        report.cfg.is_none(),
        "cfg summary should be None when pass disabled"
    );

    // Disable naming ruleset -> no naming diagnostics
    let (_cu2, spans2) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session2 = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans2);
    session2
        .config
        .enable_rulesets
        .insert("naming".into(), false);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session2);
    let report2 = AnalysisReport::from_session(&session2);
    assert!(
        !report2
            .diagnostics
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "BSW02002"),
        "naming diagnostics should be disabled by ruleset toggle"
    );
}
