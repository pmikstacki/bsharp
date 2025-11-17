use crate::parser::SpanTable;
use crate::parser::bsharp::{parse_csharp_source, parse_csharp_source_with_spans};
use nom::Finish;
use syntax::ast::CompilationUnit;
use syntax::spans::span_db::SpanDb;
use crate::span_db_build::build_span_db_from_table;

/// Public parser facade re-exported by `syntax`.
#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse<S: AsRef<str>>(&self, input: S) -> Result<CompilationUnit, String> {
        let span = Span::new(input.as_ref());
        match parse_csharp_source(span).finish() {
            Ok((remaining, compilation_unit)) => {
                // Treat significant trailing input as a parse error to surface failures in CLI
                if remaining.trim().is_empty() {
                    Ok(compilation_unit)
                } else {
                    let preview: String = remaining.chars().take(80).collect();
                    Err(format!(
                        "Unparsed trailing input after parse: {:?}",
                        preview
                    ))
                }
            }
            Err(e) => Err(format!("Failed to parse C# code: {:?}", e)),
        }
    }

    /// Parse and also return a table of byte-span ranges for top-level declarations.
    pub fn parse_with_spans<S: AsRef<str>>(
        &self,
        input: S,
    ) -> Result<(CompilationUnit, SpanTable), String> {
        let span = Span::new(input.as_ref());
        match parse_csharp_source_with_spans(span).finish() {
            Ok((_, result)) => Ok(result),
            Err(e) => Err(format!("Failed to parse C# code (with spans): {:?}", e)),
        }
    }

    /// Parse and also return a populated SpanDb alongside the CompilationUnit.
    /// Currently built from the legacy string-keyed SpanTable for compatibility.
    pub fn parse_with_span_db<S: AsRef<str>>(&self, input: S) -> Result<(CompilationUnit, SpanDb), String> {
        let (cu, table) = self.parse_with_spans(input)?;
        let db = build_span_db_from_table(&cu, &table);
        Ok((cu, db))
    }
}
use syntax::span::Span;

