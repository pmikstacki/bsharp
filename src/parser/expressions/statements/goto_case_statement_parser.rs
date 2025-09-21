use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::goto_case_statement::{GotoCaseKind, GotoCaseStatement};
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws};
use crate::parser::keywords::flow_control_keywords::kw_goto;
use crate::parser::keywords::selection_and_switch_keywords::{kw_case, kw_default};

use nom::Parser;
use nom::combinator::cut;
use nom::{branch::alt, combinator::map, sequence::tuple};
use nom_supreme::ParserExt;

/// Parse a goto case statement: goto case expr; or goto default;
pub fn parse_goto_case_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            kw_goto().context("goto keyword"),
            alt((
                map(
                    tuple((
                        kw_case().context("case keyword"),
                        bws(parse_expression).context("case expression"),
                    )),
                    |(_, expr)| GotoCaseKind::Case(expr),
                ),
                map(kw_default().context("default keyword"), |_| {
                    GotoCaseKind::Default
                }),
            ))
            .context("case or default"),
            cut(bws(bchar(';'))).context("semicolon after goto case statement"),
        )),
        |(_, kind, _)| Statement::GotoCase(GotoCaseStatement { kind }),
    )
    .context("goto case statement")
    .parse(input)
}
