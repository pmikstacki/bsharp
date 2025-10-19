// Tests for parsing method declarations

use parser::expressions::declarations::method_declaration_parser::parse_member_declaration;
use syntax::declarations::Modifier;
use syntax::statements::statement::Statement;
use syntax::types::{PrimitiveType, Type};

#[test]
fn test_simple_method() {
    let input = "public void TestMethod() { }";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "TestMethod");
            assert!(member_decl.has_method_syntax()); // Has return type
            assert!(member_decl.modifiers.contains(&Modifier::Public));
            assert!(matches!(
                member_decl.return_type,
                Some(Type::Primitive(PrimitiveType::Void))
            ));
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_expression_bodied_method() {
    // Test that expression-bodied methods parse correctly with proper expression parsing
    let input = "public int Add(int a, int b) => a + b;";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "Add");
            assert!(member_decl.has_method_syntax()); // Has return type
            assert!(member_decl.modifiers.contains(&Modifier::Public));
            assert_eq!(member_decl.parameters.len(), 2);

            // Verify that the body is properly parsed as an expression
            if let Some(Statement::Expression(_expr)) = &member_decl.body {
                // Expression is parsed successfully
            } else {
                panic!(
                    "Expected expression body to be parsed as Statement::Expression, got {:?}",
                    member_decl.body
                );
            }
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_expression_bodied_constructor() {
    // Test that expression-bodied constructors also work (though semantically unusual)
    let input = "public MyClass(int value) => this();";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "MyClass");
            assert!(member_decl.has_constructor_syntax()); // No return type
            assert!(member_decl.modifiers.contains(&Modifier::Public));
            assert_eq!(member_decl.parameters.len(), 1);

            // Verify that the body is properly parsed as an expression
            if let Some(Statement::Expression(_expr)) = &member_decl.body {
                // Expression is parsed successfully
            } else {
                panic!(
                    "Expected expression body to be parsed as Statement::Expression, got {:?}",
                    member_decl.body
                );
            }
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_debug_simple_constructor() {
    // First test: basic constructor without async
    let input = "MyClass() { }";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "MyClass");
            assert!(member_decl.has_constructor_syntax());
        }
        Err(e) => panic!(
            "Parser failed on simple constructor: {:?} for input: {}",
            e, input
        ),
    }
}

#[test]
fn test_debug_public_constructor() {
    // Second test: constructor with modifier
    let input = "public MyClass() { }";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "MyClass");
            assert!(member_decl.has_constructor_syntax());
            assert!(member_decl.modifiers.contains(&Modifier::Public));
        }
        Err(e) => panic!(
            "Parser failed on public constructor: {:?} for input: {}",
            e, input
        ),
    }
}

#[test]
fn test_async_constructor_syntax_parsing() {
    // This should now parse successfully - semantic validation is handled by analyzer
    let input = "async MyClass() { }";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "MyClass");
            assert!(member_decl.has_constructor_syntax()); // No return type
            assert!(member_decl.modifiers.contains(&Modifier::Async));
        }
        Err(e) => panic!(
            "Parser should not fail on syntactically valid async constructor: {:?}",
            e
        ),
    }
}

#[test]
fn test_method_with_return_type() {
    let input = "public async Task<string> GetDataAsync() { }";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "GetDataAsync");
            assert!(member_decl.has_method_syntax()); // Has return type
            assert!(member_decl.modifiers.contains(&Modifier::Public));
            assert!(member_decl.modifiers.contains(&Modifier::Async));
        }
        Err(e) => panic!("Parser failed with error: {:?} for input: {}", e, input),
    }
}

#[test]
fn test_abstract_virtual_method_modifiers_syntax() {
    // Parser should accept these syntactically - analyzer will validate semantics
    let input = "public abstract virtual void Method() { }";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "Method");
            assert!(member_decl.has_method_syntax());
            assert!(member_decl.modifiers.contains(&Modifier::Public));
            assert!(member_decl.modifiers.contains(&Modifier::Abstract));
            assert!(member_decl.modifiers.contains(&Modifier::Virtual));
        }
        Err(e) => panic!(
            "Parser should accept syntactically valid modifiers: {:?}",
            e
        ),
    }
}

#[test]
fn test_constructor_with_invalid_semantic_modifiers() {
    // Parser should accept these syntactically - analyzer will validate semantics
    let input = "public abstract virtual MyClass() { }";
    match parse_member_declaration(input.into()) {
        Ok((remaining, member_decl)) => {
            assert_eq!(remaining.trim(), "");
            assert_eq!(member_decl.name.to_string(), "MyClass");
            assert!(member_decl.has_constructor_syntax()); // No return type
            assert!(member_decl.modifiers.contains(&Modifier::Public));
            assert!(member_decl.modifiers.contains(&Modifier::Abstract));
            assert!(member_decl.modifiers.contains(&Modifier::Virtual));
        }
        Err(e) => panic!(
            "Parser should accept syntactically valid but semantically invalid modifiers: {:?}",
            e
        ),
    }
}
