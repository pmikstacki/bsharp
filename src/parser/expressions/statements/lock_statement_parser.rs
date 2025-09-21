use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::lock_statement::LockStatement;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws, context};
use crate::parser::keywords::exception_and_safety_keywords::kw_lock;

use nom::combinator::cut;
use nom::{
    combinator::map,
    sequence::{delimited, tuple},
};

/// Parse a lock statement: lock (expression) statement
pub fn parse_lock_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "lock statement",
        map(
            tuple((
                context("lock keyword", kw_lock()),
                context(
                    "lock object in parentheses",
                    bws(delimited(
                        context("opening parenthesis", bchar('(')),
                        context("lock object expression", parse_expression),
                        context("closing parenthesis", cut(bchar(')'))),
                    )),
                ),
                context("lock body", bws(parse_statement_ws)),
            )),
            |(_, lock_object, body)| {
                Statement::Lock(Box::new(LockStatement {
                    expr: lock_object,
                    body: Box::new(body),
                }))
            },
        ),
    )(input)
}
