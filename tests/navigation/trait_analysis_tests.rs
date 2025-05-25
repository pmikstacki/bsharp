// Tests for trait-based AST analysis functionality

use bsharp::parser::{Parser, AstAnalyze, AstAnalysis};

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
        total_methods: 2,
        total_if_statements: 3,
        total_for_loops: 1,
        total_while_loops: 0,
        total_switch_statements: 2,
    };
    
    let analysis2 = AstAnalysis {
        total_classes: 2,
        total_methods: 1,
        total_if_statements: 1,
        total_for_loops: 2,
        total_while_loops: 1,
        total_switch_statements: 0,
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