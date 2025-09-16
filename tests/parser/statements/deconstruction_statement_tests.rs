// Tests for parsing deconstruction statements

use bsharp::syntax::nodes::statements::statement::Statement;
use bsharp::syntax::nodes::expressions::{DeconstructionTarget, Expression};
use bsharp::syntax::nodes::types::{Type, PrimitiveType};
use bsharp::parser::statements::deconstruction_statement_parser::parse_deconstruction_statement;

fn parse_deconstruction_stmt(code: &str) -> Result<Statement, String> {
    match parse_deconstruction_statement(code) {
        Ok((rest, stmt)) if rest.trim().is_empty() => Ok(stmt),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_deconstruction_statement() {
    let code = "(var x, var y) = tuple;";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 2);
            assert!(matches!(deconstruction.targets[0], DeconstructionTarget::Declaration { is_var: true, .. }));
            assert!(matches!(deconstruction.targets[1], DeconstructionTarget::Declaration { is_var: true, .. }));
        }
        _ => panic!("Expected deconstruction statement"),
    }
}

#[test]
fn test_parse_typed_deconstruction_statement() {
    let code = "(int x, string y) = GetTuple();";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 2);
            
            match &deconstruction.targets[0] {
                DeconstructionTarget::Declaration { variable_type: Some(Type::Primitive(PrimitiveType::Int)), name, is_var: false } => {
                    assert_eq!(name.name, "x");
                }
                _ => panic!("Expected int declaration"),
            }
            
            match &deconstruction.targets[1] {
                DeconstructionTarget::Declaration { variable_type: Some(Type::Primitive(PrimitiveType::String)), name, is_var: false } => {
                    assert_eq!(name.name, "y");
                }
                _ => panic!("Expected string declaration"),
            }
        }
        _ => panic!("Expected deconstruction statement"),
    }
}

#[test]
fn test_parse_existing_variable_deconstruction_statement() {
    let code = "(existingX, existingY) = tuple;";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 2);
            
            match &deconstruction.targets[0] {
                DeconstructionTarget::Variable(name) => {
                    assert_eq!(name.name, "existingX");
                }
                _ => panic!("Expected variable target"),
            }
            
            match &deconstruction.targets[1] {
                DeconstructionTarget::Variable(name) => {
                    assert_eq!(name.name, "existingY");
                }
                _ => panic!("Expected variable target"),
            }
        }
        _ => panic!("Expected deconstruction statement"),
    }
}

#[test]
fn test_parse_deconstruction_with_discard_statement() {
    let code = "(var x, _) = tuple;";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 2);
            assert!(matches!(deconstruction.targets[0], DeconstructionTarget::Declaration { .. }));
            assert!(matches!(deconstruction.targets[1], DeconstructionTarget::Discard));
        }
        _ => panic!("Expected deconstruction statement"),
    }
}

#[test]
fn test_parse_nested_deconstruction_statement() {
    let code = "((var a, var b), var c) = nestedTuple;";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 2);
            
            // Check first target is nested
            match &deconstruction.targets[0] {
                DeconstructionTarget::Nested(inner) => {
                    assert_eq!(inner.len(), 2);
                    assert!(matches!(inner[0], DeconstructionTarget::Declaration { .. }));
                    assert!(matches!(inner[1], DeconstructionTarget::Declaration { .. }));
                }
                _ => panic!("Expected nested deconstruction target"),
            }
            
            // Check second target is simple declaration
            assert!(matches!(deconstruction.targets[1], DeconstructionTarget::Declaration { .. }));
        }
        _ => panic!("Expected deconstruction statement"),
    }
}

#[test]
fn test_parse_complex_value_expression_statement() {
    let code = "(var x, var y) = obj.Property.GetTuple();";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 2);
            // Value should be a complex expression (method call on member access)
            assert!(matches!(*deconstruction.value, Expression::Invocation(_)));
        }
        _ => panic!("Expected deconstruction statement"),
    }
}

#[test]
fn test_parse_multiple_deconstruction_statements() {
    let statements = [
        "(var a, var b) = tuple1;",
        "(int x, string y) = tuple2;",
        "(existing1, existing2) = tuple3;",
        "(var c, _) = tuple4;",
    ];
    
    for code in &statements {
        let result = parse_deconstruction_stmt(code);
        assert!(result.is_ok(), "Failed to parse: {}", code);
        assert!(matches!(result.unwrap(), Statement::Deconstruction(_)));
    }
}

#[test]
fn test_parse_deconstruction_whitespace_variations() {
    let variations = [
        "(var x, var y) = tuple;",
        "( var x , var y ) = tuple ;",
        "(\tvar x,\tvar y\t) = tuple\t;",
        "(var x,\n var y) = tuple\n;",
        "  (var x, var y) = tuple;  ",
    ];

    for code in &variations {
        let result = parse_deconstruction_stmt(code);
        assert!(result.is_ok(), "Failed to parse: {}", code);
        assert!(matches!(result.unwrap(), Statement::Deconstruction(_)));
    }
}

#[test]
fn test_deconstruction_statement_parsing_errors() {
    let invalid_cases = [
        "(var x, var y) = tuple",    // Missing semicolon
        "() = tuple;",               // Empty target list
        "(var x, var y) =;",         // Missing value
        "(var x, var y) tuple;",     // Missing assignment operator
        "(var x var y) = tuple;",    // Missing comma
        "var x, var y = tuple;",     // Missing parentheses
    ];

    for code in &invalid_cases {
        let result = parse_deconstruction_stmt(code);
        assert!(result.is_err(), "Should fail to parse: {}", code);
    }
}

#[test]
fn test_parse_complex_type_deconstruction_statement() {
    let code = "(List<string> items, Dictionary<int, Person> people) = GetComplexData();";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 2);
            // Both targets should be type declarations (not var)
            for target in &deconstruction.targets {
                assert!(matches!(target, DeconstructionTarget::Declaration { is_var: false, .. }));
            }
        }
        _ => panic!("Expected deconstruction statement"),
    }
}

#[test]
fn test_parse_mixed_declaration_types_statement() {
    let code = "(var x, int y, string z, existing) = GetMixedTuple();";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 4);
            
            // Check each target type
            assert!(matches!(deconstruction.targets[0], DeconstructionTarget::Declaration { is_var: true, .. }));
            assert!(matches!(deconstruction.targets[1], DeconstructionTarget::Declaration { is_var: false, .. }));
            assert!(matches!(deconstruction.targets[2], DeconstructionTarget::Declaration { is_var: false, .. }));
            assert!(matches!(deconstruction.targets[3], DeconstructionTarget::Variable(_)));
        }
        _ => panic!("Expected deconstruction statement"),
    }
}

#[test]
fn test_parse_array_access_value_statement() {
    let code = "(var x, var y) = tuples[index];";
    let result = parse_deconstruction_stmt(code);
    assert!(result.is_ok());
    
    match result.unwrap() {
        Statement::Deconstruction(deconstruction) => {
            assert_eq!(deconstruction.targets.len(), 2);
            // Value should be an indexing expression
            assert!(matches!(*deconstruction.value, Expression::Indexing(_)));
        }
        _ => panic!("Expected deconstruction statement"),
    }
} 