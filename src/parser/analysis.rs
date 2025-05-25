use crate::parser::ast::CompilationUnit;
use crate::parser::nodes::declarations::{ClassDeclaration, MethodDeclaration};
use crate::parser::nodes::statements::statement::Statement;

/// Trait for analyzing AST nodes and extracting statistics
pub trait AstAnalyze {
    /// Get analysis statistics for this AST node
    fn analyze(&self) -> AstAnalysis;
}

/// Statistics about an AST structure
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AstAnalysis {
    pub total_classes: usize,
    pub total_methods: usize,
    pub total_if_statements: usize,
    pub total_for_loops: usize,
    pub total_while_loops: usize,
    pub total_switch_statements: usize,
}

impl AstAnalysis {
    /// Combine two analyses by adding their counts
    pub fn combine(self, other: AstAnalysis) -> AstAnalysis {
        AstAnalysis {
            total_classes: self.total_classes + other.total_classes,
            total_methods: self.total_methods + other.total_methods,
            total_if_statements: self.total_if_statements + other.total_if_statements,
            total_for_loops: self.total_for_loops + other.total_for_loops,
            total_while_loops: self.total_while_loops + other.total_while_loops,
            total_switch_statements: self.total_switch_statements + other.total_switch_statements,
        }
    }
}

impl AstAnalyze for CompilationUnit {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis::default();
        
        for member in &self.declarations {
            match member {
                crate::parser::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class) = ns_member {
                            analysis = analysis.combine(class.analyze());
                        }
                    }
                }
                crate::parser::ast::TopLevelDeclaration::Class(class) => {
                    analysis = analysis.combine(class.analyze());
                }
                _ => {}
            }
        }
        
        analysis
    }
}

impl AstAnalyze for ClassDeclaration {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis {
            total_classes: 1,
            ..Default::default()
        };
        
        for member in &self.body_declarations {
            if let crate::parser::nodes::declarations::ClassBodyDeclaration::Method(method) = member {
                analysis = analysis.combine(method.analyze());
            }
        }
        
        analysis
    }
}

impl AstAnalyze for MethodDeclaration {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis {
            total_methods: 1,
            ..Default::default()
        };
        
        if let Some(body) = &self.body {
            analysis = analysis.combine(body.analyze());
        }
        
        analysis
    }
}

impl AstAnalyze for Statement {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis::default();
        
        match self {
            Statement::If(if_stmt) => {
                analysis.total_if_statements += 1;
                analysis = analysis.combine(if_stmt.consequence.analyze());
                if let Some(alt) = &if_stmt.alternative {
                    analysis = analysis.combine(alt.analyze());
                }
            }
            Statement::For(for_stmt) => {
                analysis.total_for_loops += 1;
                analysis = analysis.combine(for_stmt.body.analyze());
            }
            Statement::While(while_stmt) => {
                analysis.total_while_loops += 1;
                analysis = analysis.combine(while_stmt.body.analyze());
            }
            Statement::DoWhile(do_while_stmt) => {
                analysis.total_while_loops += 1;
                analysis = analysis.combine(do_while_stmt.body.analyze());
            }
            Statement::Switch(switch_stmt) => {
                analysis.total_switch_statements += 1;
                for section in &switch_stmt.sections {
                    for stmt in &section.statements {
                        analysis = analysis.combine(stmt.analyze());
                    }
                }
            }
            Statement::Block(statements) => {
                for stmt in statements {
                    analysis = analysis.combine(stmt.analyze());
                }
            }
            _ => {}
        }
        
        analysis
    }
}

/// Extension methods for CompilationUnit
impl CompilationUnit {
    /// Quick analysis of the compilation unit
    pub fn quick_analysis(&self) -> AstAnalysis {
        self.analyze()
    }
} 