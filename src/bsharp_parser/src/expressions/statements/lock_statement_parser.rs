use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::exception_and_safety_keywords::kw_lock;
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
use syntax::statements::LockStatement;

/// Parse a lock statement: lock (expression) statement
pub fn parse_lock_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            kw_lock().context("lock keyword"),
            delimited(
                delimited(ws, nom_char('('), ws),
                parse_expression.context("lock object expression"),
                cut(delimited(ws, nom_char(')'), ws)).context("closing parenthesis"),
            )
            .context("lock object in parentheses"),
            cut(delimited(ws, parse_statement_ws, ws)).context("lock body"),
        )),
        |(_, lock_object, body)| {
            Statement::Lock(Box::new(LockStatement {
                expr: lock_object,
                body: Box::new(body),
            }))
        },
    )
    .context("lock statement")
    .parse(input)
}
use crate::syntax::span::Span;
