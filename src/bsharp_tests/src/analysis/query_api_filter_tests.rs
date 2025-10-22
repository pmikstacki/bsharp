use analysis::context::AnalysisContext;
use analysis::framework::Query;
use analysis::framework::{AnalysisSession, AnalyzerPipeline};
use parser::facade::Parser;

#[test]
fn query_filter_and_filter_typed_match_manual_counts() {
    let src = r#"
using System;
public class C {
  public void M() {
    if (true) { Console.WriteLine(42); }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    // Count If statements via of<IfStatement>
    let if_count = Query::from(&cu)
        .of::<analysis::syntax::statements::if_statement::IfStatement>()
        .count();

    // Count If statements via filter on Dyn NodeRef
    let if_count_filter = Query::from(&cu)
        .filter(|n| {
            n.of::<analysis::syntax::statements::if_statement::IfStatement>()
                .is_some()
        })
        .count();

    assert_eq!(if_count, if_count_filter);

    // filter_typed for invocations in the subtree
    let inv_count = Query::from(&cu)
        .filter_typed::<analysis::syntax::expressions::invocation_expression::InvocationExpression>(
            |_i| true,
        )
        .count();
    assert!(inv_count >= 1);
}
