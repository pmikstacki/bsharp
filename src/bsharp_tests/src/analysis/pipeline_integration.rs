use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use parser::facade::Parser;
use syntax::declarations::ClassBodyDeclaration;
use syntax::statements::statement::Statement;

fn parse(source: &str) -> (analysis::syntax::ast::CompilationUnit, parser::SpanTable) {
    Parser::new().parse_with_spans(source).expect("parse error")
}

#[test]
fn pipeline_single_file_produces_metrics_and_cfg() {
    let src = r#"
public class C {
  public void M() {
    if (true) { while (false) { } }
  }
}
"#;
    let (cu, spans) = parse(src);
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let report = AnalysisReport::from_session(&session);
    // Metrics present
    assert!(report.metrics.is_some());
    let m = report.metrics.as_ref().unwrap();
    assert!(m.total_classes >= 1);
    assert!(m.total_methods >= 1);
    assert!(m.total_if_statements >= 1);
    // CFG summary present
    assert!(report.cfg.is_some());
    let cfg = report.cfg.as_ref().unwrap();
    assert!(cfg.total_methods >= 1);
}

#[test]
fn pipeline_counts_statements_in_method_body() {
    let src = r#"
public class A {
  public void N() {
    for (int i=0; i<2; i++) {
      if (i > 0) { }
    }
  }
}
"#;
    let (cu, spans) = parse(src);
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    // Walk to find the method body and assert it's a Block with nested For
    let class = cu
        .declarations
        .iter()
        .find_map(|d| match d {
            analysis::syntax::ast::TopLevelDeclaration::Class(c) => Some(c),
            _ => None,
        })
        .expect("class expected");
    let method = class
        .body_declarations
        .iter()
        .find_map(|m| match m {
            ClassBodyDeclaration::Method(md) => Some(md),
            _ => None,
        })
        .expect("method expected");
    let body = method.body.as_ref().expect("body expected");
    match body {
        Statement::Block(stmts) => {
            assert!(stmts.iter().any(|s| matches!(s, Statement::For(_))));
        }
        _ => panic!("expected block body"),
    }
}
