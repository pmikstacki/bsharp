// Tests for parsing local variable declarations

use bsharp::parser::nodes::declarations::LocalVariableDeclaration;
use bsharp::parser::nodes::declarations::local_variable_declaration::VariableDeclarator;
use bsharp::parser::nodes::types::{Type, PrimitiveType};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::expressions::{Expression, Literal};
use bsharp::parsers::declarations::variable_declaration_parser::parse_local_variable_declaration;

fn parse_local_var_decl_test(code: &str) -> Result<LocalVariableDeclaration, String> {
    match parse_local_variable_declaration(code) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_local_variable_with_initializer() {
    let code = "int x = 5;";
    let expected = LocalVariableDeclaration {
        is_const: false,
        declaration_type: Type::Primitive(PrimitiveType::Int),
        declarators: vec![
            VariableDeclarator {
                name: Identifier { name: "x".to_string() },
                initializer: Some(Expression::Literal(Literal::Integer(5))),
            }
        ],
    };
    assert_eq!(parse_local_var_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_local_variable_without_initializer() {
    let code = "string name;";
    let expected = LocalVariableDeclaration {
        is_const: false,
        declaration_type: Type::Primitive(PrimitiveType::String),
        declarators: vec![
            VariableDeclarator {
                name: Identifier { name: "name".to_string() },
                initializer: None,
            }
        ],
    };
    assert_eq!(parse_local_var_decl_test(code), Ok(expected));
}

