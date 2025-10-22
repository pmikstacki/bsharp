use crate::parser::keywords::modifier_keywords::kw_unsafe;
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use nom::Parser;
use nom::{combinator::map, sequence::delimited};
use nom_supreme::ParserExt;
use syntax::statements::UnsafeStatement;
use syntax::statements::statement::Statement;

/// Parse an unsafe statement: unsafe { ... }
pub fn parse_unsafe_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_unsafe().context("unsafe keyword"),
            nom::combinator::cut(delimited(ws, parse_statement_ws, ws)).context("unsafe body"),
        ),
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
