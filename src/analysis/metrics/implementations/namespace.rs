use super::super::core::{AstAnalyze, AstAnalysis};

pub fn analyze_namespace_member(member: &crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration) -> AstAnalysis {
    match member {
        crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(class) => {
            class.analyze()
        }
        crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Interface(_) => {
            AstAnalysis { total_interfaces: 1, ..Default::default() }
        }
        crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Struct(struct_decl) => {
            // Analyze struct like a class - count its methods too
            let mut analysis = AstAnalysis { total_structs: 1, ..Default::default() };
            
            // Analyze struct members
            for member in &struct_decl.body_declarations {
                match member {
                    crate::syntax::nodes::declarations::StructBodyDeclaration::Method(method) => {
                        let method_analysis = method.analyze();
                        analysis = analysis.combine(method_analysis);
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::Field(_) => {
                        analysis.total_fields += 1;
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::Property(_) => {
                        analysis.total_properties += 1;
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::Constructor(_) => {
                        analysis.total_constructors += 1;
                        analysis.total_methods += 1; // Constructors should also count as methods
                    }
                }
            }
            
            analysis
        }
        crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Enum(_) => {
            AstAnalysis { total_enums: 1, ..Default::default() }
        }
        crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Record(_) => {
            AstAnalysis { total_records: 1, ..Default::default() }
        }
        crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Delegate(_) => {
            AstAnalysis { total_delegates: 1, ..Default::default() }
        }
        crate::syntax::nodes::declarations::namespace_declaration::NamespaceBodyDeclaration::Namespace(ns) => {
            let mut analysis = AstAnalysis::default();
            for nested_member in &ns.declarations {
                analysis = analysis.combine(analyze_namespace_member(nested_member));
            }
            analysis
        }
        _ => AstAnalysis::default(), // Wildcard arm for unhandled NamespaceBodyDeclaration types
    }
} 