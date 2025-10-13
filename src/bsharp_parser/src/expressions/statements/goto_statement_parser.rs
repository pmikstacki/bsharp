use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::flow_control_keywords::kw_goto;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::combinator::cut;
use nom::Parser;
use nom::{combinator::map, sequence::tuple};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::GotoStatement;

/// Parse a goto statement: goto label;
pub fn parse_goto_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            kw_goto().context("goto keyword"),
            delimited(ws, parse_identifier, ws).context("label identifier"),
            cut(delimited(ws, nom_char(';'), ws)).context("semicolon after goto statement"),
        )),
        |(_, label, _)| Statement::Goto(GotoStatement { label }),
    )
    .context("goto statement")
    .parse(input)
}
use crate::syntax::span::Span;
