use crate::parser::ast::CompilationUnit;
use super::complexity::ComplexityMetrics;
use super::basic::BasicMetrics;
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
        
        weights.iter()
            .zip(scores.iter())
            .map(|(weight, score)| weight * score)
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaintainabilityGrade {
    Excellent,  // 85-100
    Good,       // 65-84
    Moderate,   // 45-64
    Poor,       // 25-44
    Critical,   // 0-24
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SQALERating {
    A, // <= 5% debt ratio
    B, // 6-10%
    C, // 11-20%
    D, // 21-50%
    E, // > 50%
}

impl Default for SQALERating {
    fn default() -> Self {
        SQALERating::A
    }
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
        // TODO: Implement full maintainability analysis
        let _ = unit; // Suppress unused warning for now
        
        let mut metrics = MaintainabilityMetrics::new();
        
        // Placeholder calculations - would be replaced with real analysis
        metrics.maintainability_index = 75.0;
        metrics.documentation_coverage = 60.0;
        
        metrics
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
        mi.max(0.0).min(100.0)
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
    pub fn estimate_defect_density(&self, complexity_metrics: &ComplexityMetrics, basic_metrics: &BasicMetrics) -> f64 {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for RiskLevel {
    fn default() -> Self {
        RiskLevel::Low
    }
} 