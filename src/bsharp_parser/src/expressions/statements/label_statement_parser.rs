use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::Parser;
use nom::{combinator::map, sequence::tuple};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::LabelStatement;

/// Parse a label statement: identifier:
pub fn parse_label_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            parse_identifier.context("label identifier"),
            delimited(ws, nom_char(':'), ws).context("colon after label"),
        )),
        |(label, _)| Statement::Label(LabelStatement { label }),
    )
    .context("label statement")
    .parse(input)
}
use crate::syntax::span::Span;
