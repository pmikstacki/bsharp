use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

pub struct FlowPass;

impl AnalyzerPass for FlowPass {
    fn id(&self) -> &'static str { "semantic.flow" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.generics"] }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {}
}
