use crate::parser::nodes::statements::statement::Statement;
// Parser for while statements

use nom::{
    character::complete::multispace0, 
    combinator::map,
    sequence::{delimited, tuple},
};

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::while_statement::WhileStatement;
use crate::parser::parser_helpers::{bchar, bs_context, keyword}; 

use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::statement_parser::parse_statement_ws;

// Parse a while statement
pub fn parse_while_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "while statement",
        map(
            tuple((
                keyword("while"),
                multispace0,
                delimited(bchar('('), parse_expression, bchar(')')),
                multispace0,
                parse_statement_ws
            )),
            |(_, _, condition, _, body_statement)| {
                Statement::While(Box::new(WhileStatement {
                    condition: Box::new(condition),
                    body: Box::new(body_statement)
                }))
            }
        )
    )(input)
}
