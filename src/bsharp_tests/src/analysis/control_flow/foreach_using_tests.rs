#![allow(unused_variables)]

use analysis::AnalysisContext;
use analysis::artifacts::cfg::ControlFlowIndex;
use analysis::framework::{AnalysisSession, AnalyzerPipeline};
use parser::facade::Parser;

#[test]
fn foreach_increases_complexity_and_nesting() {
    let source = r#"
namespace N { public class C {
    public void M() {
        foreach (var x in xs) { }
    }
}}
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let index = session
        .artifacts
        .get::<ControlFlowIndex>()
        .expect("missing index");
    let stats = index.0.get("N.C::M").expect("missing method stats");
    assert!(
        stats.complexity >= 2,
        "expected cyclomatic complexity to account for foreach"
    );
    assert!(
        stats.max_nesting >= 1,
        "expected nesting depth to include foreach body"
    );
}

#[test]
fn nested_foreach_increases_nesting() {
    let source = r#"
namespace N { public class C {
    public void M() {
        foreach (var x in xs) {
            foreach (var y in ys) { }
        }
    }
}}
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let index = session
        .artifacts
        .get::<ControlFlowIndex>()
        .expect("missing index");
    let stats = index.0.get("N.C::M").expect("missing method stats");
    assert!(
        stats.max_nesting >= 2,
        "expected nesting depth >= 2 for nested foreach"
    );
}

#[test]
fn using_counts_as_decision_and_nesting() {
    let source = r#"
namespace N { public class C {
    public void M() {
        using (var d = Get()) { if (true) { } }
    }
}}
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let index = session
        .artifacts
        .get::<ControlFlowIndex>()
        .expect("missing index");
    let stats = index.0.get("N.C::M").expect("missing method stats");
    assert!(stats.complexity >= 3, "using + if should add to complexity");
    assert!(
        stats.max_nesting >= 2,
        "using contains if block -> nesting >= 2"
    );
}

#[test]
fn foreach_and_using_combined() {
    let source = r#"
namespace N { public class C {
    public void M() {
        foreach (var x in xs) {
            using (var d = Get()) { if (true) { } }
        }
    }
}}
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    let index = session
        .artifacts
        .get::<ControlFlowIndex>()
        .expect("missing index");
    let stats = index.0.get("N.C::M").expect("missing method stats");
    assert!(
        stats.complexity >= 4,
        "foreach + using + if should grow complexity"
    );
    assert!(
        stats.max_nesting >= 3,
        "nested foreach -> using -> if should yield depth >= 3"
    );
}
