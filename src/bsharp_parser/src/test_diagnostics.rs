#[cfg(feature = "expose_parser_diagnostics")]
pub mod exposed {
    use crate::errors::BResult;
    use syntax::span::Span;


    #[derive(Debug, Clone)]
    pub struct ParserDiag {
        pub code: Option<String>,
        pub span: Option<(usize, usize)>,
        pub message: Option<String>,
    }

    pub fn diagnostics_supported() -> bool {
        // Currently not implemented; will be toggled to true when real diagnostics are exposed.
        false
    }

    pub fn parse_csharp_source_with_diags(
        input: Span<'_>,
    ) -> (
        BResult<'_, crate::syntax::ast::CompilationUnit>,
        Vec<ParserDiag>,
    ) {
        let r = crate::bsharp::parse_csharp_source(input);
        (r, Vec::new())
    }

    pub fn parse_statement_with_diags(
        input: Span<'_>,
    ) -> (
        BResult<'_, crate::syntax::statements::statement::Statement>,
        Vec<ParserDiag>,
    ) {
        // Use the whitespace-tolerant spanned entry used by tests
        let r = crate::statement_parser::parse_statement_ws_spanned(input).map(|(rest, s)| (rest, s.node));
        (r, Vec::new())
    }
}
