#![allow(unused_variables)]

use bsharp::analysis::metrics::complexity::*;
use bsharp::syntax::nodes::declarations::MethodDeclaration;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::statements::statement::*;
use bsharp::syntax::nodes::statements::{BreakStatement, ContinueStatement};
use bsharp::syntax::nodes::statements::{
    ForStatement, IfStatement, SwitchLabel, SwitchSection, SwitchStatement, WhileStatement,
};

fn create_test_identifier(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

fn create_test_method(name: &str, body: Option<Statement>) -> MethodDeclaration {
    MethodDeclaration {
        modifiers: Vec::new(),
        return_type: bsharp::syntax::nodes::types::Type::Reference(create_test_identifier("void")),
        name: create_test_identifier(name),
        type_parameters: None,
        parameters: Vec::new(),
        constraints: None,
        body,
    }
}

#[test]
fn test_complexity_metrics_default() {
    let metrics = ComplexityMetrics::default();
    assert_eq!(metrics.cyclomatic_complexity, 0);
    assert_eq!(metrics.cognitive_complexity, 0);
    assert_eq!(metrics.max_nesting_depth, 0);
}

#[test]
fn test_abc_complexity_magnitude() {
    let abc = ABCComplexity {
        assignments: 3,
        branches: 4,
        conditions: 5,
    };

    // sqrt(3^2 + 4^2 + 5^2) = sqrt(9 + 16 + 25) = sqrt(50) ≈ 7.07
    let expected = ((3_usize.pow(2) + 4_usize.pow(2) + 5_usize.pow(2)) as f64).sqrt();
    assert!((abc.magnitude() - expected).abs() < 0.001);
}

#[test]
fn test_halstead_metrics_vocabulary() {
    let halstead = HalsteadMetrics {
        distinct_operators: 10,
        distinct_operands: 15,
        total_operators: 50,
        total_operands: 30,
    };

    assert_eq!(halstead.vocabulary(), 25);
    assert_eq!(halstead.length(), 80);
}

#[test]
fn test_halstead_metrics_volume() {
    let halstead = HalsteadMetrics {
        distinct_operators: 4,
        distinct_operands: 4,
        total_operators: 10,
        total_operands: 10,
    };

    // Volume = length * log2(vocabulary) = 20 * log2(8) = 20 * 3 = 60
    let expected_volume = 20.0 * 8.0_f64.log2();
    assert!((halstead.volume() - expected_volume).abs() < 0.001);
}

#[test]
fn test_halstead_metrics_zero_division() {
    let empty_halstead = HalsteadMetrics::default();
    assert_eq!(empty_halstead.volume(), 0.0);
    assert_eq!(empty_halstead.difficulty(), 0.0);
    assert_eq!(empty_halstead.effort(), 0.0);
}

#[test]
fn test_complexity_analyzer_new() {
    let analyzer = ComplexityAnalyzer::new();
    assert_eq!(analyzer, ComplexityAnalyzer::new());
}

#[test]
fn test_analyze_method_without_body() {
    let analyzer = ComplexityAnalyzer::new();
    let method = create_test_method("TestMethod", None);

    let metrics = analyzer.analyze_method(&method);
    assert_eq!(metrics.cyclomatic_complexity, 1); // Base complexity
    assert_eq!(metrics.cognitive_complexity, 0);
    assert_eq!(metrics.max_nesting_depth, 0);
}

#[test]
fn test_analyze_method_with_empty_body() {
    let analyzer = ComplexityAnalyzer::new();
    let method = create_test_method("TestMethod", Some(Statement::Block(Vec::new())));

    let metrics = analyzer.analyze_method(&method);
    assert_eq!(metrics.cyclomatic_complexity, 1); // Base complexity
    assert_eq!(metrics.cognitive_complexity, 0);
    assert_eq!(metrics.max_nesting_depth, 0);
}

#[test]
fn test_cyclomatic_complexity_if_statement() {
    let analyzer = ComplexityAnalyzer::new();

    let if_stmt = Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::Block(Vec::new())),
        alternative: None,
    }));

    let complexity = ComplexityAnalyzer::calculate_cyclomatic_complexity(&if_stmt, 1);
    assert_eq!(complexity, 2); // Base + 1 for if
}

#[test]
fn test_cyclomatic_complexity_if_else() {
    let analyzer = ComplexityAnalyzer::new();

    let if_stmt = Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::Block(Vec::new())),
        alternative: Some(Box::new(Statement::Block(Vec::new()))),
    }));

    let complexity = ComplexityAnalyzer::calculate_cyclomatic_complexity(&if_stmt, 1);
    assert_eq!(complexity, 2); // Base + 1 for if (else doesn't add complexity)
}

#[test]
fn test_cyclomatic_complexity_nested_if() {
    let analyzer = ComplexityAnalyzer::new();

    let nested_if = Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::If(Box::new(IfStatement {
            condition: Expression::Literal(Literal::Boolean(false)),
            consequence: Box::new(Statement::Block(Vec::new())),
            alternative: None,
        }))),
        alternative: None,
    }));

    let complexity = ComplexityAnalyzer::calculate_cyclomatic_complexity(&nested_if, 1);
    assert_eq!(complexity, 3); // Base + 1 for outer if + 1 for inner if
}

#[test]
fn test_cyclomatic_complexity_for_loop() {
    let analyzer = ComplexityAnalyzer::new();

    let for_stmt = Statement::For(Box::new(ForStatement {
        initializer: None,
        condition: None,
        iterator: vec![],
        body: Box::new(Statement::Block(Vec::new())),
    }));

    let complexity = ComplexityAnalyzer::calculate_cyclomatic_complexity(&for_stmt, 1);
    assert_eq!(complexity, 2); // Base + 1 for for
}

#[test]
fn test_cyclomatic_complexity_switch() {
    let analyzer = ComplexityAnalyzer::new();

    let switch_stmt = Statement::Switch(Box::new(SwitchStatement {
        expression: Expression::Literal(Literal::Integer(1)),
        sections: vec![
            SwitchSection {
                labels: vec![SwitchLabel::Case(Expression::Literal(Literal::Integer(1)))],
                statements: vec![Statement::Break(BreakStatement)],
            },
            SwitchSection {
                labels: vec![SwitchLabel::Case(Expression::Literal(Literal::Integer(2)))],
                statements: vec![Statement::Break(BreakStatement)],
            },
            SwitchSection {
                labels: vec![SwitchLabel::Default],
                statements: vec![Statement::Break(BreakStatement)],
            },
        ],
    }));

    let complexity = ComplexityAnalyzer::calculate_cyclomatic_complexity(&switch_stmt, 1);
    assert_eq!(complexity, 4); // Base + 3 for each case
}

#[test]
fn test_cognitive_complexity_simple_if() {
    let analyzer = ComplexityAnalyzer::new();

    let if_stmt = Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::Block(Vec::new())),
        alternative: None,
    }));

    let complexity = ComplexityAnalyzer::calculate_cognitive_complexity(&if_stmt, 0, 0);
    assert_eq!(complexity, 1); // +1 for if, +0 for nesting depth 0
}

#[test]
fn test_cognitive_complexity_nested_if() {
    let analyzer = ComplexityAnalyzer::new();

    let nested_if = Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::If(Box::new(IfStatement {
            condition: Expression::Literal(Literal::Boolean(false)),
            consequence: Box::new(Statement::Block(Vec::new())),
            alternative: None,
        }))),
        alternative: None,
    }));

    let complexity = ComplexityAnalyzer::calculate_cognitive_complexity(&nested_if, 0, 0);
    assert_eq!(complexity, 3); // +1 for outer if, +2 for inner if (1 + nesting depth 1)
}

#[test]
fn test_cognitive_complexity_else_if() {
    let analyzer = ComplexityAnalyzer::new();

    let else_if = Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::Block(Vec::new())),
        alternative: Some(Box::new(Statement::If(Box::new(IfStatement {
            condition: Expression::Literal(Literal::Boolean(false)),
            consequence: Box::new(Statement::Block(Vec::new())),
            alternative: None,
        })))),
    }));

    let complexity = ComplexityAnalyzer::calculate_cognitive_complexity(&else_if, 0, 0);
    assert_eq!(complexity, 2); // +1 for if, +1 for else if (no extra nesting)
}

#[test]
fn test_cognitive_complexity_break_continue() {
    let analyzer = ComplexityAnalyzer::new();

    let break_stmt = Statement::Break(BreakStatement);
    let continue_stmt = Statement::Continue(ContinueStatement);

    assert_eq!(
        ComplexityAnalyzer::calculate_cognitive_complexity(&break_stmt, 0, 0),
        1
    );
    assert_eq!(
        ComplexityAnalyzer::calculate_cognitive_complexity(&continue_stmt, 0, 0),
        1
    );
}

#[test]
fn test_max_nesting_depth_simple() {
    let analyzer = ComplexityAnalyzer::new();

    let if_stmt = Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::Block(Vec::new())),
        alternative: None,
    }));

    let depth = ComplexityAnalyzer::calculate_max_nesting_depth(&if_stmt, 0);
    assert_eq!(depth, 1);
}

#[test]
fn test_max_nesting_depth_nested() {
    let analyzer = ComplexityAnalyzer::new();

    let deeply_nested = Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::For(Box::new(ForStatement {
            initializer: None,
            condition: None,
            iterator: vec![],
            body: Box::new(Statement::While(Box::new(WhileStatement {
                condition: Box::new(Expression::Literal(Literal::Boolean(true))),
                body: Box::new(Statement::Block(Vec::new())),
            }))),
        }))),
        alternative: None,
    }));

    let depth = ComplexityAnalyzer::calculate_max_nesting_depth(&deeply_nested, 0);
    assert_eq!(depth, 3); // if -> for -> while
}

#[test]
fn test_abc_complexity_calculation() {
    let analyzer = ComplexityAnalyzer::new();

    let complex_stmt = Statement::Block(vec![
        Statement::If(Box::new(IfStatement {
            condition: Expression::Literal(Literal::Boolean(true)),
            consequence: Box::new(Statement::Expression(Expression::Literal(
                Literal::Integer(1),
            ))),
            alternative: None,
        })),
        Statement::For(Box::new(ForStatement {
            initializer: None,
            condition: None,
            iterator: vec![],
            body: Box::new(Statement::Expression(Expression::Literal(
                Literal::Integer(2),
            ))),
        })),
    ]);

    let abc = ComplexityAnalyzer::calculate_abc_complexity(&complex_stmt);
    assert_eq!(abc.conditions, 2); // if + for
    assert_eq!(abc.branches, 2); // if + for
    assert_eq!(abc.assignments, 2); // 2 expressions
}

#[test]
fn test_complete_method_analysis() {
    let analyzer = ComplexityAnalyzer::new();

    let complex_method_body = Statement::Block(vec![Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::For(Box::new(ForStatement {
            initializer: None,
            condition: None,
            iterator: vec![],
            body: Box::new(Statement::If(Box::new(IfStatement {
                condition: Expression::Literal(Literal::Boolean(false)),
                consequence: Box::new(Statement::Break(BreakStatement)),
                alternative: None,
            }))),
        }))),
        alternative: Some(Box::new(Statement::While(Box::new(WhileStatement {
            condition: Box::new(Expression::Literal(Literal::Boolean(true))),
            body: Box::new(Statement::Continue(ContinueStatement)),
        })))),
    }))]);

    let method = create_test_method("ComplexMethod", Some(complex_method_body));
    let metrics = analyzer.analyze_method(&method);

    // Cyclomatic: 1 (base) + 1 (if) + 1 (for) + 1 (nested if) + 1 (while) = 5
    assert_eq!(metrics.cyclomatic_complexity, 5);

    // Cognitive: 1 (if, depth 0) + 2 (for, depth 1) + 3 (nested if, depth 2) +
    //           2 (while, depth 1) + 2 (continue, depth 2) + 1 (break, depth 3) = 11
    assert_eq!(metrics.cognitive_complexity, 11);

    // Max nesting: if -> for -> nested if = 3
    assert_eq!(metrics.max_nesting_depth, 3);
}

#[test]
fn test_essential_complexity_calculation() {
    let analyzer = ComplexityAnalyzer::new();

    // Test method with structured control flow (low essential complexity)
    let structured_method = Statement::Block(vec![
        Statement::If(Box::new(IfStatement {
            condition: Expression::Literal(Literal::Boolean(true)),
            consequence: Box::new(Statement::Block(Vec::new())),
            alternative: Some(Box::new(Statement::Block(Vec::new()))),
        })),
        Statement::For(Box::new(ForStatement {
            initializer: None,
            condition: None,
            iterator: vec![],
            body: Box::new(Statement::Block(Vec::new())),
        })),
    ]);

    let method = create_test_method("StructuredMethod", Some(structured_method));
    let metrics = analyzer.analyze_method(&method);

    // Essential complexity should be low for well-structured code
    assert_eq!(metrics.essential_complexity, 0); // Default value for now
}

#[test]
fn test_complexity_threshold_analysis() {
    let analyzer = ComplexityAnalyzer::new();

    // Create a method that exceeds typical complexity thresholds
    let high_complexity_body = Statement::Block(vec![Statement::If(Box::new(IfStatement {
        condition: Expression::Literal(Literal::Boolean(true)),
        consequence: Box::new(Statement::If(Box::new(IfStatement {
            condition: Expression::Literal(Literal::Boolean(true)),
            consequence: Box::new(Statement::If(Box::new(IfStatement {
                condition: Expression::Literal(Literal::Boolean(true)),
                consequence: Box::new(Statement::If(Box::new(IfStatement {
                    condition: Expression::Literal(Literal::Boolean(true)),
                    consequence: Box::new(Statement::If(Box::new(IfStatement {
                        condition: Expression::Literal(Literal::Boolean(true)),
                        consequence: Box::new(Statement::Block(Vec::new())),
                        alternative: None,
                    }))),
                    alternative: None,
                }))),
                alternative: None,
            }))),
            alternative: None,
        }))),
        alternative: None,
    }))]);

    let method = create_test_method("HighComplexityMethod", Some(high_complexity_body));
    let metrics = analyzer.analyze_method(&method);

    // Cyclomatic complexity should be 6 (1 base + 5 ifs)
    assert_eq!(metrics.cyclomatic_complexity, 6);

    // Cognitive complexity should be much higher due to nesting
    // 1 + 2 + 3 + 4 + 5 = 15
    assert_eq!(metrics.cognitive_complexity, 15);

    // Max nesting depth should be 5
    assert_eq!(metrics.max_nesting_depth, 5);

    // Verify it exceeds common thresholds
    assert!(metrics.cyclomatic_complexity > 4); // Common threshold
    assert!(metrics.cognitive_complexity > 10); // Common threshold
    assert!(metrics.max_nesting_depth > 3); // Common threshold
}

#[test]
fn test_halstead_calculation_with_real_expressions() {
    // This test would require implementing proper expression analysis
    // For now, we test the formulas with known values
    let halstead = HalsteadMetrics {
        distinct_operators: 8, // +, -, *, /, =, ==, <, >
        distinct_operands: 6,  // variables a, b, c, constants 1, 2, 10
        total_operators: 15,   // total operator occurrences
        total_operands: 12,    // total operand occurrences
    };

    assert_eq!(halstead.vocabulary(), 14);
    assert_eq!(halstead.length(), 27);

    let volume = halstead.volume();
    assert!(volume > 0.0);

    let difficulty = halstead.difficulty();
    assert!(difficulty > 0.0);

    let effort = halstead.effort();
    assert_eq!(effort, volume * difficulty);
}

#[test]
fn test_complexity_comparison() {
    let analyzer = ComplexityAnalyzer::new();

    // Simple method
    let simple_method = create_test_method(
        "Simple",
        Some(Statement::Block(vec![Statement::Expression(
            Expression::Literal(Literal::Integer(1)),
        )])),
    );

    // Complex method
    let complex_method = create_test_method(
        "Complex",
        Some(Statement::Block(vec![Statement::If(Box::new(
            IfStatement {
                condition: Expression::Literal(Literal::Boolean(true)),
                consequence: Box::new(Statement::For(Box::new(ForStatement {
                    initializer: None,
                    condition: None,
                    iterator: vec![],
                    body: Box::new(Statement::Switch(Box::new(SwitchStatement {
                        expression: Expression::Literal(Literal::Integer(1)),
                        sections: vec![
                            SwitchSection {
                                labels: vec![SwitchLabel::Case(Expression::Literal(
                                    Literal::Integer(1),
                                ))],
                                statements: vec![Statement::Break(BreakStatement)],
                            },
                            SwitchSection {
                                labels: vec![SwitchLabel::Default],
                                statements: vec![Statement::Break(BreakStatement)],
                            },
                        ],
                    }))),
                }))),
                alternative: None,
            },
        ))])),
    );

    let simple_metrics = analyzer.analyze_method(&simple_method);
    let complex_metrics = analyzer.analyze_method(&complex_method);

    // Complex method should have higher complexity in all measures
    assert!(complex_metrics.cyclomatic_complexity > simple_metrics.cyclomatic_complexity);
    assert!(complex_metrics.cognitive_complexity > simple_metrics.cognitive_complexity);
    assert!(complex_metrics.max_nesting_depth > simple_metrics.max_nesting_depth);
}
