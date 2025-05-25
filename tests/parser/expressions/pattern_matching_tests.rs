use bsharp::parser::nodes::expressions::pattern::*;
use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parsers::expressions::pattern_parser::parse_pattern;
use bsharp::parsers::expressions::switch_expression_parser::{parse_switch_expression, parse_is_pattern_expression};

fn parse_pattern_test(code: &str) -> Result<Pattern, String> {
    match parse_pattern(code) {
        Ok((remaining, pattern)) => {
            if remaining.trim().is_empty() {
                Ok(pattern)
            } else {
                Err(format!("Unparsed input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_switch_test(code: &str) -> Result<Expression, String> {
    match parse_switch_expression(code) {
        Ok((remaining, expr)) => {
            if remaining.trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unparsed input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_is_pattern_test(code: &str) -> Result<Expression, String> {
    match parse_is_pattern_expression(code) {
        Ok((remaining, expr)) => {
            if remaining.trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unparsed input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_discard_pattern() {
    let result = parse_pattern_test("_");
    assert!(result.is_ok(), "Failed to parse discard pattern: {:?}", result);
    
    if let Ok(Pattern::Discard) = result {
        // Expected
    } else {
        panic!("Expected discard pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_var_pattern() {
    let result = parse_pattern_test("var x");
    assert!(result.is_ok(), "Failed to parse var pattern: {:?}", result);
    
    if let Ok(Pattern::Var(identifier)) = result {
        assert_eq!(identifier.name, "x");
    } else {
        panic!("Expected var pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_constant_pattern() {
    let result = parse_pattern_test("42");
    assert!(result.is_ok(), "Failed to parse constant pattern: {:?}", result);
    
    if let Ok(Pattern::Constant(Expression::Literal(_))) = result {
        // Expected
    } else {
        panic!("Expected constant pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_type_pattern() {
    let result = parse_pattern_test("int x");
    assert!(result.is_ok(), "Failed to parse type pattern: {:?}", result);
    
    if let Ok(Pattern::Type { target_type: _, designation }) = result {
        if let Some(PatternDesignation::Variable(var)) = designation {
            assert_eq!(var.name, "x");
        } else {
            panic!("Expected variable designation, got: {:?}", designation);
        }
    } else {
        panic!("Expected type pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_relational_pattern() {
    let result = parse_pattern_test("> 5");
    assert!(result.is_ok(), "Failed to parse relational pattern: {:?}", result);
    
    if let Ok(Pattern::Relational { op, value: _ }) = result {
        assert!(matches!(op, RelationalOperator::GreaterThan));
    } else {
        panic!("Expected relational pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_logical_and_pattern() {
    let result = parse_pattern_test("> 5 and < 10");
    assert!(result.is_ok(), "Failed to parse logical AND pattern: {:?}", result);
    
    if let Ok(Pattern::LogicalAnd(left, right)) = result {
        assert!(matches!(left.as_ref(), Pattern::Relational { .. }));
        assert!(matches!(right.as_ref(), Pattern::Relational { .. }));
    } else {
        panic!("Expected logical AND pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_logical_or_pattern() {
    let result = parse_pattern_test("1 or 2");
    assert!(result.is_ok(), "Failed to parse logical OR pattern: {:?}", result);
    
    if let Ok(Pattern::LogicalOr(left, right)) = result {
        assert!(matches!(left.as_ref(), Pattern::Constant(_)));
        assert!(matches!(right.as_ref(), Pattern::Constant(_)));
    } else {
        panic!("Expected logical OR pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_not_pattern() {
    let result = parse_pattern_test("not null");
    assert!(result.is_ok(), "Failed to parse NOT pattern: {:?}", result);
    
    if let Ok(Pattern::Not(inner)) = result {
        assert!(matches!(inner.as_ref(), Pattern::Constant(_)));
    } else {
        panic!("Expected NOT pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_tuple_pattern() {
    let result = parse_pattern_test("(1, 2, 3)");
    assert!(result.is_ok(), "Failed to parse tuple pattern: {:?}", result);
    
    if let Ok(Pattern::Tuple(patterns)) = result {
        assert_eq!(patterns.len(), 3);
        assert!(patterns.iter().all(|p| matches!(p, Pattern::Constant(_))));
    } else {
        panic!("Expected tuple pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_list_pattern() {
    let result = parse_pattern_test("[1, 2, ..]");
    assert!(result.is_ok(), "Failed to parse list pattern: {:?}", result);
    
    if let Ok(Pattern::List { patterns }) = result {
        assert_eq!(patterns.len(), 3);
        assert!(matches!(patterns[0], ListPatternElement::Pattern(_)));
        assert!(matches!(patterns[1], ListPatternElement::Pattern(_)));
        assert!(matches!(patterns[2], ListPatternElement::Slice(None)));
    } else {
        panic!("Expected list pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_property_pattern() {
    let result = parse_pattern_test("{ Name: var n, Age: > 18 }");
    assert!(result.is_ok(), "Failed to parse property pattern: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_none());
        assert_eq!(subpatterns.len(), 2);
        
        assert_eq!(subpatterns[0].member_name.name, "Name");
        assert!(matches!(subpatterns[0].pattern, Pattern::Var(_)));
        
        assert_eq!(subpatterns[1].member_name.name, "Age");
        assert!(matches!(subpatterns[1].pattern, Pattern::Relational { .. }));
    } else {
        panic!("Expected property pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_positional_pattern() {
    let result = parse_pattern_test("Point(1, 2)");
    assert!(result.is_ok(), "Failed to parse positional pattern: {:?}", result);
    
    if let Ok(Pattern::Positional { type_name, subpatterns }) = result {
        assert!(type_name.is_some());
        assert_eq!(subpatterns.len(), 2);
        assert!(subpatterns.iter().all(|p| matches!(p, Pattern::Constant(_))));
    } else {
        panic!("Expected positional pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_complex_pattern() {
    let result = parse_pattern_test("(> 0 and < 100) or var x");
    assert!(result.is_ok(), "Failed to parse complex pattern: {:?}", result);
    
    if let Ok(Pattern::LogicalOr(left, right)) = result {
        assert!(matches!(left.as_ref(), Pattern::Parenthesized(_)));
        assert!(matches!(right.as_ref(), Pattern::Var(_)));
    } else {
        panic!("Expected complex pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_simple_switch_expression() {
    let result = parse_switch_test("value switch { 1 => \"one\", 2 => \"two\", _ => \"other\" }");
    assert!(result.is_ok(), "Failed to parse simple switch expression: {:?}", result);
    
    if let Ok(Expression::SwitchExpression(switch_expr)) = result {
        assert!(matches!(switch_expr.expression, Expression::Variable(_)));
        assert_eq!(switch_expr.arms.len(), 3);
        
        // Check first arm
        assert!(matches!(switch_expr.arms[0].pattern, Pattern::Constant(_)));
        assert!(switch_expr.arms[0].when_clause.is_none());
        
        // Check last arm (discard pattern)
        assert!(matches!(switch_expr.arms[2].pattern, Pattern::Discard));
    } else {
        panic!("Expected switch expression, got: {:?}", result);
    }
}

#[test]
fn test_parse_switch_expression_with_when_clause() {
    let result = parse_switch_test("value switch { var x when x > 0 => \"positive\", _ => \"other\" }");
    assert!(result.is_ok(), "Failed to parse switch expression with when clause: {:?}", result);
    
    if let Ok(Expression::SwitchExpression(switch_expr)) = result {
        assert_eq!(switch_expr.arms.len(), 2);
        
        // Check first arm has when clause
        assert!(matches!(switch_expr.arms[0].pattern, Pattern::Var(_)));
        assert!(switch_expr.arms[0].when_clause.is_some());
    } else {
        panic!("Expected switch expression with when clause, got: {:?}", result);
    }
}

#[test]
fn test_parse_switch_expression_with_patterns() {
    let result = parse_switch_test("shape switch { Circle(var r) => r, Rectangle(var w, var h) => w * h, _ => 0 }");
    assert!(result.is_ok(), "Failed to parse switch expression with patterns: {:?}", result);
    
    if let Ok(Expression::SwitchExpression(switch_expr)) = result {
        assert_eq!(switch_expr.arms.len(), 3);
        
        // Check positional patterns
        assert!(matches!(switch_expr.arms[0].pattern, Pattern::Positional { .. }));
        assert!(matches!(switch_expr.arms[1].pattern, Pattern::Positional { .. }));
        assert!(matches!(switch_expr.arms[2].pattern, Pattern::Discard));
    } else {
        panic!("Expected switch expression with patterns, got: {:?}", result);
    }
}

#[test]
fn test_parse_is_pattern_expression() {
    let result = parse_is_pattern_test("obj is string s");
    assert!(result.is_ok(), "Failed to parse is pattern expression: {:?}", result);
    
    if let Ok(Expression::IsPattern { expression, pattern }) = result {
        assert!(matches!(expression.as_ref(), Expression::Variable(_)));
        assert!(matches!(pattern.as_ref(), Pattern::Type { .. }));
    } else {
        panic!("Expected is pattern expression, got: {:?}", result);
    }
}

#[test]
fn test_parse_is_pattern_with_property_pattern() {
    let result = parse_is_pattern_test("person is { Name: var name, Age: > 18 }");
    assert!(result.is_ok(), "Failed to parse is pattern with property pattern: {:?}", result);
    
    if let Ok(Expression::IsPattern { expression, pattern }) = result {
        assert!(matches!(expression.as_ref(), Expression::Variable(_)));
        assert!(matches!(pattern.as_ref(), Pattern::Property { .. }));
    } else {
        panic!("Expected is pattern with property pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_nested_patterns() {
    let result = parse_pattern_test("(var x and > 0) or (var y and < 0)");
    assert!(result.is_ok(), "Failed to parse nested patterns: {:?}", result);
    
    if let Ok(Pattern::LogicalOr(left, right)) = result {
        assert!(matches!(left.as_ref(), Pattern::Parenthesized(_)));
        assert!(matches!(right.as_ref(), Pattern::Parenthesized(_)));
    } else {
        panic!("Expected nested patterns, got: {:?}", result);
    }
}

#[test]
fn test_parse_list_pattern_with_slice() {
    let result = parse_pattern_test("[var first, .., var last]");
    assert!(result.is_ok(), "Failed to parse list pattern with slice: {:?}", result);
    
    if let Ok(Pattern::List { patterns }) = result {
        assert_eq!(patterns.len(), 3);
        assert!(matches!(patterns[0], ListPatternElement::Pattern(Pattern::Var(_))));
        assert!(matches!(patterns[1], ListPatternElement::Slice(None)));
        assert!(matches!(patterns[2], ListPatternElement::Pattern(Pattern::Var(_))));
    } else {
        panic!("Expected list pattern with slice, got: {:?}", result);
    }
} 