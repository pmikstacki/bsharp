pub mod ast;
pub mod nodes;
pub mod errors;
pub mod parser_helpers;
pub mod test_helpers;
pub mod comment_parser;
// Re-export the new idiomatic navigation traits from analysis as single source of truth
pub use crate::analysis::{AstNavigate, FindDeclarations};

//------------------------------------------------------------------------------
// Public Parser API
//------------------------------------------------------------------------------

#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse(&self, input: &str) -> Result<ast::CompilationUnit, String> {
        use crate::parser::csharp::parse_csharp_source;
        use nom::Finish;

        // Use the actual syntax implementation from the parser module
        match parse_csharp_source(input).finish() {
            Ok((_, compilation_unit)) => Ok(compilation_unit),
            Err(e) => Err(format!("Failed to parse C# code: {:?}", e))
        }
    }
}

//------------------------------------------------------------------------------
// Basic Parsers (Helpers retained for potential reuse)
//------------------------------------------------------------------------------

/*
================================================================================
 Outdated Parsers and Tests Removed/Commented Out During Refactoring (2025-05-03)

 The syntax logic previously defined below (primitive_type, expression,
 statement, class_member, using_directive, namespace_declaration, etc.)
 and the associated test module have been moved to the `src/parser/` directory
 or are being reimplemented there.

 This section (originally lines 62-375) is commented out to allow the rest 
 of the project to compile while the refactoring is in progress.
================================================================================
*/
