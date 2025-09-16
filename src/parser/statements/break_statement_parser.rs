use nom::bytes::complete::tag;
use crate::syntax::nodes::statements::statement::Statement;
// Parser for break statements
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use nom::character::complete::char;
use nom::combinator::map;
use nom::error::context;
use nom::sequence::{preceded, terminated};
use crate::syntax::comment_parser::ws;
use crate::syntax::parser_helpers::bws;

// Original parse_break_statement function from statement_parser.rs
pub fn parse_break_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "break statement (expected 'break' keyword followed by semicolon)",
        map(
            terminated(
                context("break keyword (expected 'break')", tag("break")), 
                context("semicolon after break statement (expected ';' optionally preceded by whitespace)", preceded(ws, bws(char(';'))))
            ),
            |_| Statement::Break(BreakStatement)
        )
    )(input)
}
