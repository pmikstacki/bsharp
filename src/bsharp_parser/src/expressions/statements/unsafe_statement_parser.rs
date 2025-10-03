use crate::parser::keywords::modifier_keywords::kw_unsafe;
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::statements::unsafe_statement::UnsafeStatement;
use crate::syntax::parser_helpers::bws;

use nom::Parser;
use nom::{combinator::map, sequence::tuple};
use nom_supreme::ParserExt;

/// Parse an unsafe statement: unsafe { ... }
pub fn parse_unsafe_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            kw_unsafe().context("unsafe keyword"),
            nom::combinator::cut(bws(parse_statement_ws)).context("unsafe body"),
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
