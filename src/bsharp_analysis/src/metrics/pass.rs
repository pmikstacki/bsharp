use crate::framework::AnalysisSession;
use crate::syntax::ast::CompilationUnit;

/// Unified metric pass trait for running metric computations in a consistent way.
pub trait MetricPass: Send + Sync + 'static {
    /// Stable identifier for the pass
    fn id(&self) -> &'static str;
    /// Execute the metric pass over a compilation unit and publish artifacts to the session
    fn run_on_compilation_unit(&self, _cu: &CompilationUnit, _session: &mut AnalysisSession) {}
}
