use crate::syntax::nodes::statements::statement::Statement;
// Parser for while statements

use nom::combinator::cut;
use nom::{
    combinator::map,
    sequence::{delimited, tuple},
};

use crate::parser::keywords::iteration_keywords::kw_while;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::while_statement::WhileStatement;
use crate::syntax::parser_helpers::{bchar, bws, context};

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;

// Parse a while statement
pub fn parse_while_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "while statement (expected 'while (condition) statement')",
        map(
            tuple((
                context("while keyword (expected 'while')", kw_while()),
                bws(delimited(
                    context(
                        "opening parenthesis for while condition (expected '(')",
                        bchar('('),
                    ),
                    context(
                        "while loop condition (expected boolean expression)",
                        bws(parse_expression),
                    ),
                    cut(context(
                        "closing parenthesis for while condition (expected ')')",
                        bchar(')'),
                    )),
                )),
                context(
                    "while loop body (expected valid C# statement)",
                    cut(bws(parse_statement_ws)),
                ),
            )),
            |(_, condition, body_statement)| {
                Statement::While(Box::new(WhileStatement {
                    condition: Box::new(condition),
                    body: Box::new(body_statement),
                }))
            },
        ),
    )(input)
}
