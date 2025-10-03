#![cfg(test)]

use parser::bsharp::parse_csharp_source;
use syntax::ast::TopLevelDeclaration;
use syntax::nodes::declarations::NamespaceBodyDeclaration;

#[test]
fn test_nested_namespace_two_levels() {
    let code = r#"
namespace N1 {
    namespace N2 {
        class MyClass {}
    }
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);

    match &cu.declarations[0] {
        TopLevelDeclaration::Namespace(ns1) => {
            assert_eq!(ns1.name.name, "N1");
            assert_eq!(ns1.declarations.len(), 1);
            match &ns1.declarations[0] {
                NamespaceBodyDeclaration::Namespace(ns2) => {
                    assert_eq!(ns2.name.name, "N2");
                    assert_eq!(ns2.declarations.len(), 1);
                }
                _ => panic!("Expected nested namespace"),
            }
        }
        _ => panic!("Expected namespace declaration"),
    }
}

#[test]
fn test_nested_namespace_three_levels() {
    let code = r#"
namespace N1 {
    namespace N2 {
        namespace N3 {
            class MyClass {}
        }
    }
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);

    match &cu.declarations[0] {
        TopLevelDeclaration::Namespace(ns1) => {
            assert_eq!(ns1.name.name, "N1");
            match &ns1.declarations[0] {
                NamespaceBodyDeclaration::Namespace(ns2) => {
                    assert_eq!(ns2.name.name, "N2");
                    match &ns2.declarations[0] {
                        NamespaceBodyDeclaration::Namespace(ns3) => {
                            assert_eq!(ns3.name.name, "N3");
                            assert_eq!(ns3.declarations.len(), 1);
                        }
                        _ => panic!("Expected N3"),
                    }
                }
                _ => panic!("Expected N2"),
            }
        }
        _ => panic!("Expected N1"),
    }
}

#[test]
fn test_dotted_namespace() {
    let code = r#"
namespace N1.N2.N3 {
    class MyClass {}
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);

    match &cu.declarations[0] {
        TopLevelDeclaration::Namespace(ns) => {
            assert_eq!(ns.name.name, "N1.N2.N3");
            assert_eq!(ns.declarations.len(), 1);
        }
        _ => panic!("Expected namespace"),
    }
}

#[test]
fn test_nested_namespace_with_using() {
    let code = r#"
namespace Outer {
    using System;
    namespace Inner {
        using System.Collections;
        class MyClass {}
    }
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);

    match &cu.declarations[0] {
        TopLevelDeclaration::Namespace(ns_outer) => {
            assert_eq!(ns_outer.name.name, "Outer");
            assert_eq!(ns_outer.using_directives.len(), 1);
            assert_eq!(ns_outer.declarations.len(), 1);

            match &ns_outer.declarations[0] {
                NamespaceBodyDeclaration::Namespace(ns_inner) => {
                    assert_eq!(ns_inner.name.name, "Inner");
                    assert_eq!(ns_inner.using_directives.len(), 1);
                }
                _ => panic!("Expected inner namespace"),
            }
        }
        _ => panic!("Expected outer namespace"),
    }
}

#[test]
fn test_multiple_nested_namespaces_same_level() {
    let code = r#"
namespace Parent {
    namespace Child1 {
        class A {}
    }
    namespace Child2 {
        class B {}
    }
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);

    match &cu.declarations[0] {
        TopLevelDeclaration::Namespace(parent) => {
            assert_eq!(parent.name.name, "Parent");
            assert_eq!(parent.declarations.len(), 2);
        }
        _ => panic!("Expected parent namespace"),
    }
}

#[test]
fn test_nested_namespace_with_class_at_each_level() {
    let code = r#"
namespace Outer {
    class OuterClass {}
    namespace Inner {
        class InnerClass {}
    }
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);

    match &cu.declarations[0] {
        TopLevelDeclaration::Namespace(outer) => {
            assert_eq!(outer.name.name, "Outer");
            assert_eq!(outer.declarations.len(), 2);
            // Should have both class and nested namespace
            let has_class = outer
                .declarations
                .iter()
                .any(|d| matches!(d, NamespaceBodyDeclaration::Class(_)));
            let has_namespace = outer
                .declarations
                .iter()
                .any(|d| matches!(d, NamespaceBodyDeclaration::Namespace(_)));
            assert!(has_class && has_namespace);
        }
        _ => panic!("Expected outer namespace"),
    }
}

#[test]
fn test_deeply_nested_namespace() {
    let code = r#"
namespace L1 {
    namespace L2 {
        namespace L3 {
            namespace L4 {
                namespace L5 {
                    class DeepClass {}
                }
            }
        }
    }
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);
    // Just verify it parses without error
}
