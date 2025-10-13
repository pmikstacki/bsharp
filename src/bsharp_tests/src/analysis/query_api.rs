use analysis::context::AnalysisContext;
use analysis::framework::Query;
use analysis::framework::{AnalysisSession, AnalyzerPipeline};
use parser::facade::Parser;

#[test]
fn query_of_finds_if_statements_and_invocations() {
    let src = r#"
using System;
public class C {
  public void M() {
    if (true) Console.WriteLine(42);
  }
}
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    // Use Query API to find typed nodes
    let if_count = Query::from(&cu)
        .of::<analysis::syntax::statements::if_statement::IfStatement>()
        .count();
    assert_eq!(if_count, 1);

    let inv_count = Query::from(&cu)
        .of::<analysis::syntax::expressions::invocation_expression::InvocationExpression>()
        .count();
    assert!(inv_count >= 1);
}
