use crate::syntax::nodes::statements::statement::Statement;
// Parser for continue statements

use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, context, keyword};

use nom::combinator::map;
use nom::sequence::{preceded, terminated};
use crate::syntax::comment_parser::ws;
use crate::syntax::parser_helpers::bws;

// Original parse_continue_statement function from statement_parser.rs
pub fn parse_continue_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "continue statement (expected 'continue' keyword followed by semicolon)",
        map(
            terminated(
                context("continue keyword (expected 'continue')", keyword("continue")), 
                context("semicolon after continue statement (expected ';' optionally preceded by whitespace)", preceded(ws, bws(bchar(';'))))
            ),
            |_| Statement::Continue(ContinueStatement)
        )
    )(input)
}
