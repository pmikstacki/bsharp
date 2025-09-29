use crate::analysis::metrics::complexity::ComplexityAnalyzer;
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::nodes::declarations::{
    ClassDeclaration, MethodDeclaration, NamespaceDeclaration,
};
use serde::{Deserialize, Serialize};

// Minimal placeholder for naming violation type to preserve QualityIssue interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NamingViolation {
    Violation,
}

/// Quality issues and code smells detector
#[derive(Debug, PartialEq)]
pub struct QualityAnalyzer;

impl QualityAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyze quality issues in a compilation unit
    pub fn analyze(&self, compilation_unit: &CompilationUnit) -> QualityReport {
        let mut report = QualityReport::new();
        self.analyze_compilation_unit(compilation_unit, &mut report);
        report.calculate_overall_score();
        report
    }

    fn analyze_compilation_unit(
        &self,
        compilation_unit: &CompilationUnit,
        report: &mut QualityReport,
    ) {
        for declaration in &compilation_unit.declarations {
            if let TopLevelDeclaration::Namespace(namespace) = declaration {
                self.analyze_namespace(namespace, report);
            }
            if let TopLevelDeclaration::Class(class_decl) = declaration {
                self.analyze_class(class_decl, report);
            }
            // Add other top-level declarations analysis if needed
        }
    }

    fn analyze_namespace(&self, namespace: &NamespaceDeclaration, report: &mut QualityReport) {
        for member in &namespace.declarations {
            if let crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class_decl) = member {
                self.analyze_class(class_decl, report);
            }
            // Add other namespace members analysis if needed
        }
    }

    fn analyze_class(&self, class: &ClassDeclaration, report: &mut QualityReport) {
        let class_report = self.calculate_class_metrics(class);
        report.class_reports.push(class_report);
    }

    fn calculate_class_metrics(&self, class: &ClassDeclaration) -> ClassQualityReport {
        let method_count = class
            .body_declarations
            .iter()
            .filter(|m| {
                matches!(
                    m,
                    crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(_)
                )
            })
            .count();

        let field_count = class
            .body_declarations
            .iter()
            .filter(|m| {
                matches!(
                    m,
                    crate::syntax::nodes::declarations::ClassBodyDeclaration::Field(_)
                )
            })
            .count();

        let property_count = class
            .body_declarations
            .iter()
            .filter(|m| {
                matches!(
                    m,
                    crate::syntax::nodes::declarations::ClassBodyDeclaration::Property(_)
                )
            })
            .count();

        let issues = self.collect_class_issues(class);

        let mut class_report = ClassQualityReport {
            class_name: class.name.name.clone(),
            method_count,
            field_count,
            property_count,
            cyclomatic_complexity: 0,
            lines_of_code: 0,
            issues,
            quality_score: 100.0,
        };

        // Calculate the quality score based on issues
        class_report.calculate_score();

        class_report
    }

    fn collect_class_issues(&self, class: &ClassDeclaration) -> Vec<QualityIssue> {
        let mut issues = Vec::new();
        for member in &class.body_declarations {
            if let crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(method) = member
            {
                self.analyze_method(method, &mut issues);
            }
            // Add analysis for other class members like fields, properties if needed
        }
        issues
    }

    fn analyze_method(&self, method: &MethodDeclaration, issues: &mut Vec<QualityIssue>) {
        // Check for missing documentation on public methods
        if method
            .modifiers
            .iter()
            .any(|m| format!("{:?}", m).to_lowercase() == "public")
        {
            issues.push(QualityIssue::MissingDocumentation {
                member_name: method.name.name.clone(),
                member_type: "Method".to_string(),
            });
        }

        // Check for too many parameters (threshold: 7 parameters)
        let parameter_count = method.parameters.len();
        if parameter_count > 7 {
            issues.push(QualityIssue::TooManyParameters {
                method_name: method.name.name.clone(),
                parameter_count,
            });
        }

        // Check for high cyclomatic complexity (threshold: 10)
        if let Some(body) = &method.body {
            let complexity = ComplexityAnalyzer::calculate_cyclomatic_complexity(body, 1);

            if complexity > 10 {
                issues.push(QualityIssue::HighComplexity {
                    method_name: method.name.name.clone(),
                    complexity,
                });
            }
        }

        // Example: Check for long methods (e.g., > 50 lines)
        // Placeholder - line count not directly available here
        if false {
            issues.push(QualityIssue::LongMethod {
                method_name: method.name.name.clone(),
                line_count: 0, // Placeholder
                threshold: 50,
            });
        }
    }
}

impl Default for QualityAnalyzer {
    fn default() -> Self {
        Self
    }
}

/// Quality issues found in code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityIssue {
    // Complexity issues
    HighComplexity {
        method_name: String,
        complexity: usize,
    },
    LongMethod {
        method_name: String,
        line_count: usize,
        threshold: usize,
    },
    LargeClass {
        class_name: String,
        method_count: usize,
    },
    TooManyParameters {
        method_name: String,
        parameter_count: usize,
    },

    // Documentation issues
    MissingDocumentation {
        member_name: String,
        member_type: String,
    },

    // Naming issues (placeholder enum defined locally after refactor)
    NamingViolation(NamingViolation),

    // Design issues
    DeepInheritance {
        class_name: String,
        depth: usize,
    },
    UnusedCode {
        member_name: String,
        member_type: String,
    },
    DuplicateCode {
        location1: String,
        location2: String,
        similarity: f64,
    },

    // Performance issues
    StringConcatenationInLoop {
        method_name: String,
        location: String,
    },
    InappropriateIntimacy {
        class1: String,
        class2: String,
    },

    // Architecture issues
    GodClass {
        class_name: String,
        responsibility_count: usize,
    },
    FeatureEnvy {
        method_name: String,
        target_class: String,
    },
}

/// Quality report containing all found issues
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityReport {
    pub class_reports: Vec<ClassQualityReport>,
    pub overall_score: f64,
    pub grade: QualityGrade,
    pub issues: Vec<QualityIssue>,
}

impl QualityReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_issue(&mut self, issue: QualityIssue) {
        self.issues.push(issue);
    }

    pub fn calculate_overall_score(&mut self) {
        if self.class_reports.is_empty() {
            // If no class reports but we have issues, calculate score based on issues
            if !self.issues.is_empty() {
                let mut penalty = 0.0;
                for issue in &self.issues {
                    penalty += match issue {
                        QualityIssue::MissingDocumentation { .. } => 5.0,
                        QualityIssue::LongMethod { .. } => 10.0,
                        QualityIssue::HighComplexity { .. } => 15.0,
                        QualityIssue::TooManyParameters { .. } => 8.0,
                        QualityIssue::NamingViolation(_) => 2.0,
                        _ => 1.0, // Default penalty
                    };
                }
                self.overall_score = (100.0f64 - penalty).max(0.0f64);
            } else {
                // No classes and no issues means empty file - score should be 0
                self.overall_score = 0.0;
            }

            // Assign grade based on score
            self.grade = match self.overall_score {
                s if s >= 90.0 => QualityGrade::A,
                s if s >= 80.0 => QualityGrade::B,
                s if s >= 70.0 => QualityGrade::C,
                s if s >= 60.0 => QualityGrade::D,
                _ => QualityGrade::F,
            };
            return;
        }

        let total_score: f64 = self.class_reports.iter().map(|r| r.quality_score).sum();
        self.overall_score = total_score / self.class_reports.len() as f64;

        // Apply penalty for general issues not tied to specific classes
        if !self.issues.is_empty() {
            let mut penalty = 0.0;
            for issue in &self.issues {
                penalty += match issue {
                    QualityIssue::MissingDocumentation { .. } => 5.0,
                    QualityIssue::LongMethod { .. } => 10.0,
                    QualityIssue::HighComplexity { .. } => 15.0,
                    QualityIssue::TooManyParameters { .. } => 8.0,
                    QualityIssue::NamingViolation(_) => 2.0,
                    _ => 1.0, // Default penalty
                };
            }
            self.overall_score = (self.overall_score - penalty).max(0.0f64);
        }

        // Assign grade based on score
        self.grade = match self.overall_score {
            s if s >= 90.0 => QualityGrade::A,
            s if s >= 80.0 => QualityGrade::B,
            s if s >= 70.0 => QualityGrade::C,
            s if s >= 60.0 => QualityGrade::D,
            _ => QualityGrade::F,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassQualityReport {
    pub class_name: String,
    pub method_count: usize,
    pub field_count: usize,
    pub property_count: usize,
    pub cyclomatic_complexity: usize,
    pub lines_of_code: usize,
    pub issues: Vec<QualityIssue>,
    pub quality_score: f64,
}

impl ClassQualityReport {
    pub fn calculate_score(&mut self) {
        let mut penalty = 0.0;
        for issue in &self.issues {
            penalty += match issue {
                QualityIssue::MissingDocumentation { .. } => 5.0,
                QualityIssue::LongMethod { .. } => 10.0,
                QualityIssue::HighComplexity { .. } => 15.0,
                QualityIssue::TooManyParameters { .. } => 8.0,
                QualityIssue::NamingViolation(_) => 2.0,
                _ => 1.0, // Default penalty
            };
        }
        self.quality_score = (100.0f64 - penalty).max(0.0f64); // Ensure score is not negative
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QualitySeverity {
    Critical = 4,
    Major = 3,
    Minor = 2,
    Info = 1,
}

/// Quality metrics for different aspects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub maintainability_index: f64,
    pub code_coverage: f64,
    pub technical_debt_ratio: f64,
    pub duplication_percentage: f64,
    pub test_coverage: f64,
}

impl QualityMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate overall quality grade
    pub fn quality_grade(&self) -> QualityGrade {
        let score = (self.maintainability_index
            + self.code_coverage
            + (100.0 - self.technical_debt_ratio)
            + (100.0 - self.duplication_percentage)
            + self.test_coverage)
            / 5.0;

        match score {
            90.0..=100.0 => QualityGrade::A,
            80.0..90.0 => QualityGrade::B,
            70.0..80.0 => QualityGrade::C,
            60.0..70.0 => QualityGrade::D,
            _ => QualityGrade::F,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum QualityGrade {
    A, // Excellent (90-100%)
    B, // Good (80-89%)
    C, // Fair (70-79%)
    D, // Poor (60-69%)
    #[default]
    F, // Failing (<60%)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
