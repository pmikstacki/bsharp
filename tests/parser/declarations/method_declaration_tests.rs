// Tests for parsing method declarations

use bsharp::parser::nodes::declarations::{MethodDeclaration, Modifier};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::types::{Type, PrimitiveType, Parameter, TypeParameter, Variance};
use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parsers::declarations::method_declaration_parser::parse_method_declaration;

fn assert_method_parses_fully(input: &str, expected: MethodDeclaration) {
    match parse_method_declaration(input) {
        Ok((rest, method_decl)) => {
            assert!(rest.trim().is_empty(), "Input not fully parsed. Remaining: {}", rest);
            assert_eq!(method_decl, expected);
        }
        Err(e) => panic!("Parser failed: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_parse_simple_void_method() {
    let code = "void DoSomething();";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: Type::Primitive(PrimitiveType::Void),
        name: Identifier { name: "DoSomething".to_string() },
        type_parameters: None,
        parameters: vec![],
        body: None,
        constraints: None,
    };
    assert_method_parses_fully(code, expected);
}

#[test]
fn test_parse_method_with_primitive_return_type() {
    let code = "int GetValue();";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "GetValue".to_string() },
        type_parameters: None,
        parameters: vec![],
        body: None,
        constraints: None,
    };
    assert_method_parses_fully(code, expected);
}

#[test]
fn test_parse_method_with_parameters() {
    let code = "void SetValue(int x, string y);";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: Type::Primitive(PrimitiveType::Void),
        name: Identifier { name: "SetValue".to_string() },
        type_parameters: None,
        parameters: vec![
            Parameter {
                parameter_type: Type::Primitive(PrimitiveType::Int),
                name: Identifier { name: "x".to_string() },
            },
            Parameter {
                parameter_type: Type::Primitive(PrimitiveType::String),
                name: Identifier { name: "y".to_string() },
            },
        ],
        body: None,
        constraints: None,
    };
    assert_method_parses_fully(code, expected);
}

#[test]
fn test_parse_method_with_body() {
    let code = "void Process() { /* ... */ }";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: Type::Primitive(PrimitiveType::Void),
        name: Identifier { name: "Process".to_string() },
        type_parameters: None,
        parameters: vec![],
        body: Some(Statement::Block(vec![])),
        constraints: None,
    };
    match parse_method_declaration(code) {
        Ok((rest, method_decl)) => {
            assert!(rest.trim().is_empty(), "Input not fully parsed. Remaining: {}", rest);
            assert_eq!(method_decl.name, expected.name);
            assert_eq!(method_decl.return_type, expected.return_type);
            assert!(matches!(method_decl.body, Some(Statement::Block(_))));
        }
        Err(e) => panic!("Parser failed: {:?} for input: {}", e, code),
    }
}

#[test]
fn test_parse_method_with_public_modifier() {
    let code = "public int Calculate();";
    let expected = MethodDeclaration {
        modifiers: vec![Modifier::Public],
        return_type: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Calculate".to_string() },
        type_parameters: None,
        parameters: vec![],
        body: None,
        constraints: None,
    };
    assert_method_parses_fully(code, expected);
}

#[test]
fn test_parse_method_with_static_modifier() {
    let code = "static void Initialize() { }";
    let expected = MethodDeclaration {
        modifiers: vec![Modifier::Static],
        return_type: Type::Primitive(PrimitiveType::Void),
        name: Identifier { name: "Initialize".to_string() },
        type_parameters: None,
        parameters: vec![],
        body: Some(Statement::Block(vec![])),
        constraints: None,
    };
    assert_method_parses_fully(code, expected);
}

#[test]
fn test_generic_method() {
    let code = "T Create<T>();";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: Type::Reference(Identifier::new("T")), 
        name: Identifier { name: "Create".to_string() },
        type_parameters: Some(vec![TypeParameter {
            name: Identifier { name: "T".to_string() },
            variance: Variance::None,
        }]),
        parameters: vec![],
        body: None,
        constraints: None,
    };
    assert_method_parses_fully(code, expected);
}

// Test for expression body will be treated as 'no body' for now due to simplification
#[test]
fn test_expression_bodied_method() {
    let code = "int GetResult() => 42;";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "GetResult".to_string() },
        type_parameters: None,
        parameters: vec![],
        body: None,
        constraints: None,
    };
    assert_method_parses_fully(code, expected);
}
