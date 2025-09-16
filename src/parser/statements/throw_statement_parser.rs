use crate::syntax::nodes::statements::statement::Statement;
// Parser for throw statements

use nom::{
    combinator::{map, opt},
    sequence::{tuple},
};

use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, context, btag, bws};
use crate::syntax::comment_parser::ws;
use crate::parser::expressions::expression_parser::parse_expression;


// Original parse_throw_statement function from statement_parser.rs
pub fn parse_throw_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "throw statement (expected 'throw' optionally followed by expression and semicolon)",
        map(
            tuple((
                context("leading whitespace", ws), 
                context("throw keyword (expected 'throw')", btag("throw")), 
                context("optional exception expression", opt(parse_expression)),
                context("trailing whitespace before semicolon", ws), 
                context("semicolon after throw statement (expected ';')", bws(bchar(';')))   
            )),
            |(_, _, expr_opt, _, _)| Statement::Throw(expr_opt.map(Box::new)),
        ),
    )(input)
}
