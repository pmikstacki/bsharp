// Parser for do-while statements

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::iteration_keywords::{kw_do, kw_while};
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom::combinator::cut;
use nom::{
    combinator::map,
    sequence::{delimited, tuple},
    Parser,
};
use nom::character::complete::char as nom_char;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::DoWhileStatement;

// Parse a do-while statement
pub fn parse_do_while_statement(input: Span) -> BResult<Statement> {
    map(
        tuple((
            kw_do().context("do keyword"),
            cut(delimited(ws, parse_statement_ws, ws)).context("do body"),
            delimited(ws, kw_while(), ws).context("while keyword"),
            delimited(
                delimited(ws, nom_char('('), ws),
                parse_expression,
                cut(delimited(ws, nom_char(')'), ws)),
            )
            .context("while condition in parentheses"),
            cut(delimited(ws, nom_char(';'), ws)).context("semicolon after do-while"),
        )),
        |(_, body_statement, _, condition, _)| {
            Statement::DoWhile(Box::new(DoWhileStatement {
                condition,
                body: Box::new(body_statement),
            }))
        },
    )
    .context("do-while statement")
    .parse(input)
}
use crate::syntax::span::Span;
