use crate::parser::nodes::statements::statement::Statement;
// Parser for continue statements

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::*;
use crate::parser::parser_helpers::{bchar, bs_context, keyword, nom_to_bs};

use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{preceded, terminated};

// Original parse_continue_statement function from statement_parser.rs
pub fn parse_continue_statement<'a>(input: &'a str) -> BResult<&'a str, Statement<'a>> {
    bs_context(
        "continue statement",
        nom_to_bs(map(
            terminated(keyword("continue"), preceded(multispace0, bchar(';'))),
            |_| Statement::Continue(ContinueStatement)
        ))
    )(input)
}
