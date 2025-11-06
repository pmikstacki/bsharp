use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

pub struct TypeCheckPass;

impl AnalyzerPass for TypeCheckPass {
    fn id(&self) -> &'static str { "semantic.types" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.binding"] }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {
        // no-op in this iteration
    }
}
