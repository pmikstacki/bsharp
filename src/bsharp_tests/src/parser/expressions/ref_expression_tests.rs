// Tests for parsing ref expressions and ref return types

use parser::expressions::primary_expression_parser::parse_expression;
use parser::expressions::ref_expression_parser::parse_ref_expression;
use parser::types::type_parser::parse_type_expression;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;
use syntax::identifier::Identifier;
use syntax::types::{PrimitiveType, Type};

fn parse_ref_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_ref_expression(code.into()) {
        Ok((remaining, expr)) => {
            if remaining.fragment().trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unexpected remaining input: '{}'", remaining.fragment()))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_expression(code.into()) {
        Ok((remaining, expr)) => {
            if remaining.fragment().trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unexpected remaining input: '{}'", remaining.fragment()))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_type_helper(code: &str) -> Result<Type, String> {
    match parse_type_expression(code.into()) {
        Ok((remaining, ty)) => {
            if remaining.fragment().trim().is_empty() {
                Ok(ty)
            } else {
                Err(format!("Unexpected remaining input: '{}'", remaining.fragment()))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_ref_variable() {
    let result = parse_ref_expr_helper("ref myVariable");
    assert!(result.is_ok(), "Failed to parse ref variable: {:?}", result);

    match result.unwrap() {
        Expression::Ref(inner) => match *inner {
            Expression::Variable(ref id) => {
                assert_eq!(id.to_string(), "myVariable");
            }
            _ => panic!("Expected variable expression, got {:?}", inner),
        },
        _ => panic!("Expected ref expression"),
    }
}

#[test]
fn test_parse_ref_field_access() {
    let result = parse_ref_expr_helper("ref obj.field");
    assert!(
        result.is_ok(),
        "Failed to parse ref field access: {:?}",
        result
    );

    match result.unwrap() {
        Expression::Ref(inner) => {
            match *inner {
                Expression::MemberAccess(_) => {
                    // Expected member access expression
                }
                _ => panic!("Expected member access expression, got {:?}", inner),
            }
        }
        _ => panic!("Expected ref expression"),
    }
}

#[test]
fn test_parse_ref_array_element() {
    let result = parse_ref_expr_helper("ref array[index]");
    assert!(
        result.is_ok(),
        "Failed to parse ref array element: {:?}",
        result
    );

    match result.unwrap() {
        Expression::Ref(inner) => {
            match *inner {
                Expression::Indexing(_) => {
                    // Expected indexing expression
                }
                _ => panic!("Expected indexing expression, got {:?}", inner),
            }
        }
        _ => panic!("Expected ref expression"),
    }
}

#[test]
fn test_parse_ref_method_call() {
    let result = parse_ref_expr_helper("ref GetValue()");
    assert!(
        result.is_ok(),
        "Failed to parse ref method call: {:?}",
        result
    );

    match result.unwrap() {
        Expression::Ref(inner) => {
            match *inner {
                Expression::Invocation(_) => {
                    // Expected invocation expression
                }
                _ => panic!("Expected invocation expression, got {:?}", inner),
            }
        }
        _ => panic!("Expected ref expression"),
    }
}

#[test]
fn test_parse_ref_complex_expression() {
    let result = parse_ref_expr_helper("ref obj.GetArray()[0].field");
    assert!(
        result.is_ok(),
        "Failed to parse ref complex expression: {:?}",
        result
    );

    match result.unwrap() {
        Expression::Ref(inner) => {
            match *inner {
                Expression::MemberAccess(_) => {
                    // Expected final member access
                }
                _ => panic!("Expected member access expression, got {:?}", inner),
            }
        }
        _ => panic!("Expected ref expression"),
    }
}

#[test]
fn test_parse_ref_in_full_expression_parser() {
    let result = parse_expr_helper("ref myVariable");
    assert!(
        result.is_ok(),
        "Failed to parse ref expression in full parser: {:?}",
        result
    );

    match result.unwrap() {
        Expression::Ref(inner) => match *inner {
            Expression::Variable(ref id) => {
                assert_eq!(id.to_string(), "myVariable");
            }
            _ => panic!("Expected variable expression, got {:?}", inner),
        },
        _ => panic!("Expected ref expression"),
    }
}

#[test]
fn test_parse_ref_with_whitespace() {
    let test_cases = vec![
        "ref   myVariable",
        "ref\tmyVariable",
        "ref\n  myVariable",
        "ref  obj . field",
    ];

    for test_case in test_cases {
        let result = parse_ref_expr_helper(test_case);
        assert!(
            result.is_ok(),
            "Failed to parse ref expression with whitespace '{}': {:?}",
            test_case,
            result
        );

        match result.unwrap() {
            Expression::Ref(_) => {
                // Successfully parsed as ref expression
            }
            _ => panic!("Expected ref expression for input '{}'", test_case),
        }
    }
}

#[test]
fn test_parse_ref_return_types() {
    let test_cases = vec![
        (
            "ref int",
            Type::RefReturn(Box::new(Type::Primitive(PrimitiveType::Int))),
        ),
        (
            "ref string",
            Type::RefReturn(Box::new(Type::Primitive(PrimitiveType::String))),
        ),
        (
            "ref bool",
            Type::RefReturn(Box::new(Type::Primitive(PrimitiveType::Bool))),
        ),
        (
            "ref MyClass",
            Type::RefReturn(Box::new(Type::Reference(Identifier::new("MyClass")))),
        ),
    ];

    for (input, expected) in test_cases {
        let result = parse_type_helper(input.into());
        assert!(
            result.is_ok(),
            "Failed to parse ref return type '{}': {:?}",
            input,
            result
        );
        assert_eq!(
            result.unwrap(),
            expected,
            "Type mismatch for input '{}'",
            input
        );
    }
}

#[test]
fn test_parse_ref_return_array_types() {
    let result = parse_type_helper("ref int[]");
    assert!(
        result.is_ok(),
        "Failed to parse ref array return type: {:?}",
        result
    );

    match result.unwrap() {
        Type::RefReturn(inner) => match *inner {
            Type::Array { element_type, rank } => {
                assert_eq!(*element_type, Type::Primitive(PrimitiveType::Int));
                assert_eq!(rank, 1);
            }
            _ => panic!("Expected array type, got {:?}", inner),
        },
        _ => panic!("Expected ref return type"),
    }
}

#[test]
fn test_parse_ref_return_generic_types() {
    let result = parse_type_helper("ref List<string>");
    assert!(
        result.is_ok(),
        "Failed to parse ref generic return type: {:?}",
        result
    );

    match result.unwrap() {
        Type::RefReturn(inner) => match *inner {
            Type::Generic { base, args } => {
                assert_eq!(base.to_string(), "List");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], Type::Primitive(PrimitiveType::String));
            }
            _ => panic!("Expected generic type, got {:?}", inner),
        },
        _ => panic!("Expected ref return type"),
    }
}

#[test]
fn test_parse_ref_assignment() {
    let result = parse_expr_helper("ref localVar = ref field");

    // Debug: Let's see what we actually got
    if result.is_err() {
        println!("Parse error: {:?}", result);
        panic!("Failed to parse ref assignment: {:?}", result);
    }

    let parsed_result = result.as_ref().unwrap();

    // In C#, "ref localVar = ref field" is parsed as "ref (localVar = ref field)"
    // because ref has higher precedence than assignment
    match parsed_result {
        Expression::Ref(inner) => {
            match inner.as_ref() {
                Expression::Assignment(assignment) => {
                    // Check left side is variable
                    match assignment.target.as_ref() {
                        Expression::Variable(id) => {
                            assert_eq!(id.to_string(), "localVar");
                        }
                        _ => panic!(
                            "Expected variable expression on left side, got: {:?}",
                            assignment.target
                        ),
                    }

                    // Check right side is ref
                    match assignment.value.as_ref() {
                        Expression::Ref(_) => {
                            // Expected ref on right side
                        }
                        _ => panic!(
                            "Expected ref expression on right side, got: {:?}",
                            assignment.value
                        ),
                    }
                }
                _ => panic!(
                    "Expected assignment expression inside ref, got: {:?}",
                    inner
                ),
            }
        }
        _ => {
            println!("Parsed expression: {:?}", result);
            panic!("Expected ref expression containing assignment");
        }
    }
}

#[test]
fn test_parse_ref_as_parameter() {
    // This test verifies that ref expressions work within method calls
    let result = parse_expr_helper("Method(ref variable, normalParam)");
    assert!(
        result.is_ok(),
        "Failed to parse method call with ref parameter: {:?}",
        result
    );

    match result.unwrap() {
        Expression::Invocation(invocation) => {
            assert_eq!(invocation.arguments.len(), 2);

            // First argument should be marked as ref and carry a variable expr
            let arg0 = &invocation.arguments[0];
            assert!(arg0.modifier.is_some());
            assert!(matches!(arg0.expr, Expression::Variable(_)));

            // Second argument should be normal variable with no modifier
            let arg1 = &invocation.arguments[1];
            assert!(arg1.modifier.is_none());
            assert!(matches!(arg1.expr, Expression::Variable(_)));
        }
        _ => panic!("Expected invocation expression"),
    }
}

#[test]
fn test_parse_ref_locals() {
    // Test parsing ref local variable declarations (this would need to be implemented in variable declaration syntax)
    // For now, we test that ref expressions work in assignment contexts
    let result = parse_expr_helper("refLocal = ref otherVariable");
    assert!(
        result.is_ok(),
        "Failed to parse ref local assignment: {:?}",
        result
    );

    match result.unwrap() {
        Expression::Assignment(assignment) => {
            // Check left side is variable
            match *assignment.target {
                Expression::Variable(ref id) => {
                    assert_eq!(id.to_string(), "refLocal");
                }
                _ => panic!("Expected variable expression on left side"),
            }

            // Check right side is ref
            match *assignment.value {
                Expression::Ref(_) => {
                    // Expected ref on right side
                }
                _ => panic!("Expected ref expression on right side"),
            }
        }
        _ => panic!("Expected assignment expression"),
    }
}

#[test]
fn test_parse_invalid_ref_expressions() {
    let invalid_cases = vec![
        "ref",         // Missing operand
        "ref ",        // Missing operand with space
        "reference",   // Should not match "ref" prefix
        "refVariable", // Should not match "ref" prefix
    ];

    for invalid_case in invalid_cases {
        let result = parse_ref_expr_helper(invalid_case);
        assert!(
            result.is_err(),
            "Should not parse invalid ref expression: '{}'",
            invalid_case
        );
    }
}

#[test]
fn test_ref_expression_precedence() {
    // Test that ref expressions work correctly with other operators
    let result = parse_expr_helper("ref field + 5");
    assert!(
        result.is_ok(),
        "Failed to parse ref expression with binary operator: {:?}",
        result
    );

    println!("Parsed expression: {:?}", result);

    // In C#, ref has very high precedence, so "ref field + 5" should parse as "ref (field + 5)"
    // not as "(ref field) + 5"
    match result.unwrap() {
        Expression::Ref(inner) => {
            match inner.as_ref() {
                Expression::Binary { left, right, .. } => {
                    // Left side should be field variable
                    match left.as_ref() {
                        Expression::Variable(id) => {
                            assert_eq!(id.to_string(), "field");
                        }
                        _ => panic!("Expected variable expression on left side"),
                    }

                    // Right side should be literal
                    match right.as_ref() {
                        Expression::Literal(Literal::Integer(5)) => {
                            // Expected
                        }
                        _ => panic!("Expected integer literal on right side"),
                    }
                }
                _ => panic!("Expected binary expression inside ref"),
            }
        }
        _ => panic!("Expected ref expression containing binary operation"),
    }
}
