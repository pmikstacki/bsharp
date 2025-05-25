// Tests for trait-based AST navigation functionality

use bsharp::parser::{Parser, AstNavigate, FindDeclarations};

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
    assert_eq!(switch_statements.len(), 1, "Should find exactly one switch statement");
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
    assert_eq!(if_statements.len(), 2, "Should find 2 if statements (outer and nested)");
    
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
    assert_eq!(first_class_methods.len(), 2, "First class should have 2 methods");
    
    let second_class_methods = classes[1].find_methods();
    assert_eq!(second_class_methods.len(), 1, "Second class should have 1 method");
} 