#[cfg(test)]
mod tests {
    use super::super::basic::*;
    use crate::parser::ast::*;
    use crate::parser::nodes::declarations::*;
    use crate::parser::nodes::statements::statement::*;
    use crate::parser::nodes::identifiers::Identifier;

    fn create_test_identifier(name: &str) -> Identifier {
        Identifier {
            name: name.to_string(),
            span: None,
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
            base_list: None,
            constraints: Vec::new(),
            body_declarations: vec![
                ClassBodyDeclaration::Method(MethodDeclaration {
                    documentation: None,
                    attributes: Vec::new(),
                    modifiers: Vec::new(),
                    return_type: None,
                    name: create_test_identifier("Method1"),
                    type_parameters: None,
                    parameters: Vec::new(),
                    constraints: Vec::new(),
                    body: Some(Statement::Block(Vec::new())),
                    expression_body: None,
                    span: None,
                }),
                ClassBodyDeclaration::Field(FieldDeclaration {
                    documentation: None,
                    attributes: Vec::new(),
                    modifiers: Vec::new(),
                    field_type: None,
                    name: create_test_identifier("field1"),
                    initializer: None,
                    span: None,
                }),
                ClassBodyDeclaration::Property(PropertyDeclaration {
                    documentation: None,
                    attributes: Vec::new(),
                    modifiers: Vec::new(),
                    property_type: None,
                    name: create_test_identifier("Property1"),
                    accessors: None,
                    initializer: None,
                    span: None,
                }),
            ],
            span: None,
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
        
        let if_stmt = Statement::If(IfStatement {
            condition: crate::parser::nodes::expressions::expression::Expression::Literal(
                crate::parser::nodes::literals::Literal::Boolean(true)
            ),
            consequence: Box::new(Statement::Block(vec![
                Statement::For(ForStatement {
                    initializer: None,
                    condition: None,
                    iterator: None,
                    body: Box::new(Statement::Block(Vec::new())),
                    span: None,
                })
            ])),
            alternative: None,
            span: None,
        });

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
            base_list: None,
            constraints: Vec::new(),
            body_declarations: Vec::new(),
            span: None,
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
            Statement::While(WhileStatement {
                condition: crate::parser::nodes::expressions::expression::Expression::Literal(
                    crate::parser::nodes::literals::Literal::Boolean(true)
                ),
                body: Box::new(Statement::Block(Vec::new())),
                span: None,
            }),
            Statement::DoWhile(DoWhileStatement {
                body: Box::new(Statement::Block(Vec::new())),
                condition: crate::parser::nodes::expressions::expression::Expression::Literal(
                    crate::parser::nodes::literals::Literal::Boolean(true)
                ),
                span: None,
            }),
            Statement::Switch(SwitchStatement {
                expression: crate::parser::nodes::expressions::expression::Expression::Literal(
                    crate::parser::nodes::literals::Literal::Integer(1)
                ),
                sections: vec![
                    SwitchSection {
                        labels: vec![SwitchLabel::Case(crate::parser::nodes::expressions::expression::Expression::Literal(
                            crate::parser::nodes::literals::Literal::Integer(1)
                        ))],
                        statements: vec![Statement::Break(None)],
                        span: None,
                    }
                ],
                span: None,
            }),
        ];

        for stmt in statements {
            collector.collect_from_statement(&stmt);
        }

        let metrics = collector.get_metrics();
        assert_eq!(metrics.total_while_loops, 2); // while + do-while
        assert_eq!(metrics.total_switch_statements, 1);
        assert_eq!(metrics.logical_lines, 3);
    }
} 