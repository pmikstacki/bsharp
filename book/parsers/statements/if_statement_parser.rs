use crate::parser::nodes::statements::statement::Statement;
// Parser for if/else statements

use nom::{
    character::complete::multispace0,
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
};

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::*;
use crate::parser::parser_helpers::{bchar, bs_context, keyword};

use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::statement_parser::parse_statement_ws;

/// Parse an if statement with optional else branch
/// Format: if (expr) stmt [else stmt]
pub fn parse_if_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "if statement",
        map(
            tuple((
                preceded(keyword("if"), multispace0),
                bchar('('),
                delimited(multispace0, parse_expression, multispace0),
                bchar(')'),
                multispace0,
                // Use parse_statement_ws for the body to handle whitespace better
                parse_statement_ws,
                opt(preceded(
                    delimited(multispace0, keyword("else"), multispace0),
                    parse_statement_ws,
                )),
            )),
            |(_, _, condition, _, _, then_branch, else_branch)| {
                Statement::If(Box::new(IfStatement {
                    condition,
                    consequence: Box::new(then_branch),
                    alternative: else_branch.map(Box::new),
                }))
            },
        ),
    )(input)
}
