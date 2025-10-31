// Integration tests for expression_statement_parser.rs
// Content moved from src/parser/statements/expression_statement_parser.rs

use parser::expressions::statements::expression_statement_parser::parse_expression_statement;
use parser::statement_parser::parse_statement_ws_spanned;
use parser::syntax::test_helpers::parse_all;
use syntax::expressions::BinaryOperator;
use syntax::expressions::assignment_expression::AssignmentExpression;
use syntax::expressions::expression::Expression;
use syntax::expressions::invocation_expression::InvocationExpression;
use syntax::expressions::literal::Literal;
use syntax::identifier::Identifier;
use syntax::statements::statement::Statement;

#[test]
fn test_parse_expression_statement() {
    // Simple assignment
    let input_assign = "x = 10;";
    let result_assign = parse_all(parse_expression_statement, input_assign.into());
    assert!(result_assign.is_ok());
    match result_assign.unwrap().1 {
        Statement::Expression(expr) => match expr {
            Expression::Assignment(ass_expr) => {
                assert!(matches!(*ass_expr.target, Expression::Variable(_)));
                assert_eq!(ass_expr.op, BinaryOperator::Assign);
                assert!(matches!(
                    *ass_expr.value,
                    Expression::Literal(Literal::Integer(_))
                ));
            }
            _ => panic!("Expected AssignmentExpression"),
        },
        _ => panic!("Expected Expression statement"),
    }

    // Invocation
    let input_invoke = "DoSomething();";
    let result_invoke = parse_all(parse_expression_statement, input_invoke.into());
    assert!(result_invoke.is_ok());
    match result_invoke.unwrap().1 {
        Statement::Expression(expr) => match expr {
            Expression::Invocation(_) => (),
            _ => panic!("Expected InvocationExpression"),
        },
        _ => panic!("Expected Expression statement"),
    }

    // Complex expression
    let input_complex = "obj.Method(a + b);";
    let result_complex = parse_all(parse_expression_statement, input_complex.into());
    println!("Complex expression result: {:?}", result_complex);
    assert!(
        result_complex.is_ok(),
        "Failed to parse '{}'! Error: {:?}",
        input_complex,
        result_complex.err()
    );
    match result_complex.unwrap().1 {
        Statement::Expression(expr) => match expr {
            Expression::Invocation(inv_expr) => {
                assert!(matches!(*inv_expr.callee, Expression::MemberAccess(_)));
                assert_eq!(inv_expr.arguments.len(), 1);
                assert!(matches!(
                    inv_expr.arguments[0].expr,
                    Expression::Binary { .. }
                ));
            }
            _ => panic!("Expected InvocationExpression"),
        },
        _ => panic!("Expected Expression statement"),
    }
}

// Helper function from statement_tests.rs
fn assert_statement_parses(code: &str, expected: Statement) {
    let code_trimmed = code.trim();
    match parse_all(parse_statement_ws_spanned, code_trimmed.into()) {
        Ok((_, parsed_statement)) => {
            assert_eq!(
                parsed_statement.node, expected,
                "Parsed statement does not match expected for code: {}\n",
                code_trimmed
            );
        }
        Err(e) => {
            panic!("Parser failed for code: '{}'\nError: {:?}", code_trimmed, e);
        }
    }
}

#[test]
fn test_parse_expression_statement_call() {
    assert_statement_parses(
        "DoSomething();",
        Statement::Expression(Expression::Invocation(Box::new(InvocationExpression {
            callee: Box::new(Expression::Variable(Identifier::new("DoSomething"))),
            arguments: vec![],
        }))),
    );
}

#[test]
fn test_parse_expression_statement_assignment() {
    assert_statement_parses(
        "x = 10;",
        Statement::Expression(Expression::Assignment(Box::new(AssignmentExpression {
            target: Box::new(Expression::Variable(Identifier::new("x"))),
            op: BinaryOperator::Assign,
            value: Box::new(Expression::Literal(Literal::Integer(10))),
        }))),
    );
}
