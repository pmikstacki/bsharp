use crate::parser::nodes::statements::statement::Statement;
// Parser for return statements

use nom::{
    character::complete::multispace0,
    combinator::{map, opt},
    sequence::tuple,
};

use crate::parser::errors::BResult;
use crate::parser::parser_helpers::{bchar, bs_context, keyword};
use crate::parsers::expressions::expression_parser::parse_expression;


// Parse a return statement with an optional expression
pub fn parse_return_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "return statement",
        map(
            tuple((
                keyword("return"),
                multispace0,
                // Optional expression, which may or may not be preceded by whitespace
                opt(parse_expression),
                multispace0,
                bchar(';'),
            )),
            |(_, _, expr_opt, _, _)| Statement::Return(expr_opt.map(Box::new)),
        ),
    )(input)
}
