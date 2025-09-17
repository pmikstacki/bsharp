// Tests for parsing constructor declarations

use bsharp::parser::expressions::declarations::constructor_declaration_parser::parse_any_member_declaration;
use bsharp::syntax::nodes::declarations::Modifier;
use bsharp::syntax::nodes::statements::statement::Statement;

fn assert_constructor_parses(input: &str, expected_name: &str, num_params: usize) {
    match parse_any_member_declaration(input) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.name, expected_name);
            assert_eq!(member_decl.parameters.len(), num_params);
            assert!(member_decl.has_constructor_syntax()); // No return type
            // Further checks for body, modifiers etc. can be added
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_simple_constructor() {
    assert_constructor_parses("MyClass() { }", "MyClass", 0);
}

#[test]
fn test_constructor_with_parameters() {
    assert_constructor_parses("MyClass(int a, string b) { }", "MyClass", 2);
}

#[test]
fn test_constructor_with_public_modifier() {
    let input = "public MyClass() {}";
    match parse_any_member_declaration(input) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.name, "MyClass");
            assert_eq!(member_decl.parameters.len(), 0);
            assert!(member_decl.has_constructor_syntax());
            assert!(member_decl.modifiers.contains(&Modifier::Public));
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_constructor_with_body() {
    let input = "MyClass() { int x = 0; }";
    match parse_any_member_declaration(input) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert!(member_decl.body.is_some());
            assert!(member_decl.has_constructor_syntax());
            if let Some(Statement::Block(body_stmts)) = member_decl.body {
                assert_eq!(body_stmts.len(), 1);
                match &body_stmts[0] {
                    Statement::Declaration(_) => {} // Expected LocalVariableDeclaration
                    _ => panic!(
                        "Expected a declaration statement in constructor body, got {:?}",
                        body_stmts[0]
                    ),
                }
            } else {
                panic!("Constructor body was not a Statement::Block");
            }
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_async_constructor_syntax_accepted() {
    // Parser should now accept this syntactically - analyzer will handle semantic validation
    let input = "async MyClass() { }";
    match parse_any_member_declaration(input) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.name, "MyClass");
            assert!(member_decl.has_constructor_syntax());
            assert!(member_decl.modifiers.contains(&Modifier::Async));
        }
        Err(e) => panic!(
            "Parser should accept syntactically valid async constructor: {:?}",
            e
        ),
    }
}
