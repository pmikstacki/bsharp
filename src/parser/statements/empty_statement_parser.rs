use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, context, bws};
use nom::combinator::map;

// Parse an empty statement: ;
pub fn parse_empty_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "empty statement",
        map(bws(bchar(';')), |_| Statement::Empty),
    )(input)
} 