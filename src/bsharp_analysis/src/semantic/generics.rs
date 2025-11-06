use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

pub struct GenericsPass;

impl AnalyzerPass for GenericsPass {
    fn id(&self) -> &'static str { "semantic.generics" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.overload"] }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {}
}
