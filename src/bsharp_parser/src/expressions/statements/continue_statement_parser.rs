// Parser for continue statements

use crate::parser::keywords::flow_control_keywords::kw_continue;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use nom::combinator::cut;
use nom::combinator::map;
use nom::sequence::{terminated, delimited};
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::ContinueStatement;

// Original parse_continue_statement function from statement_parser.rs
pub fn parse_continue_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        terminated(
            kw_continue().context("continue keyword"),
            cut(delimited(ws, nom_char(';'), ws))
                .context("semicolon after continue statement"),
        ),
        |_| Statement::Continue(ContinueStatement),
    )
    .context("continue statement")
    .parse(input)
}
use crate::syntax::span::Span;
