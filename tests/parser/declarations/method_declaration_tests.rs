// Tests for parsing method declarations

use bsharp::parser::nodes::declarations::{MethodDeclaration, Modifier};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::types;
use bsharp::parsers::declarations::method_declaration_parser::parse_method_declaration;
use std::marker::PhantomData;

fn parse_method_decl_test(code: &str) -> Result<MethodDeclaration, String> {
    match parse_method_declaration(code) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_void_method() {
    let code = "void DoSomething() {}";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: types::Type::Primitive(types::PrimitiveType::Void),
        name: Identifier { name: "DoSomething".to_string() },
        type_parameters: vec![],
        parameters: vec![],
        body: Some("".to_string()),
        constraints: vec![],
        _phantom: PhantomData,
    };
    assert_eq!(parse_method_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_method_with_return_type() {
    let code = "int GetCount() {}";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: types::Type::Primitive(types::PrimitiveType::Int),
        name: Identifier { name: "GetCount".to_string() },
        type_parameters: vec![],
        parameters: vec![],
        body: Some("".to_string()),
        constraints: vec![],
        _phantom: PhantomData,
    };
    assert_eq!(parse_method_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_method_with_parameters() {
    let code = "void SetValue(int value, string name) {}";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: types::Type::Primitive(types::PrimitiveType::Void),
        name: Identifier { name: "SetValue".to_string() },
        type_parameters: vec![],
        parameters: vec![
            types::Parameter {
                ty: types::Type::Primitive(types::PrimitiveType::Int),
                name: Identifier { name: "value".to_string() },
                _phantom: PhantomData,
            },
            types::Parameter {
                ty: types::Type::Primitive(types::PrimitiveType::String),
                name: Identifier { name: "name".to_string() },
                _phantom: PhantomData,
            },
        ],
        body: Some("".to_string()),
        constraints: vec![],
        _phantom: PhantomData,
    };
    assert_eq!(parse_method_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_generic_method() {
    let code = "T Process<T>(T input) {}";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: types::Type::Reference(Identifier { name: "T".to_string() }),
        name: Identifier { name: "Process".to_string() },
        type_parameters: vec![types::TypeParameter {
            name: Identifier { name: "T".to_string() },
            variance: types::Variance::None,
        }],
        parameters: vec![types::Parameter {
            ty: types::Type::Reference(Identifier { name: "T".to_string() }),
            name: Identifier { name: "input".to_string() },
            _phantom: PhantomData,
        }],
        body: Some("".to_string()),
        constraints: vec![],
        _phantom: PhantomData,
    };
    assert_eq!(parse_method_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_method_with_body_content() {
    let code = "int Calculate() { int x = 5; return x * 2; }";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: types::Type::Primitive(types::PrimitiveType::Int),
        name: Identifier { name: "Calculate".to_string() },
        type_parameters: vec![],
        parameters: vec![],
        body: Some("int x = 5; return x * 2;".to_string()),
        constraints: vec![],
        _phantom: PhantomData,
    };
    assert_eq!(parse_method_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_abstract_method() {
    let code = "void AbstractMethod();";
    let expected = MethodDeclaration {
        modifiers: vec![],
        return_type: types::Type::Primitive(types::PrimitiveType::Void),
        name: Identifier { name: "AbstractMethod".to_string() },
        type_parameters: vec![],
        parameters: vec![],
        body: None,
        constraints: vec![],
        _phantom: PhantomData,
    };
    assert_eq!(parse_method_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_method_with_modifiers() {
    let input = "public static void DoSomething() {}";
    let expected_modifiers = vec![Modifier::Public, Modifier::Static];
    let expected = Ok((
        "",
        MethodDeclaration {
            modifiers: expected_modifiers,
            return_type: types::Type::Primitive(types::PrimitiveType::Void),
            name: Identifier { name: "DoSomething".to_string() },
            type_parameters: vec![],
            parameters: vec![],
            body: Some("".to_string()),
            constraints: vec![],
            _phantom: PhantomData,
        },
    ));
    assert_eq!(parse_method_declaration(input), expected);

    let input = "private async Task ProcessAsync(int x)";
    let input_with_body = "private async Task ProcessAsync(int x);";
    let expected_modifiers_async = vec![Modifier::Private, Modifier::Async];
    let expected_async: Result<(&str, MethodDeclaration), String> = Ok((
        "",
        MethodDeclaration {
            modifiers: expected_modifiers_async.clone(),
            return_type: types::Type::Reference(Identifier { name: "Task".to_string() }),
            name: Identifier { name: "ProcessAsync".to_string() },
            type_parameters: vec![],
            parameters: vec![],
            constraints: vec![],
            body: Some(";".to_string()),
            _phantom: PhantomData,
        },
    ));
    let result = parse_method_declaration(input_with_body);
    assert!(result.is_ok());
    let (_, parsed_decl) = result.unwrap();
    assert_eq!(parsed_decl.modifiers, expected_modifiers_async); // expected_modifiers_async is not moved because we cloned it above
    assert_eq!(parsed_decl.return_type, types::Type::Reference(Identifier { name: "Task".to_string() }));
    assert_eq!(parsed_decl.name, Identifier { name: "ProcessAsync".to_string() });
}
