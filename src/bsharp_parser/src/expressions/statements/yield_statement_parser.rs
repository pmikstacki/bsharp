use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::flow_control_keywords::{kw_break, kw_return, kw_yield};
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use nom::combinator::cut;
use nom::Parser;
use nom::{branch::alt, combinator::map, sequence::delimited};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::YieldStatement;

/// Parse a yield statement: yield return expr; or yield break;
pub fn parse_yield_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_yield().context("yield keyword"),
            alt((
                map(
                    (
                        kw_return().context("return keyword"),
                        delimited(ws, parse_expression, ws).context("return expression"),
                    ),
                    |(_, expr)| YieldStatement::Return(expr),
                ),
                map(kw_break().context("break keyword"), |_| YieldStatement::Break),
            ))
                .context("return or break"),
            cut(delimited(ws, tok_semicolon(), ws)).context("semicolon after yield statement"),
        ),
        |(_, yield_kind, _)| Statement::Yield(yield_kind),
    )
        .context("yield statement")
        .parse(input.into())
}
use crate::syntax::span::Span;
use crate::tokens::separators::tok_semicolon;
