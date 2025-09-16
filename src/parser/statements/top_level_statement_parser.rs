use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::parser::statement_parser::parse_statement;
use crate::syntax::parser_helpers::context;
use crate::syntax::comment_parser::ws;
use nom::multi::many0;
use nom::sequence::terminated;

/// Parse top-level statements (C# 9+)
/// Top-level statements are statements that appear at the top level of a file,
/// outside of any class or method. They are implicitly wrapped in a Main method.
/// 
/// Example:
/// ```csharp
/// using System;
/// 
/// Console.WriteLine("Hello, World!");
/// var name = "Alice";
/// Console.WriteLine($"Hello, {name}!");
/// ```
pub fn parse_top_level_statements(input: &str) -> BResult<&str, Vec<Statement>> {
    context(
        "top-level statements (expected zero or more valid statements at the file root)",
        many0(
            terminated(
                parse_statement,
                ws // consume whitespace/comments after each statement
            )
        )
    )(input)
}

/// Parse a single top-level statement
/// This is a wrapper around parse_statement that handles top-level context
pub fn parse_top_level_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "top-level statement (expected a valid statement at the file root)",
        parse_statement,
    )(input)
} 