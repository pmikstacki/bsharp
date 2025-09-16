use bsharp::analysis::quality::*;

#[test]
fn test_quality_metrics_new() {
    let metrics = QualityMetrics::new();
    
    assert_eq!(metrics.maintainability_index, 0.0);
    assert_eq!(metrics.code_coverage, 0.0);
    assert_eq!(metrics.technical_debt_ratio, 0.0);
    assert_eq!(metrics.duplication_percentage, 0.0);
    assert_eq!(metrics.test_coverage, 0.0);
}

#[test]
fn test_quality_grade_calculation_excellent() {
    let metrics = QualityMetrics {
        maintainability_index: 95.0,
        code_coverage: 90.0,
        technical_debt_ratio: 5.0,
        duplication_percentage: 2.0,
        test_coverage: 85.0,
    };
    
    assert_eq!(metrics.quality_grade(), QualityGrade::A);
}

#[test]
fn test_quality_grade_calculation_good() {
    let metrics = QualityMetrics {
        maintainability_index: 82.0,
        code_coverage: 75.0,
        technical_debt_ratio: 15.0,
        duplication_percentage: 8.0,
        test_coverage: 70.0,
    };
    
    assert_eq!(metrics.quality_grade(), QualityGrade::B);
}

#[test]
fn test_quality_grade_calculation_fair() {
    let metrics = QualityMetrics {
        maintainability_index: 72.0,
        code_coverage: 65.0,
        technical_debt_ratio: 25.0,
        duplication_percentage: 12.0,
        test_coverage: 60.0,
    };
    
    assert_eq!(metrics.quality_grade(), QualityGrade::C);
}

#[test]
fn test_quality_grade_calculation_poor() {
    let metrics = QualityMetrics {
        maintainability_index: 62.0,
        code_coverage: 50.0,
        technical_debt_ratio: 35.0,
        duplication_percentage: 18.0,
        test_coverage: 45.0,
    };
    
    assert_eq!(metrics.quality_grade(), QualityGrade::D);
}

#[test]
fn test_quality_grade_calculation_failing() {
    let metrics = QualityMetrics {
        maintainability_index: 45.0,
        code_coverage: 30.0,
        technical_debt_ratio: 50.0,
        duplication_percentage: 25.0,
        test_coverage: 25.0,
    };
    
    assert_eq!(metrics.quality_grade(), QualityGrade::F);
}

#[test]
fn test_quality_grade_boundary_conditions() {
    // Test exact boundary values
    let boundary_cases = [
        (90.0, QualityGrade::A),
        (89.9, QualityGrade::B),
        (80.0, QualityGrade::B),
        (79.9, QualityGrade::C),
        (70.0, QualityGrade::C),
        (69.9, QualityGrade::D),
        (60.0, QualityGrade::D),
        (59.9, QualityGrade::F),
    ];
    
    for (score, expected_grade) in boundary_cases.iter() {
        let metrics = QualityMetrics {
            maintainability_index: *score,
            code_coverage: *score,
            technical_debt_ratio: 100.0 - score, // Inverted for debt
            duplication_percentage: 100.0 - score, // Inverted for duplication
            test_coverage: *score,
        };
        
        assert_eq!(metrics.quality_grade(), *expected_grade,
            "Failed for score {}", score);
    }
}

#[test]
fn test_quality_metrics_with_extreme_values() {
    // Test with extreme values
    let extreme_high = QualityMetrics {
        maintainability_index: 100.0,
        code_coverage: 100.0,
        technical_debt_ratio: 0.0,
        duplication_percentage: 0.0,
        test_coverage: 100.0,
    };
    
    let extreme_low = QualityMetrics {
        maintainability_index: 0.0,
        code_coverage: 0.0,
        technical_debt_ratio: 100.0,
        duplication_percentage: 100.0,
        test_coverage: 0.0,
    };
    
    assert_eq!(extreme_high.quality_grade(), QualityGrade::A);
    assert_eq!(extreme_low.quality_grade(), QualityGrade::F);
}

#[test]
fn test_quality_metrics_mixed_values() {
    // Test with mixed good and bad metrics
    let mixed_metrics = QualityMetrics {
        maintainability_index: 95.0, // Excellent
        code_coverage: 30.0,         // Poor
        technical_debt_ratio: 60.0,  // High debt (bad)
        duplication_percentage: 5.0, // Good
        test_coverage: 80.0,         // Good
    };
    
    let grade = mixed_metrics.quality_grade();
    
    // With mixed metrics, grade should be somewhere in the middle
    assert!(matches!(grade, QualityGrade::B | QualityGrade::C | QualityGrade::D));
}

#[test]
fn test_quality_severity_enum_values() {
    // Test that severity enum has correct numeric values
    assert_eq!(QualitySeverity::Critical as u8, 4);
    assert_eq!(QualitySeverity::Major as u8, 3);
    assert_eq!(QualitySeverity::Minor as u8, 2);
    assert_eq!(QualitySeverity::Info as u8, 1);
}

#[test]
fn test_quality_severity_ordering() {
    // Test severity ordering
    let severities = [
        QualitySeverity::Info,
        QualitySeverity::Minor,
        QualitySeverity::Major,
        QualitySeverity::Critical,
    ];
    
    for i in 0..severities.len() - 1 {
        assert!((severities[i] as u8) < (severities[i + 1] as u8),
            "{:?} should be less severe than {:?}", severities[i], severities[i + 1]);
    }
}

#[test]
fn test_quality_grade_default() {
    let default_grade = QualityGrade::default();
    assert_eq!(default_grade, QualityGrade::F);
}

#[test]
fn test_quality_grade_enum_variants() {
    // Test all grade variants exist
    let grades = [
        QualityGrade::A,
        QualityGrade::B,
        QualityGrade::C,
        QualityGrade::D,
        QualityGrade::F,
    ];
    
    // All grades should be different
    for i in 0..grades.len() {
        for j in (i + 1)..grades.len() {
            assert_ne!(grades[i], grades[j],
                "Grades {:?} and {:?} should be different", grades[i], grades[j]);
        }
    }
}

#[test]
fn test_quality_metrics_serialization() {
    let metrics = QualityMetrics {
        maintainability_index: 85.5,
        code_coverage: 72.3,
        technical_debt_ratio: 18.7,
        duplication_percentage: 6.2,
        test_coverage: 68.9,
    };
    
    // Test JSON serialization/deserialization
    let json = serde_json::to_string(&metrics).expect("Failed to serialize metrics");
    let deserialized: QualityMetrics = serde_json::from_str(&json).expect("Failed to deserialize metrics");
    
    assert_eq!(deserialized.maintainability_index, metrics.maintainability_index);
    assert_eq!(deserialized.code_coverage, metrics.code_coverage);
    assert_eq!(deserialized.technical_debt_ratio, metrics.technical_debt_ratio);
    assert_eq!(deserialized.duplication_percentage, metrics.duplication_percentage);
    assert_eq!(deserialized.test_coverage, metrics.test_coverage);
}

#[test]
fn test_quality_grade_serialization() {
    let grades = [
        QualityGrade::A,
        QualityGrade::B,
        QualityGrade::C,
        QualityGrade::D,
        QualityGrade::F,
    ];
    
    for grade in grades.iter() {
        let json = serde_json::to_string(grade).expect("Failed to serialize grade");
        let deserialized: QualityGrade = serde_json::from_str(&json).expect("Failed to deserialize grade");
        assert_eq!(deserialized, *grade);
    }
}

#[test]
fn test_quality_severity_serialization() {
    let severities = [
        QualitySeverity::Info,
        QualitySeverity::Minor,
        QualitySeverity::Major,
        QualitySeverity::Critical,
    ];
    
    for severity in severities.iter() {
        let json = serde_json::to_string(severity).expect("Failed to serialize severity");
        let deserialized: QualitySeverity = serde_json::from_str(&json).expect("Failed to deserialize severity");
        assert_eq!(deserialized as u8, *severity as u8);
    }
}

#[test]
fn test_realistic_quality_scenarios() {
    // Test realistic quality scenarios
    
    // Scenario 1: Well-maintained legacy code
    let legacy_code = QualityMetrics {
        maintainability_index: 65.0, // Moderate due to age
        code_coverage: 85.0,          // Good test coverage
        technical_debt_ratio: 30.0,   // Some accumulated debt
        duplication_percentage: 15.0, // Some duplication
        test_coverage: 80.0,          // Good testing
    };
    
    // Scenario 2: New greenfield project
    let greenfield = QualityMetrics {
        maintainability_index: 90.0, // High for new code
        code_coverage: 70.0,          // Still building coverage
        technical_debt_ratio: 8.0,    // Low debt
        duplication_percentage: 3.0,  // Minimal duplication
        test_coverage: 75.0,          // Good testing practices
    };
    
    // Scenario 3: Problematic codebase
    let problematic = QualityMetrics {
        maintainability_index: 40.0, // Poor maintainability
        code_coverage: 25.0,          // Low coverage
        technical_debt_ratio: 65.0,   // High debt
        duplication_percentage: 30.0, // High duplication
        test_coverage: 20.0,          // Poor testing
    };
    
    // Verify appropriate grades for each scenario
    assert!(matches!(legacy_code.quality_grade(), QualityGrade::C | QualityGrade::D));
    assert!(matches!(greenfield.quality_grade(), QualityGrade::A | QualityGrade::B));
    assert_eq!(problematic.quality_grade(), QualityGrade::F);
} 