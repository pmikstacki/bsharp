use super::core::{AstAnalyze, AstAnalysis};
use crate::parser::ast::CompilationUnit;
use crate::parser::nodes::declarations::{ClassDeclaration, MethodDeclaration};
use crate::parser::nodes::statements::statement::Statement;

impl AstAnalyze for CompilationUnit {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis::default();
        
        // Analyze top-level declarations
        for member in &self.declarations {
            match member {
                crate::parser::ast::TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        analysis = analysis.combine(analyze_namespace_member(ns_member));
                    }
                }
                crate::parser::ast::TopLevelDeclaration::Class(class) => {
                    analysis = analysis.combine(class.analyze());
                }
                crate::parser::ast::TopLevelDeclaration::Interface(_) => {
                    analysis.total_interfaces += 1;
                }
                crate::parser::ast::TopLevelDeclaration::Struct(_) => {
                    analysis.total_structs += 1;
                }
                crate::parser::ast::TopLevelDeclaration::Enum(_) => {
                    analysis.total_enums += 1;
                }
                crate::parser::ast::TopLevelDeclaration::Record(_) => {
                    analysis.total_records += 1;
                }
                crate::parser::ast::TopLevelDeclaration::Delegate(_) => {
                    analysis.total_delegates += 1;
                }
                _ => {}
            }
        }
        
        // Analyze file-scoped namespace if present
        if let Some(file_scoped_ns) = &self.file_scoped_namespace {
            for declaration in &file_scoped_ns.declarations {
                analysis = analysis.combine(analyze_namespace_member(declaration));
            }
        }
        
        // Analyze top-level statements
        for stmt in &self.top_level_statements {
            analysis = analysis.combine(stmt.analyze());
        }
        
        analysis
    }
}

fn analyze_namespace_member(member: &crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration) -> AstAnalysis {
    match member {
        crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class) => {
            class.analyze()
        }
        crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Interface(_) => {
            AstAnalysis { total_interfaces: 1, ..Default::default() }
        }
        crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Struct(struct_decl) => {
            // Analyze struct like a class - count its methods too
            let mut analysis = AstAnalysis { total_structs: 1, ..Default::default() };
            
            // Analyze struct members
            for member in &struct_decl.body_declarations {
                match member {
                    crate::parser::nodes::declarations::StructBodyDeclaration::Method(method) => {
                        let method_analysis = method.analyze();
                        analysis = analysis.combine(method_analysis);
                    }
                    crate::parser::nodes::declarations::StructBodyDeclaration::Field(_) => {
                        analysis.total_fields += 1;
                    }
                    crate::parser::nodes::declarations::StructBodyDeclaration::Property(_) => {
                        analysis.total_properties += 1;
                    }
                    crate::parser::nodes::declarations::StructBodyDeclaration::Constructor(_) => {
                        analysis.total_constructors += 1;
                        analysis.total_methods += 1; // Constructors should also count as methods
                    }
                }
            }
            
            analysis
        }
        crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Enum(_) => {
            AstAnalysis { total_enums: 1, ..Default::default() }
        }
        crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Record(_) => {
            AstAnalysis { total_records: 1, ..Default::default() }
        }
        crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Delegate(_) => {
            AstAnalysis { total_delegates: 1, ..Default::default() }
        }
        crate::parser::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Namespace(ns) => {
            let mut analysis = AstAnalysis::default();
            for nested_member in &ns.declarations {
                analysis = analysis.combine(analyze_namespace_member(nested_member));
            }
            analysis
        }
        _ => AstAnalysis::default(), // Wildcard arm for unhandled NamespaceBodyDeclaration types
    }
}

impl AstAnalyze for ClassDeclaration {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis {
            total_classes: 1,
            documented_classes: if self.documentation.is_some() { 1 } else { 0 },
            ..Default::default()
        };
        
        for member in &self.body_declarations {
            match member {
                crate::parser::nodes::declarations::ClassBodyDeclaration::Method(method) => {
                    let method_analysis = method.analyze();
                    analysis = analysis.combine(method_analysis);
                }
                crate::parser::nodes::declarations::ClassBodyDeclaration::Field(_) => {
                    analysis.total_fields += 1;
                }
                crate::parser::nodes::declarations::ClassBodyDeclaration::Property(_) => {
                    analysis.total_properties += 1;
                }
                crate::parser::nodes::declarations::ClassBodyDeclaration::Event(_) => {
                    analysis.total_events += 1;
                }
                crate::parser::nodes::declarations::ClassBodyDeclaration::Constructor(_) => {
                    analysis.total_constructors += 1;
                    analysis.total_methods += 1; // Constructors should also count as methods
                }
                crate::parser::nodes::declarations::ClassBodyDeclaration::NestedClass(nested_class) => {
                    analysis = analysis.combine(nested_class.analyze());
                }
                _ => {}
            }
        }
        
        analysis
    }
}

impl AstAnalyze for MethodDeclaration {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis {
            total_methods: 1,
            documented_methods: 0, // if self.documentation.is_some() { 1 } else { 0 },
            cyclomatic_complexity: 1, // Base complexity
            ..Default::default()
        };
        
        if let Some(body) = &self.body {
            let body_analysis = body.analyze();
            analysis = analysis.combine(body_analysis);
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
                analysis.cyclomatic_complexity += 1; // Each branch adds complexity
                analysis = analysis.combine(if_stmt.consequence.analyze());
                if let Some(alt) = &if_stmt.alternative {
                    analysis = analysis.combine(alt.analyze());
                }
            }
            Statement::For(for_stmt) => {
                analysis.total_for_loops += 1;
                analysis.cyclomatic_complexity += 1;
                analysis = analysis.combine(for_stmt.body.analyze());
            }
            Statement::While(while_stmt) => {
                analysis.total_while_loops += 1;
                analysis.cyclomatic_complexity += 1;
                analysis = analysis.combine(while_stmt.body.analyze());
            }
            Statement::DoWhile(do_while_stmt) => {
                analysis.total_while_loops += 1;
                analysis.cyclomatic_complexity += 1;
                analysis = analysis.combine(do_while_stmt.body.analyze());
            }
            Statement::Switch(switch_stmt) => {
                analysis.total_switch_statements += 1;
                analysis.cyclomatic_complexity += switch_stmt.sections.len(); // Each case adds complexity
                for section in &switch_stmt.sections {
                    for stmt in &section.statements {
                        analysis = analysis.combine(stmt.analyze());
                    }
                }
            }
            Statement::Try(_) => {
                analysis.total_try_statements += 1;
                analysis.cyclomatic_complexity += 1;
                // TODO: Analyze try/catch/finally blocks when implemented
            }
            Statement::Using(_) => {
                analysis.total_using_statements += 1;
                // TODO: Analyze using statement body when implemented
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