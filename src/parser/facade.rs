use crate::syntax::ast;
use crate::parser::bsharp::{parse_csharp_source, parse_csharp_source_with_spans};
use crate::parser::SpanTable;
use nom::Finish;

/// Public parser facade re-exported by `syntax`.
#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, input: &str) -> Result<ast::CompilationUnit, String> {
        match parse_csharp_source(input).finish() {
            Ok((_, compilation_unit)) => Ok(compilation_unit),
            Err(e) => Err(format!("Failed to parse C# code: {:?}", e)),
        }
    }

    /// Parse and also return a table of byte-span ranges for top-level declarations.
    pub fn parse_with_spans(&self, input: &str) -> Result<(ast::CompilationUnit, SpanTable), String> {
        match parse_csharp_source_with_spans(input).finish() {
            Ok((_, result)) => Ok(result),
            Err(e) => Err(format!("Failed to parse C# code (with spans): {:?}", e)),
        }
    }
}
