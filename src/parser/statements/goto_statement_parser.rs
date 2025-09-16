use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::goto_statement::GotoStatement;
use crate::syntax::parser_helpers::{bchar, keyword, bws};
use crate::parser::identifier_parser::parse_identifier;

use nom_supreme::ParserExt;
use nom::Parser;
use nom::{
    combinator::map,
    sequence::tuple,
};

/// Parse a goto statement: goto label;
pub fn parse_goto_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            keyword("goto").context("goto keyword"),
            bws(parse_identifier).context("label identifier"),
            bws(bchar(';')).context("semicolon after goto statement"),
        )),
        |(_, label, _)| {
            Statement::Goto(GotoStatement { label })
        },
    )
    .context("goto statement")
    .parse(input)
} 