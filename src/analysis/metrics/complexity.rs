use crate::syntax::nodes::declarations::MethodDeclaration;
use crate::syntax::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

/// Advanced complexity metrics beyond basic cyclomatic complexity
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub cyclomatic_complexity: usize,
    pub cognitive_complexity: usize,
    pub essential_complexity: usize,
    pub abc_complexity: ABCComplexity,
    pub halstead_metrics: HalsteadMetrics,
    pub max_nesting_depth: usize,
}

/// ABC (Assignment, Branch, Condition) complexity metrics
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ABCComplexity {
    pub assignments: usize,
    pub branches: usize,
    pub conditions: usize,
}

impl ABCComplexity {
    pub fn magnitude(&self) -> f64 {
        ((self.assignments.pow(2) + self.branches.pow(2) + self.conditions.pow(2)) as f64).sqrt()
    }
}

/// Halstead complexity metrics
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HalsteadMetrics {
    pub distinct_operators: usize,
    pub distinct_operands: usize,
    pub total_operators: usize,
    pub total_operands: usize,
}

impl HalsteadMetrics {
    /// Calculate program vocabulary
    pub fn vocabulary(&self) -> usize {
        self.distinct_operators + self.distinct_operands
    }

    /// Calculate program length
    pub fn length(&self) -> usize {
        self.total_operators + self.total_operands
    }

    /// Calculate calculated length
    pub fn calculated_length(&self) -> f64 {
        if self.distinct_operators == 0 || self.distinct_operands == 0 {
            0.0
        } else {
            (self.distinct_operators as f64) * (self.distinct_operators as f64).log2()
                + (self.distinct_operands as f64) * (self.distinct_operands as f64).log2()
        }
    }

    /// Calculate volume
    pub fn volume(&self) -> f64 {
        if self.vocabulary() == 0 {
            0.0
        } else {
            (self.length() as f64) * (self.vocabulary() as f64).log2()
        }
    }

    /// Calculate difficulty
    pub fn difficulty(&self) -> f64 {
        if self.distinct_operands == 0 || self.total_operands == 0 {
            0.0
        } else {
            ((self.distinct_operators as f64) / 2.0)
                * ((self.total_operands as f64) / (self.distinct_operands as f64))
        }
    }

    /// Calculate effort
    pub fn effort(&self) -> f64 {
        self.difficulty() * self.volume()
    }
}

/// Complexity analyzer for calculating various complexity metrics
#[derive(Debug, PartialEq)]
pub struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Calculate comprehensive complexity metrics for a method
    pub fn analyze_method(&self, method: &MethodDeclaration) -> ComplexityMetrics {
        let mut metrics = ComplexityMetrics::default();

        if let Some(body) = &method.body {
            metrics.cyclomatic_complexity = Self::calculate_cyclomatic_complexity(body, 1);
            metrics.cognitive_complexity = Self::calculate_cognitive_complexity(body, 0, 0);
            metrics.max_nesting_depth = Self::calculate_max_nesting_depth(body, 0);
            metrics.abc_complexity = Self::calculate_abc_complexity(body);
            // TODO: Implement Halstead and essential complexity
        } else {
            metrics.cyclomatic_complexity = 1; // Base complexity for methods without body
        }

        metrics
    }

    /// Calculate cyclomatic complexity (McCabe)
    pub fn calculate_cyclomatic_complexity(
        stmt: &Statement,
        base_complexity: usize,
    ) -> usize {
        match stmt {
            Statement::If(if_stmt) => {
                let mut complexity = base_complexity + 1; // +1 for if
                complexity += Self::calculate_cyclomatic_complexity(&if_stmt.consequence, 0);
                if let Some(alt) = &if_stmt.alternative {
                    complexity += Self::calculate_cyclomatic_complexity(alt, 0);
                }
                complexity
            }
            Statement::For(for_stmt) => {
                base_complexity + 1 + Self::calculate_cyclomatic_complexity(&for_stmt.body, 0)
            }
            Statement::While(while_stmt) => {
                base_complexity + 1 + Self::calculate_cyclomatic_complexity(&while_stmt.body, 0)
            }
            Statement::DoWhile(do_while_stmt) => {
                base_complexity + 1 + Self::calculate_cyclomatic_complexity(&do_while_stmt.body, 0)
            }
            Statement::Switch(switch_stmt) => {
                let mut complexity = base_complexity + switch_stmt.sections.len(); // Each case adds complexity
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        complexity += Self::calculate_cyclomatic_complexity(s, 0);
                    }
                }
                complexity
            }
            Statement::Try(_) => {
                base_complexity + 1 // TODO: Add proper try/catch/finally analysis
            }
            Statement::Block(statements) => {
                let mut complexity = base_complexity;
                for stmt in statements {
                    complexity += Self::calculate_cyclomatic_complexity(stmt, 0);
                }
                complexity
            }
            _ => base_complexity,
        }
    }

    /// Calculate cognitive complexity (SonarSource methodology)
    pub fn calculate_cognitive_complexity(
        stmt: &Statement,
        current_depth: usize,
        base_complexity: usize,
    ) -> usize {
        match stmt {
            Statement::If(if_stmt) => {
                let mut complexity = base_complexity + 1 + current_depth; // +1 for if, +nesting
                complexity +=
                    Self::calculate_cognitive_complexity(&if_stmt.consequence, current_depth + 1, 0);
                if let Some(alt) = &if_stmt.alternative {
                    if matches!(**alt, Statement::If(_)) {
                        // else if doesn't add nesting
                        complexity += Self::calculate_cognitive_complexity(alt, current_depth, 0);
                    } else {
                        // else adds nesting
                        complexity +=
                            Self::calculate_cognitive_complexity(alt, current_depth + 1, 0);
                    }
                }
                complexity
            }
            Statement::For(for_stmt) => {
                base_complexity
                    + 1
                    + current_depth
                    + Self::calculate_cognitive_complexity(&for_stmt.body, current_depth + 1, 0)
            }
            Statement::While(while_stmt) => {
                base_complexity
                    + 1
                    + current_depth
                    + Self::calculate_cognitive_complexity(&while_stmt.body, current_depth + 1, 0)
            }
            Statement::DoWhile(do_while_stmt) => {
                base_complexity
                    + 1
                    + current_depth
                    + Self::calculate_cognitive_complexity(&do_while_stmt.body, current_depth + 1, 0)
            }
            Statement::Switch(switch_stmt) => {
                let mut complexity = base_complexity + 1 + current_depth; // +1 for switch, +nesting
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        complexity += Self::calculate_cognitive_complexity(s, current_depth + 1, 0);
                    }
                }
                complexity
            }
            Statement::Try(_) => {
                base_complexity + 1 + current_depth // TODO: Add proper try/catch/finally analysis
            }
            Statement::Block(statements) => {
                let mut complexity = base_complexity;
                for stmt in statements {
                    complexity += Self::calculate_cognitive_complexity(stmt, current_depth, 0);
                }
                complexity
            }
            Statement::Break(_) => {
                base_complexity + 1 // Break adds cognitive load but no nesting penalty
            }
            Statement::Continue(_) => {
                let nesting_bonus = if current_depth > 0 { 1 } else { 0 };
                base_complexity + 1 + nesting_bonus // Continue adds cognitive load plus limited nesting penalty
            }
            _ => base_complexity,
        }
    }

    /// Calculate maximum nesting depth
    pub fn calculate_max_nesting_depth(stmt: &Statement, current_depth: usize) -> usize {
        match stmt {
            Statement::If(if_stmt) => {
                let new_depth = current_depth + 1;
                let consequence_depth =
                    Self::calculate_max_nesting_depth(&if_stmt.consequence, new_depth);
                let alternative_depth = if let Some(alt) = &if_stmt.alternative {
                    Self::calculate_max_nesting_depth(alt, new_depth)
                } else {
                    new_depth
                };
                consequence_depth.max(alternative_depth)
            }
            Statement::For(for_stmt) => {
                Self::calculate_max_nesting_depth(&for_stmt.body, current_depth + 1)
            }
            Statement::While(while_stmt) => {
                Self::calculate_max_nesting_depth(&while_stmt.body, current_depth + 1)
            }
            Statement::DoWhile(do_while_stmt) => {
                Self::calculate_max_nesting_depth(&do_while_stmt.body, current_depth + 1)
            }
            Statement::Switch(switch_stmt) => {
                let mut max_depth = current_depth + 1;
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        let depth = Self::calculate_max_nesting_depth(s, current_depth + 1);
                        max_depth = max_depth.max(depth);
                    }
                }
                max_depth
            }
            Statement::Block(statements) => {
                let mut max_depth = current_depth;
                for stmt in statements {
                    let depth = Self::calculate_max_nesting_depth(stmt, current_depth);
                    max_depth = max_depth.max(depth);
                }
                max_depth
            }
            _ => current_depth,
        }
    }

    /// Calculate ABC complexity metrics
    pub fn calculate_abc_complexity(stmt: &Statement) -> ABCComplexity {
        let mut abc = ABCComplexity::default();
        Self::collect_abc_complexity(stmt, &mut abc);
        abc
    }

    fn collect_abc_complexity(stmt: &Statement, abc: &mut ABCComplexity) {
        match stmt {
            Statement::If(if_stmt) => {
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_complexity(&if_stmt.consequence, abc);
                if let Some(alt) = &if_stmt.alternative {
                    Self::collect_abc_complexity(alt, abc);
                }
            }
            Statement::For(for_stmt) => {
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_complexity(&for_stmt.body, abc);
            }
            Statement::While(while_stmt) => {
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_complexity(&while_stmt.body, abc);
            }
            Statement::DoWhile(do_while_stmt) => {
                abc.conditions += 1;
                abc.branches += 1;
                Self::collect_abc_complexity(&do_while_stmt.body, abc);
            }
            Statement::Switch(switch_stmt) => {
                abc.conditions += 1;
                abc.branches += switch_stmt.sections.len();
                for section in &switch_stmt.sections {
                    for s in &section.statements {
                        Self::collect_abc_complexity(s, abc);
                    }
                }
            }
            Statement::Expression(_) => {
                // TODO: Analyze expressions for assignments
                abc.assignments += 1; // Simplified for now
            }
            Statement::Block(statements) => {
                for stmt in statements {
                    Self::collect_abc_complexity(stmt, abc);
                }
            }
            _ => {}
        }
    }
}

impl Default for ComplexityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
