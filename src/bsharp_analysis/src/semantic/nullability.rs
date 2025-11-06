use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

pub struct NullabilityPass;

impl AnalyzerPass for NullabilityPass {
    fn id(&self) -> &'static str { "semantic.nullability" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.flow"] }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {}
}
