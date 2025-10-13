use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::flow_control_keywords::kw_goto;
use crate::parser::keywords::selection_and_switch_keywords::{kw_case, kw_default};
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::combinator::cut;
use nom::Parser;
use nom::{branch::alt, combinator::map, sequence::tuple};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::{GotoCaseKind, GotoCaseStatement};

/// Parse a goto case statement: goto case expr; or goto default;
pub fn parse_goto_case_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            kw_goto().context("goto keyword"),
            alt((
                map(
                    tuple((
                        kw_case().context("case keyword"),
                        delimited(ws, parse_expression, ws).context("case expression"),
                    )),
                    |(_, expr)| GotoCaseKind::Case(expr),
                ),
                map(kw_default().context("default keyword"), |_| {
                    GotoCaseKind::Default
                }),
            ))
            .context("case or default"),
            cut(delimited(ws, nom_char(';'), ws))
                .context("semicolon after goto case statement"),
        )),
        |(_, kind, _)| Statement::GotoCase(GotoCaseStatement { kind }),
    )
    .context("goto case statement")
    .parse(input)
}
use crate::syntax::span::Span;
