use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

pub struct AccessPass;

impl AnalyzerPass for AccessPass {
    fn id(&self) -> &'static str { "semantic.access" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.attributes"] }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {}
}
