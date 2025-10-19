// Tests for parsing nested type declarations

use parser::expressions::declarations::type_declaration_parser::parse_class_declaration;
use syntax::declarations::{ClassBodyDeclaration, ClassDeclaration, Modifier};
use syntax::identifier::Identifier;

fn parse_class_declaration_helper(code: &str) -> Result<ClassDeclaration, String> {
    match parse_class_declaration(code.into()) {
        Ok((remaining, declaration)) => {
            if remaining.trim().is_empty() {
                Ok(declaration)
            } else {
                Err(format!("Unexpected remaining input: '{}'", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_class_with_nested_class() {
    let code = r#"
        public class OuterClass {
            public class InnerClass {
                public int Value { get; set; }
            }
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse class with nested class: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.name, Identifier::new("OuterClass"));
    assert_eq!(declaration.body_declarations.len(), 1);

    match &declaration.body_declarations[0] {
        ClassBodyDeclaration::NestedClass(inner_class) => {
            assert_eq!(inner_class.name, Identifier::new("InnerClass"));
            assert!(inner_class.modifiers.contains(&Modifier::Public));
        }
        _ => panic!(
            "Expected nested class, got {:?}",
            declaration.body_declarations[0]
        ),
    }
}

#[test]
fn test_parse_class_with_nested_struct() {
    let code = r#"
        public class OuterClass {
            private struct InnerStruct {
                public int X;
                public int Y;
            }
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse class with nested struct: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.body_declarations.len(), 1);

    match &declaration.body_declarations[0] {
        ClassBodyDeclaration::NestedStruct(inner_struct) => {
            assert_eq!(inner_struct.name, Identifier::new("InnerStruct"));
            assert!(inner_struct.modifiers.contains(&Modifier::Private));
        }
        _ => panic!(
            "Expected nested struct, got {:?}",
            declaration.body_declarations[0]
        ),
    }
}

#[test]
fn test_parse_class_with_nested_interface() {
    let code = r#"
        public class OuterClass {
            protected interface IInnerInterface {
                void DoSomething();
            }
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse class with nested interface: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.body_declarations.len(), 1);

    match &declaration.body_declarations[0] {
        ClassBodyDeclaration::NestedInterface(inner_interface) => {
            assert_eq!(inner_interface.name, Identifier::new("IInnerInterface"));
            assert!(inner_interface.modifiers.contains(&Modifier::Protected));
        }
        _ => panic!(
            "Expected nested interface, got {:?}",
            declaration.body_declarations[0]
        ),
    }
}

#[test]
fn test_parse_class_with_nested_enum() {
    let code = r#"
        public class OuterClass {
            public enum Status {
                Active,
                Inactive,
                Pending
            }
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse class with nested enum: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.body_declarations.len(), 1);

    match &declaration.body_declarations[0] {
        ClassBodyDeclaration::NestedEnum(inner_enum) => {
            assert_eq!(inner_enum.name, Identifier::new("Status"));
            assert!(inner_enum.modifiers.contains(&Modifier::Public));
        }
        _ => panic!(
            "Expected nested enum, got {:?}",
            declaration.body_declarations[0]
        ),
    }
}

#[test]
fn test_parse_class_with_nested_record() {
    let code = r#"
        public class OuterClass {
            public record InnerRecord(string Name, int Age);
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse class with nested record: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.body_declarations.len(), 1);

    match &declaration.body_declarations[0] {
        ClassBodyDeclaration::NestedRecord(inner_record) => {
            assert_eq!(inner_record.name, Identifier::new("InnerRecord"));
            assert!(inner_record.modifiers.contains(&Modifier::Public));
        }
        _ => panic!(
            "Expected nested record, got {:?}",
            declaration.body_declarations[0]
        ),
    }
}

#[test]
fn test_parse_class_with_multiple_nested_types() {
    let code = r#"
        public class OuterClass {
            public class InnerClass { }
            private struct InnerStruct { }
            protected interface IInnerInterface { }
            public enum InnerEnum { Value1, Value2 }
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse class with multiple nested types: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.body_declarations.len(), 4);

    // Check that we have all the expected nested types
    let mut has_class = false;
    let mut has_struct = false;
    let mut has_interface = false;
    let mut has_enum = false;

    for member in &declaration.body_declarations {
        match member {
            ClassBodyDeclaration::NestedClass(_) => has_class = true,
            ClassBodyDeclaration::NestedStruct(_) => has_struct = true,
            ClassBodyDeclaration::NestedInterface(_) => has_interface = true,
            ClassBodyDeclaration::NestedEnum(_) => has_enum = true,
            _ => {}
        }
    }

    assert!(has_class, "Expected nested class");
    assert!(has_struct, "Expected nested struct");
    assert!(has_interface, "Expected nested interface");
    assert!(has_enum, "Expected nested enum");
}

#[test]
fn test_parse_class_with_deeply_nested_types() {
    let code = r#"
        public class OuterClass {
            public class MiddleClass {
                public class InnerClass {
                    public int Value;
                }
            }
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse class with deeply nested types: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.body_declarations.len(), 1);

    match &declaration.body_declarations[0] {
        ClassBodyDeclaration::NestedClass(middle_class) => {
            assert_eq!(middle_class.name, Identifier::new("MiddleClass"));
            assert_eq!(middle_class.body_declarations.len(), 1);

            match &middle_class.body_declarations[0] {
                ClassBodyDeclaration::NestedClass(inner_class) => {
                    assert_eq!(inner_class.name, Identifier::new("InnerClass"));
                }
                _ => panic!("Expected nested class in middle class"),
            }
        }
        _ => panic!("Expected nested class in outer class"),
    }
}

#[test]
fn test_parse_class_with_nested_types_and_members() {
    let code = r#"
        public class OuterClass {
            public int Field;
            
            public class InnerClass { }
            
            public void Method() { }
            
            private struct InnerStruct { }
            
            public string Property { get; set; }
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse class with mixed nested types and members: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.body_declarations.len(), 5);

    // Check that we have a mix of regular members and nested types
    let mut has_field = false;
    let mut has_method = false;
    let mut has_property = false;
    let mut has_nested_class = false;
    let mut has_nested_struct = false;

    for member in &declaration.body_declarations {
        match member {
            ClassBodyDeclaration::Field(_) => has_field = true,
            ClassBodyDeclaration::Method(_) => has_method = true,
            ClassBodyDeclaration::Property(_) => has_property = true,
            ClassBodyDeclaration::NestedClass(_) => has_nested_class = true,
            ClassBodyDeclaration::NestedStruct(_) => has_nested_struct = true,
            _ => {}
        }
    }

    assert!(has_field, "Expected field");
    assert!(has_method, "Expected method");
    assert!(has_property, "Expected property");
    assert!(has_nested_class, "Expected nested class");
    assert!(has_nested_struct, "Expected nested struct");
}

#[test]
fn test_parse_nested_class_with_different_access_modifiers() {
    let access_modifiers = vec![
        ("public", Modifier::Public),
        ("private", Modifier::Private),
        ("protected", Modifier::Protected),
        ("internal", Modifier::Internal),
    ];

    for (modifier_str, expected_modifier) in access_modifiers {
        let code = format!(
            r#"
            public class OuterClass {{
                {} class InnerClass {{ }}
            }}
        "#,
            modifier_str
        );

        let result = parse_class_declaration_helper(&code);
        assert!(
            result.is_ok(),
            "Failed to parse class with {} nested class: {:?}",
            modifier_str,
            result
        );

        let declaration = result.unwrap();
        match &declaration.body_declarations[0] {
            ClassBodyDeclaration::NestedClass(inner_class) => {
                assert!(
                    inner_class.modifiers.contains(&expected_modifier),
                    "Expected {} modifier for nested class",
                    modifier_str
                );
            }
            _ => panic!("Expected nested class"),
        }
    }
}

#[test]
fn test_parse_nested_class_with_inheritance() {
    let code = r#"
        public class OuterClass {
            public class InnerClass : BaseClass, IInterface {
                public void Method() { }
            }
        }
    "#;

    let result = parse_class_declaration_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse nested class with inheritance: {:?}",
        result
    );

    let declaration = result.unwrap();
    match &declaration.body_declarations[0] {
        ClassBodyDeclaration::NestedClass(inner_class) => {
            assert_eq!(inner_class.name, Identifier::new("InnerClass"));
            assert_eq!(inner_class.base_types.len(), 2);
        }
        _ => panic!("Expected nested class"),
    }
}
