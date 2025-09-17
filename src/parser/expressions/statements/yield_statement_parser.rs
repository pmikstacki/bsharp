use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::statements::yield_statement::YieldStatement;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

use nom::combinator::cut;
use nom::{branch::alt, combinator::map, sequence::tuple};

/// Parse a yield statement: yield return expr; or yield break;
pub fn parse_yield_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "yield statement",
        map(
            tuple((
                context("yield keyword", keyword("yield")),
                context(
                    "return or break",
                    alt((
                        map(
                            tuple((
                                context("return keyword", keyword("return")),
                                context("return expression", bws(parse_expression)),
                            )),
                            |(_, expr)| YieldStatement::Return(expr),
                        ),
                        map(context("break keyword", keyword("break")), |_| {
                            YieldStatement::Break
                        }),
                    )),
                ),
                context(
                    "semicolon after yield statement",
                    cut(bws(bchar(';'))),
                ),
            )),
            |(_, yield_kind, _)| Statement::Yield(yield_kind),
        ),
    )(input)
}
