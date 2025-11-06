use bsharp_parser::facade::Parser;
use bsharp_syntax::declarations::{ClassDeclaration, MethodDeclaration};
use bsharp_syntax::query::Query;
use bsharp_syntax::span::Span;
use bsharp_syntax::statements::statement::Statement;

use bsharp_analysis::metrics::shared::{
    compute_statement_metrics, count_statements, decision_points, max_nesting_of,
};

fn parse_method_bodies(src: &str) -> Vec<Statement> {
    let parser = Parser::new();
    let (cu, _spans) = parser.parse_with_spans(Span::new(src)).expect("parse");
    let mut bodies = Vec::new();
    for class in Query::from(&cu).of::<ClassDeclaration>() {
        for method in Query::from(class).of::<MethodDeclaration>() {
            if let Some(b) = &method.body {
                bodies.push(b.clone());
            }
        }
    }
    bodies
}

#[test]
fn stmt_metrics_match_legacy_helpers_simple() {
    let src = r#"
class C {
    void M(int n) {
        if (n > 0) {
            for (int i = 0; i < n; i++) {
                if (i % 2 == 0) {}
            }
        } else {
            while (n > 0) { n--; }
        }
    }
}
"#;
    for body in parse_method_bodies(src) {
        let m = compute_statement_metrics(&body);
        assert_eq!(m.decision_points, decision_points(&body));
        assert_eq!(m.max_nesting, max_nesting_of(&body, 0));
        assert_eq!(m.statement_count, count_statements(Some(&body)));
    }
}

#[test]
fn stmt_metrics_match_legacy_helpers_control_flow() {
    let src = r#"
class C2 {
    void M2(int x) {
        try {
            switch (x) {
                case 0: break;
                case 1: using (var d = new D()) { if (x == 1) { } }
                        break;
                default: do { x--; } while (x > 0);
            }
        } catch (System.Exception) { }
        finally { }
    }
}
"#;
    for body in parse_method_bodies(src) {
        let m = compute_statement_metrics(&body);
        assert_eq!(m.decision_points, decision_points(&body));
        assert_eq!(m.max_nesting, max_nesting_of(&body, 0));
        assert_eq!(m.statement_count, count_statements(Some(&body)));
    }
}
