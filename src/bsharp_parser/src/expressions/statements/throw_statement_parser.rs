// Parser for throw statements

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::exception_and_safety_keywords::kw_throw;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::cut;
use nom::{
    combinator::{map, opt},
    sequence::{tuple, delimited},
};
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

// Original parse_throw_statement function from statement_parser.rs
pub fn parse_throw_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            kw_throw().context("throw keyword"),
            opt(delimited(ws, parse_expression, ws))
                .context("optional exception expression"),
            cut(delimited(ws, nom_char(';'), ws))
                .context("semicolon after throw statement"),
        )),
        |(_, expr_opt, _)| Statement::Throw(expr_opt.map(Box::new)),
    )
    .context("throw statement")
    .parse(input)
}
use crate::syntax::span::Span;
