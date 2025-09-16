use crate::syntax::nodes::statements::statement::Statement;
// Parser for do-while statements

use nom::{
    combinator::map,
    sequence::{delimited, tuple},
};
use nom::combinator::cut;

use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::do_while_statement::DoWhileStatement;
use crate::syntax::parser_helpers::{bchar, context, keyword, bws};
use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;

// Parse a do-while statement
pub fn parse_do_while_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "do-while statement (expected 'do' followed by statement body, then 'while' with condition in parentheses and semicolon)",
        map(
            tuple((
                keyword("do"),
                bws(parse_statement_ws),
                bws(keyword("while")),
                bws(delimited(bchar('('), parse_expression, cut(bchar(')')))),
                bws(bchar(';')),
            )),
            |(_, body_statement, _, condition, _)| { 
                Statement::DoWhile(Box::new(DoWhileStatement {
                    condition,
                    body: Box::new(body_statement), 
                }))
            },
        ),
    )(input)
}
