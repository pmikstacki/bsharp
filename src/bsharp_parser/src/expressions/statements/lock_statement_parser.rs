use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::parser::keywords::exception_and_safety_keywords::kw_lock;
use crate::parser::statement_parser::parse_statement_ws_spanned;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;

use nom::combinator::cut;
use nom::{Parser, combinator::map, sequence::delimited};
use nom_supreme::ParserExt;
use syntax::statements::LockStatement;
use syntax::statements::statement::Statement;

/// Parse a lock statement: lock (expression) statement
pub fn parse_lock_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_lock().context("lock keyword"),
            delimited(
                delimited(ws, tok_l_paren(), ws),
                parse_expression_spanned
                    .context("lock object expression")
                    .map(|s| s.node),
                cut(delimited(ws, tok_r_paren(), ws)).context("closing parenthesis"),
            )
            .context("lock object in parentheses"),
            cut(delimited(ws, parse_statement_ws_spanned, ws))
                .map(|s| s.node)
                .context("lock body"),
        ),
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
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
