// Parser for throw statements

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::exception_and_safety_keywords::kw_throw;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::cut;
use nom::Parser;
use nom::{
    combinator::{map, opt},
    sequence::delimited,
};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

// Original parse_throw_statement function from statement_parser.rs
pub fn parse_throw_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_throw().context("throw keyword"),
            opt(delimited(ws, parse_expression, ws))
                .context("optional exception expression"),
            cut(delimited(ws, tok_semicolon(), ws))
                .context("semicolon after throw statement"),
        ),
        |(_, expr_opt, _)| Statement::Throw(expr_opt.map(Box::new)),
    )
        .context("throw statement")
        .parse(input.into())
}
use crate::syntax::span::Span;
use crate::tokens::separators::tok_semicolon;
