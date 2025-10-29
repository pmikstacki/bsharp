#![allow(clippy::approx_constant)]
// Integration tests for local variable declaration statements
// Moved from statement_tests.rs

use parser::statement_parser::parse_statement_ws_spanned;
use parser::syntax::test_helpers::parse_all;
use syntax::declarations::LocalVariableDeclaration;
use syntax::declarations::local_variable_declaration::VariableDeclaration;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;
use syntax::expressions::new_expression::NewExpression;
use syntax::identifier::Identifier;
use syntax::statements::statement::Statement;
use syntax::types::{PrimitiveType, Type};

// Helper function from statement_tests.rs
fn assert_statement_parses(code: &str, expected: Statement) {
    let code_trimmed = code.trim();
    match parse_all(parse_statement_ws_spanned, code_trimmed.into()) {
        Ok((_, parsed_statement)) => {
            assert_eq!(
                parsed_statement.node, expected,
                "Parsed statement does not match expected for code: {}\n",
                code_trimmed
            );
        }
        Err(e) => panic!("Parser failed for code: '{}'\nError: {:?}", code_trimmed, e),
    }
}

#[test]
fn test_parse_local_variable_declaration_no_init() {
    assert_statement_parses(
        "string message;",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Primitive(PrimitiveType::String),
            declarators: vec![VariableDeclaration {
                name: Identifier::new("message"),
                initializer: None,
            }],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_with_initializer() {
    assert_statement_parses(
        "bool flag = true;",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Primitive(PrimitiveType::Bool),
            declarators: vec![VariableDeclaration {
                name: Identifier::new("flag"),
                initializer: Some(Expression::Literal(Literal::Boolean(true))),
            }],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_int_with_initializer() {
    assert_statement_parses(
        "int count = 10;",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Primitive(PrimitiveType::Int),
            declarators: vec![VariableDeclaration {
                name: Identifier::new("count"),
                initializer: Some(Expression::Literal(Literal::Integer(10))),
            }],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_list_new_expression() {
    assert_statement_parses(
        "List<string> names = new List<string>();",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Generic {
                base: Identifier::new("List"),
                args: vec![Type::Primitive(PrimitiveType::String)],
            },
            declarators: vec![VariableDeclaration {
                name: Identifier::new("names"),
                initializer: Some(Expression::New(Box::new(NewExpression {
                    target_type: Some(Type::Generic {
                        base: Identifier::new("List"),
                        args: vec![Type::Primitive(PrimitiveType::String)],
                    }),
                    arguments: vec![],
                    object_initializer: None,
                    collection_initializer: None,
                }))),
            }],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_multiple_declarators() {
    assert_statement_parses(
        "int x = 1, y = 2;",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Primitive(PrimitiveType::Int),
            declarators: vec![
                VariableDeclaration {
                    name: Identifier::new("x"),
                    initializer: Some(Expression::Literal(Literal::Integer(1))),
                },
                VariableDeclaration {
                    name: Identifier::new("y"),
                    initializer: Some(Expression::Literal(Literal::Integer(2))),
                },
            ],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_var_keyword() {
    assert_statement_parses(
        "var name = \"BSharp\";",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Var,
            declarators: vec![VariableDeclaration {
                name: Identifier::new("name"),
                initializer: Some(Expression::Literal(Literal::String("BSharp".to_string()))),
            }],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_const_modifier() {
    assert_statement_parses(
        "const double PI = 3.14;",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: true,
            is_ref: false,
            declaration_type: Type::Primitive(PrimitiveType::Double),
            declarators: vec![VariableDeclaration {
                name: Identifier::new("PI"),
                initializer: Some(Expression::Literal(Literal::Float(3.14))),
            }],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_var_with_new_expression() {
    assert_statement_parses(
        "var list = new List<string>();",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Var,
            declarators: vec![VariableDeclaration {
                name: Identifier::new("list"),
                initializer: Some(Expression::New(Box::new(NewExpression {
                    target_type: Some(Type::Generic {
                        base: Identifier::new("List"),
                        args: vec![Type::Primitive(PrimitiveType::String)],
                    }),
                    arguments: vec![],
                    object_initializer: None,
                    collection_initializer: None,
                }))),
            }],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_var_with_numeric_literal() {
    assert_statement_parses(
        "var count = 42;",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Var,
            declarators: vec![VariableDeclaration {
                name: Identifier::new("count".to_string()),
                initializer: Some(Expression::Literal(Literal::Integer(42))),
            }],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_var_multiple_declarators() {
    assert_statement_parses(
        "var x = 1, y = 2;",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Var,
            declarators: vec![
                VariableDeclaration {
                    name: Identifier::new("x"),
                    initializer: Some(Expression::Literal(Literal::Integer(1))),
                },
                VariableDeclaration {
                    name: Identifier::new("y"),
                    initializer: Some(Expression::Literal(Literal::Integer(2))),
                },
            ],
        }),
    );
}

#[test]
fn test_parse_local_variable_declaration_var_with_numeric_literal_is_ref() {
    assert_statement_parses(
        "var count = 42;",
        Statement::Declaration(LocalVariableDeclaration {
            is_const: false,
            is_ref: false,
            declaration_type: Type::Var,
            declarators: vec![VariableDeclaration {
                name: Identifier::new("count"),
                initializer: Some(Expression::Literal(Literal::Integer(42))),
            }],
        }),
    );
}
