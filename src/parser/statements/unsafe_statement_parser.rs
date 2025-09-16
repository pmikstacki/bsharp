use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::unsafe_statement::UnsafeStatement;
use crate::syntax::parser_helpers::{keyword, bws};
use crate::parser::statement_parser::parse_statement_ws;

use nom_supreme::ParserExt;
use nom::Parser;
use nom::{
    combinator::map,
    sequence::tuple,
};

/// Parse an unsafe statement: unsafe { ... }
pub fn parse_unsafe_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            keyword("unsafe").context("unsafe keyword"),
            bws(parse_statement_ws).context("unsafe body"),
        )),
        |(_, body)| {
            Statement::Unsafe(Box::new(UnsafeStatement { 
                body: Box::new(body) 
            }))
        },
    )
    .context("unsafe statement")
    .parse(input)
} 