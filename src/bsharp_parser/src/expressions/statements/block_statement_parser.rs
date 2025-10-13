use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom::combinator::map;
use nom::combinator::{cut, not, peek};
use nom::multi::many0;
use nom::sequence::delimited;
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

// Function to extract statements from a Block statement, if that's what it is
// This is a helper for tests and other code that expects a Vec<Statement>
pub fn extract_statements_from_block(statement: Statement) -> Vec<Statement> {
    match statement {
        Statement::Block(statements) => statements,
        _ => vec![statement], // If it's not a block, wrap it in a Vec as a fallback
    }
}

/// Block statement syntax with enhanced error context
/// Uses nom's context function for better error messages
/// Uses parse_statement_for_block_ws to prevent infinite recursion with nested blocks
pub fn parse_block_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        delimited(
            delimited(ws, nom_char('{'), ws)
                .context("opening brace for block statement"),
            many0(|i| {
                // Do not attempt a statement if next non-ws is '}'
                let guard = nom::combinator::map(delimited(ws, nom_char('}'), ws), |_| ());
                peek(not(guard)).parse(i)?;
                parse_statement_ws(i)
            })
            .context("statement in block"),
            cut(delimited(ws, nom_char('}'), ws))
                .context("closing brace for block statement"),
        ),
        Statement::Block,
    )
    .context("block statement")
    .parse(input)
}
use crate::syntax::span::Span;
