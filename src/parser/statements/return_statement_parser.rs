use crate::syntax::nodes::statements::statement::Statement;
// Parser for return statements

use nom::{
    combinator::{map, opt},
    sequence::tuple,
};

use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, context, keyword, bws};
use crate::syntax::comment_parser::ws;
use crate::parser::expressions::expression_parser::parse_expression;


// Parse a return statement with an optional expression
pub fn parse_return_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "return statement (expected 'return' optionally followed by expression and semicolon)",
        map(
            tuple((
                context("return keyword (expected 'return')", keyword("return")),
                context("whitespace after return keyword", ws),
                // Optional expression, which may or may not be preceded by whitespace
                context("optional return value (expected valid C# expression or none)", opt(parse_expression)),
                context("whitespace before semicolon", ws),
                context("semicolon after return statement (expected ';')", bws(bchar(';'))),
            )),
            |(_, _, expr_opt, _, _)| Statement::Return(expr_opt.map(Box::new)),
        ),
    )(input)
}
