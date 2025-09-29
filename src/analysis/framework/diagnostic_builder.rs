use crate::analysis::diagnostics::diagnostic::Diagnostic;
use crate::analysis::diagnostics::diagnostic_code::DiagnosticCode;
use crate::analysis::framework::session::AnalysisSession;
use crate::analysis::DiagnosticSeverity;

pub struct DiagnosticBuilder {
    code: DiagnosticCode,
    message: Option<String>,
    location: Option<crate::analysis::diagnostics::source_location::SourceLocation>,
    related: Vec<crate::analysis::diagnostics::diagnostic_related_information::DiagnosticRelatedInformation>,
}

impl DiagnosticBuilder {
    pub fn new(code: DiagnosticCode) -> Self {
        Self { code, message: None, location: None, related: Vec::new() }
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }

    pub fn at_span(mut self, session: &AnalysisSession, start: usize, len: usize) -> Self {
        let loc = session.ctx.location_from_span(start, len);
        self.location = Some(loc);
        self
    }

    pub fn at_key(mut self, session: &AnalysisSession, key: &str) -> Self {
        if let Some(range) = session.spans.get(key) {
            let loc = session.ctx.location_from_range(range.start, range.end);
            self.location = Some(loc);
        }
        self
    }

    pub fn with_related_at_key(mut self, session: &AnalysisSession, key: &str, message: impl Into<String>) -> Self {
        if let Some(range) = session.spans.get(key) {
            let info = crate::analysis::diagnostics::diagnostic_related_information::DiagnosticRelatedInformation::from_span(&session.ctx, range.start, range.end - range.start, message);
            self.related.push(info);
        }
        self
    }

    pub fn emit(self, session: &mut AnalysisSession) {
        let msg = self.message.unwrap_or_else(|| self.code.default_message().to_string());
        let mut d = Diagnostic::new(self.code, msg);
        if let Some(loc) = self.location { d = d.with_location(loc); }
        for r in self.related { d = d.with_related(r); }
        // Apply severity override if configured
        if let Some(sev) = session.config.rule_severities.get(self.code.as_str()) {
            d.severity = match sev {
                DiagnosticSeverity::Error => DiagnosticSeverity::Error,
                DiagnosticSeverity::Warning => DiagnosticSeverity::Warning,
                DiagnosticSeverity::Info => DiagnosticSeverity::Info,
                DiagnosticSeverity::Hint => DiagnosticSeverity::Hint,
            };
        }
        session.diagnostics.add(d);
    }
}
