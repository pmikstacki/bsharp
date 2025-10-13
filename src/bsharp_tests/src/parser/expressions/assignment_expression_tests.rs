// Tests for parsing assignment expressions

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::expressions::assignment_expression::AssignmentExpression;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;
use syntax::expressions::BinaryOperator;
use syntax::Identifier;

fn parse_assignment_expr_helper(code: &str) -> Result<AssignmentExpression, String> {
    match parse_expression(code) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => match expr {
            Expression::Assignment(boxed_assignment_expr) => Ok(*boxed_assignment_expr),
            _ => Err(format!("Expected Expression::Assignment, got {:?}", expr)),
        },
        Ok((remaining, _)) => Err(format!(
            "Didn't consume all input. Remaining: '{}'",
            remaining
        )),
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
        _ => panic!(
            "Expected target to be Expression::Variable 'x', got {:?}",
            assignment_expr.target
        ),
    }

    match *assignment_expr.value {
        Expression::Literal(Literal::Integer(5)) => { /* Correct */ }
        _ => panic!(
            "Expected value to be Expression::Literal(Integer(5)), got {:?}",
            assignment_expr.value
        ),
    }
}

#[test]
fn test_null_coalescing_assignment() {
    let code = "x ??= 42";
    let result = parse_assignment_expr_helper(code);
    assert!(
        result.is_ok(),
        "Parsing 'x ??= 42' failed: {:?}",
        result.err()
    );
    let assignment_expr = result.unwrap();

    assert_eq!(assignment_expr.op, BinaryOperator::NullCoalescingAssign);

    match *assignment_expr.target {
        Expression::Variable(Identifier { ref name }) => assert_eq!(name, "x"),
        _ => panic!(
            "Expected target to be Expression::Variable 'x', got {:?}",
            assignment_expr.target
        ),
    }

    match *assignment_expr.value {
        Expression::Literal(Literal::Integer(42)) => { /* Correct */ }
        _ => panic!(
            "Expected value to be Expression::Literal(Integer(42)), got {:?}",
            assignment_expr.value
        ),
    }
}

#[test]
fn test_null_coalescing_assignment_with_complex_expression() {
    let code = "data.Value ??= GetDefaultValue()";
    let result = parse_assignment_expr_helper(code);
    assert!(
        result.is_ok(),
        "Parsing 'data.Value ??= GetDefaultValue()' failed: {:?}",
        result.err()
    );
    let assignment_expr = result.unwrap();

    assert_eq!(assignment_expr.op, BinaryOperator::NullCoalescingAssign);

    // Target should be a member access expression
    match *assignment_expr.target {
        Expression::MemberAccess(_) => { /* Correct */ }
        _ => panic!(
            "Expected target to be Expression::MemberAccess, got {:?}",
            assignment_expr.target
        ),
    }

    // Value should be an invocation expression
    match *assignment_expr.value {
        Expression::Invocation(_) => { /* Correct */ }
        _ => panic!(
            "Expected value to be Expression::Invocation, got {:?}",
            assignment_expr.value
        ),
    }
}

#[test]
fn test_null_coalescing_assignment_chain() {
    let code = "a ??= b ??= c";
    let result = parse_assignment_expr_helper(code);
    assert!(
        result.is_ok(),
        "Parsing 'a ??= b ??= c' failed: {:?}",
        result.err()
    );
    let assignment_expr = result.unwrap();

    assert_eq!(assignment_expr.op, BinaryOperator::NullCoalescingAssign);

    // The value should be another assignment expression due to right-associativity
    match *assignment_expr.value {
        Expression::Assignment(inner_assignment) => {
            assert_eq!(inner_assignment.op, BinaryOperator::NullCoalescingAssign);
        }
        _ => panic!(
            "Expected value to be another Assignment expression for right-associativity, got {:?}",
            assignment_expr.value
        ),
    }
}

#[test]
fn test_null_coalescing_vs_null_coalescing_assignment() {
    // Test that ?? and ??= are parsed correctly and don't interfere with each other
    let code = "result = x ?? y";
    let result = parse_assignment_expr_helper(code);
    assert!(
        result.is_ok(),
        "Parsing 'result = x ?? y' failed: {:?}",
        result.err()
    );
    let assignment_expr = result.unwrap();

    assert_eq!(assignment_expr.op, BinaryOperator::Assign);

    // Value should be a binary expression with null coalescing
    match *assignment_expr.value {
        Expression::Binary {
            op: BinaryOperator::NullCoalescing,
            ..
        } => { /* Correct */ }
        _ => panic!(
            "Expected value to be Binary expression with NullCoalescing, got {:?}",
            assignment_expr.value
        ),
    }
}

#[test]
fn test_null_coalescing_assignment_precedence() {
    let code = "x = y ??= z + 1";
    let result = parse_assignment_expr_helper(code);
    assert!(
        result.is_ok(),
        "Parsing 'x = y ??= z + 1' failed: {:?}",
        result.err()
    );
    let assignment_expr = result.unwrap();

    assert_eq!(assignment_expr.op, BinaryOperator::Assign);

    // Value should be a null-coalescing assignment with the addition as the right side
    match *assignment_expr.value {
        Expression::Assignment(inner_assignment) => {
            assert_eq!(inner_assignment.op, BinaryOperator::NullCoalescingAssign);
            match *inner_assignment.value {
                Expression::Binary {
                    op: BinaryOperator::Add,
                    ..
                } => { /* Correct */ }
                _ => panic!(
                    "Expected right side of ??= to be addition, got {:?}",
                    inner_assignment.value
                ),
            }
        }
        _ => panic!(
            "Expected value to be Assignment expression, got {:?}",
            assignment_expr.value
        ),
    }
}
