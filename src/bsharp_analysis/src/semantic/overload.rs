use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

pub struct OverloadPass;

impl AnalyzerPass for OverloadPass {
    fn id(&self) -> &'static str { "semantic.overload" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.types"] }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {
        // no-op stub
    }
}
