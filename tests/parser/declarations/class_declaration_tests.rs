// Tests for parsing class declarations

use bsharp::parser::nodes::declarations::{ClassDeclaration, ClassMember, Modifier};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::types::{TypeParameter, Variance, Type, PrimitiveType, Parameter};
use bsharp::parser::nodes::declarations::{MethodDeclaration, FieldDeclaration}; 
use bsharp::parser::nodes::expressions::expression::Expression; 
use bsharp::parser::nodes::expressions::literal::Literal; 
use bsharp::parsers::declarations::class_declaration_parser::parse_class_declaration;
use std::marker::PhantomData;

fn parse_class_decl_test(code: &str) -> Result<ClassDeclaration, String> {
    match parse_class_declaration(code) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_class() {
    let code = "class MyClass {}";
    let expected = ClassDeclaration {
        modifiers: vec![],
        name: Identifier { name: "MyClass".to_string() },
        type_parameters: vec![],
        members: vec![], 
    };
    assert_eq!(parse_class_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_generic_class() {
    let code = "class Dictionary<TKey, TValue> {}";
    let expected = ClassDeclaration {
        modifiers: vec![],
        name: Identifier { name: "Dictionary".to_string() },
        type_parameters: vec![
            TypeParameter {
                name: Identifier { name: "TKey".to_string() },
                variance: Variance::None,
            },
            TypeParameter {
                name: Identifier { name: "TValue".to_string() },
                variance: Variance::None,
            },
        ],
        members: vec![], 
    };
    assert_eq!(parse_class_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_class_with_method() {
    let code = r#"
        class Calculator {
            int Add(int a, int b) {}
        }
    "#;
    let expected = ClassDeclaration {
        modifiers: vec![],
        name: Identifier { name: "Calculator".to_string() },
        type_parameters: vec![],
        members: vec![
            ClassMember::Method(MethodDeclaration {
                modifiers: vec![], // Added for test compatibility
                return_type: Type::Primitive(PrimitiveType::Int),
                name: Identifier { name: "Add".to_string() },
                type_parameters: vec![],
                parameters: vec![
                    Parameter {
                        ty: Type::Primitive(PrimitiveType::Int),
                        name: Identifier { name: "a".to_string() },
                        _phantom: PhantomData,
                    },
                    Parameter {
                        ty: Type::Primitive(PrimitiveType::Int),
                        name: Identifier { name: "b".to_string() },
                        _phantom: PhantomData,
                    },
                ],
                body: Some("".to_string()), 
                constraints: vec![],
                _phantom: PhantomData,
            })
        ],
    };
    assert_eq!(parse_class_decl_test(code.trim()), Ok(expected));
}

#[test]
fn test_parse_class_with_multiple_members() {
    let code = r#"
        class Service {
            void Start() {}
            void Stop() {}
        }
    "#;
    let expected = ClassDeclaration {
        modifiers: vec![],
        name: Identifier { name: "Service".to_string() },
        type_parameters: vec![],
        members: vec![
            ClassMember::Method(MethodDeclaration {
                modifiers: vec![], // Added for test compatibility
                return_type: Type::Primitive(PrimitiveType::Void),
                name: Identifier { name: "Start".to_string() },
                type_parameters: vec![],
                parameters: vec![],
                body: Some("".to_string()), 
                constraints: vec![],
                _phantom: PhantomData,
            }),
            ClassMember::Method(MethodDeclaration {
                modifiers: vec![], // Added for test compatibility
                return_type: Type::Primitive(PrimitiveType::Void),
                name: Identifier { name: "Stop".to_string() },
                type_parameters: vec![],
                parameters: vec![],
                body: Some("".to_string()), 
                constraints: vec![],
                _phantom: PhantomData,
            }),
        ],
    };
    assert_eq!(parse_class_decl_test(code.trim()), Ok(expected));
}

#[test]
fn test_parse_class_with_field() {
    let code = r#"
        class Data {
            int value = 42;
        }
    "#;
    let expected = ClassDeclaration {
        modifiers: vec![],
        name: Identifier { name: "Data".to_string() },
        type_parameters: vec![],
        members: vec![
            ClassMember::Field(FieldDeclaration {
                ty: Type::Primitive(PrimitiveType::Int),
                name: Identifier { name: "value".to_string() },
                initializer: Some(Expression::Literal(Literal::Integer(42))),
            })
        ],
    };
    assert_eq!(parse_class_decl_test(code.trim()), Ok(expected));
}

#[test]
fn test_parse_class_with_mixed_members() {
    // Field followed by Method
    let code = r#"
        class MyComponent {
            string _name = "Default";
            void Initialize() {}
        }
    "#;
    let expected = ClassDeclaration {
        modifiers: vec![],
        name: Identifier { name: "MyComponent".to_string() },
        type_parameters: vec![],
        members: vec![
            ClassMember::Field(FieldDeclaration {
                ty: Type::Primitive(PrimitiveType::String),
                name: Identifier { name: "_name".to_string() },
                initializer: Some(Expression::Literal(Literal::String("Default".to_string()))),
            }),
            ClassMember::Method(MethodDeclaration {
                modifiers: vec![], // Added for test compatibility
                return_type: Type::Primitive(PrimitiveType::Void),
                name: Identifier { name: "Initialize".to_string() },
                type_parameters: vec![],
                parameters: vec![],
                body: Some("".to_string()), 
                constraints: vec![],
                _phantom: PhantomData,
            }),
        ],
    };
    assert_eq!(parse_class_decl_test(code.trim()), Ok(expected));
}

#[test]
fn test_parse_class_with_method_with_body() {
    let code = r#"
        class Greeter {
            void SayHello() { Console.WriteLine("Hello"); }
        }
    "#;
    let expected = ClassDeclaration {
        modifiers: vec![],
        name: Identifier { name: "Greeter".to_string() },
        type_parameters: vec![],
        members: vec![
            ClassMember::Method(MethodDeclaration {
                modifiers: vec![], // Added for test compatibility
                return_type: Type::Primitive(PrimitiveType::Void),
                name: Identifier { name: "SayHello".to_string() },
                type_parameters: vec![],
                parameters: vec![],
                body: Some("Console.WriteLine(\"Hello\");".to_string()), 
                constraints: vec![],
                _phantom: PhantomData,
            })
        ],
    };
    assert_eq!(parse_class_decl_test(code.trim()), Ok(expected));
}

#[test]
fn test_parse_class_with_modifiers() {
    let input = "public abstract class BaseClass {}";
    let expected_modifiers = vec![Modifier::Public, Modifier::Abstract];
    let expected = Ok(ClassDeclaration {
        modifiers: expected_modifiers,
        name: Identifier { name: "BaseClass".to_string() },
        type_parameters: vec![],
        members: vec![], 
    });
    let result = parse_class_decl_test(input);
    assert!(result.is_ok(), "Parsing failed for: {input}");
    assert_eq!(result, expected);

    let input = "internal sealed class FinalClass { void M() {} }";
    let expected_modifiers_sealed = vec![Modifier::Internal, Modifier::Sealed];
    let expected_sealed: Result<ClassDeclaration, String> = Ok(ClassDeclaration {
        modifiers: expected_modifiers_sealed.clone(),
        name: Identifier { name: "FinalClass".to_string() },
        type_parameters: vec![],
        members: vec![ 
            ClassMember::Method(MethodDeclaration {
                modifiers: vec![],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: Identifier { name: "M".to_string() },
                type_parameters: vec![],
                parameters: vec![],
                body: Some("".to_string()), 
                constraints: vec![],
                _phantom: PhantomData,
            })
        ],
    });
    let result_sealed_case = parse_class_decl_test(input);
    assert!(result_sealed_case.is_ok(), "Parsing failed for: {input}");
    assert_eq!(result_sealed_case, expected_sealed);

    let input_static = "public static class Utility {}";
    let expected_modifiers_static = vec![Modifier::Public, Modifier::Static];
    let expected_static = Ok(ClassDeclaration {
        modifiers: expected_modifiers_static,
        name: Identifier { name: "Utility".to_string() },
        type_parameters: vec![],
        members: vec![],
    });
    let result_static = parse_class_decl_test(input_static);
    assert!(result_static.is_ok(), "Parsing failed for: {input_static}");
    assert_eq!(result_static, expected_static);
}
