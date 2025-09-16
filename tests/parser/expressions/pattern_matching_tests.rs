use bsharp::syntax::nodes::expressions::pattern::*;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::parser::expressions::pattern_parser::parse_pattern;
use bsharp::parser::expressions::switch_expression_parser::{parse_switch_expression, parse_is_pattern_expression};

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

// ===== COMPREHENSIVE RECORD PATTERN TESTS =====

#[test]
fn test_parse_record_pattern_property_syntax() {
    let result = parse_pattern_test("Person { FirstName: \"John\" }");
    assert!(result.is_ok(), "Failed to parse record pattern with property syntax: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 1);
        assert_eq!(subpatterns[0].member_name.name, "FirstName");
        assert!(matches!(subpatterns[0].pattern, Pattern::Constant(_)));
    } else {
        panic!("Expected record property pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_multiple_properties() {
    let result = parse_pattern_test("Person { FirstName: \"John\", LastName: \"Doe\", Age: > 18 }");
    assert!(result.is_ok(), "Failed to parse record pattern with multiple properties: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 3);
        
        assert_eq!(subpatterns[0].member_name.name, "FirstName");
        assert!(matches!(subpatterns[0].pattern, Pattern::Constant(_)));
        
        assert_eq!(subpatterns[1].member_name.name, "LastName");
        assert!(matches!(subpatterns[1].pattern, Pattern::Constant(_)));
        
        assert_eq!(subpatterns[2].member_name.name, "Age");
        assert!(matches!(subpatterns[2].pattern, Pattern::Relational { .. }));
    } else {
        panic!("Expected record property pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_with_var_patterns() {
    let result = parse_pattern_test("Person { FirstName: var first, LastName: var last }");
    assert!(result.is_ok(), "Failed to parse record pattern with var patterns: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 2);
        
        assert_eq!(subpatterns[0].member_name.name, "FirstName");
        assert!(matches!(subpatterns[0].pattern, Pattern::Var(_)));
        
        assert_eq!(subpatterns[1].member_name.name, "LastName");
        assert!(matches!(subpatterns[1].pattern, Pattern::Var(_)));
    } else {
        panic!("Expected record property pattern with var patterns, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_positional_syntax() {
    let result = parse_pattern_test("Person(\"John\", \"Doe\")");
    assert!(result.is_ok(), "Failed to parse record pattern with positional syntax: {:?}", result);
    
    if let Ok(Pattern::Positional { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 2);
        assert!(subpatterns.iter().all(|p| matches!(p, Pattern::Constant(_))));
    } else {
        panic!("Expected record positional pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_positional_with_var() {
    let result = parse_pattern_test("Person(var first, var last)");
    assert!(result.is_ok(), "Failed to parse record pattern with positional var: {:?}", result);
    
    if let Ok(Pattern::Positional { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 2);
        assert!(subpatterns.iter().all(|p| matches!(p, Pattern::Var(_))));
    } else {
        panic!("Expected record positional pattern with var, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_positional_mixed() {
    let result = parse_pattern_test("Person(\"John\", var last, > 18)");
    assert!(result.is_ok(), "Failed to parse record pattern with mixed positional: {:?}", result);
    
    if let Ok(Pattern::Positional { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 3);
        assert!(matches!(subpatterns[0], Pattern::Constant(_)));
        assert!(matches!(subpatterns[1], Pattern::Var(_)));
        assert!(matches!(subpatterns[2], Pattern::Relational { .. }));
    } else {
        panic!("Expected record positional pattern with mixed patterns, got: {:?}", result);
    }
}

#[test]
fn test_parse_nested_record_pattern() {
    let result = parse_pattern_test("Person { Address: { City: \"NYC\" } }");
    assert!(result.is_ok(), "Failed to parse nested record pattern: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 1);
        assert_eq!(subpatterns[0].member_name.name, "Address");
        assert!(matches!(subpatterns[0].pattern, Pattern::Property { .. }));
    } else {
        panic!("Expected nested record pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_with_discard() {
    let result = parse_pattern_test("Person { FirstName: var name, LastName: _ }");
    assert!(result.is_ok(), "Failed to parse record pattern with discard: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 2);
        
        assert_eq!(subpatterns[0].member_name.name, "FirstName");
        assert!(matches!(subpatterns[0].pattern, Pattern::Var(_)));
        
        assert_eq!(subpatterns[1].member_name.name, "LastName");
        assert!(matches!(subpatterns[1].pattern, Pattern::Discard));
    } else {
        panic!("Expected record pattern with discard, got: {:?}", result);
    }
}

#[test]
fn test_parse_generic_record_pattern() {
    let result = parse_pattern_test("Result<string>(var value)");
    assert!(result.is_ok(), "Failed to parse generic record pattern: {:?}", result);
    
    if let Ok(Pattern::Positional { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for generic record pattern");
        assert_eq!(subpatterns.len(), 1);
        assert!(matches!(subpatterns[0], Pattern::Var(_)));
    } else {
        panic!("Expected generic record pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_qualified_record_pattern() {
    let result = parse_pattern_test("MyNamespace.Person { Name: var name }");
    assert!(result.is_ok(), "Failed to parse qualified record pattern: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for qualified record pattern");
        assert_eq!(subpatterns.len(), 1);
        assert_eq!(subpatterns[0].member_name.name, "Name");
        assert!(matches!(subpatterns[0].pattern, Pattern::Var(_)));
    } else {
        panic!("Expected qualified record pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_in_switch_expression() {
    let result = parse_switch_test("person switch { Person { Age: > 18 } => \"adult\", Person { Age: <= 18 } => \"minor\", _ => \"unknown\" }");
    assert!(result.is_ok(), "Failed to parse switch expression with record patterns: {:?}", result);
    
    if let Ok(Expression::SwitchExpression(switch_expr)) = result {
        assert_eq!(switch_expr.arms.len(), 3);
        
        // Check first arm has record pattern
        assert!(matches!(switch_expr.arms[0].pattern, Pattern::Property { .. }));
        
        // Check second arm has record pattern
        assert!(matches!(switch_expr.arms[1].pattern, Pattern::Property { .. }));
        
        // Check third arm is discard
        assert!(matches!(switch_expr.arms[2].pattern, Pattern::Discard));
    } else {
        panic!("Expected switch expression with record patterns, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_with_relational_and_logical() {
    let result = parse_pattern_test("Person { Age: > 18 and < 65, Name: not null }");
    assert!(result.is_ok(), "Failed to parse record pattern with complex sub-patterns: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 2);
        
        assert_eq!(subpatterns[0].member_name.name, "Age");
        assert!(matches!(subpatterns[0].pattern, Pattern::LogicalAnd(_, _)));
        
        assert_eq!(subpatterns[1].member_name.name, "Name");
        assert!(matches!(subpatterns[1].pattern, Pattern::Not(_)));
    } else {
        panic!("Expected record pattern with complex sub-patterns, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_empty() {
    let result = parse_pattern_test("Person { }");
    assert!(result.is_ok(), "Failed to parse empty record pattern: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 0);
    } else {
        panic!("Expected empty record pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_positional_record_pattern_empty() {
    let result = parse_pattern_test("Person()");
    assert!(result.is_ok(), "Failed to parse empty positional record pattern: {:?}", result);
    
    if let Ok(Pattern::Positional { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 0);
    } else {
        panic!("Expected empty positional record pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_complex_nested_record_pattern() {
    let result = parse_pattern_test("Order { Customer: Person { Name: var customerName }, Items: [var first, ..] }");
    assert!(result.is_ok(), "Failed to parse complex nested record pattern: {:?}", result);
    
    if let Ok(Pattern::Property { type_name, subpatterns }) = result {
        assert!(type_name.is_some(), "Expected type name for record pattern");
        assert_eq!(subpatterns.len(), 2);
        
        // Check Customer property has nested record pattern
        assert_eq!(subpatterns[0].member_name.name, "Customer");
        assert!(matches!(subpatterns[0].pattern, Pattern::Property { .. }));
        
        // Check Items property has list pattern
        assert_eq!(subpatterns[1].member_name.name, "Items");
        assert!(matches!(subpatterns[1].pattern, Pattern::List { .. }));
    } else {
        panic!("Expected complex nested record pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_is_expression() {
    let result = parse_is_pattern_test("obj is Person { Name: var name }");
    assert!(result.is_ok(), "Failed to parse is expression with record pattern: {:?}", result);
    
    if let Ok(Expression::IsPattern { expression, pattern }) = result {
        assert!(matches!(expression.as_ref(), Expression::Variable(_)));
        assert!(matches!(pattern.as_ref(), Pattern::Property { .. }));
    } else {
        panic!("Expected is expression with record pattern, got: {:?}", result);
    }
}

#[test]
fn test_parse_record_pattern_whitespace_variations() {
    let inputs = vec![
        "Person{Name:var name}",
        "Person { Name : var name }",
        "Person{\n    Name: var name\n}",
        "Person\n{\n    Name:\n        var name\n}",
    ];
    
    for input in inputs {
        let result = parse_pattern_test(input);
        assert!(result.is_ok(), "Failed to parse record pattern with whitespace variation '{}': {:?}", input, result);
        
        if let Ok(Pattern::Property { type_name, subpatterns }) = result {
            assert!(type_name.is_some(), "Expected type name for record pattern");
            assert_eq!(subpatterns.len(), 1);
            assert_eq!(subpatterns[0].member_name.name, "Name");
        } else {
            panic!("Expected record pattern for input '{}', got: {:?}", input, result);
        }
    }
}

#[test]
fn test_parse_record_pattern_errors() {
    let invalid_inputs = vec![
        "Person { : var name }",  // Missing property name
        "Person { Name }",        // Missing pattern
        "Person { Name: }",       // Missing pattern after colon
        "Person Name: var name }", // Missing opening brace
        "Person { Name: var name", // Missing closing brace
    ];
    
    for input in invalid_inputs {
        let result = parse_pattern_test(input);
        assert!(result.is_err(), "Expected error for invalid record pattern '{}', got: {:?}", input, result);
    }
} 