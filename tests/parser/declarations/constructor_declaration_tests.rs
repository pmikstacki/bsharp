// Tests for parsing constructor declarations

use bsharp::parser::nodes::declarations::Modifier;
use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parsers::declarations::constructor_declaration_parser::parse_constructor_declaration;

fn assert_constructor_parses(input: &str, expected_name: &str, num_params: usize) {
    match parse_constructor_declaration(input) {
        Ok((remaining, constructor_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(constructor_decl.name.name, expected_name);
            assert_eq!(constructor_decl.parameters.len(), num_params);
            // Further checks for body, modifiers etc. can be added
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_simple_constructor() {
    assert_constructor_parses(
        "MyClass() { }", 
        "MyClass", 
        0
    );
}

#[test]
fn test_constructor_with_parameters() {
    assert_constructor_parses(
        "MyClass(int a, string b) { }", 
        "MyClass", 
        2
    );
}

#[test]
fn test_constructor_with_public_modifier() {
    let input = "public MyClass() {}";
    match parse_constructor_declaration(input) {
        Ok((remaining, constructor_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(constructor_decl.name.name, "MyClass");
            assert_eq!(constructor_decl.parameters.len(), 0);
            assert!(constructor_decl.modifiers.contains(&Modifier::Public));
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_constructor_with_body() {
    let input = "MyClass() { int x = 0; }";
     match parse_constructor_declaration(input) {
        Ok((remaining, constructor_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert!(constructor_decl.body.is_some());
            if let Some(Statement::Block(body_stmts)) = constructor_decl.body {
                assert_eq!(body_stmts.len(), 1);
                match &body_stmts[0] {
                    Statement::Declaration(_) => {} // Expected LocalVariableDeclaration
                    _ => panic!("Expected a declaration statement in constructor body, got {:?}", body_stmts[0]),
                }
            } else {
                panic!("Constructor body was not a Statement::Block");
            }
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}
