use crate::syntax::nodes::statements::statement::Statement;
// Parser for continue statements

use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

use nom::combinator::map;
use nom::sequence::terminated;
use nom::combinator::cut;

// Original parse_continue_statement function from statement_parser.rs
pub fn parse_continue_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "continue statement (expected 'continue' keyword followed by semicolon)",
        map(
            terminated(
                context(
                    "continue keyword (expected 'continue')",
                    keyword("continue"),
                ),
                context(
                    "semicolon after continue statement (expected ';')",
                    cut(bws(bchar(';'))),
                ),
            ),
            |_| Statement::Continue(ContinueStatement),
        ),
    )(input)
}
