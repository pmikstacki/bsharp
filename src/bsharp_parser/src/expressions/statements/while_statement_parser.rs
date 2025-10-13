// Parser for while statements

use crate::parser::keywords::iteration_keywords::kw_while;
use crate::syntax::errors::BResult;
use nom::combinator::cut;
use nom::{
    combinator::map,
    sequence::{delimited, tuple},
};
use syntax::statements::statement::Statement;
use syntax::statements::WhileStatement;
use crate::syntax::comment_parser::ws;
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;


// Parse a while statement
pub fn parse_while_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            kw_while().context("while keyword"),
            delimited(
                delimited(ws, nom_char('('), ws),
                (|i| crate::parser::expressions::primary_expression_parser::parse_expression(i))
                    .context("while loop condition"),
                cut(delimited(ws, nom_char(')'), ws))
                    .context("closing parenthesis for while condition"),
            ),
            cut(delimited(ws, |i| crate::parser::statement_parser::parse_statement_ws(i), ws))
                .context("while loop body"),
        )),
        |(_, condition, body_statement)| {
            Statement::While(Box::new(WhileStatement {
                condition: Box::new(condition),
                body: Box::new(body_statement),
            }))
        },
    )
    .context("while statement")
    .parse(input)
}
use crate::syntax::span::Span;
