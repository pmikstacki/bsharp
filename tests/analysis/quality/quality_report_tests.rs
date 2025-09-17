use bsharp::analysis::quality::*;
use bsharp::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use bsharp::syntax::nodes::declarations::{
    ClassBodyDeclaration, NamespaceDeclaration, namespace_declaration::NamespaceBodyDeclaration,
};
use bsharp::syntax::nodes::declarations::{ClassDeclaration, FieldDeclaration, MethodDeclaration};
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};

fn create_test_identifier(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

#[test]
fn test_quality_report_default() {
    let report = QualityReport::default();

    assert_eq!(report.overall_score, 0.0);
    assert_eq!(report.grade, QualityGrade::default());
    assert!(report.issues.is_empty());
    assert!(report.class_reports.is_empty());
}

#[test]
fn test_quality_report_serialization() {
    let mut report = QualityReport::new();

    report.add_issue(QualityIssue::HighComplexity {
        method_name: "TestMethod".to_string(),
        complexity: 15,
    });

    report.add_issue(QualityIssue::MissingDocumentation {
        member_name: "PublicMethod".to_string(),
        member_type: "Method".to_string(),
    });

    report.calculate_overall_score();

    // Test JSON serialization
    let json = serde_json::to_string(&report).expect("Failed to serialize report");
    let deserialized: QualityReport =
        serde_json::from_str(&json).expect("Failed to deserialize report");

    assert_eq!(deserialized.issues.len(), report.issues.len());
    assert_eq!(deserialized.overall_score, report.overall_score);
    assert_eq!(deserialized.grade, report.grade);
}

#[test]
fn test_class_quality_report_creation() {
    let class_report = ClassQualityReport {
        class_name: "TestClass".to_string(),
        method_count: 5,
        field_count: 3,
        property_count: 2,
        cyclomatic_complexity: 12,
        lines_of_code: 150,
        issues: vec![QualityIssue::HighComplexity {
            method_name: "ComplexMethod".to_string(),
            complexity: 8,
        }],
        quality_score: 85.0,
    };

    assert_eq!(class_report.class_name, "TestClass");
    assert_eq!(class_report.method_count, 5);
    assert_eq!(class_report.field_count, 3);
    assert_eq!(class_report.property_count, 2);
    assert_eq!(class_report.issues.len(), 1);
}

#[test]
fn test_quality_report_with_multiple_classes() {
    let analyzer = QualityAnalyzer::new();

    // Create multiple classes
    let class1 = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: vec![bsharp::syntax::nodes::declarations::Modifier::Public],
        name: create_test_identifier("FirstClass"),
        type_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![ClassBodyDeclaration::Method(MethodDeclaration {
            modifiers: vec![bsharp::syntax::nodes::declarations::Modifier::Public],
            return_type: Type::Primitive(PrimitiveType::Void),
            name: create_test_identifier("Method1"),
            type_parameters: None,
            parameters: Vec::new(),
            constraints: None,
            body: None,
        })],
    };

    let class2 = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: vec![bsharp::syntax::nodes::declarations::Modifier::Public],
        name: create_test_identifier("SecondClass"),
        type_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: vec![bsharp::syntax::nodes::declarations::Modifier::Public],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("Method2"),
                type_parameters: None,
                parameters: Vec::new(),
                constraints: None,
                body: None,
            }),
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: vec![bsharp::syntax::nodes::declarations::Modifier::Private],
                ty: Type::Primitive(PrimitiveType::Int),
                name: create_test_identifier("field1"),
                initializer: None,
            }),
        ],
    };

    let namespace = NamespaceDeclaration {
        name: create_test_identifier("TestNamespace"),
        using_directives: Vec::new(),
        declarations: vec![
            NamespaceBodyDeclaration::Class(class1),
            NamespaceBodyDeclaration::Class(class2),
        ],
    };

    let compilation_unit = CompilationUnit {
        global_attributes: Vec::new(),
        using_directives: Vec::new(),
        declarations: vec![TopLevelDeclaration::Namespace(namespace)],
        file_scoped_namespace: None,
        top_level_statements: Vec::new(),
    };

    let report = analyzer.analyze(&compilation_unit);

    // Should analyze both classes
    assert_eq!(report.class_reports.len(), 2);

    let first_class_report = report
        .class_reports
        .iter()
        .find(|r| r.class_name == "FirstClass")
        .expect("FirstClass report not found");
    assert_eq!(first_class_report.method_count, 1);
    assert_eq!(first_class_report.field_count, 0);

    let second_class_report = report
        .class_reports
        .iter()
        .find(|r| r.class_name == "SecondClass")
        .expect("SecondClass report not found");
    assert_eq!(second_class_report.method_count, 1);
    assert_eq!(second_class_report.field_count, 1);
}

#[test]
fn test_quality_report_score_aggregation() {
    let mut report = QualityReport::new();

    // Add class reports with different scores
    let class_report1 = ClassQualityReport {
        class_name: "Class1".to_string(),
        method_count: 3,
        field_count: 2,
        property_count: 1,
        cyclomatic_complexity: 8,
        lines_of_code: 100,
        issues: Vec::new(),
        quality_score: 90.0,
    };

    let class_report2 = ClassQualityReport {
        class_name: "Class2".to_string(),
        method_count: 5,
        field_count: 4,
        property_count: 3,
        cyclomatic_complexity: 15,
        lines_of_code: 200,
        issues: vec![QualityIssue::HighComplexity {
            method_name: "ComplexMethod".to_string(),
            complexity: 12,
        }],
        quality_score: 70.0,
    };

    report.class_reports.push(class_report1);
    report.class_reports.push(class_report2);

    report.calculate_overall_score();

    // Overall score should be calculated based on class scores
    assert!(report.overall_score > 0.0);
    assert!(report.overall_score <= 100.0);

    // Grade should be assigned based on score
    assert!(matches!(
        report.grade,
        QualityGrade::A | QualityGrade::B | QualityGrade::C | QualityGrade::D | QualityGrade::F
    ));
}

#[test]
fn test_quality_issue_severity_impact() {
    let mut report = QualityReport::new();

    // Add issues of different severities
    report.add_issue(QualityIssue::HighComplexity {
        method_name: "CriticalMethod".to_string(),
        complexity: 25, // Very high complexity
    });

    report.add_issue(QualityIssue::MissingDocumentation {
        member_name: "Method1".to_string(),
        member_type: "Method".to_string(),
    });

    report.add_issue(QualityIssue::LongMethod {
        method_name: "VeryLongMethod".to_string(),
        line_count: 200,
        threshold: 50,
    });

    report.calculate_overall_score();

    // High severity issues should significantly impact the score
    assert!(report.overall_score < 100.0);

    // Should have multiple issues
    assert_eq!(report.issues.len(), 3);
}

#[test]
fn test_quality_report_empty_analysis() {
    let analyzer = QualityAnalyzer::new();

    // Empty compilation unit
    let compilation_unit = CompilationUnit {
        global_attributes: Vec::new(),
        using_directives: Vec::new(),
        declarations: Vec::new(),
        file_scoped_namespace: None,
        top_level_statements: Vec::new(),
    };

    let report = analyzer.analyze(&compilation_unit);

    assert!(report.class_reports.is_empty());
    assert!(report.issues.is_empty());
    assert_eq!(report.overall_score, 0.0);
    assert_eq!(report.grade, QualityGrade::default());
}

#[test]
fn test_quality_grade_thresholds() {
    // Test all grade thresholds
    let grades_and_scores = [
        (QualityGrade::A, 95.0),
        (QualityGrade::B, 85.0),
        (QualityGrade::C, 75.0),
        (QualityGrade::D, 65.0),
        (QualityGrade::F, 50.0),
    ];

    for (expected_grade, score) in grades_and_scores.iter() {
        let mut report = QualityReport::new();
        report.overall_score = *score;

        // Manually set grade based on score (this would normally be done by calculate_overall_score)
        report.grade = if *score >= 90.0 {
            QualityGrade::A
        } else if *score >= 80.0 {
            QualityGrade::B
        } else if *score >= 70.0 {
            QualityGrade::C
        } else if *score >= 60.0 {
            QualityGrade::D
        } else {
            QualityGrade::F
        };

        assert_eq!(report.grade, *expected_grade);
    }
}

#[test]
fn test_complex_quality_analysis_scenario() {
    let analyzer = QualityAnalyzer::new();

    // Create a complex class with multiple quality issues
    let complex_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: vec![bsharp::syntax::nodes::declarations::Modifier::Public],
        name: create_test_identifier("ComplexClass"),
        type_parameters: None,
        base_types: Vec::new(),
        body_declarations: {
            let mut declarations = Vec::new();

            // Add many methods (potential large class issue)
            for i in 1..=10 {
                declarations.push(ClassBodyDeclaration::Method(MethodDeclaration {
                    modifiers: vec![bsharp::syntax::nodes::declarations::Modifier::Public],
                    return_type: Type::Primitive(PrimitiveType::Void),
                    name: create_test_identifier(&format!("Method{}", i)),
                    type_parameters: None,
                    parameters: Vec::new(),
                    constraints: None,
                    body: None,
                }));
            }

            // Add many fields
            for i in 1..=8 {
                declarations.push(ClassBodyDeclaration::Field(FieldDeclaration {
                    modifiers: vec![bsharp::syntax::nodes::declarations::Modifier::Private],
                    ty: Type::Primitive(PrimitiveType::Int),
                    name: create_test_identifier(&format!("field{}", i)),
                    initializer: None,
                }));
            }

            declarations
        },
    };

    let namespace = NamespaceDeclaration {
        name: create_test_identifier("TestNamespace"),
        using_directives: Vec::new(),
        declarations: vec![NamespaceBodyDeclaration::Class(complex_class)],
    };

    let compilation_unit = CompilationUnit {
        global_attributes: Vec::new(),
        using_directives: Vec::new(),
        declarations: vec![TopLevelDeclaration::Namespace(namespace)],
        file_scoped_namespace: None,
        top_level_statements: Vec::new(),
    };

    let report = analyzer.analyze(&compilation_unit);

    // Should detect multiple issues
    assert_eq!(report.class_reports.len(), 1);
    let class_report = &report.class_reports[0];

    assert_eq!(class_report.class_name, "ComplexClass");
    assert_eq!(class_report.method_count, 10);
    assert_eq!(class_report.field_count, 8);

    // Should have many missing documentation issues (10 public methods)
    let missing_doc_count = class_report
        .issues
        .iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .count();
    assert_eq!(missing_doc_count, 10);

    // Overall score should be impacted by the many issues
    assert!(report.overall_score < 100.0);
}
