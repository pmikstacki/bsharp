use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::goto_case_statement::{GotoCaseStatement, GotoCaseKind};
use crate::syntax::parser_helpers::{bchar, keyword, bws};
use crate::parser::expressions::expression_parser::parse_expression;

use nom_supreme::ParserExt;
use nom::Parser;
use nom::{
    branch::alt,
    combinator::map,
    sequence::tuple,
};

/// Parse a goto case statement: goto case expr; or goto default;
pub fn parse_goto_case_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            keyword("goto").context("goto keyword"),
            alt((
                map(
                    tuple((
                        keyword("case").context("case keyword"),
                        bws(parse_expression).context("case expression"),
                    )),
                    |(_, expr)| GotoCaseKind::Case(expr)
                ),
                map(
                    keyword("default").context("default keyword"),
                    |_| GotoCaseKind::Default
                ),
            )).context("case or default"),
            bws(bchar(';')).context("semicolon after goto case statement"),
        )),
        |(_, kind, _)| {
            Statement::GotoCase(GotoCaseStatement { kind })
        },
    )
    .context("goto case statement")
    .parse(input)
} 