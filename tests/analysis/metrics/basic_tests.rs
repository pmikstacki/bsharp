use bsharp::analysis::metrics::basic::{BasicMetrics, BasicMetricsCollector};
use bsharp::syntax::ast::*;
use bsharp::syntax::nodes::Identifier;
use bsharp::syntax::nodes::declarations::*;
use bsharp::syntax::nodes::statements::statement::*;
use bsharp::syntax::nodes::statements::{
    BreakStatement, CatchClause, DoWhileStatement, ForStatement, IfStatement, SwitchLabel,
    SwitchSection, SwitchStatement, TryStatement, WhileStatement,
};
use bsharp::syntax::nodes::types::{PrimitiveType, Type};

fn create_test_identifier(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

#[test]
fn test_basic_metrics_default() {
    let metrics = BasicMetrics::default();
    assert_eq!(metrics.total_classes, 0);
    assert_eq!(metrics.total_methods, 0);
    assert_eq!(metrics.physical_lines, 0);
    assert_eq!(metrics.logical_lines, 0);
}

#[test]
fn test_basic_metrics_combine() {
    let metrics1 = BasicMetrics {
        total_classes: 2,
        total_methods: 5,
        physical_lines: 100,
        logical_lines: 80,
        comment_lines: 20,
        ..Default::default()
    };

    let metrics2 = BasicMetrics {
        total_classes: 1,
        total_methods: 3,
        physical_lines: 50,
        logical_lines: 40,
        comment_lines: 10,
        ..Default::default()
    };

    let combined = metrics1.combine(metrics2);
    assert_eq!(combined.total_classes, 3);
    assert_eq!(combined.total_methods, 8);
    assert_eq!(combined.physical_lines, 150);
    assert_eq!(combined.logical_lines, 120);
    assert_eq!(combined.comment_lines, 30);
}

#[test]
fn test_total_types() {
    let metrics = BasicMetrics {
        total_classes: 2,
        total_interfaces: 1,
        total_structs: 1,
        total_enums: 1,
        total_records: 1,
        total_delegates: 1,
        ..Default::default()
    };

    assert_eq!(metrics.total_types(), 7);
}

#[test]
fn test_total_members() {
    let metrics = BasicMetrics {
        total_methods: 5,
        total_properties: 3,
        total_fields: 2,
        total_events: 1,
        total_constructors: 2,
        ..Default::default()
    };

    assert_eq!(metrics.total_members(), 13);
}

#[test]
fn test_total_control_structures() {
    let metrics = BasicMetrics {
        total_if_statements: 3,
        total_for_loops: 2,
        total_while_loops: 1,
        total_switch_statements: 1,
        total_try_statements: 1,
        total_using_statements: 2,
        ..Default::default()
    };

    assert_eq!(metrics.total_control_structures(), 10);
}

#[test]
fn test_code_density() {
    let metrics = BasicMetrics {
        physical_lines: 100,
        logical_lines: 80,
        ..Default::default()
    };

    assert_eq!(metrics.code_density(), 0.8);

    // Test zero division
    let empty_metrics = BasicMetrics::default();
    assert_eq!(empty_metrics.code_density(), 0.0);
}

#[test]
fn test_comment_ratio() {
    let metrics = BasicMetrics {
        physical_lines: 100,
        comment_lines: 25,
        ..Default::default()
    };

    assert_eq!(metrics.comment_ratio(), 0.25);

    // Test zero division
    let empty_metrics = BasicMetrics::default();
    assert_eq!(empty_metrics.comment_ratio(), 0.0);
}

#[test]
fn test_basic_metrics_collector_new() {
    let collector = BasicMetricsCollector::new();
    assert_eq!(collector.get_metrics().total_classes, 0);
}

#[test]
fn test_collect_from_class() {
    let mut collector = BasicMetricsCollector::new();

    let class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("TestClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: Vec::new(),
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("Method1"),
                type_parameters: None,
                parameters: Vec::new(),
                constraints: Some(Vec::new()),
                body: Some(Statement::Block(Vec::new())),
            }),
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: Vec::new(),
                ty: Type::Primitive(PrimitiveType::Int),
                name: create_test_identifier("field1"),
                initializer: None,
            }),
            ClassBodyDeclaration::Property(PropertyDeclaration {
                attributes: Vec::new(),
                modifiers: Vec::new(),
                ty: Type::Primitive(PrimitiveType::String),
                name: create_test_identifier("Property1"),
                accessors: Vec::new(),
                initializer: None,
            }),
        ],
    };

    collector.collect_from_class(&class);
    let metrics = collector.get_metrics();

    assert_eq!(metrics.total_classes, 1);
    assert_eq!(metrics.total_methods, 1);
    assert_eq!(metrics.total_fields, 1);
    assert_eq!(metrics.total_properties, 1);
}

#[test]
fn test_collect_from_statement() {
    let mut collector = BasicMetricsCollector::new();

    let if_stmt = Statement::If(Box::new(IfStatement {
        condition: bsharp::syntax::nodes::expressions::expression::Expression::Literal(
            bsharp::syntax::nodes::expressions::literal::Literal::Boolean(true),
        ),
        consequence: Box::new(Statement::Block(vec![Statement::For(Box::new(
            ForStatement {
                initializer: None,
                condition: None,
                iterator: vec![],
                body: Box::new(Statement::Block(Vec::new())),
            },
        ))])),
        alternative: None,
    }));

    collector.collect_from_statement(&if_stmt);
    let metrics = collector.get_metrics();

    assert_eq!(metrics.total_if_statements, 1);
    assert_eq!(metrics.total_for_loops, 1);
    assert_eq!(metrics.logical_lines, 3); // if + block + for
}

#[test]
fn test_collector_reset() {
    let mut collector = BasicMetricsCollector::new();

    let class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("TestClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: Vec::new(),
    };

    collector.collect_from_class(&class);
    assert_eq!(collector.get_metrics().total_classes, 1);

    collector.reset();
    assert_eq!(collector.get_metrics().total_classes, 0);
}

#[test]
fn test_collect_various_statement_types() {
    let mut collector = BasicMetricsCollector::new();

    let statements = vec![
        Statement::While(Box::new(WhileStatement {
            condition: Box::new(
                bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                    bsharp::syntax::nodes::expressions::literal::Literal::Boolean(true),
                ),
            ),
            body: Box::new(Statement::Block(Vec::new())),
        })),
        Statement::DoWhile(Box::new(DoWhileStatement {
            body: Box::new(Statement::Block(Vec::new())),
            condition: bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                bsharp::syntax::nodes::expressions::literal::Literal::Boolean(true),
            ),
        })),
        Statement::Switch(Box::new(SwitchStatement {
            expression: bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                bsharp::syntax::nodes::expressions::literal::Literal::Integer(1),
            ),
            sections: vec![SwitchSection {
                labels: vec![SwitchLabel::Case(
                    bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                        bsharp::syntax::nodes::expressions::literal::Literal::Integer(1),
                    ),
                )],
                statements: vec![Statement::Break(BreakStatement)],
            }],
        })),
    ];

    for stmt in statements {
        collector.collect_from_statement(&stmt);
    }

    let metrics = collector.get_metrics();
    assert_eq!(metrics.total_while_loops, 2); // while + do-while
    assert_eq!(metrics.total_switch_statements, 1);
    assert_eq!(metrics.logical_lines, 4);
}

#[test]
fn test_metrics_accuracy_with_real_code() {
    let mut collector = BasicMetricsCollector::new();

    // Simulate a more realistic class structure
    let class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: vec![Modifier::Public],
        name: create_test_identifier("Calculator"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            // Field
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: vec![Modifier::Private],
                ty: Type::Primitive(PrimitiveType::Int),
                name: create_test_identifier("history"),
                initializer: None,
            }),
            // Constructor
            ClassBodyDeclaration::Constructor(ConstructorDeclaration {
                modifiers: vec![Modifier::Public],
                name: create_test_identifier("Calculator"),
                parameters: Vec::new(),
                body: Some(Statement::Block(Vec::new())),
                initializer: None,
            }),
            // Method with complex body
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: vec![Modifier::Public],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("Calculate"),
                type_parameters: None,
                parameters: Vec::new(),
                constraints: Some(Vec::new()),
                body: Some(Statement::Block(vec![Statement::If(Box::new(
                    IfStatement {
                        condition:
                            bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                                bsharp::syntax::nodes::expressions::literal::Literal::Boolean(true),
                            ),
                        consequence: Box::new(Statement::Block(vec![Statement::For(Box::new(
                            ForStatement {
                                initializer: None,
                                condition: None,
                                iterator: vec![],
                                body: Box::new(Statement::Block(vec![Statement::Try(Box::new(
                                    TryStatement {
                                        try_block: Box::new(Statement::Block(Vec::new())),
                                        catches: vec![CatchClause {
                                            exception_type: None,
                                            exception_variable: None,
                                            block: Box::new(Statement::Block(Vec::new())),
                                            when_clause: None,
                                        }],
                                        finally_clause: None,
                                    },
                                ))])),
                            },
                        ))])),
                        alternative: Some(Box::new(Statement::While(Box::new(WhileStatement {
                            condition: Box::new(
                                bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                                    bsharp::syntax::nodes::expressions::literal::Literal::Boolean(
                                        false,
                                    ),
                                ),
                            ),
                            body: Box::new(Statement::Block(Vec::new())),
                        })))),
                    },
                ))])),
            }),
        ],
    };

    collector.collect_from_class(&class);
    let metrics = collector.get_metrics();

    // Verify collected metrics
    assert_eq!(metrics.total_classes, 1);
    assert_eq!(metrics.total_fields, 1);
    assert_eq!(metrics.total_constructors, 1);
    assert_eq!(metrics.total_methods, 1);
    assert_eq!(metrics.total_if_statements, 1);
    assert_eq!(metrics.total_for_loops, 1);
    assert_eq!(metrics.total_while_loops, 1);
    assert_eq!(metrics.total_try_statements, 1);

    // Total counts
    assert_eq!(metrics.total_members(), 3); // 1 field + 1 constructor + 1 method
    assert_eq!(metrics.total_control_structures(), 4); // if + for + while + try
}

#[test]
fn test_collector_with_multiple_classes() {
    let mut collector = BasicMetricsCollector::new();

    // First class
    let class1 = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("Class1"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![ClassBodyDeclaration::Method(MethodDeclaration {
            modifiers: Vec::new(),
            return_type: Type::Primitive(PrimitiveType::Void),
            name: create_test_identifier("Method1"),
            type_parameters: None,
            parameters: Vec::new(),
            constraints: Some(Vec::new()),
            body: None,
        })],
    };

    // Second class
    let class2 = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: Vec::new(),
        name: create_test_identifier("Class2"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: Vec::new(),
                ty: Type::Primitive(PrimitiveType::Int),
                name: create_test_identifier("field1"),
                initializer: None,
            }),
            ClassBodyDeclaration::Property(PropertyDeclaration {
                attributes: Vec::new(),
                modifiers: Vec::new(),
                ty: Type::Primitive(PrimitiveType::Int),
                name: create_test_identifier("Property2"),
                accessors: Vec::new(),
                initializer: None,
            }),
        ],
    };
    collector.collect_from_class(&class1);
    collector.collect_from_class(&class2);

    let metrics = collector.get_metrics();
    assert_eq!(metrics.total_classes, 2);
    assert_eq!(metrics.total_methods, 1);
    assert_eq!(metrics.total_fields, 1);
    assert_eq!(metrics.total_properties, 1);
    assert_eq!(metrics.total_members(), 3);
}

#[test]
fn test_collector_integration() {
    let mut collector = BasicMetricsCollector::new();

    // Create statements with different control structures
    let statements = vec![
        Statement::If(Box::new(IfStatement {
            condition: bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                bsharp::syntax::nodes::expressions::literal::Literal::Boolean(false),
            ),
            consequence: Box::new(Statement::Block(vec![Statement::For(Box::new(
                ForStatement {
                    initializer: None,
                    condition: None,
                    iterator: vec![],
                    body: Box::new(Statement::Block(Vec::new())),
                },
            ))])),
            alternative: None,
        })),
        Statement::While(Box::new(WhileStatement {
            condition: Box::new(
                bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                    bsharp::syntax::nodes::expressions::literal::Literal::Boolean(true),
                ),
            ),
            body: Box::new(Statement::Block(Vec::new())),
        })),
        Statement::DoWhile(Box::new(DoWhileStatement {
            body: Box::new(Statement::Block(Vec::new())),
            condition: bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                bsharp::syntax::nodes::expressions::literal::Literal::Boolean(true),
            ),
        })),
    ];

    for stmt in &statements {
        collector.collect_from_statement(stmt);
    }

    let metrics = collector.get_metrics();
    assert_eq!(metrics.total_if_statements, 1);
    assert_eq!(metrics.total_for_loops, 1);
    assert_eq!(metrics.total_while_loops, 2); // while + do-while
}

fn create_complex_method_for_analysis() -> MethodDeclaration {
    let complex_body = Statement::Block(vec![Statement::If(Box::new(IfStatement {
        condition: bsharp::syntax::nodes::expressions::expression::Expression::Literal(
            bsharp::syntax::nodes::expressions::literal::Literal::Boolean(true),
        ),
        consequence: Box::new(Statement::Block(vec![Statement::For(Box::new(
            ForStatement {
                initializer: None,
                condition: None,
                iterator: vec![],
                body: Box::new(Statement::Block(vec![Statement::Try(Box::new(
                    TryStatement {
                        try_block: Box::new(Statement::Block(Vec::new())),
                        catches: vec![CatchClause {
                            exception_type: None,
                            exception_variable: None,
                            block: Box::new(Statement::Block(Vec::new())),
                            when_clause: None,
                        }],
                        finally_clause: None,
                    },
                ))])),
            },
        ))])),
        alternative: Some(Box::new(Statement::While(Box::new(WhileStatement {
            condition: Box::new(
                bsharp::syntax::nodes::expressions::expression::Expression::Literal(
                    bsharp::syntax::nodes::expressions::literal::Literal::Boolean(false),
                ),
            ),
            body: Box::new(Statement::Block(Vec::new())),
        })))),
    }))]);

    MethodDeclaration {
        modifiers: vec![Modifier::Public],
        return_type: Type::Primitive(PrimitiveType::Void),
        name: create_test_identifier("Calculate"),
        type_parameters: None,
        parameters: Vec::new(),
        constraints: Some(Vec::new()),
        body: Some(complex_body),
    }
}
