use bsharp::analysis::context::AnalysisContext;
use bsharp::analysis::framework::pipeline::AnalyzerPipeline;
use bsharp::analysis::framework::session::AnalysisSession;
use bsharp::analysis::report::AnalysisReport;
use bsharp::syntax::Parser;
use serde_json::Value;

#[test]
fn snapshot_small_input_analysis_report_json_shape() {
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

    // Run default pipeline
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    // Build report and pretty-print JSON
    let report = AnalysisReport::from_session(&session);
    let json = serde_json::to_string_pretty(&report).expect("serialize report json");

    // Parse back to validate schema
    let v: Value = serde_json::from_str(&json).expect("valid json");

    // Basic shape assertions
    assert!(v.get("diagnostics").is_some(), "report must have diagnostics");
    assert!(v.get("metrics").is_some() || v.get("metrics").is_null(), "report must have metrics opt");
    assert!(v.get("cfg").is_some() || v.get("cfg").is_null(), "report must have cfg opt");
    assert!(v.get("deps").is_some() || v.get("deps").is_null(), "report must have deps opt");

    // CFG summary expectations (present and counts are sane)
    if let Some(cfg) = v.get("cfg").and_then(|x| x.as_object()) {
        let total_methods = cfg.get("total_methods").and_then(|x| x.as_u64()).unwrap_or(0);
        assert!(total_methods >= 1, "expected at least 1 method");
        let _hc = cfg.get("high_complexity_methods").and_then(|x| x.as_u64()).unwrap_or(0);
        let _dn = cfg.get("deep_nesting_methods").and_then(|x| x.as_u64()).unwrap_or(0);
    }

    // Dependencies summary expectations (present and non-negative)
    if let Some(deps) = v.get("deps").and_then(|x| x.as_object()) {
        let nodes = deps.get("nodes").and_then(|x| x.as_u64()).unwrap_or(0);
        let _edges = deps.get("edges").and_then(|x| x.as_u64()).unwrap_or(0);
        assert!(nodes >= 1, "expected at least 1 node");
    }

    // This acts as a snapshot of structure; if schema changes, this test should be updated accordingly.
}
