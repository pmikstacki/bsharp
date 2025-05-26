// Tests for trait-based AST analysis functionality

use bsharp::parser::Parser;
use bsharp::analysis::{AstAnalyze, AstAnalysis};

#[test]
fn test_trait_based_analysis() {
    let parser = Parser::new();
    let source = r#"
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
        }

        public void Loops()
        {
            for (int i = 0; i < 5; i++)
            {
                Console.WriteLine("For loop: " + i);
            }
        }
    }
}
"#;

    let ast = parser.parse(source).expect("Failed to parse source");
    
    // Test trait-based analysis
    let analysis = ast.analyze();
    
    assert_eq!(analysis.total_classes, 1);
    assert_eq!(analysis.total_methods, 2);
    assert_eq!(analysis.total_if_statements, 1);
    assert_eq!(analysis.total_for_loops, 1);
    assert_eq!(analysis.total_while_loops, 0);
    assert_eq!(analysis.total_switch_statements, 0);
    
    // Test the extension method
    let quick_analysis = ast.quick_analysis();
    assert_eq!(analysis, quick_analysis);
}

#[test]
fn test_analysis_combine() {
    let analysis1 = AstAnalysis {
        total_classes: 1,
        total_interfaces: 0,
        total_structs: 0,
        total_enums: 0,
        total_records: 0,
        total_delegates: 0,
        total_methods: 2,
        total_properties: 0,
        total_fields: 0,
        total_events: 0,
        total_constructors: 0,
        total_if_statements: 3,
        total_for_loops: 1,
        total_while_loops: 0,
        total_switch_statements: 2,
        total_try_statements: 0,
        total_using_statements: 0,
        cyclomatic_complexity: 0,
        lines_of_code: 0,
        max_nesting_depth: 0,
        documented_methods: 0,
        documented_classes: 0,
    };
    
    let analysis2 = AstAnalysis {
        total_classes: 2,
        total_interfaces: 0,
        total_structs: 0,
        total_enums: 0,
        total_records: 0,
        total_delegates: 0,
        total_methods: 1,
        total_properties: 0,
        total_fields: 0,
        total_events: 0,
        total_constructors: 0,
        total_if_statements: 1,
        total_for_loops: 2,
        total_while_loops: 1,
        total_switch_statements: 0,
        total_try_statements: 0,
        total_using_statements: 0,
        cyclomatic_complexity: 0,
        lines_of_code: 0,
        max_nesting_depth: 0,
        documented_methods: 0,
        documented_classes: 0,
    };
    
    let combined = analysis1.combine(analysis2);
    
    assert_eq!(combined.total_classes, 3);
    assert_eq!(combined.total_methods, 3);
    assert_eq!(combined.total_if_statements, 4);
    assert_eq!(combined.total_for_loops, 3);
    assert_eq!(combined.total_while_loops, 1);
    assert_eq!(combined.total_switch_statements, 2);
}

#[test]
fn test_complex_analysis() {
    let parser = Parser::new();
    let source = r#"
namespace TestApp
{
    public class ComplexClass
    {
        public void Method1()
        {
            if (true)
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
                                do
                                {
                                    Console.WriteLine("Do-while");
                                } while (false);
                                break;
                        }
                    }
                }
            }
        }

        public void Method2()
        {
            for (int j = 0; j < 5; j++)
            {
                if (j > 2)
                {
                    Console.WriteLine("Another if");
                }
            }
        }
    }
}
"#;

    let ast = parser.parse(source).expect("Failed to parse source");
    let analysis = ast.analyze();
    
    assert_eq!(analysis.total_classes, 1);
    assert_eq!(analysis.total_methods, 2);
    assert_eq!(analysis.total_if_statements, 3); // if(true), if(i%2==0), if(j>2)
    assert_eq!(analysis.total_for_loops, 2); // for(i), for(j)
    assert_eq!(analysis.total_while_loops, 2); // while(true), do-while(false)
    assert_eq!(analysis.total_switch_statements, 1); // switch(i)
}

#[test]
fn test_analysis_default() {
    let analysis = AstAnalysis::default();
    
    assert_eq!(analysis.total_classes, 0);
    assert_eq!(analysis.total_methods, 0);
    assert_eq!(analysis.total_if_statements, 0);
    assert_eq!(analysis.total_for_loops, 0);
    assert_eq!(analysis.total_while_loops, 0);
    assert_eq!(analysis.total_switch_statements, 0);
}

#[test]
fn test_analysis_metrics_collection() {
    let parser = Parser::new();
    let source = r#"
namespace TestApp
{
    public class MetricsTestClass
    {
        private int field1;
        public string Property1 { get; set; }
        
        public MetricsTestClass()
        {
            field1 = 0;
        }
        
        public void SimpleMethod()
        {
            // Simple method with no control structures
            Console.WriteLine("Hello");
        }
        
        public void ComplexMethod()
        {
            if (field1 > 0)
            {
                for (int i = 0; i < field1; i++)
                {
                    switch (i)
                    {
                        case 0:
                            while (true)
                            {
                                if (i == 0) break;
                            }
                            break;
                        default:
                            break;
                    }
                }
            }
            else
            {
                do
                {
                    field1++;
                } while (field1 < 10);
            }
        }
    }
    
    public interface ITestInterface
    {
        void InterfaceMethod();
    }
    
    public struct TestStruct
    {
        public int Value;
        
        public void StructMethod()
        {
            if (Value > 0)
            {
                Value++;
            }
        }
    }
}
"#;

    let ast = parser.parse(source).expect("Failed to parse source");
    let analysis = ast.analyze();
    
    // Verify comprehensive metrics collection
    assert_eq!(analysis.total_classes, 1); // Only counting classes, not interfaces or structs
    assert_eq!(analysis.total_methods, 4); // Constructor + 3 methods
    assert_eq!(analysis.total_if_statements, 3); // Main if, nested if, struct if
    assert_eq!(analysis.total_for_loops, 1);
    assert_eq!(analysis.total_while_loops, 2); // while + do-while
    assert_eq!(analysis.total_switch_statements, 1);
}

#[test]
fn test_analysis_with_empty_code() {
    let parser = Parser::new();
    let source = r#"
namespace EmptyApp
{
    // Empty namespace
}
"#;

    let ast = parser.parse(source).expect("Failed to parse empty source");
    let analysis = ast.analyze();
    
    // Should handle empty code gracefully
    assert_eq!(analysis.total_classes, 0);
    assert_eq!(analysis.total_methods, 0);
    assert_eq!(analysis.total_if_statements, 0);
    assert_eq!(analysis.total_for_loops, 0);
    assert_eq!(analysis.total_while_loops, 0);
    assert_eq!(analysis.total_switch_statements, 0);
}

#[test]
fn test_analysis_aggregation() {
    let parser = Parser::new();
    
    // Create multiple analysis instances as if from different files
    let source1 = r#"
namespace App1
{
    public class Class1
    {
        public void Method1()
        {
            if (true) { }
            for (int i = 0; i < 10; i++) { }
        }
    }
}
"#;
    
    let source2 = r#"
namespace App2
{
    public class Class2
    {
        public void Method2()
        {
            while (true) { break; }
            switch (1) { default: break; }
        }
        
        public void Method3()
        {
            if (false) { }
        }
    }
}
"#;
    
    let ast1 = parser.parse(source1).expect("Failed to parse source1");
    let ast2 = parser.parse(source2).expect("Failed to parse source2");
    
    let analysis1 = ast1.analyze();
    let analysis2 = ast2.analyze();
    
    // Test individual analyses
    assert_eq!(analysis1.total_classes, 1);
    assert_eq!(analysis1.total_methods, 1);
    assert_eq!(analysis1.total_if_statements, 1);
    assert_eq!(analysis1.total_for_loops, 1);
    
    assert_eq!(analysis2.total_classes, 1);
    assert_eq!(analysis2.total_methods, 2);
    assert_eq!(analysis2.total_if_statements, 1);
    assert_eq!(analysis2.total_while_loops, 1);
    assert_eq!(analysis2.total_switch_statements, 1);
    
    // Test combined analysis
    let combined = analysis1.combine(analysis2);
    assert_eq!(combined.total_classes, 2);
    assert_eq!(combined.total_methods, 3);
    assert_eq!(combined.total_if_statements, 2);
    assert_eq!(combined.total_for_loops, 1);
    assert_eq!(combined.total_while_loops, 1);
    assert_eq!(combined.total_switch_statements, 1);
} 