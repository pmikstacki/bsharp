// Tests for parsing file-scoped namespace declarations

use parser::expressions::declarations::file_scoped_namespace_parser::parse_file_scoped_namespace_declaration;
use syntax::nodes::declarations::{
    namespace_declaration::NamespaceBodyDeclaration, FileScopedNamespaceDeclaration, UsingDirective,
};
use syntax::nodes::identifier::Identifier;

fn parse_file_scoped_namespace_declaration_helper(
    code: &str,
) -> Result<FileScopedNamespaceDeclaration, String> {
    match parse_file_scoped_namespace_declaration(code) {
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
fn test_parse_simple_file_scoped_namespace() {
    let code = "namespace MyNs;";
    let expected = FileScopedNamespaceDeclaration {
        name: Identifier {
            name: "MyNs".to_string(),
        },
        using_directives: vec![],
        declarations: vec![],
    };
    assert_eq!(
        parse_file_scoped_namespace_declaration_helper(code),
        Ok(expected)
    );
}

#[test]
fn test_parse_qualified_file_scoped_namespace() {
    let code = "namespace MyCompany.MyProject;";
    let expected = FileScopedNamespaceDeclaration {
        name: Identifier {
            name: "MyCompany.MyProject".to_string(),
        },
        using_directives: vec![],
        declarations: vec![],
    };
    assert_eq!(
        parse_file_scoped_namespace_declaration_helper(code),
        Ok(expected)
    );
}

#[test]
fn test_parse_file_scoped_namespace_with_using() {
    let code = r#"namespace MyNs;

using System;
using System.Collections.Generic;"#;

    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse file-scoped namespace with using directives: {:?}",
        result
    );

    let namespace = result.unwrap();
    assert_eq!(namespace.name.name, "MyNs");
    assert_eq!(namespace.using_directives.len(), 2);

    match &namespace.using_directives[0] {
        UsingDirective::Namespace { namespace } => {
            assert_eq!(namespace.name, "System");
        }
        _ => panic!("Expected namespace using directive"),
    }

    match &namespace.using_directives[1] {
        UsingDirective::Namespace { namespace } => {
            assert_eq!(namespace.name, "System.Collections.Generic");
        }
        _ => panic!("Expected namespace using directive"),
    }
}

#[test]
fn test_parse_file_scoped_namespace_with_class() {
    let code = r#"namespace MyNs;

public class TestClass {
    public void TestMethod() { }
}"#;

    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse file-scoped namespace with class: {:?}",
        result
    );

    let namespace = result.unwrap();
    assert_eq!(namespace.name.name, "MyNs");
    assert_eq!(namespace.declarations.len(), 1);

    match &namespace.declarations[0] {
        NamespaceBodyDeclaration::Class(class_decl) => {
            assert_eq!(class_decl.name.name, "TestClass");
        }
        _ => panic!("Expected class declaration"),
    }
}

#[test]
fn test_parse_file_scoped_namespace_complete_example() {
    let code = r#"namespace MyCompany.MyProject;

using System;
using System.Collections.Generic;

public class Calculator {
    public int Add(int a, int b) {
        return a + b;
    }
}

public interface ICalculator {
    int Add(int a, int b);
}

public enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}"#;

    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse complete file-scoped namespace: {:?}",
        result
    );

    let namespace = result.unwrap();
    assert_eq!(namespace.name.name, "MyCompany.MyProject");
    assert_eq!(namespace.using_directives.len(), 2);
    assert_eq!(namespace.declarations.len(), 3); // class, interface, enum

    // Check class
    match &namespace.declarations[0] {
        NamespaceBodyDeclaration::Class(class_decl) => {
            assert_eq!(class_decl.name.name, "Calculator");
        }
        _ => panic!("Expected class declaration"),
    }

    // Check interface
    match &namespace.declarations[1] {
        NamespaceBodyDeclaration::Interface(interface_decl) => {
            assert_eq!(interface_decl.name.name, "ICalculator");
        }
        _ => panic!("Expected interface declaration"),
    }

    // Check enum
    match &namespace.declarations[2] {
        NamespaceBodyDeclaration::Enum(enum_decl) => {
            assert_eq!(enum_decl.name.name, "Operation");
        }
        _ => panic!("Expected enum declaration"),
    }
}

#[test]
fn test_parse_file_scoped_namespace_with_record() {
    let code = r#"namespace Records;

public record Person(string FirstName, string LastName);

public record struct Point(int X, int Y);"#;

    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse file-scoped namespace with records: {:?}",
        result
    );

    let namespace = result.unwrap();
    assert_eq!(namespace.name.name, "Records");
    assert_eq!(namespace.declarations.len(), 2);

    // Check records
    match &namespace.declarations[0] {
        NamespaceBodyDeclaration::Record(record_decl) => {
            assert_eq!(record_decl.name.name, "Person");
        }
        _ => panic!("Expected record declaration"),
    }

    match &namespace.declarations[1] {
        NamespaceBodyDeclaration::Record(record_decl) => {
            assert_eq!(record_decl.name.name, "Point");
        }
        _ => panic!("Expected record declaration"),
    }
}

#[test]
fn test_parse_file_scoped_namespace_with_struct() {
    let code = r#"namespace Structs;

public struct Vector3 {
    public float X;
    public float Y;
    public float Z;
}"#;

    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse file-scoped namespace with struct: {:?}",
        result
    );

    let namespace = result.unwrap();
    assert_eq!(namespace.name.name, "Structs");
    assert_eq!(namespace.declarations.len(), 1);

    match &namespace.declarations[0] {
        NamespaceBodyDeclaration::Struct(struct_decl) => {
            assert_eq!(struct_decl.name.name, "Vector3");
        }
        _ => panic!("Expected struct declaration"),
    }
}

#[test]
fn test_parse_file_scoped_namespace_with_delegate() {
    let code = r#"namespace Delegates;

public delegate int Calculator(int x, int y);
public delegate void EventHandler<T>(T args);"#;

    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse file-scoped namespace with delegates: {:?}",
        result
    );

    let namespace = result.unwrap();
    assert_eq!(namespace.name.name, "Delegates");
    assert_eq!(namespace.declarations.len(), 2);

    match &namespace.declarations[0] {
        NamespaceBodyDeclaration::Delegate(delegate_decl) => {
            assert_eq!(delegate_decl.name.name, "Calculator");
        }
        _ => panic!("Expected delegate declaration"),
    }

    match &namespace.declarations[1] {
        NamespaceBodyDeclaration::Delegate(delegate_decl) => {
            assert_eq!(delegate_decl.name.name, "EventHandler");
        }
        _ => panic!("Expected delegate declaration"),
    }
}

#[test]
fn test_parse_file_scoped_namespace_whitespace_variations() {
    let code = r#"  namespace   MyNs  ;

  using   System  ;

  public   class   TestClass   {
      public void TestMethod() { }
  }"#;

    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse file-scoped namespace with whitespace variations: {:?}",
        result
    );

    let namespace = result.unwrap();
    assert_eq!(namespace.name.name, "MyNs");
    assert_eq!(namespace.using_directives.len(), 1);
    assert_eq!(namespace.declarations.len(), 1);
}

#[test]
fn test_parse_file_scoped_namespace_errors() {
    // Missing semicolon should fail
    let code = "namespace MyNs";
    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(result.is_err(), "Expected error for missing semicolon");

    // Empty namespace name should fail
    let code = "namespace ;";
    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(result.is_err(), "Expected error for empty namespace name");

    // Invalid namespace name should fail
    let code = "namespace 123Invalid;";
    let result = parse_file_scoped_namespace_declaration_helper(code);
    assert!(result.is_err(), "Expected error for invalid namespace name");
}

#[test]
fn test_file_scoped_namespace_vs_block_scoped() {
    // This should parse as file-scoped (has semicolon)
    let file_scoped = "namespace MyNs;";
    let result = parse_file_scoped_namespace_declaration_helper(file_scoped);
    assert!(
        result.is_ok(),
        "File-scoped namespace should parse successfully"
    );

    // This should fail for file-scoped syntax (has braces, not semicolon)
    let block_scoped = "namespace MyNs { }";
    let result = parse_file_scoped_namespace_declaration_helper(block_scoped);
    assert!(
        result.is_err(),
        "Block-scoped namespace should not parse as file-scoped"
    );
}
