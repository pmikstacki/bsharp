use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::{expression::Expression, BinaryOperator};

fn parse_ok(input: &str) -> Expression {
    let (remaining, expr) = parse_expression(input).expect("parse ok");
    assert_eq!(remaining.trim(), "", "remaining input should be empty");
    expr
}

#[test]
fn ternary_vs_null_conditional_and_coalescing() {
    // a ? b : c -> Conditional
    let expr = parse_ok("a ? b : c");
    match expr {
        Expression::Conditional(_) => {}
        other => panic!("Expected Conditional, got {:?}", other),
    }

    // a?.b -> NullConditional (member)
    let expr = parse_ok("a?.b");
    match expr {
        Expression::NullConditional(nc) => {
            assert!(!nc.is_element_access);
        }
        other => panic!("Expected NullConditional member, got {:?}", other),
    }

    // a?[i] -> NullConditional (index)
    let expr = parse_ok("a?[i]");
    match expr {
        Expression::NullConditional(nc) => {
            assert!(nc.is_element_access);
        }
        other => panic!("Expected NullConditional index, got {:?}", other),
    }

    // a ?? b ? c : d -> Conditional with (a ?? b) as condition
    let expr = parse_ok("a ?? b ? c : d");
    match expr {
        Expression::Conditional(cond) => match *cond.condition {
            Expression::Binary { op: BinaryOperator::NullCoalescing, .. } => {}
            ref other => panic!("Expected condition to be NullCoalescing, got {:?}", other),
        },
        other => panic!("Expected Conditional, got {:?}", other),
    }
}

#[test]
fn caret_vs_range() {
    // ^1..^0
    let expr = parse_ok("^1..^0");
    match expr {
        Expression::Range(r) => {
            assert!(r.start.is_some());
            assert!(r.end.is_some());
        }
        other => panic!("Expected Range, got {:?}", other),
    }

    // ..^1
    let expr = parse_ok("..^1");
    match expr {
        Expression::Range(r) => {
            assert!(r.start.is_none());
            assert!(r.end.is_some());
        }
        other => panic!("Expected Range, got {:?}", other),
    }

    // ^1..
    let expr = parse_ok("^1..");
    match expr {
        Expression::Range(r) => {
            assert!(r.start.is_some());
            assert!(r.end.is_none());
        }
        other => panic!("Expected Range, got {:?}", other),
    }

    // a[^1]
    let expr = parse_ok("a[^1]");
    match expr {
        Expression::Indexing(_) => {}
        other => panic!("Expected Indexing of ^1, got {:?}", other),
    }
}

#[test]
fn operator_lookahead_boundaries() {
    // a | b || c -> top-level LogicalOr; left is BitwiseOr
    let expr = parse_ok("a | b || c");
    match expr {
        Expression::Binary { op: BinaryOperator::LogicalOr, left, .. } => match *left {
            Expression::Binary { op: BinaryOperator::BitwiseOr, .. } => {}
            ref other => panic!("Expected left to be BitwiseOr, got {:?}", other),
        },
        other => panic!("Expected top-level LogicalOr, got {:?}", other),
    }

    // a & b && c -> top-level LogicalAnd; left is BitwiseAnd
    let expr = parse_ok("a & b && c");
    match expr {
        Expression::Binary { op: BinaryOperator::LogicalAnd, left, .. } => match *left {
            Expression::Binary { op: BinaryOperator::BitwiseAnd, .. } => {}
            ref other => panic!("Expected left to be BitwiseAnd, got {:?}", other),
        },
        other => panic!("Expected top-level LogicalAnd, got {:?}", other),
    }

    // a & b &= c -> BitwiseAnd with right being AndAssign assignment
    let expr = parse_ok("a & b &= c");
    match expr {
        Expression::Binary { op: BinaryOperator::BitwiseAnd, right, .. } => match *right {
            Expression::Assignment(box assign) => {
                assert!(matches!(assign.op, BinaryOperator::AndAssign));
            }
            ref other => panic!("Expected right to be AndAssign assignment, got {:?}", other),
        },
        other => panic!("Expected top-level BitwiseAnd, got {:?}", other),
    }

    // a | b |= c -> BitwiseOr with right being OrAssign assignment
    let expr = parse_ok("a | b |= c");
    match expr {
        Expression::Binary { op: BinaryOperator::BitwiseOr, right, .. } => match *right {
            Expression::Assignment(box assign) => {
                assert!(matches!(assign.op, BinaryOperator::OrAssign));
            }
            ref other => panic!("Expected right to be OrAssign assignment, got {:?}", other),
        },
        other => panic!("Expected top-level BitwiseOr, got {:?}", other),
    }

    // a ^ b ^= c -> BitwiseXor with right being XorAssign assignment
    let expr = parse_ok("a ^ b ^= c");
    match expr {
        Expression::Binary { op: BinaryOperator::BitwiseXor, right, .. } => match *right {
            Expression::Assignment(box assign) => {
                assert!(matches!(assign.op, BinaryOperator::XorAssign));
            }
            ref other => panic!("Expected right to be XorAssign assignment, got {:?}", other),
        },
        other => panic!("Expected top-level BitwiseXor, got {:?}", other),
    }

    // a << b <<= c -> LeftShift with right being LeftShiftAssign assignment
    let expr = parse_ok("a << b <<= c");
    match expr {
        Expression::Binary { op: BinaryOperator::LeftShift, right, .. } => match *right {
            Expression::Assignment(box assign) => {
                assert!(matches!(assign.op, BinaryOperator::LeftShiftAssign));
            }
            ref other => panic!("Expected right to be LeftShiftAssign assignment, got {:?}", other),
        },
        other => panic!("Expected top-level LeftShift, got {:?}", other),
    }

    // a >> b >>= c -> RightShift with right being RightShiftAssign assignment
    let expr = parse_ok("a >> b >>= c");
    match expr {
        Expression::Binary { op: BinaryOperator::RightShift, right, .. } => match *right {
            Expression::Assignment(box assign) => {
                assert!(matches!(assign.op, BinaryOperator::RightShiftAssign));
            }
            ref other => panic!("Expected right to be RightShiftAssign assignment, got {:?}", other),
        },
        other => panic!("Expected top-level RightShift, got {:?}", other),
    }
}
