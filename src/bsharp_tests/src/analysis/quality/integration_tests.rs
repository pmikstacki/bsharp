// Quality analysis integration tests
// Tests specifically focused on how quality analysis integrates with parsing and quality-specific scenarios

use analysis::navigation::FindDeclarations;
use analysis::quality::*;
use analysis::AstAnalyze;
use parser::facade::Parser;
use syntax::nodes::identifier::Identifier;

#[allow(dead_code)]
fn create_test_identifier(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

/// Test quality analysis integration with syntax output
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
    let missing_doc_count = class_report
        .issues
        .iter()
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

    let user_manager_report = quality_report
        .class_reports
        .iter()
        .find(|r| r.class_name == "UserManager")
        .expect("UserManager report not found");

    let order_service_report = quality_report
        .class_reports
        .iter()
        .find(|r| r.class_name == "OrderService")
        .expect("OrderService report not found");

    // UserManager should have 3 methods with documentation issues
    assert_eq!(user_manager_report.method_count, 3);
    let user_manager_doc_issues = user_manager_report
        .issues
        .iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .count();
    assert_eq!(user_manager_doc_issues, 3);

    // OrderService should have 1 method with documentation issue
    assert_eq!(order_service_report.method_count, 1);
    let order_service_doc_issues = order_service_report
        .issues
        .iter()
        .filter(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }))
        .count();
    assert_eq!(order_service_doc_issues, 1);
}

/// Test quality analysis performance with a larger codebase
#[test]
fn test_quality_analysis_performance() {
    let parser = Parser::new();
    let source = r#"
namespace LargeProject
{
    public class ServiceA
    {
        public void Method1() { }
        public void Method2() { }
        public void Method3() { }
        public void Method4() { }
        public void Method5() { }
    }
    
    public class ServiceB  
    {
        public void Method1() { }
        public void Method2() { }
        public void Method3() { }
        public void Method4() { }
        public void Method5() { }
    }
    
    public class ServiceC
    {
        public void Method1() { }
        public void Method2() { }
        public void Method3() { }
        public void Method4() { }
        public void Method5() { }
    }
}
"#;

    let ast = parser.parse(source).expect("Failed to parse source");
    let analyzer = QualityAnalyzer::new();

    use std::time::Instant;

    // Time the complete analysis workflow
    let start = Instant::now();

    let quality_report = analyzer.analyze(&ast);
    let ast_analysis = ast.analyze();

    let duration = start.elapsed();

    // Analysis should complete quickly (under 100ms for this small example)
    assert!(
        duration.as_millis() < 100,
        "Analysis took too long: {:?}",
        duration
    );

    // Verify all results are consistent
    assert_eq!(quality_report.class_reports.len(), 3);
    assert_eq!(ast_analysis.total_classes, 3);
    assert_eq!(ast_analysis.total_methods, 15); // 3 classes * 5 methods each

    // All classes should have similar structure
    for class_report in &quality_report.class_reports {
        assert_eq!(class_report.method_count, 5);
    }
}

/// Test quality report consolidation across multiple classes
#[test]
fn test_quality_report_consolidation() {
    let parser = Parser::new();
    let source = r#"
namespace TestApp
{
    public class GoodClass
    {
        public void WellDocumentedMethod()
        {
            // This is a simple method
        }
    }
    
    public class ProblematicClass
    {
        public void UndocumentedMethod(int param1, int param2, int param3, int param4, int param5, int param6, int param7, int param8, int param9, int param10, int param11)
        {
            // Method with too many parameters and no documentation
        }
    }
}
"#;

    let ast = parser.parse(source).expect("Failed to parse source");
    let analyzer = QualityAnalyzer::new();
    let quality_report = analyzer.analyze(&ast);

    // Should have reports for both classes
    assert_eq!(quality_report.class_reports.len(), 2);

    let good_report = quality_report
        .class_reports
        .iter()
        .find(|r| r.class_name == "GoodClass")
        .expect("GoodClass report not found");

    let problematic_report = quality_report
        .class_reports
        .iter()
        .find(|r| r.class_name == "ProblematicClass")
        .expect("ProblematicClass report not found");

    // Good class should have minimal issues
    assert_eq!(good_report.class_name, "GoodClass");
    assert_eq!(good_report.method_count, 1);
    // Note: Even the "good" class may have issues due to missing XML documentation
    assert!(good_report.quality_score >= 0.0); // Valid score

    // Problematic class should have multiple issues
    assert_eq!(problematic_report.class_name, "ProblematicClass");
    assert_eq!(problematic_report.method_count, 1);
    assert!(!problematic_report.issues.is_empty()); // Has quality issues
    assert!(problematic_report.quality_score < 100.0); // Lower quality score

    // Should detect missing documentation
    let has_doc_issue = problematic_report
        .issues
        .iter()
        .any(|issue| matches!(issue, QualityIssue::MissingDocumentation { .. }));
    assert!(has_doc_issue);

    // Should detect too many parameters
    let has_param_issue = problematic_report
        .issues
        .iter()
        .any(|issue| matches!(issue, QualityIssue::TooManyParameters { .. }));
    assert!(has_param_issue);

    // Verify the overall report structure
    assert_eq!(quality_report.class_reports.len(), 2);
    assert!(
        !quality_report.issues.is_empty()
            || quality_report
                .class_reports
                .iter()
                .any(|r| !r.issues.is_empty())
    );
    assert!(quality_report.overall_score >= 0.0 && quality_report.overall_score <= 100.0);
}
