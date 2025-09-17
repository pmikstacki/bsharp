use crate::syntax::ast;
use crate::parser::csharp::parse_csharp_source;
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
}
