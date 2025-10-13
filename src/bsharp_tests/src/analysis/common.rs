use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::report::AnalysisReport;
use analysis::syntax::ast::CompilationUnit;
use parser::facade::Parser;

pub fn parse_with_session(src: &str) -> (CompilationUnit, AnalysisSession) {
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(src).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", src);
    let session = AnalysisSession::new(ctx, spans);
    (cu, session)
}

pub fn run_pipeline(src: &str) -> AnalysisReport {
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("test.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    AnalysisReport::from_session(&session)
}
