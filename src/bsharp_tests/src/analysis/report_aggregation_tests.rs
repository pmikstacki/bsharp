use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use parser::facade::Parser;

#[test]
fn report_includes_metrics_cfg_and_deps_with_counts() {
    let src = r#"
using System;
namespace N {
  public class B { public void N() {} }
  public class A {
    public void M(B p) { if (true) Console.WriteLine(1); p.N(); }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    // Lower thresholds to ensure counts in summary
    session.config.cf_high_complexity_threshold = 1;
    session.config.cf_deep_nesting_threshold = 0;

    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let report = AnalysisReport::from_session(&session);
    assert!(report.metrics.is_some(), "metrics missing in report");
    let cfg = report.cfg.expect("cfg summary missing");
    assert!(cfg.total_methods >= 1);

    let deps = report.deps.expect("deps summary missing");
    assert!(deps.nodes >= 1);
    assert!(deps.edges >= 1);

    // High complexity / deep nesting methods should be counted relative to thresholds
    assert!(cfg.high_complexity_methods <= cfg.total_methods);
    assert!(cfg.deep_nesting_methods <= cfg.total_methods);
}
