use super::super::core::{AstAnalysis, AstAnalyze};
use bsharp_syntax::declarations::NamespaceBodyDeclaration;

pub fn analyze_namespace_member(member: &NamespaceBodyDeclaration) -> AstAnalysis {
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
                    crate::syntax::nodes::declarations::StructBodyDeclaration::Event(_) => {
                        analysis.total_events += 1;
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::Indexer(_) => {
                        // Treat indexers like properties for analysis summarization
                        analysis.total_properties += 1;
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::Operator(_) => {
                        // Treat operators as methods for analysis summarization
                        analysis.total_methods += 1;
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::NestedClass(class) => {
                        // Recurse into nested class
                        analysis = analysis.combine(class.analyze());
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::NestedStruct(nested) => {
                        // Shallow analysis: count struct and iterate its members
                        analysis.total_structs += 1;
                        for m in &nested.body_declarations {
                            match m {
                                crate::syntax::nodes::declarations::StructBodyDeclaration::Method(method) => {
                                    analysis = analysis.combine(method.analyze());
                                }
                                crate::syntax::nodes::declarations::StructBodyDeclaration::Field(_) => analysis.total_fields += 1,
                                crate::syntax::nodes::declarations::StructBodyDeclaration::Property(_) => analysis.total_properties += 1,
                                crate::syntax::nodes::declarations::StructBodyDeclaration::Constructor(_) => {
                                    analysis.total_constructors += 1;
                                    analysis.total_methods += 1;
                                }
                                crate::syntax::nodes::declarations::StructBodyDeclaration::Event(_) => analysis.total_events += 1,
                                crate::syntax::nodes::declarations::StructBodyDeclaration::Indexer(_) => analysis.total_properties += 1,
                                crate::syntax::nodes::declarations::StructBodyDeclaration::Operator(_) => analysis.total_methods += 1,
                                _ => {}
                            }
                        }
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::NestedInterface(_) => {
                        analysis.total_interfaces += 1;
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::NestedEnum(_) => {
                        analysis.total_enums += 1;
                    }
                    crate::syntax::nodes::declarations::StructBodyDeclaration::NestedRecord(_) => {
                        analysis.total_records += 1;
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
