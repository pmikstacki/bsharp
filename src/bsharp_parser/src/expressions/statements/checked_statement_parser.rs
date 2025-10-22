use crate::parser::keywords::exception_and_safety_keywords::{kw_checked, kw_unchecked};
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::Parser;
use nom::sequence::delimited;
use nom::{branch::alt, combinator::map};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::{CheckedStatement, UncheckedStatement};

/// Parse a checked statement: checked { ... }
pub fn parse_checked_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_checked().context("checked keyword"),
            nom::combinator::cut(delimited(ws, parse_statement_ws, ws)).context("checked body"),
        ),
        |(_, body)| {
            Statement::Checked(Box::new(CheckedStatement {
                body: Box::new(body),
            }))
        },
    )
    .context("checked statement")
    .parse(input)
}

/// Parse an unchecked statement: unchecked { ... }
pub fn parse_unchecked_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_unchecked().context("unchecked keyword"),
            nom::combinator::cut(delimited(ws, parse_statement_ws, ws)).context("unchecked body"),
        ),
        |(_, body)| {
            Statement::Unchecked(Box::new(UncheckedStatement {
                body: Box::new(body),
            }))
        },
    )
    .context("unchecked statement")
    .parse(input)
}

/// Parse either a checked or unchecked statement
pub fn parse_checked_unchecked_statement(input: Span) -> BResult<Statement> {
    alt((parse_checked_statement, parse_unchecked_statement))
        .context("checked or unchecked statement")
        .parse(input)
}
use crate::syntax::span::Span;
