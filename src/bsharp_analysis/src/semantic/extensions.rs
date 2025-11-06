use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

pub struct ExtensionsPass;

impl AnalyzerPass for ExtensionsPass {
    fn id(&self) -> &'static str { "semantic.extensions" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.access"] }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {}
}
