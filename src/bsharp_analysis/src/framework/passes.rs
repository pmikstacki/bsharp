use crate::framework::AnalysisSession;
use crate::syntax::ast::CompilationUnit;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Phase {
    Index,
    LocalRules,
    Global,
    Semantic,
    Reporting,
}

pub trait AnalyzerPass: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn phase(&self) -> Phase;
    fn depends_on(&self) -> &'static [&'static str] {
        &[]
    }
    fn run(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {}
}
