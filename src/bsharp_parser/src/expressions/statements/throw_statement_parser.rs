use crate::syntax::nodes::statements::statement::Statement;
// Parser for throw statements

use nom::combinator::cut;
use nom::{
    combinator::{map, opt},
    sequence::tuple,
};

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::exception_and_safety_keywords::kw_throw;
use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, bws, context};

// Original parse_throw_statement function from statement_parser.rs
pub fn parse_throw_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "throw statement (expected 'throw' optionally followed by expression and semicolon)",
        map(
            tuple((
                context("throw keyword (expected 'throw')", kw_throw()),
                context("optional exception expression", opt(bws(parse_expression))),
                context(
                    "semicolon after throw statement (expected ';')",
                    cut(bws(bchar(';'))),
                ),
            )),
            |(_, expr_opt, _)| Statement::Throw(expr_opt.map(Box::new)),
        ),
    )(input)
}
