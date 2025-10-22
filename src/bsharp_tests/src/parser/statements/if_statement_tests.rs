// Integration tests for if_statement_parser.rs

use parser::expressions::statements::if_statement_parser::parse_if_statement;
use parser::syntax::test_helpers::parse_all;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;
use syntax::identifier::Identifier;
use syntax::statements::statement::Statement;

#[test]
fn test_parse_if_statement() {
    let input_if_only = "if (true) { return 1; }";
    let result_if_only = parse_all(parse_if_statement, input_if_only.into());
    assert!(result_if_only.is_ok());
    match result_if_only.unwrap().1 {
        Statement::If(if_stmt) => {
            // Check the condition
            assert!(matches!(
                if_stmt.condition,
                Expression::Literal(Literal::Boolean(true))
            ));
            // Check the consequence block
            assert!(matches!(*if_stmt.consequence, Statement::Block(_)));
            assert!(if_stmt.alternative.is_none());
        }
        _ => panic!("Expected If statement"),
    }

    let input_if_else = "if (x) DoTrue(); else DoFalse();";
    let result_if_else = parse_all(parse_if_statement, input_if_else.into());
    assert!(result_if_else.is_ok());
    match result_if_else.unwrap().1 {
        Statement::If(if_stmt) => {
            // Check the condition
            assert!(matches!(if_stmt.condition, Expression::Variable(_)));
            // Check the consequence (ExpressionStatement)
            assert!(matches!(*if_stmt.consequence, Statement::Expression(_)));
            // Check the alternative (else block with ExpressionStatement)
            assert!(matches!(
                *if_stmt.alternative.unwrap(),
                Statement::Expression(_)
            ));
        }
        _ => panic!("Expected If statement"),
    }

    let input_if_else_if = "if (a) 1; else if (b) 2; else 3;"; // Requires careful parsing of else part
    // This structure is handled by how parse_statement recursively handles the else branch.
    let result_if_else_if = parse_all(parse_if_statement, input_if_else_if.into());
    assert!(result_if_else_if.is_ok());
    if let Statement::If(outer_if) = result_if_else_if.unwrap().1 {
        // Check outer if condition
        assert!(matches!(outer_if.condition, Expression::Variable(_))); // a
        // Check outer if consequence (should be an ExpressionStatement, not a Block)
        assert!(matches!(*outer_if.consequence, Statement::Expression(_))); // 1

        if let Statement::If(inner_if) = *outer_if.alternative.unwrap() {
            // Check inner if condition
            assert!(matches!(inner_if.condition, Expression::Variable(_))); // b
            // Check inner if consequence
            assert!(matches!(*inner_if.consequence, Statement::Expression(_))); // 2
            // Check inner if alternative
            assert!(matches!(
                *inner_if.alternative.unwrap(),
                Statement::Expression(_)
            )); // 3
        } else {
            panic!("Expected inner IfStatement as alternative of outer if");
        }
    } else {
        panic!("Expected If statement");
    }
}
