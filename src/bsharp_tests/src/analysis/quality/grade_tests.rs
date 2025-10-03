use bsharp_analysis::quality::{QualityGrade, QualityIssue, QualityMetrics, QualityReport};

#[test]
fn test_quality_score_calculation() {
    let mut report = QualityReport::new();

    // Add some issues
    report.add_issue(QualityIssue::HighComplexity {
        method_name: "TestMethod".to_string(),
        complexity: 15,
    });

    report.add_issue(QualityIssue::LongMethod {
        method_name: "AnotherMethod".to_string(),
        line_count: 40,
        threshold: 50,
    });

    report.calculate_overall_score();

    // Score should be less than 100 due to issues
    assert!(report.overall_score < 100.0);
    assert!(report.overall_score >= 0.0);
}

#[test]
fn test_severity_classification() {
    let report = QualityReport::new();

    let _high_complexity = QualityIssue::HighComplexity {
        method_name: "Test".to_string(),
        complexity: 25,
    };

    assert_eq!(report.grade, QualityGrade::F);
}

#[test]
fn test_quality_grade() {
    let mut metrics = QualityMetrics::new();
    metrics.maintainability_index = 95.0;
    metrics.code_coverage = 90.0;
    metrics.technical_debt_ratio = 5.0;
    metrics.duplication_percentage = 2.0;
    metrics.test_coverage = 85.0;

    assert_eq!(metrics.quality_grade(), QualityGrade::A);
}
