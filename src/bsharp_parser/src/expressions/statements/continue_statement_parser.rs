use crate::syntax::nodes::statements::statement::Statement;
// Parser for continue statements

use crate::parser::keywords::flow_control_keywords::kw_continue;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, bws, context};

use nom::combinator::cut;
use nom::combinator::map;
use nom::sequence::terminated;

// Original parse_continue_statement function from statement_parser.rs
pub fn parse_continue_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "continue statement (expected 'continue' keyword followed by semicolon)",
        map(
            terminated(
                context("continue keyword (expected 'continue')", kw_continue()),
                context(
                    "semicolon after continue statement (expected ';')",
                    cut(bws(bchar(';'))),
                ),
            ),
            |_| Statement::Continue(ContinueStatement),
        ),
    )(input)
}
