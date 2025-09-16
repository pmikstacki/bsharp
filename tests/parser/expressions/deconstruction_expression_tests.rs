// Tests for parsing deconstruction expressions

use bsharp::syntax::nodes::expressions::{DeconstructionExpression, DeconstructionTarget, Expression};
use bsharp::syntax::nodes::types::{Type, PrimitiveType};
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::parser::expressions::deconstruction_expression_parser::parse_deconstruction_expression;

fn parse_deconstruction_expr(code: &str) -> Result<DeconstructionExpression, String> {
    match parse_deconstruction_expression(code) {
        Ok((rest, expr)) if rest.trim().is_empty() => Ok(expr),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_var_deconstruction() {
    let code = "(var x, var y) = tuple";
    let expected = DeconstructionExpression {
        targets: vec![
            DeconstructionTarget::Declaration {
                variable_type: None,
                name: Identifier::new("x"),
                is_var: true,
            },
            DeconstructionTarget::Declaration {
                variable_type: None,
                name: Identifier::new("y"),
                is_var: true,
            },
        ],
        value: Box::new(Expression::Variable(Identifier::new("tuple"))),
    };
    assert_eq!(parse_deconstruction_expr(code), Ok(expected));
}

#[test]
fn test_parse_typed_deconstruction() {
    let code = "(int a, string b) = GetTuple()";
    let expected = DeconstructionExpression {
        targets: vec![
            DeconstructionTarget::Declaration {
                variable_type: Some(Type::Primitive(PrimitiveType::Int)),
                name: Identifier::new("a"),
                is_var: false,
            },
            DeconstructionTarget::Declaration {
                variable_type: Some(Type::Primitive(PrimitiveType::String)),
                name: Identifier::new("b"),
                is_var: false,
            },
        ],
        value: Box::new(Expression::Invocation(Box::new(
            bsharp::syntax::nodes::expressions::InvocationExpression {
                callee: Box::new(Expression::Variable(Identifier::new("GetTuple"))),
                arguments: vec![],
            }
        ))),
    };
    assert_eq!(parse_deconstruction_expr(code), Ok(expected));
}

#[test]
fn test_parse_mixed_deconstruction() {
    let code = "(var x, int y) = point";
    let expected = DeconstructionExpression {
        targets: vec![
            DeconstructionTarget::Declaration {
                variable_type: None,
                name: Identifier::new("x"),
                is_var: true,
            },
            DeconstructionTarget::Declaration {
                variable_type: Some(Type::Primitive(PrimitiveType::Int)),
                name: Identifier::new("y"),
                is_var: false,
            },
        ],
        value: Box::new(Expression::Variable(Identifier::new("point"))),
    };
    assert_eq!(parse_deconstruction_expr(code), Ok(expected));
}

#[test]
fn test_parse_existing_variable_deconstruction() {
    let code = "(existingX, existingY) = tuple";
    let expected = DeconstructionExpression {
        targets: vec![
            DeconstructionTarget::Variable(Identifier::new("existingX")),
            DeconstructionTarget::Variable(Identifier::new("existingY")),
        ],
        value: Box::new(Expression::Variable(Identifier::new("tuple"))),
    };
    assert_eq!(parse_deconstruction_expr(code), Ok(expected));
}

#[test]
fn test_parse_deconstruction_with_discard() {
    let code = "(var x, _) = tuple";
    let expected = DeconstructionExpression {
        targets: vec![
            DeconstructionTarget::Declaration {
                variable_type: None,
                name: Identifier::new("x"),
                is_var: true,
            },
            DeconstructionTarget::Discard,
        ],
        value: Box::new(Expression::Variable(Identifier::new("tuple"))),
    };
    assert_eq!(parse_deconstruction_expr(code), Ok(expected));
}

#[test]
fn test_parse_nested_deconstruction() {
    let code = "((var a, var b), var c) = nestedTuple";
    let expected = DeconstructionExpression {
        targets: vec![
            DeconstructionTarget::Nested(vec![
                DeconstructionTarget::Declaration {
                    variable_type: None,
                    name: Identifier::new("a"),
                    is_var: true,
                },
                DeconstructionTarget::Declaration {
                    variable_type: None,
                    name: Identifier::new("b"),
                    is_var: true,
                },
            ]),
            DeconstructionTarget::Declaration {
                variable_type: None,
                name: Identifier::new("c"),
                is_var: true,
            },
        ],
        value: Box::new(Expression::Variable(Identifier::new("nestedTuple"))),
    };
    assert_eq!(parse_deconstruction_expr(code), Ok(expected));
}

#[test]
fn test_parse_complex_value_expression() {
    let code = "(var x, var y) = obj.GetTuple()";
    let result = parse_deconstruction_expr(code);
    assert!(result.is_ok());
    let deconstruction = result.unwrap();
    assert_eq!(deconstruction.targets.len(), 2);
    // The value should be a member access expression calling GetTuple
    assert!(matches!(*deconstruction.value, Expression::Invocation(_)));
}

#[test]
fn test_parse_deconstruction_with_array_access() {
    let code = "(var x, var y) = tuples[0]";
    let result = parse_deconstruction_expr(code);
    assert!(result.is_ok());
    let deconstruction = result.unwrap();
    assert_eq!(deconstruction.targets.len(), 2);
    // The value should be an indexing expression
    assert!(matches!(*deconstruction.value, Expression::Indexing(_)));
}

#[test]
fn test_parse_deconstruction_whitespace_variations() {
    let variations = [
        "(var x, var y) = tuple",
        "( var x , var y ) = tuple",
        "(\tvar x,\tvar y\t) = tuple",
        "(var x,\n var y) = tuple",
    ];

    for code in &variations {
        let result = parse_deconstruction_expr(code);
        assert!(result.is_ok(), "Failed to parse: {}", code);
        let deconstruction = result.unwrap();
        assert_eq!(deconstruction.targets.len(), 2);
    }
}

#[test]
fn test_parse_complex_types_in_deconstruction() {
    let code = "(List<string> items, Dictionary<int, string> dict) = GetComplexTuple()";
    let result = parse_deconstruction_expr(code);
    assert!(result.is_ok());
    let deconstruction = result.unwrap();
    assert_eq!(deconstruction.targets.len(), 2);
    
    // Check that both targets are declarations with complex types
    for target in &deconstruction.targets {
        assert!(matches!(target, DeconstructionTarget::Declaration { is_var: false, .. }));
    }
}

#[test]
fn test_parse_nullable_types_in_deconstruction() {
    let code = "(int? x, string? y) = GetNullableTuple()";
    let result = parse_deconstruction_expr(code);
    assert!(result.is_ok());
    let deconstruction = result.unwrap();
    assert_eq!(deconstruction.targets.len(), 2);
}

#[test]
fn test_deconstruction_parsing_errors() {
    let invalid_cases = [
        "() = tuple",                    // Empty target list
        "(var) = tuple",                 // Single target without comma
        "(var x var y) = tuple",         // Missing comma
        "(var x,) = tuple",              // Trailing comma without target
        "var x, var y = tuple",          // Missing parentheses
        "(var x, var y) =",              // Missing value
        "(var x, var y) tuple",          // Missing assignment operator
    ];

    for code in &invalid_cases {
        let result = parse_deconstruction_expr(code);
        assert!(result.is_err(), "Should fail to parse: {}", code);
    }
}

#[test]
fn test_deeply_nested_deconstruction() {
    let code = "(((var a, var b), var c), var d) = deeplyNested";
    let result = parse_deconstruction_expr(code);
    assert!(result.is_ok());
    let deconstruction = result.unwrap();
    assert_eq!(deconstruction.targets.len(), 2);
    
    // Verify the nested structure
    match &deconstruction.targets[0] {
        DeconstructionTarget::Nested(inner) => {
            assert_eq!(inner.len(), 2);
            match &inner[0] {
                DeconstructionTarget::Nested(deep_inner) => {
                    assert_eq!(deep_inner.len(), 2);
                }
                _ => panic!("Expected nested deconstruction target"),
            }
        }
        _ => panic!("Expected nested deconstruction target"),
    }
}
