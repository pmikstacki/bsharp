use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::flow_control_keywords::kw_goto;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::goto_statement::GotoStatement;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws};

use nom::Parser;
use nom::combinator::cut;
use nom::{combinator::map, sequence::tuple};
use nom_supreme::ParserExt;

/// Parse a goto statement: goto label;
pub fn parse_goto_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            kw_goto().context("goto keyword"),
            bws(parse_identifier).context("label identifier"),
            cut(bws(bchar(';'))).context("semicolon after goto statement"),
        )),
        |(_, label, _)| Statement::Goto(GotoStatement { label }),
    )
    .context("goto statement")
    .parse(input)
}
