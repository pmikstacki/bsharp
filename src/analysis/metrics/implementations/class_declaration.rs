use super::super::core::{AstAnalyze, AstAnalysis};
use crate::syntax::nodes::declarations::ClassDeclaration;

impl AstAnalyze for ClassDeclaration {
    fn analyze(&self) -> AstAnalysis {
        let mut analysis = AstAnalysis {
            total_classes: 1,
            documented_classes: if self.documentation.is_some() { 1 } else { 0 },
            ..Default::default()
        };
        
        for member in &self.body_declarations {
            match member {
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Method(method) => {
                    let method_analysis = method.analyze();
                    analysis = analysis.combine(method_analysis);
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Field(_) => {
                    analysis.total_fields += 1;
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Property(_) => {
                    analysis.total_properties += 1;
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Event(_) => {
                    analysis.total_events += 1;
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::Constructor(_) => {
                    analysis.total_constructors += 1;
                    analysis.total_methods += 1; // Constructors should also count as methods
                }
                crate::syntax::nodes::declarations::ClassBodyDeclaration::NestedClass(nested_class) => {
                    analysis = analysis.combine(nested_class.analyze());
                }
                _ => {}
            }
        }
        
        analysis
    }
} 