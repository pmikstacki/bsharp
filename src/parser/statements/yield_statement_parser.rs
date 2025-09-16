use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::yield_statement::YieldStatement;
use crate::syntax::parser_helpers::{bchar, keyword, bws};
use crate::parser::expressions::expression_parser::parse_expression;

use nom_supreme::ParserExt;
use nom::Parser;
use nom::{
    branch::alt,
    combinator::map,
    sequence::tuple,
};

/// Parse a yield statement: yield return expr; or yield break;
pub fn parse_yield_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            keyword("yield").context("yield keyword"),
            alt((
                map(
                    tuple((
                        keyword("return").context("return keyword"),
                        bws(parse_expression).context("return expression"),
                    )),
                    |(_, expr)| YieldStatement::Return(expr)
                ),
                map(
                    keyword("break").context("break keyword"),
                    |_| YieldStatement::Break
                ),
            )).context("return or break"),
            bws(bchar(';')).context("semicolon after yield statement"),
        )),
        |(_, yield_kind, _)| {
            Statement::Yield(yield_kind)
        },
    )
    .context("yield statement")
    .parse(input)
} 