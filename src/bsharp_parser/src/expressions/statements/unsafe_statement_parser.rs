use crate::parser::keywords::modifier_keywords::kw_unsafe;
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use nom::Parser;
use nom::{combinator::map, sequence::{tuple, delimited}};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::UnsafeStatement;

/// Parse an unsafe statement: unsafe { ... }
pub fn parse_unsafe_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            kw_unsafe().context("unsafe keyword"),
            nom::combinator::cut(delimited(ws, parse_statement_ws, ws)).context("unsafe body"),
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
use crate::syntax::span::Span;
