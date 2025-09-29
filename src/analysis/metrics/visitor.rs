use crate::analysis::framework::session::AnalysisSession;
use crate::analysis::framework::walker::{NodeRef, Visit};
use crate::analysis::metrics::core::AstAnalysis;
use crate::syntax::ast::TopLevelDeclaration;

pub struct MetricsVisitor {
    analysis: AstAnalysis,
}

impl MetricsVisitor {
    pub fn new() -> Self { Self { analysis: AstAnalysis::default() } }
}

impl Visit for MetricsVisitor {
    fn enter(&mut self, node: &NodeRef, _session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu };
        for decl in &cu.declarations {
            match decl {
                TopLevelDeclaration::Class(_) => self.analysis.total_classes += 1,
                TopLevelDeclaration::Interface(_) => self.analysis.total_interfaces += 1,
                TopLevelDeclaration::Struct(_) => self.analysis.total_structs += 1,
                TopLevelDeclaration::Record(_) => self.analysis.total_records += 1,
                TopLevelDeclaration::Enum(_) => self.analysis.total_enums += 1,
                TopLevelDeclaration::Delegate(_) => self.analysis.total_delegates += 1,
                _ => {}
            }
        }
    }

    fn exit(&mut self, node: &NodeRef, session: &mut AnalysisSession) {
        match node {
            NodeRef::CompilationUnit(_) => {
                // Store the analysis artifact for reporting phase
                session.artifacts.insert(self.analysis.clone());
            }
        }
    }
}
