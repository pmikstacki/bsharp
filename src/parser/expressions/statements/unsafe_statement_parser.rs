use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::statements::unsafe_statement::UnsafeStatement;
use crate::syntax::parser_helpers::{bws, keyword};

use nom::Parser;
use nom::{combinator::map, sequence::tuple};
use nom_supreme::ParserExt;

/// Parse an unsafe statement: unsafe { ... }
pub fn parse_unsafe_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            keyword("unsafe").context("unsafe keyword"),
            bws(parse_statement_ws).context("unsafe body"),
        )),
        |(_, body)| {
            Statement::Unsafe(Box::new(UnsafeStatement {
                body: Box::new(body),
            }))
        },
    )
    .context("unsafe statement")
    .parse(input)
}
