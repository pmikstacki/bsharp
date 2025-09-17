use super::super::core::{AstAnalysis, AstAnalyze};
use crate::syntax::nodes::declarations::MethodDeclaration;

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
