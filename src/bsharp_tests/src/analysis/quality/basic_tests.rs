use analysis::quality::*;
use syntax::ast::{CompilationUnit, TopLevelDeclaration};
use syntax::nodes::declarations::{
    ClassBodyDeclaration, ClassDeclaration, FieldDeclaration, MethodDeclaration,
    PropertyDeclaration,
};
use syntax::nodes::declarations::{
    NamespaceDeclaration, namespace_declaration::NamespaceBodyDeclaration,
};
use syntax::nodes::identifier::Identifier;
use syntax::nodes::types::{PrimitiveType, Type};

fn create_test_identifier(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

#[test]
fn test_quality_analyzer_new() {
    let analyzer = QualityAnalyzer::new();
    // Test that the analyzer is created successfully
    assert_eq!(analyzer, QualityAnalyzer::new());
}

#[test]
fn test_quality_report_new() {
    let report = QualityReport::new();
    assert_eq!(report.overall_score, 0.0);
    assert_eq!(report.grade, QualityGrade::default());
    assert!(report.issues.is_empty());
    assert!(report.class_reports.is_empty());
}

#[test]
fn test_quality_issue_types() {
    // Test different quality issue variants
    let high_complexity_issue = QualityIssue::HighComplexity {
        method_name: "ComplexMethod".to_string(),
        complexity: 15,
    };

    let long_method_issue = QualityIssue::LongMethod {
        method_name: "VeryLongMethod".to_string(),
        line_count: 80,
        threshold: 50,
    };

    let missing_doc_issue = QualityIssue::MissingDocumentation {
        member_name: "PublicMethod".to_string(),
        member_type: "Method".to_string(),
    };

    // Test that issues are created properly
    match high_complexity_issue {
        QualityIssue::HighComplexity {
            method_name,
            complexity,
        } => {
            assert_eq!(method_name, "ComplexMethod");
            assert_eq!(complexity, 15);
        }
        _ => panic!("Wrong issue type"),
    }

    match long_method_issue {
        QualityIssue::LongMethod {
            method_name,
            line_count,
            threshold,
        } => {
            assert_eq!(method_name, "VeryLongMethod");
            assert_eq!(line_count, 80);
            assert_eq!(threshold, 50);
        }
        _ => panic!("Wrong issue type"),
    }

    match missing_doc_issue {
        QualityIssue::MissingDocumentation {
            member_name,
            member_type,
        } => {
            assert_eq!(member_name, "PublicMethod");
            assert_eq!(member_type, "Method");
        }
        _ => panic!("Wrong issue type"),
    }
}

#[test]
fn test_quality_grade_classification() {
    // Test grade determination based on scores
    let excellent_metrics = QualityMetrics {
        maintainability_index: 95.0,
        code_coverage: 90.0,
        technical_debt_ratio: 5.0,
        duplication_percentage: 2.0,
        test_coverage: 85.0,
    };

    let poor_metrics = QualityMetrics {
        maintainability_index: 30.0,
        code_coverage: 40.0,
        technical_debt_ratio: 60.0,
        duplication_percentage: 25.0,
        test_coverage: 35.0,
    };

    assert_eq!(excellent_metrics.quality_grade(), QualityGrade::A);
    assert_eq!(poor_metrics.quality_grade(), QualityGrade::F);
}

#[test]
fn test_quality_severity_ordering() {
    assert!(QualitySeverity::Critical as u8 > QualitySeverity::Major as u8);
    assert!(QualitySeverity::Major as u8 > QualitySeverity::Minor as u8);
    assert!(QualitySeverity::Minor as u8 > QualitySeverity::Info as u8);
}

#[test]
fn test_analyze_simple_class() {
    let analyzer = QualityAnalyzer::new();

    // Create a simple test class
    let class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: vec![syntax::nodes::declarations::Modifier::Public],
        name: create_test_identifier("TestClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: vec![syntax::nodes::declarations::Modifier::Public],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("TestMethod"),
                type_parameters: None,
                parameters: Vec::new(),
                constraints: None,
                body: None,
            }),
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: vec![syntax::nodes::declarations::Modifier::Private],
                field_type: Type::Primitive(PrimitiveType::Int),
                name: create_test_identifier("testField"),
                initializer: None,
            }),
        ],
    };

    // Create a minimal compilation unit
    let namespace = NamespaceDeclaration {
        name: create_test_identifier("TestNamespace"),
        using_directives: Vec::new(),
        declarations: vec![NamespaceBodyDeclaration::Class(class)],
    };

    let compilation_unit = CompilationUnit {
        global_attributes: Vec::new(),
        using_directives: Vec::new(),
        declarations: vec![TopLevelDeclaration::Namespace(namespace)],
        file_scoped_namespace: None,
        top_level_statements: Vec::new(),
    };

    let report = analyzer.analyze(&compilation_unit);

    // Verify basic analysis
    assert_eq!(report.class_reports.len(), 1);
    assert_eq!(report.class_reports[0].class_name, "TestClass");
    assert_eq!(report.class_reports[0].method_count, 1);
    assert_eq!(report.class_reports[0].field_count, 1);
    assert_eq!(report.class_reports[0].property_count, 0);

    // Should detect missing documentation issue for public method
    assert!(!report.class_reports[0].issues.is_empty());
    let has_missing_doc = report.class_reports[0]
        .issues
        .iter()
        .any(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }));
    assert!(has_missing_doc);
}

#[test]
fn test_analyze_complex_class() {
    let analyzer = QualityAnalyzer::new();

    // Create a class with multiple quality issues
    let class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: vec![syntax::nodes::declarations::Modifier::Public],
        name: create_test_identifier("ComplexClass"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            // Multiple public methods without documentation
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: vec![syntax::nodes::declarations::Modifier::Public],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("Method1"),
                type_parameters: None,
                parameters: Vec::new(),
                constraints: None,
                body: None,
            }),
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: vec![syntax::nodes::declarations::Modifier::Public],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("Method2"),
                type_parameters: None,
                parameters: Vec::new(),
                constraints: None,
                body: None,
            }),
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: vec![syntax::nodes::declarations::Modifier::Public],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("Method3"),
                type_parameters: None,
                parameters: Vec::new(),
                constraints: None,
                body: None,
            }),
            // Multiple fields
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: vec![syntax::nodes::declarations::Modifier::Private],
                field_type: Type::Primitive(PrimitiveType::Int),
                name: create_test_identifier("field1"),
                initializer: None,
            }),
            ClassBodyDeclaration::Field(FieldDeclaration {
                modifiers: vec![syntax::nodes::declarations::Modifier::Private],
                field_type: Type::Primitive(PrimitiveType::String),
                name: create_test_identifier("field2"),
                initializer: None,
            }),
            // Properties
            ClassBodyDeclaration::Property(PropertyDeclaration {
                attributes: Vec::new(),
                modifiers: vec![syntax::nodes::declarations::Modifier::Public],
                property_type: Type::Primitive(PrimitiveType::String),
                name: create_test_identifier("Property1"),
                accessors: Vec::new(),
                initializer: None,
            }),
            ClassBodyDeclaration::Property(PropertyDeclaration {
                attributes: Vec::new(),
                modifiers: vec![syntax::nodes::declarations::Modifier::Public],
                property_type: Type::Primitive(PrimitiveType::Int),
                name: create_test_identifier("Property2"),
                accessors: Vec::new(),
                initializer: None,
            }),
        ],
    };

    let namespace = NamespaceDeclaration {
        name: create_test_identifier("TestNamespace"),
        using_directives: Vec::new(),
        declarations: vec![NamespaceBodyDeclaration::Class(class)],
    };

    let compilation_unit = CompilationUnit {
        global_attributes: Vec::new(),
        using_directives: Vec::new(),
        declarations: vec![TopLevelDeclaration::Namespace(namespace)],
        file_scoped_namespace: None,
        top_level_statements: Vec::new(),
    };

    let report = analyzer.analyze(&compilation_unit);

    // Verify complex analysis
    assert_eq!(report.class_reports.len(), 1);
    let class_report = &report.class_reports[0];

    assert_eq!(class_report.class_name, "ComplexClass");
    assert_eq!(class_report.method_count, 3);
    assert_eq!(class_report.field_count, 2);
    assert_eq!(class_report.property_count, 2);

    // Should have multiple missing documentation issues
    let missing_doc_count = class_report
        .issues
        .iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .count();
    assert_eq!(missing_doc_count, 3); // 3 public methods without documentation
}

#[test]
fn test_quality_report_add_issue() {
    let mut report = QualityReport::new();

    let issue = QualityIssue::HighComplexity {
        method_name: "ComplexMethod".to_string(),
        complexity: 20,
    };

    report.add_issue(issue);

    assert_eq!(report.issues.len(), 1);
    match &report.issues[0] {
        QualityIssue::HighComplexity {
            method_name,
            complexity,
        } => {
            assert_eq!(method_name, "ComplexMethod");
            assert_eq!(*complexity, 20);
        }
        _ => panic!("Wrong issue type"),
    }
}

#[test]
fn test_quality_report_overall_score_calculation() {
    let mut report = QualityReport::new();

    // Add some quality issues
    report.add_issue(QualityIssue::HighComplexity {
        method_name: "Method1".to_string(),
        complexity: 15,
    });

    report.add_issue(QualityIssue::MissingDocumentation {
        member_name: "Method2".to_string(),
        member_type: "Method".to_string(),
    });

    report.calculate_overall_score();

    // Score should be calculated based on issues
    assert!(report.overall_score <= 100.0);
    assert!(report.overall_score >= 0.0);

    // Grade should be assigned based on score
    assert!(matches!(
        report.grade,
        QualityGrade::A | QualityGrade::B | QualityGrade::C | QualityGrade::D | QualityGrade::F
    ));
}

#[test]
fn test_class_quality_report_score_calculation() {
    let mut class_report = ClassQualityReport {
        class_name: "TestClass".to_string(),
        method_count: 5,
        field_count: 3,
        property_count: 2,
        cyclomatic_complexity: 15,
        lines_of_code: 100,
        issues: vec![
            QualityIssue::HighComplexity {
                method_name: "ComplexMethod".to_string(),
                complexity: 12,
            },
            QualityIssue::MissingDocumentation {
                member_name: "PublicMethod".to_string(),
                member_type: "Method".to_string(),
            },
        ],
        quality_score: 0.0, // Will be calculated
    };

    class_report.calculate_score();

    // Score should be reduced based on issues
    assert!(class_report.quality_score < 100.0);
    assert!(class_report.quality_score >= 0.0);
}

#[test]
fn test_empty_compilation_unit_analysis() {
    let analyzer = QualityAnalyzer::new();

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
}

#[test]
fn test_quality_metrics_default() {
    let metrics = QualityMetrics::new();

    assert_eq!(metrics.maintainability_index, 0.0);
    assert_eq!(metrics.code_coverage, 0.0);
    assert_eq!(metrics.technical_debt_ratio, 0.0);
    assert_eq!(metrics.duplication_percentage, 0.0);
    assert_eq!(metrics.test_coverage, 0.0);
}

#[test]
fn test_all_quality_issue_variants() {
    // Test all quality issue variants can be created
    let issues = vec![
        QualityIssue::HighComplexity {
            method_name: "Method1".to_string(),
            complexity: 20,
        },
        QualityIssue::LongMethod {
            method_name: "Method2".to_string(),
            line_count: 100,
            threshold: 50,
        },
        QualityIssue::LargeClass {
            class_name: "BigClass".to_string(),
            method_count: 30,
        },
        QualityIssue::TooManyParameters {
            method_name: "Method3".to_string(),
            parameter_count: 8,
        },
        QualityIssue::MissingDocumentation {
            member_name: "Method4".to_string(),
            member_type: "Method".to_string(),
        },
        QualityIssue::DeepInheritance {
            class_name: "DerivedClass".to_string(),
            depth: 6,
        },
        QualityIssue::UnusedCode {
            member_name: "UnusedMethod".to_string(),
            member_type: "Method".to_string(),
        },
        QualityIssue::DuplicateCode {
            location1: "Class1.Method1".to_string(),
            location2: "Class2.Method2".to_string(),
            similarity: 0.95,
        },
        QualityIssue::StringConcatenationInLoop {
            method_name: "BadMethod".to_string(),
            location: "line 15".to_string(),
        },
        QualityIssue::InappropriateIntimacy {
            class1: "ClassA".to_string(),
            class2: "ClassB".to_string(),
        },
        QualityIssue::GodClass {
            class_name: "MegaClass".to_string(),
            responsibility_count: 15,
        },
        QualityIssue::FeatureEnvy {
            method_name: "EnviousMethod".to_string(),
            target_class: "TargetClass".to_string(),
        },
    ];

    // All issues should be created successfully
    assert_eq!(issues.len(), 12);

    // Test serialization/deserialization
    for issue in issues {
        let serialized = serde_json::to_string(&issue).expect("Failed to serialize issue");
        let _deserialized: QualityIssue =
            serde_json::from_str(&serialized).expect("Failed to deserialize issue");
    }
}
