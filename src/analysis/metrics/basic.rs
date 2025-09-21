use crate::syntax::ast::CompilationUnit;
use crate::syntax::nodes::declarations::{ClassDeclaration, MethodDeclaration};
use crate::syntax::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

/// Basic code metrics for counting fundamental elements
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BasicMetrics {
    // Type counts
    pub total_classes: usize,
    pub total_interfaces: usize,
    pub total_structs: usize,
    pub total_enums: usize,
    pub total_records: usize,
    pub total_delegates: usize,

    // Member counts
    pub total_methods: usize,
    pub total_properties: usize,
    pub total_fields: usize,
    pub total_events: usize,
    pub total_constructors: usize,

    // Statement counts
    pub total_if_statements: usize,
    pub total_for_loops: usize,
    pub total_while_loops: usize,
    pub total_switch_statements: usize,
    pub total_try_statements: usize,
    pub total_using_statements: usize,

    // Lines of code metrics
    pub physical_lines: usize,
    pub logical_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
}

impl BasicMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    /// Combine two basic metrics by adding their counts
    pub fn combine(self, other: BasicMetrics) -> BasicMetrics {
        BasicMetrics {
            total_classes: self.total_classes + other.total_classes,
            total_interfaces: self.total_interfaces + other.total_interfaces,
            total_structs: self.total_structs + other.total_structs,
            total_enums: self.total_enums + other.total_enums,
            total_records: self.total_records + other.total_records,
            total_delegates: self.total_delegates + other.total_delegates,

            total_methods: self.total_methods + other.total_methods,
            total_properties: self.total_properties + other.total_properties,
            total_fields: self.total_fields + other.total_fields,
            total_events: self.total_events + other.total_events,
            total_constructors: self.total_constructors + other.total_constructors,

            total_if_statements: self.total_if_statements + other.total_if_statements,
            total_for_loops: self.total_for_loops + other.total_for_loops,
            total_while_loops: self.total_while_loops + other.total_while_loops,
            total_switch_statements: self.total_switch_statements + other.total_switch_statements,
            total_try_statements: self.total_try_statements + other.total_try_statements,
            total_using_statements: self.total_using_statements + other.total_using_statements,

            physical_lines: self.physical_lines + other.physical_lines,
            logical_lines: self.logical_lines + other.logical_lines,
            comment_lines: self.comment_lines + other.comment_lines,
            blank_lines: self.blank_lines + other.blank_lines,
        }
    }

    /// Get total types count
    pub fn total_types(&self) -> usize {
        self.total_classes
            + self.total_interfaces
            + self.total_structs
            + self.total_enums
            + self.total_records
            + self.total_delegates
    }

    /// Get total members count
    pub fn total_members(&self) -> usize {
        self.total_methods
            + self.total_properties
            + self.total_fields
            + self.total_events
            + self.total_constructors
    }

    /// Get total control structures count
    pub fn total_control_structures(&self) -> usize {
        self.total_if_statements
            + self.total_for_loops
            + self.total_while_loops
            + self.total_switch_statements
            + self.total_try_statements
            + self.total_using_statements
    }

    /// Calculate code density (logical lines / physical lines)
    pub fn code_density(&self) -> f64 {
        if self.physical_lines == 0 {
            0.0
        } else {
            self.logical_lines as f64 / self.physical_lines as f64
        }
    }

    /// Calculate comment ratio (comment lines / total lines)
    pub fn comment_ratio(&self) -> f64 {
        if self.physical_lines == 0 {
            0.0
        } else {
            self.comment_lines as f64 / self.physical_lines as f64
        }
    }
}

/// Basic metrics collector
pub struct BasicMetricsCollector {
    metrics: BasicMetrics,
}

impl BasicMetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: BasicMetrics::new(),
        }
    }

    /// Collect basic metrics from a compilation unit
    pub fn collect_from_compilation_unit(&mut self, unit: &CompilationUnit) {
        // TODO: Implement collection from compilation unit
        // This would traverse the AST and count elements
        let _ = unit; // Suppress unused warning for now
    }

    /// Collect basic metrics from a class
    pub fn collect_from_class(&mut self, class: &ClassDeclaration) {
        self.metrics.total_classes += 1;

        // Count members and analyze their bodies
        for member in &class.body_declarations {
            match member {
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(method) => {
                    self.metrics.total_methods += 1;
                    self.collect_from_method(method);
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Field(_) => {
                    self.metrics.total_fields += 1;
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Property(_) => {
                    self.metrics.total_properties += 1;
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Event(_) => {
                    self.metrics.total_events += 1;
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Constructor(
                    constructor,
                ) => {
                    self.metrics.total_constructors += 1;
                    if let Some(body) = &constructor.body {
                        self.collect_from_statement(body);
                    }
                }
                _ => {}
            }
        }
    }

    /// Collect basic metrics from a method
    pub fn collect_from_method(&mut self, method: &MethodDeclaration) {
        if let Some(body) = &method.body {
            self.collect_from_statement(body);
        }
    }

    /// Collect basic metrics from a statement
    pub fn collect_from_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::If(if_stmt) => {
                self.metrics.total_if_statements += 1;
                self.metrics.logical_lines += 1; // Count the if statement itself
                self.collect_from_statement(&if_stmt.consequence);
                if let Some(alternative) = &if_stmt.alternative {
                    self.collect_from_statement(alternative);
                }
            }
            Statement::For(for_stmt) => {
                self.metrics.total_for_loops += 1;
                self.metrics.logical_lines += 1; // Count the for statement itself
                self.collect_from_statement(&for_stmt.body);
            }
            Statement::While(while_stmt) => {
                self.metrics.total_while_loops += 1;
                self.metrics.logical_lines += 1; // Count the while statement itself
                self.collect_from_statement(&while_stmt.body);
            }
            Statement::DoWhile(do_while_stmt) => {
                self.metrics.total_while_loops += 1;
                self.metrics.logical_lines += 1; // Count the do-while statement itself
                self.collect_from_statement(&do_while_stmt.body);
            }
            Statement::Switch(switch_stmt) => {
                self.metrics.total_switch_statements += 1;
                self.metrics.logical_lines += 1; // Count the switch statement itself
                for section in &switch_stmt.sections {
                    for stmt in &section.statements {
                        self.collect_from_statement(stmt);
                    }
                }
            }
            Statement::Try(try_stmt) => {
                self.metrics.total_try_statements += 1;
                self.metrics.logical_lines += 1; // Count the try statement itself
                self.collect_from_statement(&try_stmt.try_block);
                for catch in &try_stmt.catches {
                    self.collect_from_statement(&catch.block);
                }
                if let Some(finally) = &try_stmt.finally_clause {
                    self.collect_from_statement(&finally.block);
                }
            }
            Statement::Using(using_stmt) => {
                self.metrics.total_using_statements += 1;
                self.metrics.logical_lines += 1; // Count the using statement itself
                if let Some(body) = &using_stmt.body {
                    self.collect_from_statement(body);
                }
            }
            Statement::Block(statements) => {
                // Count only non-empty blocks as logical lines
                if !statements.is_empty() {
                    self.metrics.logical_lines += 1;
                }
                for stmt in statements {
                    self.collect_from_statement(stmt);
                }
            }
            Statement::Expression(_)
            | Statement::Return(_)
            | Statement::Throw(_)
            | Statement::Break(_)
            | Statement::Continue(_)
            | Statement::Declaration(_) => {
                // These are simple statements that count as logical lines
                self.metrics.logical_lines += 1;
            }
            Statement::Empty => {
                // Empty statements don't count as logical lines
            }
            _ => {
                // Other statements count as logical lines
                self.metrics.logical_lines += 1;
            }
        }
    }

    /// Get the collected metrics
    pub fn get_metrics(&self) -> &BasicMetrics {
        &self.metrics
    }

    /// Reset the collector
    pub fn reset(&mut self) {
        self.metrics = BasicMetrics::new();
    }
}

impl Default for BasicMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
