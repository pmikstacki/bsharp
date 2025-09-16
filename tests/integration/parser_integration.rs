// Parser integration tests for the B# syntax
// Tests that verify complete syntax functionality across different C# language features

use bsharp::parser::csharp::parse_csharp_source;

#[test]
fn test_integration_file_scoped_namespace() {
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
}"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse file-scoped namespace integration test: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have no global attributes
    assert!(compilation_unit.global_attributes.is_empty());
    
    // Should have no global using directives (they're local to the namespace)
    assert_eq!(compilation_unit.using_directives.len(), 0);
    
    // Should have file-scoped namespace
    assert!(compilation_unit.file_scoped_namespace.is_some());
    let file_scoped_ns = compilation_unit.file_scoped_namespace.unwrap();
    assert_eq!(file_scoped_ns.name.name, "MyCompany.MyProject");
    assert_eq!(file_scoped_ns.declarations.len(), 2); // class and interface
    assert_eq!(file_scoped_ns.using_directives.len(), 2); // local using directives
    
    // Should not have top-level statements
    assert!(compilation_unit.top_level_statements.is_empty());
    
    // Should not have regular declarations (everything is in file-scoped namespace)
    assert!(compilation_unit.declarations.is_empty());
}

#[test]
fn test_integration_top_level_program() {
    let code = r#"using System;

Console.WriteLine("Hello, World!");

var name = "Alice";
Console.WriteLine($"Hello, {name}!");

int Add(int a, int b) {
    return a + b;
}

var result = Add(5, 3);
Console.WriteLine($"5 + 3 = {result}");"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse top-level program integration test: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have no global attributes
    assert!(compilation_unit.global_attributes.is_empty());
    
    // Should have using directives
    assert_eq!(compilation_unit.using_directives.len(), 1);
    
    // Should have top-level statements
    assert!(!compilation_unit.top_level_statements.is_empty());
    assert!(compilation_unit.top_level_statements.len() >= 4); // Multiple statements
    
    // Should not have file-scoped namespace
    assert!(compilation_unit.file_scoped_namespace.is_none());
    
    // Should not have regular declarations (everything is top-level)
    assert!(compilation_unit.declarations.is_empty());
}

#[test]
fn test_integration_traditional_namespace() {
    let code = r#"using System;

namespace MyApp {
    public class Program {
        public static void Main(string[] args) {
            Console.WriteLine("Hello, World!");
        }
    }
}"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse traditional namespace integration test: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have no global attributes
    assert!(compilation_unit.global_attributes.is_empty());
    
    // Should have using directives
    assert_eq!(compilation_unit.using_directives.len(), 1);
    
    // Should have regular declarations (namespace)
    assert_eq!(compilation_unit.declarations.len(), 1);
    
    // Should not have file-scoped namespace
    assert!(compilation_unit.file_scoped_namespace.is_none());
    
    // Should not have top-level statements
    assert!(compilation_unit.top_level_statements.is_empty());
}

#[test]
fn test_integration_mixed_content() {
    // This tests the case where we have global using directives and top-level classes
    let code = r#"using System;

public class GlobalClass {
    public void DoSomething() {
        Console.WriteLine("Global class");
    }
}

public interface IGlobalInterface {
    void DoSomething();
}"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse mixed content integration test: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have no global attributes
    assert!(compilation_unit.global_attributes.is_empty());
    
    // Should have using directives
    assert_eq!(compilation_unit.using_directives.len(), 1);
    
    // Should have regular declarations (top-level class and interface)
    assert_eq!(compilation_unit.declarations.len(), 2);
    
    // Should not have file-scoped namespace
    assert!(compilation_unit.file_scoped_namespace.is_none());
    
    // Should not have top-level statements
    assert!(compilation_unit.top_level_statements.is_empty());
}

#[test]
fn test_integration_minimal_file_scoped_namespace() {
    let code = r#"namespace MyNs;

public class MyClass {
}"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse minimal file-scoped namespace: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have no global attributes
    assert!(compilation_unit.global_attributes.is_empty());
    
    // Should have no using directives
    assert_eq!(compilation_unit.using_directives.len(), 0);
    
    // Should have file-scoped namespace
    assert!(compilation_unit.file_scoped_namespace.is_some());
    let file_scoped_ns = compilation_unit.file_scoped_namespace.unwrap();
    assert_eq!(file_scoped_ns.name.name, "MyNs");
    assert_eq!(file_scoped_ns.declarations.len(), 1);
}

#[test]
fn test_integration_minimal_top_level_program() {
    let code = r#"Console.WriteLine("Hello!");"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse minimal top-level program: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have no global attributes
    assert!(compilation_unit.global_attributes.is_empty());
    
    // Should have no using directives
    assert_eq!(compilation_unit.using_directives.len(), 0);
    
    // Should have top-level statements
    assert!(!compilation_unit.top_level_statements.is_empty());
    assert_eq!(compilation_unit.top_level_statements.len(), 1);
    
    // Should not have file-scoped namespace
    assert!(compilation_unit.file_scoped_namespace.is_none());
    
    // Should not have regular declarations
    assert!(compilation_unit.declarations.is_empty());
}

#[test]
fn test_integration_empty_file() {
    let code = "";
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse empty file: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Everything should be empty
    assert!(compilation_unit.global_attributes.is_empty());
    assert_eq!(compilation_unit.using_directives.len(), 0);
    assert!(compilation_unit.top_level_statements.is_empty());
    assert!(compilation_unit.file_scoped_namespace.is_none());
    assert!(compilation_unit.declarations.is_empty());
}

#[test]
fn test_integration_just_using_directives() {
    let code = r#"using System;
using System.Collections.Generic;
using static System.Console;"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse just using directives: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have using directives only
    assert!(compilation_unit.global_attributes.is_empty());
    assert_eq!(compilation_unit.using_directives.len(), 3);
    assert!(compilation_unit.top_level_statements.is_empty());
    assert!(compilation_unit.file_scoped_namespace.is_none());
    assert!(compilation_unit.declarations.is_empty());
}

#[test]
fn test_integration_file_scoped_namespace_with_global_usings() {
    let code = r#"using System;
using System.Collections.Generic;

namespace MyProject;

public class Calculator {
    public void Calculate() {
        var list = new List<int>();
        Console.WriteLine("Calculation complete");
    }
}"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse file-scoped namespace with global usings: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have global using directives
    assert!(compilation_unit.global_attributes.is_empty());
    assert_eq!(compilation_unit.using_directives.len(), 2);
    
    // Should have file-scoped namespace
    assert!(compilation_unit.file_scoped_namespace.is_some());
    let file_scoped_ns = compilation_unit.file_scoped_namespace.unwrap();
    assert_eq!(file_scoped_ns.name.name, "MyProject");
    assert_eq!(file_scoped_ns.declarations.len(), 1);
    assert_eq!(file_scoped_ns.using_directives.len(), 0); // No local using directives
    
    // Should not have top-level statements or regular declarations
    assert!(compilation_unit.top_level_statements.is_empty());
    assert!(compilation_unit.declarations.is_empty());
}

#[test]
fn test_integration_global_attributes() {
    let code = r#"[assembly: System.Reflection.AssemblyTitle("Test Assembly")]
[assembly: System.Reflection.AssemblyVersion("1.0.0.0")]

using System;

namespace TestProject {
    public class TestClass {
        public void TestMethod() {
            Console.WriteLine("Test");
        }
    }
}"#;
    
    let result = parse_csharp_source(code);
    assert!(result.is_ok(), "Failed to parse global attributes: {:?}", result);
    
    let (remaining, compilation_unit) = result.unwrap();
    assert!(remaining.trim().is_empty(), "Parser should consume all input");
    
    // Should have global attributes
    assert_eq!(compilation_unit.global_attributes.len(), 2);
    
    // Should have using directives
    assert_eq!(compilation_unit.using_directives.len(), 1);
    
    // Should have regular declarations (namespace)
    assert_eq!(compilation_unit.declarations.len(), 1);
    
    // Should not have file-scoped namespace or top-level statements
    assert!(compilation_unit.file_scoped_namespace.is_none());
    assert!(compilation_unit.top_level_statements.is_empty());
} 