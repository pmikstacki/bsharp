use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::rules::RuleSet;
use analysis::report::AnalysisReport;
use analysis::{context::AnalysisContext, framework::session::AnalysisSession};
use parser::facade::Parser;

#[test]
fn ruleset_macros_build_expected_sets() {
    // Use macro-generated functions
    let naming: RuleSet = analysis::rules::naming::naming_ruleset();
    let semantic: RuleSet = analysis::rules::semantic::semantic_ruleset();
    let cf: RuleSet = analysis::rules::control_flow_smells::control_flow_smells_ruleset();

    assert_eq!(naming.id, "naming");
    assert!(naming.iter().count() > 0);

    assert_eq!(semantic.id, "semantic");
    assert!(semantic.iter().count() > 0);

    assert_eq!(cf.id, "control_flow_smells");
    assert!(cf.iter().count() > 0);
}

#[test]
fn naming_rule_emits_diagnostics_via_macros() {
    let src = r#"
public class nbad {
    public int x_field;
    public void mbad() {}
    public int Pbad { get; set; }
}
"#;
    // Build session and run default pipeline
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let report: AnalysisReport = AnalysisReport::from_session(&session);
    let diags = report.diagnostics.unwrap_or_default();
    // Expect at least one naming warning (BSW02002)
    let has_naming = diags
        .iter()
        .any(|d| d.code.as_str() == "BSW02002");
    assert!(has_naming, "expected naming diagnostics to be emitted");
}
