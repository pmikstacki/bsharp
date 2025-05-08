pub mod ast;
pub mod nodes;
pub mod errors;
pub mod parser_helpers;
pub mod test_helpers;

//------------------------------------------------------------------------------
// Public Parser API
//------------------------------------------------------------------------------

#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse<'a>(&self, input: &'a str) -> Result<ast::SourceFile<'a>, String> {
        use crate::parsers::csharp::parse_csharp_source;
        use nom::Finish;

        // Use the actual parser implementation from the parsers module
        match parse_csharp_source(input).finish() {
            Ok((_, source_file)) => Ok(source_file),
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

 The parser logic previously defined below (primitive_type, expression,
 statement, class_member, using_directive, namespace_declaration, etc.)
 and the associated test module have been moved to the `src/parsers/` directory
 or are being reimplemented there.

 This section (originally lines 62-375) is commented out to allow the rest 
 of the project to compile while the refactoring is in progress.
================================================================================
*/
