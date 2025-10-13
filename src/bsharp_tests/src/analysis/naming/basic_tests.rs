use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use parser::facade::Parser;

fn analyze(source: &str) -> AnalysisReport {
    let (cu, spans) = Parser::new().parse_with_spans(source).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("test.cs", source), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    AnalysisReport::from_session(&session)
}

#[test]
fn naming_violations_are_reported_for_properties_methods_fields() {
    let src = r#"
public class C {
  public int bad_property { get; set; }
  public void bad_method() {}
  public const int badConst = 1;
  private int BadField;
}
"#;
    let report = analyze(src);
    // Expect at least one naming violation diagnostic (BSW02002) produced by naming rules
    assert!(
        report
            .diagnostics
            .diagnostics
            .iter()
            .any(|d| d.code.as_str() == "BSW02002")
    );
}

#[test]
fn constants_and_fields_follow_expected_casing() {
    let src = r#"
public class C {
  public const int MAX_COUNT = 10; // OK
  public const int BadConst = 1;   // Not OK
  private int camelCaseOk;         // OK
  private int _underscore;         // Not OK
}
"#;
    let report = analyze(src);
    let bsw02002_count = report
        .diagnostics
        .diagnostics
        .iter()
        .filter(|d| d.code.as_str() == "BSW02002")
        .count();
    assert!(bsw02002_count >= 2);
}

#[test]
fn methods_and_properties_should_be_pascal_case_and_formatting_body_is_rendered() {
    use analysis::diagnostics::format::render_body;
    let src = r#"
public class C {
  public int bad_property { get; set; }
  public void bad_method() { }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("test.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    // Pick a naming diagnostic that has a location set (e.g., method rule sets span)
    let diag_with_loc = session
        .diagnostics
        .diagnostics
        .iter()
        .find(|d| d.code.as_str() == "BSW02002" && d.location.is_some())
        .cloned()
        .expect("expected at least one naming diagnostic with a location");

    // Check pretty body rendering
    let ctx = AnalysisContext::new("test.cs", src);
    let body = render_body(&ctx, &diag_with_loc);
    assert!(body.contains("at "));
    // Body should reference the offending line; method or property name should appear on that line
    assert!(body.contains("bad_method") || body.contains("bad_property"));
    assert!(body.contains("^"));

    // Check Diagnostic Display formatting includes code, severity and message
    let display = format!("{}", diag_with_loc);
    assert!(display.contains("BSW02002"));
    assert!(display.contains("WARNING"));
    assert!(
        display.contains("PascalCase")
            || display.contains("camelCase")
            || display.contains("UPPER_CASE")
    );
}
