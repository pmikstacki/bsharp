use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::flow_control_keywords::kw_goto;
use crate::parser::keywords::selection_and_switch_keywords::{kw_case, kw_default};
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::combinator::cut;
use nom::sequence::delimited;
use nom::{branch::alt, combinator::map};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::{GotoCaseKind, GotoCaseStatement};

/// Parse a goto case statement: goto case expr; or goto default;
pub fn parse_goto_case_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_goto().context("goto keyword"),
            alt((
                map(
                    (
                        kw_case().context("case keyword"),
                        delimited(ws, parse_expression, ws).context("case expression"),
                    ),
                    |(_, expr)| GotoCaseKind::Case(expr),
                ),
                map(kw_default().context("default keyword"), |_| {
                    GotoCaseKind::Default
                }),
            ))
            .context("case or default"),
            cut(delimited(ws, tok_semicolon(), ws)).context("semicolon after goto case statement"),
        ),
        |(_, kind, _)| Statement::GotoCase(GotoCaseStatement { kind }),
    )
    .context("goto case statement")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::separators::tok_semicolon;
