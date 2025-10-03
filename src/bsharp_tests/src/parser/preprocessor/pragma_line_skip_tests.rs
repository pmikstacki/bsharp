#![cfg(test)]

use parser::bsharp::parse_csharp_source;
use syntax::ast::TopLevelDeclaration;

#[test]
fn test_pragma_before_using() {
    let code = r#"
#pragma warning disable CS0168
using System;

class Test {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.using_directives.len(), 1);
    assert_eq!(cu.declarations.len(), 1);
}

#[test]
fn test_line_before_class() {
    let code = r#"
#line 100 "original.cs"
class MyClass {
    void Method() {}
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);
    match &cu.declarations[0] {
        TopLevelDeclaration::Class(c) => assert_eq!(c.name.name, "MyClass"),
        _ => panic!("Expected class declaration"),
    }
}

#[test]
fn test_pragma_between_members() {
    let code = r#"
using System;
#pragma warning disable CS0219
class First {}
#pragma warning restore CS0219
class Second {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.using_directives.len(), 1);
    assert_eq!(cu.declarations.len(), 2);
}

#[test]
fn test_line_hidden_directive() {
    let code = r#"
#line hidden
namespace Test {
    class MyClass {}
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);
    match &cu.declarations[0] {
        TopLevelDeclaration::Namespace(ns) => assert_eq!(ns.name.name, "Test"),
        _ => panic!("Expected namespace declaration"),
    }
}

#[test]
fn test_multiple_directives_interleaved() {
    let code = r#"
#pragma warning disable
using System;
#line 50
using System.Collections.Generic;
#pragma checksum "file.cs" "{12345678-1234-1234-1234-123456789012}" "ABCDEF"
class Test {}
#line default
struct Data {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.using_directives.len(), 2);
    assert_eq!(cu.declarations.len(), 2);
}

#[test]
fn test_pragma_with_file_scoped_namespace() {
    let code = r#"
#pragma warning disable CS8600
namespace MyNamespace;
#line 200
using System;

class MyClass {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert!(cu.file_scoped_namespace.is_some());
    assert_eq!(cu.using_directives.len(), 1);
    assert_eq!(cu.declarations.len(), 1);
}

#[test]
fn test_line_directive_with_numbers_only() {
    let code = r#"
#line 1000
class Test {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);
}

#[test]
fn test_pragma_nullable_directives() {
    let code = r#"
#pragma nullable enable
using System;
#pragma nullable disable
class Test {}
#pragma nullable restore
struct Data {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.using_directives.len(), 1);
    assert_eq!(cu.declarations.len(), 2);
}

#[test]
fn test_mixed_directives_complex() {
    let code = r#"
#pragma warning disable CS0168
#line 500 "generated.cs"
using System;
#pragma checksum "test.cs" "{00000000-0000-0000-0000-000000000000}" "HASH"
using System.Linq;

#line hidden
namespace Outer {
    #pragma warning restore
    class Inner {}
}

#line default
class Standalone {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.using_directives.len(), 2);
    assert_eq!(cu.declarations.len(), 2);
}

#[test]
fn test_empty_pragma() {
    let code = r#"
#pragma
class Test {}
"#;
    // Empty pragma should still parse
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);
}

#[test]
fn test_line_default() {
    let code = r#"
#line default
class Test {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);
}

#[test]
fn test_consecutive_directives() {
    let code = r#"
#pragma warning disable
#pragma nullable enable
#line 100
#line hidden
class Test {}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);
}

#[test]
fn test_directives_do_not_affect_member_parsing() {
    let code = r#"
class Test {
    #pragma warning disable
    void Method1() {}
    #line 200
    void Method2() {}
}
"#;
    let result = parse_csharp_source(code);
    assert!(result.is_ok());
    let (_, cu) = result.unwrap();
    assert_eq!(cu.declarations.len(), 1);
    match &cu.declarations[0] {
        TopLevelDeclaration::Class(c) => {
            // Class body contains method declarations
            assert_eq!(c.body_declarations.len(), 2);
        }
        _ => panic!("Expected class declaration"),
    }
}
