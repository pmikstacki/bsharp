// Parser for break statements
use crate::parser::keywords::flow_control_keywords::kw_break;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, bws, context};
use nom::combinator::cut;
use nom::combinator::map;
use nom::sequence::terminated;

// Original parse_break_statement function from statement_parser.rs
pub fn parse_break_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "break statement (expected 'break' keyword followed by semicolon)",
        map(
            terminated(
                context("break keyword (expected 'break')", kw_break()),
                context(
                    "semicolon after break statement (expected ';')",
                    cut(bws(bchar(';'))),
                ),
            ),
            |_| Statement::Break(BreakStatement),
        ),
    )(input)
}
