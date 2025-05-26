use crate::parser::nodes::statements::statement::Statement;
// Parser for do-while statements

use nom::{
    character::complete::multispace0,
    combinator::map,
    sequence::{delimited, tuple},
};

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::do_while_statement::DoWhileStatement;
use crate::parser::parser_helpers::{bchar, bs_context, keyword};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::statement_parser::parse_statement_ws;

// Parse a do-while statement
pub fn parse_do_while_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "do-while statement",
        map(
            tuple((
                keyword("do"),
                multispace0,
                parse_statement_ws,
                keyword("while"),
                multispace0,
                delimited(bchar('('), parse_expression, bchar(')')),
                multispace0,
                bchar(';'),
            )),
            |(_, _, body_statement, _, _, condition, _, _)| { 
                Statement::DoWhile(Box::new(DoWhileStatement {
                    condition,
                    body: Box::new(body_statement), 
                }))
            },
        ),
    )(input)
}
