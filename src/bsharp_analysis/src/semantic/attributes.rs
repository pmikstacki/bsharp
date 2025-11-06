use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::CompilationUnit;

pub struct AttributesPass;

impl AnalyzerPass for AttributesPass {
    fn id(&self) -> &'static str { "semantic.attributes" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.nullability"] }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {}
}
