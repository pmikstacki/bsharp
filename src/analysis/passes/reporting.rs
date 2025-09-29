use crate::analysis::framework::passes::{AnalyzerPass, Phase};
use crate::analysis::framework::session::AnalysisSession;
use crate::syntax::ast::CompilationUnit;

pub struct ReportingPass;

impl AnalyzerPass for ReportingPass {
    fn id(&self) -> &'static str { "passes.reporting" }
    fn phase(&self) -> Phase { Phase::Reporting }

    fn run(&self, _cu: &CompilationUnit, session: &mut AnalysisSession) {
        // Deterministic ordering of diagnostics: (file, line, column, code)
        session.diagnostics.diagnostics.sort_by(|a, b| {
            let (af, al, ac) = a.location.as_ref()
                .map(|l| (l.file.clone(), l.line, l.column))
                .unwrap_or_else(|| (String::new(), 0, 0));
            let (bf, bl, bc) = b.location.as_ref()
                .map(|l| (l.file.clone(), l.line, l.column))
                .unwrap_or_else(|| (String::new(), 0, 0));
            (af, al, ac, a.code.as_str()).cmp(&(bf, bl, bc, b.code.as_str()))
        });

        // Build final report and store as an artifact for consumers (CLI/tests)
        let report = crate::analysis::AnalysisReport::from_session(session);
        session.artifacts.insert(report);
    }
}
