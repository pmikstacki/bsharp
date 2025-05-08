// Tests for parsing assignment expressions

use bsharp::parser::nodes::Identifier;
use bsharp::parser::nodes::expressions::assignment_expression::AssignmentExpression;
use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::expressions::literal::Literal;
use bsharp::parser::nodes::expressions::BinaryOperator;
use bsharp::parsers::expressions::assignment_expression_parser::parse_assignment_expression;
use bsharp::parser::test_helpers::parse_all;

fn parse_assignment_expr_helper(code: &str) -> Result<AssignmentExpression, String> {
    match parse_all(parse_assignment_expression, code) {
        Ok((_, expr)) => {
            match expr {
                Expression::Assignment(boxed_assignment_expr) => Ok(*boxed_assignment_expr),
                _ => Err(format!("Expected Expression::Assignment, got {:?}", expr)),
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_simple_assignment() {
    let code = "x = 5";
    let result = parse_assignment_expr_helper(code);
    assert!(result.is_ok(), "Parsing 'x = 5' failed: {:?}", result.err());
    let assignment_expr = result.unwrap();

    assert_eq!(assignment_expr.op, BinaryOperator::Assign);

    match *assignment_expr.target {
        Expression::Variable(Identifier { ref name }) => assert_eq!(name, "x"),
        _ => panic!("Expected target to be Expression::Variable 'x', got {:?}", assignment_expr.target),
    }

    match *assignment_expr.value {
        Expression::Literal(Literal::Integer(5)) => { /* Correct */ }
        _ => panic!("Expected value to be Expression::Literal(Integer(5)), got {:?}", assignment_expr.value),
    }
}
