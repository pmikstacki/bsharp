// Parser for continue statements

use crate::parser::keywords::flow_control_keywords::kw_continue;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use nom::Parser;
use nom::combinator::cut;
use nom::combinator::map;
use nom::sequence::{delimited, terminated};
use nom_supreme::ParserExt;
use syntax::statements::ContinueStatement;
use syntax::statements::statement::Statement;

// Original parse_continue_statement function from statement_parser.rs
pub fn parse_continue_statement(input: Span) -> BResult<Statement> {
    map(
        terminated(
            kw_continue().context("continue keyword"),
            cut(delimited(ws, tok_semicolon(), ws)).context("semicolon after continue statement"),
        ),
        |_| Statement::Continue(ContinueStatement),
    )
    .context("continue statement")
    .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::separators::tok_semicolon;
