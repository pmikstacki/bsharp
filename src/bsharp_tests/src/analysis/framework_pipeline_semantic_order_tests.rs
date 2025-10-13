use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use parser::facade::Parser;

#[test]
fn semantic_rules_depend_on_global_cfg_artifact() {
    let src = r#"
public class C { public void M() { if (true) { } } }
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");

    // With control_flow enabled, smell rules should run and possibly emit diagnostics (threshold low)
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans.clone());
    session.config.cf_high_complexity_threshold = 0;
    session.config.cf_deep_nesting_threshold = 0;
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let report = AnalysisReport::from_session(&session);
    assert!(report.diagnostics.diagnostics.iter().any(|d| {
        let c = d.code.as_str();
        c == "BSW01001" || c == "BSW01005"
    }));

    // When control_flow pass is disabled, semantic rules should not produce these diagnostics
    let (cu2, spans2) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session2 = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans2);
    session2
        .config
        .enable_passes
        .insert("passes.control_flow".into(), false);
    session2.config.cf_high_complexity_threshold = 0;
    session2.config.cf_deep_nesting_threshold = 0;
    AnalyzerPipeline::run_with_defaults(&cu2, &mut session2);
    let report2 = AnalysisReport::from_session(&session2);
    assert!(
        report2
            .diagnostics
            .diagnostics
            .iter()
            .all(|d| d.code.as_str() != "BSW01001" && d.code.as_str() != "BSW01005")
    );
}
