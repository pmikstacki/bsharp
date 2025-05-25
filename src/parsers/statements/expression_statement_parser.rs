use crate::parser::nodes::statements::statement::Statement;
// Parser for expression statements (e.g., x = 5;, DoSomething();)

use crate::parser::errors::BResult;
use crate::parser::parser_helpers::{bchar, bs_context};
use crate::parsers::expressions::expression_parser::parse_expression;
use nom::{
    combinator::map,
    sequence::terminated,
};

// Parse an expression statement: expression;
pub fn parse_expression_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "expression statement",
        map(
            terminated(parse_expression, bchar(';')),
            |expr| Statement::Expression(expr),
        ),
    )(input)
}
