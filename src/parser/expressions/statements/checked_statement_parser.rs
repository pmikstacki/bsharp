use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::checked_statement::{CheckedStatement, UncheckedStatement};
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bws, context};
use crate::parser::keywords::exception_and_safety_keywords::{kw_checked, kw_unchecked};

use nom::{branch::alt, combinator::map, sequence::tuple};

/// Parse a checked statement: checked { ... }
pub fn parse_checked_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "checked statement",
        map(
            tuple((
                context("checked keyword", kw_checked()),
                context("checked body", bws(parse_statement_ws)),
            )),
            |(_, body)| {
                Statement::Checked(Box::new(CheckedStatement {
                    body: Box::new(body),
                }))
            },
        ),
    )(input)
}

/// Parse an unchecked statement: unchecked { ... }
pub fn parse_unchecked_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "unchecked statement",
        map(
            tuple((
                context("unchecked keyword", kw_unchecked()),
                context("unchecked body", bws(parse_statement_ws)),
            )),
            |(_, body)| {
                Statement::Unchecked(Box::new(UncheckedStatement {
                    body: Box::new(body),
                }))
            },
        ),
    )(input)
}

/// Parse either a checked or unchecked statement
pub fn parse_checked_unchecked_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "checked or unchecked statement",
        alt((parse_checked_statement, parse_unchecked_statement)),
    )(input)
}
