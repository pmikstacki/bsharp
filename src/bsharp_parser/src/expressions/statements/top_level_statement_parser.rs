use crate::parser::statement_parser::{parse_statement_spanned, parse_statement_ws_spanned};
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use syntax::statements::statement::Statement;

use nom::Parser;
use nom_supreme::ParserExt;

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
pub fn parse_top_level_statements(input: Span) -> BResult<Vec<Statement>> {
    (|mut current| {
        let mut statements = Vec::new();
        loop {
            let (after_ws, _) = ws(current)?;
            current = after_ws;

            if current.is_empty() {
                break;
            }

            let before_len = current.len();

            match parse_statement_spanned(current) {
                Ok((rest, stmt_s)) => {
                    if rest.len() == before_len {
                        break;
                    }
                    statements.push(stmt_s.node);
                    current = rest;
                }
                Err(nom::Err::Error(e)) => {
                    return Err(nom::Err::Error(e));
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok((current, statements))
    })
    .context("top-level statements")
    .parse(input)
}

/// Parse a single top-level statement
/// This is a wrapper around parse_statement that handles top-level context
pub fn parse_top_level_statement(input: Span) -> BResult<Statement> {
    parse_statement_ws_spanned
        .map(|s| s.node)
        .context("top-level statement")
        .parse(input)
}
use syntax::span::Span;

