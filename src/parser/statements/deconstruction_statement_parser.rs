use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{context, bws, bchar};
use crate::parser::expressions::deconstruction_expression_parser::parse_deconstruction_expression;

use nom::{
    combinator::map,
    sequence::terminated,
};

/// Parse a deconstruction statement: (var x, var y) = tuple;
pub fn parse_deconstruction_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "deconstruction statement (expected tuple pattern assignment like '(var x, var y) = tuple;')",
        map(
            terminated(
                context("deconstruction expression (expected tuple pattern assignment)", bws(parse_deconstruction_expression)),
                context("semicolon after deconstruction statement (expected ';')", bws(bchar(';'))),
            ),
            |deconstruction| Statement::Deconstruction(Box::new(deconstruction)),
        ),
    )(input)
} 