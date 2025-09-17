use super::super::core::{AstAnalysis, AstAnalyze};
use crate::syntax::ast::CompilationUnit;

impl AstAnalyze for CompilationUnit {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis::default();

        // Analyze top-level declarations
        for member in &self.declarations {
            match member {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        analysis =
                            analysis.combine(super::namespace::analyze_namespace_member(ns_member));
                    }
                }
                crate::syntax::ast::TopLevelDeclaration::Class(class) => {
                    analysis = analysis.combine(class.analyze());
                }
                crate::syntax::ast::TopLevelDeclaration::Interface(_) => {
                    analysis.total_interfaces += 1;
                }
                crate::syntax::ast::TopLevelDeclaration::Struct(_) => {
                    analysis.total_structs += 1;
                }
                crate::syntax::ast::TopLevelDeclaration::Enum(_) => {
                    analysis.total_enums += 1;
                }
                crate::syntax::ast::TopLevelDeclaration::Record(_) => {
                    analysis.total_records += 1;
                }
                crate::syntax::ast::TopLevelDeclaration::Delegate(_) => {
                    analysis.total_delegates += 1;
                }
                _ => {}
            }
        }

        // Analyze file-scoped namespace if present
        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                analysis =
                    analysis.combine(super::namespace::analyze_namespace_member(declaration));
            }
        }

        // Analyze top-level statements
        for stmt in &self.top_level_statements {
            analysis = analysis.combine(stmt.analyze());
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
