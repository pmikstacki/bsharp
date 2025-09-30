use super::basic::BasicMetrics;
use super::complexity::ComplexityMetrics;
use crate::syntax::ast::CompilationUnit;
use serde::{Deserialize, Serialize};

/// Maintainability metrics and indices
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MaintainabilityMetrics {
    pub maintainability_index: f64,
    pub technical_debt_ratio: f64,
    pub code_coverage: f64,
    pub documentation_coverage: f64,
    pub test_coverage: f64,
    pub duplication_percentage: f64,
    pub code_churn: f64,
    pub defect_density: f64,
}

impl MaintainabilityMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate overall maintainability grade
    pub fn maintainability_grade(&self) -> MaintainabilityGrade {
        match self.maintainability_index {
            85.0..=100.0 => MaintainabilityGrade::Excellent,
            65.0..85.0 => MaintainabilityGrade::Good,
            45.0..65.0 => MaintainabilityGrade::Moderate,
            25.0..45.0 => MaintainabilityGrade::Poor,
            _ => MaintainabilityGrade::Critical,
        }
    }

    /// Calculate composite quality score
    pub fn quality_score(&self) -> f64 {
        let weights = [0.3, 0.2, 0.2, 0.1, 0.1, 0.1]; // Weighted scoring
        let scores = [
            self.maintainability_index,
            self.code_coverage,
            100.0 - self.technical_debt_ratio,
            self.documentation_coverage,
            self.test_coverage,
            100.0 - self.duplication_percentage,
        ];

        weights
            .iter()
            .zip(scores.iter())
            .map(|(weight, score)| weight * score)
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaintainabilityGrade {
    Excellent, // 85-100
    Good,      // 65-84
    Moderate,  // 45-64
    Poor,      // 25-44
    Critical,  // 0-24
}

/// Technical debt quantification
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TechnicalDebt {
    pub debt_ratio: f64,
    pub debt_in_hours: f64,
    pub debt_per_line: f64,
    pub sqale_rating: SQALERating,
    pub remediation_cost: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SQALERating {
    #[default]
    A, // <= 5% debt ratio
    B, // 6-10%
    C, // 11-20%
    D, // 21-50%
    E, // > 50%
}

impl SQALERating {
    pub fn from_debt_ratio(ratio: f64) -> Self {
        match ratio {
            r if r <= 5.0 => SQALERating::A,
            r if r <= 10.0 => SQALERating::B,
            r if r <= 20.0 => SQALERating::C,
            r if r <= 50.0 => SQALERating::D,
            _ => SQALERating::E,
        }
    }
}

/// Maintainability analyzer
#[derive(Debug, PartialEq)]
pub struct MaintainabilityAnalyzer;

impl MaintainabilityAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Calculate maintainability metrics for a compilation unit
    pub fn analyze_compilation_unit(&self, unit: &CompilationUnit) -> MaintainabilityMetrics {
        // Collect basic metrics
        let mut basic_collector = crate::analysis::metrics::basic::BasicMetricsCollector::new();
        basic_collector.collect_from_compilation_unit(unit);
        let basic = basic_collector.get_metrics().clone();

        // Aggregate complexity across methods in the compilation unit
        let mut complexity_total = crate::analysis::metrics::complexity::ComplexityMetrics::default();
        let complexity_analyzer = crate::analysis::metrics::complexity::ComplexityAnalyzer::new();

        // Walk methods via a simple traversal of declarations
        // Note: For deeper coverage, use the analysis walker; for now we traverse directly.
        for decl in &unit.declarations {
            if let crate::syntax::ast::TopLevelDeclaration::Class(class) = decl {
                for m in &class.body_declarations {
                    if let crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(method) = m {
                        let cm = complexity_analyzer.analyze_method(method);
                        // Aggregate
                        complexity_total.cyclomatic_complexity += cm.cyclomatic_complexity;
                        complexity_total.cognitive_complexity += cm.cognitive_complexity;
                        complexity_total.essential_complexity += cm.essential_complexity;
                        complexity_total.max_nesting_depth = complexity_total.max_nesting_depth.max(cm.max_nesting_depth);
                        complexity_total.abc_complexity.assignments += cm.abc_complexity.assignments;
                        complexity_total.abc_complexity.branches += cm.abc_complexity.branches;
                        complexity_total.abc_complexity.conditions += cm.abc_complexity.conditions;
                        // Halstead metrics left as default for now
                    }
                }
            }
        }

        // Calculate maintainability index
        let mi = self.calculate_maintainability_index(&basic, &complexity_total);
        let debt = self.calculate_technical_debt(&basic);

        MaintainabilityMetrics {
            maintainability_index: mi,
            technical_debt_ratio: debt.debt_ratio,
            code_coverage: 0.0, // Not available
            documentation_coverage: basic.comment_ratio() * 100.0,
            test_coverage: 0.0, // Not available
            duplication_percentage: 0.0, // Not available
            code_churn: 0.0, // Not available here
            defect_density: self.estimate_defect_density(&complexity_total, &basic),
        }
    }

    /// Calculate maintainability index using Microsoft formula
    pub fn calculate_maintainability_index(
        &self,
        basic_metrics: &BasicMetrics,
        complexity_metrics: &ComplexityMetrics,
    ) -> f64 {
        // Microsoft Maintainability Index formula:
        // MI = 171 - 5.2 * ln(HalsteadVolume) - 0.23 * CyclomaticComplexity - 16.2 * ln(LinesOfCode)
        // + 50 * sin(sqrt(2.4 * PercentOfComments))

        let lines_of_code = basic_metrics.logical_lines as f64;
        let cyclomatic_complexity = complexity_metrics.cyclomatic_complexity as f64;
        let halstead_volume = complexity_metrics.halstead_metrics.volume();
        let comment_percentage = basic_metrics.comment_ratio() * 100.0;

        if lines_of_code == 0.0 {
            return 100.0; // Perfect score for empty code
        }

        let halstead_term = if halstead_volume > 0.0 {
            5.2 * halstead_volume.ln()
        } else {
            0.0
        };

        let complexity_term = 0.23 * cyclomatic_complexity;
        let loc_term = 16.2 * lines_of_code.ln();
        let comment_term = 50.0 * (2.4f64 * comment_percentage).sqrt().sin();

        let mi = 171.0 - halstead_term - complexity_term - loc_term + comment_term;

        // Normalize to 0-100 range
        mi.clamp(0.0, 100.0)
    }

    /// Calculate technical debt metrics
    pub fn calculate_technical_debt(&self, basic_metrics: &BasicMetrics) -> TechnicalDebt {
        // Simplified calculation - would be more sophisticated in practice
        let total_lines = basic_metrics.logical_lines as f64;
        let complexity_issues = basic_metrics.total_control_structures() as f64;

        let debt_ratio = if total_lines > 0.0 {
            (complexity_issues / total_lines) * 100.0
        } else {
            0.0
        };

        let debt_in_hours = complexity_issues * 0.5; // 30 minutes per issue
        let debt_per_line = if total_lines > 0.0 {
            debt_in_hours / total_lines
        } else {
            0.0
        };

        TechnicalDebt {
            debt_ratio,
            debt_in_hours,
            debt_per_line,
            sqale_rating: SQALERating::from_debt_ratio(debt_ratio),
            remediation_cost: debt_in_hours * 100.0, // $100/hour
        }
    }

    /// Calculate code churn (frequency of changes)
    pub fn calculate_code_churn(&self, _file_path: &str, _time_period_days: u32) -> f64 {
        // TODO: Implement code churn analysis using git history
        // This would analyze commit frequency and change size
        0.0
    }

    /// Estimate defect density
    pub fn estimate_defect_density(
        &self,
        complexity_metrics: &ComplexityMetrics,
        basic_metrics: &BasicMetrics,
    ) -> f64 {
        // Empirical formula based on complexity and size
        let lines_of_code = basic_metrics.logical_lines as f64;
        let avg_complexity = if basic_metrics.total_methods > 0 {
            complexity_metrics.cyclomatic_complexity as f64 / basic_metrics.total_methods as f64
        } else {
            1.0
        };

        if lines_of_code == 0.0 {
            return 0.0;
        }

        // Defects per 1000 lines of code
        let base_defect_rate = 2.0; // 2 defects per 1000 LOC (industry average)
        let complexity_multiplier = (avg_complexity / 10.0).max(1.0);

        (base_defect_rate * complexity_multiplier * 1000.0) / lines_of_code
    }
}

impl Default for MaintainabilityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Change impact analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeImpactAnalysis {
    pub affected_classes: Vec<String>,
    pub affected_methods: Vec<String>,
    pub risk_level: RiskLevel,
    pub estimated_effort_hours: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum RiskLevel {
    #[default]
    Low,
    Medium,
    High,
    Critical,
}
