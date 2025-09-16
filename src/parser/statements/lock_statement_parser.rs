use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::lock_statement::LockStatement;
use crate::syntax::parser_helpers::{bchar, keyword, bws};
use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;

use nom_supreme::ParserExt;
use nom::Parser;
use nom::{
    combinator::map,
    sequence::{delimited, tuple},
};
use nom::combinator::cut;

/// Parse a lock statement: lock (expression) statement
pub fn parse_lock_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            keyword("lock").context("lock keyword"),
            bws(delimited(
                bchar('(').context("opening parenthesis"),
                parse_expression.context("lock object expression"),
                cut(bchar(')')).context("closing parenthesis")
            )).context("lock object in parentheses"),
            bws(parse_statement_ws).context("lock body"),
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