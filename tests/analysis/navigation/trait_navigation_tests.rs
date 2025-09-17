// Tests for trait-based AST navigation functionality

use bsharp::syntax::{AstNavigate, FindDeclarations, Parser};

#[test]
fn test_trait_based_navigation() {
    let parser = Parser::new();
    let source = r#"
using System;

namespace TestApp
{
    public class ControlFlowExamples
    {
        public void IfStatements(int value)
        {
            if (value > 10)
            {
                Console.WriteLine("Value is greater than 10");
            }
            else if (value > 5)
            {
                Console.WriteLine("Value is between 6 and 10");
            }
            else
            {
                Console.WriteLine("Value is 5 or less");
            }
        }

        public void SwitchStatement(string fruit)
        {
            switch (fruit.ToLower())
            {
                case "apple":
                    Console.WriteLine("Selected an apple");
                    break;
                case "banana":
                    Console.WriteLine("Selected a banana");
                    break;
                default:
                    Console.WriteLine("Unknown fruit");
                    break;
            }
        }

        public void Loops()
        {
            for (int i = 0; i < 5; i++)
            {
                Console.WriteLine("For loop: " + i);
            }

            int counter = 0;
            while (counter < 3)
            {
                Console.WriteLine("While loop: " + counter);
                counter++;
            }

            int doCounter = 0;
            do
            {
                Console.WriteLine("Do-while loop: " + doCounter);
                doCounter++;
            } while (doCounter < 3);
        }
    }
}
"#;

    let ast = parser.parse(source).expect("Failed to parse source");

    // Test trait-based navigation
    let classes = ast.find_classes();
    assert_eq!(classes.len(), 1);
    assert_eq!(classes[0].name.name, "ControlFlowExamples");

    let methods = ast.find_methods();
    assert_eq!(methods.len(), 3);

    let if_statements = ast.find_if_statements();
    assert!(if_statements.len() > 0, "Should find if statements");

    let for_loops = ast.find_for_loops();
    assert_eq!(for_loops.len(), 1, "Should find exactly one for loop");

    let while_loops = ast.find_while_loops();
    assert_eq!(while_loops.len(), 2, "Should find while and do-while loops");

    let switch_statements = ast.find_switch_statements();
    assert_eq!(
        switch_statements.len(),
        1,
        "Should find exactly one switch statement"
    );
}

#[test]
fn test_navigation_with_nested_statements() {
    let parser = Parser::new();
    let source = r#"
namespace TestApp
{
    public class NestedControlFlow
    {
        public void ComplexMethod()
        {
            for (int i = 0; i < 10; i++)
            {
                if (i % 2 == 0)
                {
                    switch (i)
                    {
                        case 0:
                            while (true)
                            {
                                break;
                            }
                            break;
                        case 2:
                            if (i > 1)
                            {
                                Console.WriteLine("Nested if");
                            }
                            break;
                    }
                }
            }
        }
    }
}
"#;

    let ast = parser.parse(source).expect("Failed to parse source");

    // Test deeply nested navigation
    let if_statements = ast.find_if_statements();
    assert_eq!(
        if_statements.len(),
        2,
        "Should find 2 if statements (outer and nested)"
    );

    let for_loops = ast.find_for_loops();
    assert_eq!(for_loops.len(), 1, "Should find 1 for loop");

    let while_loops = ast.find_while_loops();
    assert_eq!(while_loops.len(), 1, "Should find 1 while loop");

    let switch_statements = ast.find_switch_statements();
    assert_eq!(switch_statements.len(), 1, "Should find 1 switch statement");
}

#[test]
fn test_find_declarations_trait() {
    let parser = Parser::new();
    let source = r#"
namespace TestApp
{
    public class FirstClass
    {
        public void FirstMethod() { }
        public void SecondMethod() { }
    }

    public class SecondClass
    {
        public void ThirdMethod() { }
    }
}
"#;

    let ast = parser.parse(source).expect("Failed to parse source");

    // Test FindDeclarations trait
    let classes = ast.find_classes();
    assert_eq!(classes.len(), 2, "Should find 2 classes");
    assert_eq!(classes[0].name.name, "FirstClass");
    assert_eq!(classes[1].name.name, "SecondClass");

    let methods = ast.find_methods();
    assert_eq!(methods.len(), 3, "Should find 3 methods total");

    // Test finding methods within a specific class
    let first_class_methods = classes[0].find_methods();
    assert_eq!(
        first_class_methods.len(),
        2,
        "First class should have 2 methods"
    );

    let second_class_methods = classes[1].find_methods();
    assert_eq!(
        second_class_methods.len(),
        1,
        "Second class should have 1 method"
    );
}

#[test]
fn test_navigation_performance_with_large_ast() {
    let parser = Parser::new();

    // Create a larger code sample for performance testing
    let mut source = String::from("namespace TestApp\n{\n");

    // Add multiple classes with methods
    for class_idx in 0..10 {
        source.push_str(&format!("    public class Class{}\n    {{\n", class_idx));

        for method_idx in 0..5 {
            source.push_str(&format!(
                "        public void Method{}()\n        {{\n",
                method_idx
            ));
            source.push_str("            if (true)\n            {\n");
            source.push_str("                for (int i = 0; i < 10; i++)\n                {\n");
            source.push_str("                    while (i > 0)\n                    {\n");
            source.push_str("                        break;\n");
            source.push_str("                    }\n");
            source.push_str("                }\n");
            source.push_str("            }\n");
            source.push_str("        }\n");
        }

        source.push_str("    }\n");
    }

    source.push_str("}\n");

    let ast = parser.parse(&source).expect("Failed to parse large source");

    // Test navigation on larger AST
    let classes = ast.find_classes();
    assert_eq!(classes.len(), 10, "Should find 10 classes");

    let methods = ast.find_methods();
    assert_eq!(methods.len(), 50, "Should find 50 methods total");

    let if_statements = ast.find_if_statements();
    assert_eq!(if_statements.len(), 50, "Should find 50 if statements");

    let for_loops = ast.find_for_loops();
    assert_eq!(for_loops.len(), 50, "Should find 50 for loops");

    let while_loops = ast.find_while_loops();
    assert_eq!(while_loops.len(), 50, "Should find 50 while loops");
}

#[test]
fn test_navigation_with_edge_cases() {
    let parser = Parser::new();
    let source = r#"
namespace TestApp
{
    public class EdgeCases
    {
        // Empty method
        public void EmptyMethod() { }
        
        // Method with only expression body
        public int ExpressionBodyMethod() => 42;
        
        // Method with nested control structures
        public void NestedControlStructures()
        {
            // Nested if-else chains
            if (true)
            {
                if (false)
                {
                    if (true)
                    {
                        // Deep nesting
                    }
                }
                else
                {
                    // Else branch
                }
            }
            
            // Switch with various case types
            switch (42)
            {
                case 1:
                case 2:
                case 3:
                    break;
                default:
                    while (true)
                    {
                        for (int i = 0; i < 10; i++)
                        {
                            if (i == 5) break;
                            if (i == 3) continue;
                        }
                        break;
                    }
                    break;
            }
        }
    }
}
"#;

    let ast = parser
        .parse(source)
        .expect("Failed to parse edge cases source");

    // Test navigation handles edge cases
    let classes = ast.find_classes();
    assert_eq!(classes.len(), 1, "Should find 1 class");

    let methods = ast.find_methods();
    assert_eq!(
        methods.len(),
        3,
        "Should find 3 methods including expression-bodied"
    );

    let if_statements = ast.find_if_statements();
    assert_eq!(
        if_statements.len(),
        5,
        "Should find all nested if statements"
    );

    let while_loops = ast.find_while_loops();
    assert_eq!(while_loops.len(), 1, "Should find 1 while loop");

    let for_loops = ast.find_for_loops();
    assert_eq!(for_loops.len(), 1, "Should find 1 for loop");

    let switch_statements = ast.find_switch_statements();
    assert_eq!(switch_statements.len(), 1, "Should find 1 switch statement");
}
