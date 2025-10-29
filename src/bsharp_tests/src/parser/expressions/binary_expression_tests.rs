use parser::expressions::primary_expression_parser::parse_expression_spanned as parse_expression;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;
use syntax::expressions::{AssignmentExpression, BinaryOperator};
use syntax::identifier::Identifier;

#[test]
fn test_simple_binary_expression() {
    let input = "1 + 2";
    let expected = Expression::Binary {
        left: Box::new(Expression::Literal(Literal::Integer(1))),
        op: BinaryOperator::Add,
        right: Box::new(Expression::Literal(Literal::Integer(2))),
    };
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);

    let input = "a * b";
    let expected = Expression::Binary {
        left: Box::new(Expression::Variable(Identifier::new("a"))),
        op: BinaryOperator::Multiply,
        right: Box::new(Expression::Variable(Identifier::new("b"))),
    };
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_simple_assignment_expression() {
    let input = "x = 10";
    let expected = Expression::Assignment(Box::new(AssignmentExpression {
        target: Box::new(Expression::Variable(Identifier::new("x"))),
        op: BinaryOperator::Assign,
        value: Box::new(Expression::Literal(Literal::Integer(10))),
    }));
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_compound_assignment_expression() {
    let input = "y += 5";
    let expected = Expression::Assignment(Box::new(AssignmentExpression {
        target: Box::new(Expression::Variable(Identifier::new("y"))),
        op: BinaryOperator::AddAssign,
        value: Box::new(Expression::Literal(Literal::Integer(5))),
    }));
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);

    let input = "z *= a";
    let expected = Expression::Assignment(Box::new(AssignmentExpression {
        target: Box::new(Expression::Variable(Identifier::new("z"))),
        op: BinaryOperator::MultiplyAssign,
        value: Box::new(Expression::Variable(Identifier::new("a"))),
    }));
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_precedence() {
    // a + b * c -> a + (b * c)
    let input = "a + b * c";
    let expected = Expression::Binary {
        left: Box::new(Expression::Variable(Identifier::new("a"))),
        op: BinaryOperator::Add,
        right: Box::new(Expression::Binary {
            left: Box::new(Expression::Variable(Identifier::new("b"))),
            op: BinaryOperator::Multiply,
            right: Box::new(Expression::Variable(Identifier::new("c"))),
        }),
    };
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);

    // x = y == z -> x = (y == z)
    let input = "x = y == z";
    let expected = Expression::Assignment(Box::new(AssignmentExpression {
        target: Box::new(Expression::Variable(Identifier::new("x"))),
        op: BinaryOperator::Assign,
        value: Box::new(Expression::Binary {
            left: Box::new(Expression::Variable(Identifier::new("y"))),
            op: BinaryOperator::Equal,
            right: Box::new(Expression::Variable(Identifier::new("z"))),
        }),
    }));
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_parentheses() {
    // (a + b) * c
    let input = "(a + b) * c";
    let expected = Expression::Binary {
        left: Box::new(Expression::Binary {
            left: Box::new(Expression::Variable(Identifier::new("a"))),
            op: BinaryOperator::Add,
            right: Box::new(Expression::Variable(Identifier::new("b"))),
        }),
        op: BinaryOperator::Multiply,
        right: Box::new(Expression::Variable(Identifier::new("c"))),
    };
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_left_associativity() {
    // a - b + c -> (a - b) + c
    let input = "a - b + c";
    let expected = Expression::Binary {
        left: Box::new(Expression::Binary {
            left: Box::new(Expression::Variable(Identifier::new("a"))),
            op: BinaryOperator::Subtract,
            right: Box::new(Expression::Variable(Identifier::new("b"))),
        }),
        op: BinaryOperator::Add,
        right: Box::new(Expression::Variable(Identifier::new("c"))),
    };
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_assignment_associativity() {
    let code = "x = y = 5";
    let expected = Expression::Assignment(Box::new(AssignmentExpression {
        target: Box::new(Expression::Variable(Identifier::new("x"))),
        op: BinaryOperator::Assign,
        value: Box::new(Expression::Assignment(Box::new(AssignmentExpression {
            target: Box::new(Expression::Variable(Identifier::new("y"))),
            op: BinaryOperator::Assign,
            value: Box::new(Expression::Literal(Literal::Integer(5))),
        }))),
    }));
    let (_, actual) = parse_expression(code.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_null_coalescing_assignment_expression() {
    let input = "x ??= 42";
    let expected = Expression::Assignment(Box::new(AssignmentExpression {
        target: Box::new(Expression::Variable(Identifier::new("x"))),
        op: BinaryOperator::NullCoalescingAssign,
        value: Box::new(Expression::Literal(Literal::Integer(42))),
    }));
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_null_coalescing_assignment_chain() {
    let input = "a ??= b ??= c";
    let expected = Expression::Assignment(Box::new(AssignmentExpression {
        target: Box::new(Expression::Variable(Identifier::new("a"))),
        op: BinaryOperator::NullCoalescingAssign,
        value: Box::new(Expression::Assignment(Box::new(AssignmentExpression {
            target: Box::new(Expression::Variable(Identifier::new("b"))),
            op: BinaryOperator::NullCoalescingAssign,
            value: Box::new(Expression::Variable(Identifier::new("c"))),
        }))),
    }));
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_null_coalescing_vs_null_coalescing_assignment() {
    // Test that ?? and ??= are parsed correctly and don't interfere with each other
    let input = "result = x ?? y";
    let expected = Expression::Assignment(Box::new(AssignmentExpression {
        target: Box::new(Expression::Variable(Identifier::new("result"))),
        op: BinaryOperator::Assign,
        value: Box::new(Expression::Binary {
            left: Box::new(Expression::Variable(Identifier::new("x"))),
            op: BinaryOperator::NullCoalescing,
            right: Box::new(Expression::Variable(Identifier::new("y"))),
        }),
    }));
    let (_, actual) = parse_expression(input.into()).map(|(rest, s)| (rest, s.node)).unwrap();
    assert_eq!(actual, expected);
}
