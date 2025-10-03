use analysis::metrics::basic::BasicMetrics;
use analysis::metrics::complexity::ComplexityMetrics;
use analysis::metrics::maintainability::*;

#[test]
fn test_maintainability_metrics_default() {
    let metrics = MaintainabilityMetrics::default();
    assert_eq!(metrics.maintainability_index, 0.0);
    assert_eq!(metrics.technical_debt_ratio, 0.0);
    assert_eq!(metrics.code_coverage, 0.0);
    assert_eq!(metrics.documentation_coverage, 0.0);
}

#[test]
fn test_maintainability_grade_excellent() {
    let metrics = MaintainabilityMetrics {
        maintainability_index: 90.0,
        ..Default::default()
    };

    assert_eq!(
        metrics.maintainability_grade(),
        MaintainabilityGrade::Excellent
    );
}

#[test]
fn test_maintainability_grade_good() {
    let metrics = MaintainabilityMetrics {
        maintainability_index: 75.0,
        ..Default::default()
    };

    assert_eq!(metrics.maintainability_grade(), MaintainabilityGrade::Good);
}

#[test]
fn test_maintainability_grade_moderate() {
    let metrics = MaintainabilityMetrics {
        maintainability_index: 55.0,
        ..Default::default()
    };

    assert_eq!(
        metrics.maintainability_grade(),
        MaintainabilityGrade::Moderate
    );
}

#[test]
fn test_maintainability_grade_poor() {
    let metrics = MaintainabilityMetrics {
        maintainability_index: 35.0,
        ..Default::default()
    };

    assert_eq!(metrics.maintainability_grade(), MaintainabilityGrade::Poor);
}

#[test]
fn test_maintainability_grade_critical() {
    let metrics = MaintainabilityMetrics {
        maintainability_index: 15.0,
        ..Default::default()
    };

    assert_eq!(
        metrics.maintainability_grade(),
        MaintainabilityGrade::Critical
    );
}

#[test]
fn test_quality_score_calculation() {
    let metrics = MaintainabilityMetrics {
        maintainability_index: 80.0,
        code_coverage: 85.0,
        technical_debt_ratio: 10.0, // Will be inverted to 90.0
        documentation_coverage: 70.0,
        test_coverage: 90.0,
        duplication_percentage: 5.0, // Will be inverted to 95.0
        ..Default::default()
    };

    let quality_score = metrics.quality_score();

    // Score should be weighted average of: 80*0.3 + 85*0.2 + 90*0.2 + 70*0.1 + 90*0.1 + 95*0.1
    // = 24 + 17 + 18 + 7 + 9 + 9.5 = 84.5
    assert!((quality_score - 84.5).abs() < 0.1);
}

#[test]
fn test_maintainability_analyzer_new() {
    let analyzer = MaintainabilityAnalyzer::new();
    // Test that the analyzer is created successfully
    assert_eq!(analyzer, MaintainabilityAnalyzer::new());
}

#[test]
fn test_maintainability_index_calculation() {
    let analyzer = MaintainabilityAnalyzer::new();

    let basic_metrics = BasicMetrics {
        logical_lines: 100,
        comment_lines: 20,
        physical_lines: 120,
        ..Default::default()
    };

    let complexity_metrics = ComplexityMetrics {
        cyclomatic_complexity: 5,
        halstead_metrics: analysis::metrics::complexity::HalsteadMetrics {
            distinct_operators: 10,
            distinct_operands: 15,
            total_operators: 50,
            total_operands: 30,
        },
        ..Default::default()
    };

    let mi = analyzer.calculate_maintainability_index(&basic_metrics, &complexity_metrics);

    // MI should be calculated according to Microsoft formula
    assert!((0.0..=100.0).contains(&mi));
    assert!(mi > 0.0); // Should not be zero for non-trivial code
}

#[test]
fn test_maintainability_index_empty_code() {
    let analyzer = MaintainabilityAnalyzer::new();

    let empty_basic = BasicMetrics::default();
    let empty_complexity = ComplexityMetrics::default();

    let mi = analyzer.calculate_maintainability_index(&empty_basic, &empty_complexity);

    // Empty code should have perfect maintainability
    assert_eq!(mi, 100.0);
}

#[test]
fn test_technical_debt_calculation() {
    let analyzer = MaintainabilityAnalyzer::new();

    let basic_metrics = BasicMetrics {
        logical_lines: 1000,
        total_if_statements: 50,
        total_for_loops: 20,
        total_while_loops: 10,
        total_switch_statements: 5,
        total_try_statements: 3,
        ..Default::default()
    };

    let debt = analyzer.calculate_technical_debt(&basic_metrics);

    // Total control structures: 50+20+10+5+3 = 88
    // Debt ratio: (88/1000) * 100 = 8.8%
    assert!((debt.debt_ratio - 8.8).abs() < 0.1);

    // Debt in hours: 88 * 0.5 = 44 hours
    assert!((debt.debt_in_hours - 44.0).abs() < 0.1);

    // Debt per line: 44/1000 = 0.044
    assert!((debt.debt_per_line - 0.044).abs() < 0.001);

    // SQALE rating should be B (6-10%)
    assert_eq!(debt.sqale_rating, SQALERating::B);

    // Remediation cost: 44 * 100 = $4400
    assert!((debt.remediation_cost - 4400.0).abs() < 0.1);
}

#[test]
fn test_sqale_rating_from_debt_ratio() {
    assert_eq!(SQALERating::from_debt_ratio(3.0), SQALERating::A);
    assert_eq!(SQALERating::from_debt_ratio(7.0), SQALERating::B);
    assert_eq!(SQALERating::from_debt_ratio(15.0), SQALERating::C);
    assert_eq!(SQALERating::from_debt_ratio(30.0), SQALERating::D);
    assert_eq!(SQALERating::from_debt_ratio(60.0), SQALERating::E);
}

#[test]
fn test_defect_density_estimation() {
    let analyzer = MaintainabilityAnalyzer::new();

    let complexity_metrics = ComplexityMetrics {
        cyclomatic_complexity: 50, // Total complexity across methods
        ..Default::default()
    };

    let basic_metrics = BasicMetrics {
        logical_lines: 1000,
        total_methods: 10, // Average complexity: 50/10 = 5
        ..Default::default()
    };

    let defect_density = analyzer.estimate_defect_density(&complexity_metrics, &basic_metrics);

    // Base rate: 2 defects per 1000 LOC
    // Complexity multiplier: max(5/10, 1) = 1
    // Expected: (2 * 1 * 1000) / 1000 = 2.0
    assert!((defect_density - 2.0).abs() < 0.1);
}

#[test]
fn test_defect_density_high_complexity() {
    let analyzer = MaintainabilityAnalyzer::new();

    let complexity_metrics = ComplexityMetrics {
        cyclomatic_complexity: 200, // High total complexity
        ..Default::default()
    };

    let basic_metrics = BasicMetrics {
        logical_lines: 1000,
        total_methods: 10, // Average complexity: 200/10 = 20
        ..Default::default()
    };

    let defect_density = analyzer.estimate_defect_density(&complexity_metrics, &basic_metrics);

    // Base rate: 2 defects per 1000 LOC
    // Complexity multiplier: 20/10 = 2
    // Expected: (2 * 2 * 1000) / 1000 = 4.0
    assert!((defect_density - 4.0).abs() < 0.1);
}

#[test]
fn test_change_impact_analysis_default() {
    let impact = ChangeImpactAnalysis::default();

    assert!(impact.affected_classes.is_empty());
    assert!(impact.affected_methods.is_empty());
    assert_eq!(impact.risk_level, RiskLevel::Low);
    assert_eq!(impact.estimated_effort_hours, 0.0);
}

#[test]
fn test_technical_debt_edge_cases() {
    let analyzer = MaintainabilityAnalyzer::new();

    // Test with no code
    let empty_metrics = BasicMetrics::default();
    let empty_debt = analyzer.calculate_technical_debt(&empty_metrics);

    assert_eq!(empty_debt.debt_ratio, 0.0);
    assert_eq!(empty_debt.debt_in_hours, 0.0);
    assert_eq!(empty_debt.debt_per_line, 0.0);
    assert_eq!(empty_debt.sqale_rating, SQALERating::A);

    // Test with extremely high complexity
    let high_complexity_metrics = BasicMetrics {
        logical_lines: 100,
        total_if_statements: 200, // More control structures than lines!
        ..Default::default()
    };

    let high_debt = analyzer.calculate_technical_debt(&high_complexity_metrics);

    // Should handle extreme cases gracefully
    assert!(high_debt.debt_ratio > 100.0); // Can exceed 100%
    assert_eq!(high_debt.sqale_rating, SQALERating::E);
}

#[test]
fn test_maintainability_metrics_comprehensive() {
    let analyzer = MaintainabilityAnalyzer::new();

    // Create realistic metrics for a medium-sized project
    let basic_metrics = BasicMetrics {
        logical_lines: 5000,
        comment_lines: 1000,
        physical_lines: 6500,
        total_methods: 100,
        total_classes: 20,
        total_if_statements: 250,
        total_for_loops: 80,
        total_while_loops: 40,
        total_switch_statements: 15,
        total_try_statements: 25,
        ..Default::default()
    };

    let complexity_metrics = ComplexityMetrics {
        cyclomatic_complexity: 500, // Average 5 per method
        halstead_metrics: analysis::metrics::complexity::HalsteadMetrics {
            distinct_operators: 25,
            distinct_operands: 100,
            total_operators: 2000,
            total_operands: 3000,
        },
        ..Default::default()
    };

    let mi = analyzer.calculate_maintainability_index(&basic_metrics, &complexity_metrics);
    let debt = analyzer.calculate_technical_debt(&basic_metrics);
    let defect_density = analyzer.estimate_defect_density(&complexity_metrics, &basic_metrics);

    // Verify realistic ranges
    assert!((0.0..=100.0).contains(&mi));
    assert!(debt.debt_ratio > 0.0);
    assert!(debt.debt_in_hours > 0.0);
    assert!(defect_density > 0.0);

    // Verify SQALE rating is reasonable
    assert!(matches!(
        debt.sqale_rating,
        SQALERating::A | SQALERating::B | SQALERating::C
    ));

    // Create comprehensive maintainability metrics
    let comprehensive_metrics = MaintainabilityMetrics {
        maintainability_index: mi,
        technical_debt_ratio: debt.debt_ratio,
        code_coverage: 75.0, // Assumed
        documentation_coverage: basic_metrics.comment_ratio() * 100.0,
        test_coverage: 65.0,         // Assumed
        duplication_percentage: 8.0, // Assumed
        code_churn: 15.0,            // Assumed
        defect_density,
    };

    let quality_score = comprehensive_metrics.quality_score();
    let grade = comprehensive_metrics.maintainability_grade();

    assert!((0.0..=100.0).contains(&quality_score));
    assert!(matches!(
        grade,
        MaintainabilityGrade::Excellent
            | MaintainabilityGrade::Good
            | MaintainabilityGrade::Moderate
            | MaintainabilityGrade::Poor
            | MaintainabilityGrade::Critical
    ));
}
