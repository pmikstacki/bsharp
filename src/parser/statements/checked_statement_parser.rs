use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::checked_statement::{CheckedStatement, UncheckedStatement};
use crate::syntax::parser_helpers::{keyword, bws};
use crate::parser::statement_parser::parse_statement_ws;

use nom_supreme::ParserExt;
use nom::Parser;
use nom::{
    branch::alt,
    combinator::map,
    sequence::tuple,
};

/// Parse a checked statement: checked { ... }
pub fn parse_checked_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            keyword("checked").context("checked keyword"),
            bws(parse_statement_ws).context("checked body"),
        )),
        |(_, body)| {
            Statement::Checked(Box::new(CheckedStatement { 
                body: Box::new(body) 
            }))
        },
    )
    .context("checked statement")
    .parse(input)
}

/// Parse an unchecked statement: unchecked { ... }
pub fn parse_unchecked_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            keyword("unchecked").context("unchecked keyword"),
            bws(parse_statement_ws).context("unchecked body"),
        )),
        |(_, body)| {
            Statement::Unchecked(Box::new(UncheckedStatement { 
                body: Box::new(body) 
            }))
        },
    )
    .context("unchecked statement")
    .parse(input)
}

/// Parse either a checked or unchecked statement
pub fn parse_checked_unchecked_statement(input: &str) -> BResult<&str, Statement> {
    alt((
        parse_checked_statement,
        parse_unchecked_statement,
    ))
    .context("checked or unchecked statement")
    .parse(input)
} 