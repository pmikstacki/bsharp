// Tests for parsing local variable declarations

use parser::expressions::declarations::variable_declaration_parser::parse_local_variable_declaration;
use syntax::declarations::LocalVariableDeclaration;
use syntax::declarations::local_variable_declaration::VariableDeclaration;
use syntax::expressions::{Expression, Literal};
use syntax::identifier::Identifier;
use syntax::types::{PrimitiveType, Type};

fn parse_local_var_decl_test(code: &str) -> Result<LocalVariableDeclaration, String> {
    match parse_local_variable_declaration(code.into()) {
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
        is_ref: false,
        declaration_type: Type::Primitive(PrimitiveType::Int),
        declarators: vec![VariableDeclaration {
            name: Identifier::Simple("x".to_string()),
            initializer: Some(Expression::Literal(Literal::Integer(5))),
        }],
    };
    assert_eq!(parse_local_var_decl_test(code.into()), Ok(expected));
}

#[test]
fn test_parse_local_variable_without_initializer() {
    let code = "string name;";
    let expected = LocalVariableDeclaration {
        is_const: false,
        is_ref: false,
        declaration_type: Type::Primitive(PrimitiveType::String),
        declarators: vec![VariableDeclaration {
            name: Identifier::Simple("name".to_string()),
            initializer: None,
        }],
    };
    assert_eq!(parse_local_var_decl_test(code.into()), Ok(expected));
}
