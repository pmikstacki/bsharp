// Parser for break statements
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};
use nom::combinator::map;
use nom::sequence::terminated;
use nom::combinator::cut;

// Original parse_break_statement function from statement_parser.rs
pub fn parse_break_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "break statement (expected 'break' keyword followed by semicolon)",
        map(
            terminated(
                context("break keyword (expected 'break')", keyword("break")),
                context(
                    "semicolon after break statement (expected ';')",
                    cut(bws(bchar(';'))),
                ),
            ),
            |_| Statement::Break(BreakStatement),
        ),
    )(input)
}
