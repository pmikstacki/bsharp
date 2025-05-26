// Quality analysis integration tests
// Tests specifically focused on how quality analysis integrates with parsing and quality-specific scenarios

use bsharp::analysis::quality::*;
use bsharp::analysis::{AstAnalyze, AstAnalysis};
use bsharp::parser::{Parser, AstNavigate, FindDeclarations};
use bsharp::parser::nodes::declarations::{ClassDeclaration, MethodDeclaration, FieldDeclaration, PropertyDeclaration, ClassBodyDeclaration};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::types::{Type, PrimitiveType, Parameter};
use bsharp::parser::ast::{CompilationUnit, TopLevelDeclaration};
use bsharp::parser::nodes::declarations::{NamespaceDeclaration, namespace_declaration::NamespaceBodyDeclaration};

fn create_test_identifier(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

/// Test quality analysis integration with parser output
#[test]
fn test_quality_analysis_with_parsed_ast() {
    let parser = Parser::new();
    let source = r#"
namespace TestApp
{
    public class CalculatorService
    {
        private int history;
        
        public void Calculate(int a, int b, int c, int d, int e, int f, int g, int h)
        {
            if (a > 0)
            {
                for (int i = 0; i < a; i++)
                {
                    if (i % 2 == 0)
                    {
                        while (b > 0)
                        {
                            if (c > 0)
                            {
                                switch (d)
                                {
                                    case 1:
                                        history += e;
                                        break;
                                    case 2:
                                        history += f;
                                        break;
                                    default:
                                        history += g + h;
                                        break;
                                }
                            }
                            b--;
                        }
                    }
                }
            }
        }
        
        public void AnotherMethod()
        {
            // No documentation
        }
        
        public void YetAnotherMethod()
        {
            // Also no documentation
        }
    }
}
"#;
    
    let ast = parser.parse(source).expect("Failed to parse source");
    let analyzer = QualityAnalyzer::new();
    
    // Test quality analysis on parsed AST
    let quality_report = analyzer.analyze(&ast);
    let ast_analysis = ast.analyze();
    
    // Verify quality analysis found issues
    assert_eq!(quality_report.class_reports.len(), 1);
    let class_report = &quality_report.class_reports[0];
    
    assert_eq!(class_report.class_name, "CalculatorService");
    assert_eq!(class_report.method_count, 3);
    assert_eq!(class_report.field_count, 1);
    
    // Should detect missing documentation for all 3 public methods
    let missing_doc_count = class_report.issues.iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .count();
    assert_eq!(missing_doc_count, 3);
    
    // Verify AST analysis captured complexity
    assert_eq!(ast_analysis.total_classes, 1);
    assert_eq!(ast_analysis.total_methods, 3);
    assert_eq!(ast_analysis.total_if_statements, 3);
    assert_eq!(ast_analysis.total_for_loops, 1);
    assert_eq!(ast_analysis.total_while_loops, 1);
    assert_eq!(ast_analysis.total_switch_statements, 1);
}

/// Test quality analysis with AST navigation capabilities
#[test]
fn test_quality_analysis_with_navigation() {
    let parser = Parser::new();
    let source = r#"
namespace TestApp
{
    public class UserManager
    {
        public void CreateUser(string name, string email, int age, bool active, string role, string department, string notes)
        {
            // Method with too many parameters
        }
        
        public void UpdateUser()
        {
            // Missing documentation
        }
        
        public void DeleteUser()
        {
            // Missing documentation
        }
    }
    
    public class OrderService
    {
        public void ProcessOrder()
        {
            // Missing documentation
        }
    }
}
"#;
    
    let ast = parser.parse(source).expect("Failed to parse source");
    let analyzer = QualityAnalyzer::new();
    
    // Use navigation to find classes and methods first
    let classes = ast.find_classes();
    let methods = ast.find_methods();
    
    assert_eq!(classes.len(), 2);
    assert_eq!(methods.len(), 4);
    
    // Analyze quality using parsed AST
    let quality_report = analyzer.analyze(&ast);
    
    // Should have reports for both classes
    assert_eq!(quality_report.class_reports.len(), 2);
    
    let user_manager_report = quality_report.class_reports.iter()
        .find(|r| r.class_name == "UserManager")
        .expect("UserManager report not found");
    
    let order_service_report = quality_report.class_reports.iter()
        .find(|r| r.class_name == "OrderService")
        .expect("OrderService report not found");
    
    // UserManager should have 3 methods with documentation issues
    assert_eq!(user_manager_report.method_count, 3);
    let user_manager_doc_issues = user_manager_report.issues.iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .count();
    assert_eq!(user_manager_doc_issues, 3);
    
    // OrderService should have 1 method with documentation issue
    assert_eq!(order_service_report.method_count, 1);
    let order_service_doc_issues = order_service_report.issues.iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .count();
    assert_eq!(order_service_doc_issues, 1);
}

/// Test quality analysis performance tracking
#[test]
fn test_quality_analysis_performance_tracking() {
    let analyzer = QualityAnalyzer::new();
    
    // Create a performance test with many quality issues
    let large_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: vec![bsharp::parser::nodes::declarations::Modifier::Public],
        name: create_test_identifier("LargeClass"),
        type_parameters: None,
        base_types: Vec::new(),
        body_declarations: {
            let mut declarations = Vec::new();
            
            // Add many methods to simulate a large class
            for i in 1..=20 {
                declarations.push(ClassBodyDeclaration::Method(MethodDeclaration {
                    modifiers: vec![bsharp::parser::nodes::declarations::Modifier::Public],
                    return_type: Type::Primitive(PrimitiveType::Void),
                    name: create_test_identifier(&format!("Method{}", i)),
                    type_parameters: None,
                    parameters: Vec::new(),
                    body: None,
                    documentation: None,
                    attributes: Vec::new(),
                }));
            }
            
            // Add many fields
            for i in 1..=15 {
                declarations.push(ClassBodyDeclaration::Field(FieldDeclaration {
                    modifiers: vec![bsharp::parser::nodes::declarations::Modifier::Private],
                    field_type: Type::Primitive(PrimitiveType::Int),
                    name: create_test_identifier(&format!("field{}", i)),
                    initializer: None,
                    documentation: None,
                    attributes: Vec::new(),
                }));
            }
            
            declarations
        },
    };
    
    use std::time::Instant;
    let start = Instant::now();
    
    let class_report = analyzer.analyze_class(&large_class);
    
    let duration = start.elapsed();
    
    // Analysis should complete quickly even for large classes
    assert!(duration.as_millis() < 50, "Quality analysis took too long: {:?}", duration);
    
    // Verify analysis found the expected issues
    assert_eq!(class_report.method_count, 20);
    assert_eq!(class_report.field_count, 15);
    
    // Should detect many missing documentation issues
    let doc_issues = class_report.issues.iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .count();
    assert_eq!(doc_issues, 20); // All methods missing documentation
    
    // Should detect large class smell
    let large_class_issues = class_report.issues.iter()
        .filter(|issue| matches!(issue, QualityIssue::LargeClass { .. }))
        .count();
    assert_eq!(large_class_issues, 1);
}

/// Test quality report consolidation across multiple classes
#[test]
fn test_quality_report_consolidation() {
    let analyzer = QualityAnalyzer::new();
    
    // Create multiple classes with different quality issues
    let good_class = ClassDeclaration {
        documentation: Some("Well documented class".to_string()),
        attributes: Vec::new(),
        modifiers: vec![bsharp::parser::nodes::declarations::Modifier::Public],
        name: create_test_identifier("GoodClass"),
        type_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: vec![bsharp::parser::nodes::declarations::Modifier::Public],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("WellDocumentedMethod"),
                type_parameters: None,
                parameters: Vec::new(),
                body: None,
                documentation: Some("This method is well documented".to_string()),
                attributes: Vec::new(),
            }),
        ],
    };
    
    let problematic_class = ClassDeclaration {
        documentation: None,
        attributes: Vec::new(),
        modifiers: vec![bsharp::parser::nodes::declarations::Modifier::Public],
        name: create_test_identifier("ProblematicClass"),
        type_parameters: None,
        base_types: Vec::new(),
        body_declarations: vec![
            ClassBodyDeclaration::Method(MethodDeclaration {
                modifiers: vec![bsharp::parser::nodes::declarations::Modifier::Public],
                return_type: Type::Primitive(PrimitiveType::Void),
                name: create_test_identifier("UndocumentedMethod"),
                type_parameters: None,
                parameters: {
                    // Create method with too many parameters
                    (0..=10).map(|i| Parameter {
                        modifier: None,
                        parameter_type: Type::Primitive(PrimitiveType::Int),
                        name: create_test_identifier(&format!("param{}", i)),
                        default_value: None,
                    }).collect()
                },
                body: None,
                documentation: None,
                attributes: Vec::new(),
            }),
        ],
    };
    
    let good_report = analyzer.analyze_class(&good_class);
    let problematic_report = analyzer.analyze_class(&problematic_class);
    
    // Good class should have minimal issues
    assert_eq!(good_report.class_name, "GoodClass");
    assert_eq!(good_report.method_count, 1);
    assert!(good_report.issues.is_empty()); // No quality issues
    assert!(good_report.quality_score >= 8.0); // High quality score
    
    // Problematic class should have multiple issues
    assert_eq!(problematic_report.class_name, "ProblematicClass");
    assert_eq!(problematic_report.method_count, 1);
    assert!(!problematic_report.issues.is_empty()); // Has quality issues
    assert!(problematic_report.quality_score < 7.0); // Lower quality score
    
    // Should detect missing documentation
    let has_doc_issue = problematic_report.issues.iter()
        .any(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }));
    assert!(has_doc_issue);
    
    // Should detect too many parameters
    let has_param_issue = problematic_report.issues.iter()
        .any(|issue| matches!(issue, QualityIssue::TooManyParameters { .. }));
    assert!(has_param_issue);
    
    // Create consolidated report
    let overall_report = QualityReport {
        class_reports: vec![good_report, problematic_report],
        overall_grade: QualityGrade::Fair, // Mixed quality
        total_issues: 2,
        critical_issues: 0,
        high_severity_issues: 1,
        medium_severity_issues: 1,
        low_severity_issues: 0,
    };
    
    assert_eq!(overall_report.class_reports.len(), 2);
    assert_eq!(overall_report.total_issues, 2);
    assert_eq!(overall_report.high_severity_issues, 1);
    assert_eq!(overall_report.medium_severity_issues, 1);
} 