use crate::parser::errors::BResult;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::parser_helpers::{bchar, bs_context, bws};
use nom::combinator::map;

// Parse an empty statement: ;
pub fn parse_empty_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "empty statement",
        map(bws(bchar(';')), |_| Statement::Empty),
    )(input)
} 