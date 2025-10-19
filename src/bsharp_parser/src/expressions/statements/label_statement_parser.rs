use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::LabelStatement;

/// Parse a label statement: identifier:
pub fn parse_label_statement(input: Span) -> BResult<Statement> {
    map(
        (
            delimited(ws, parse_identifier, ws).context("label identifier"),
            delimited(ws, tok_colon(), ws).context("colon after label"),
        ),
        |(label, _)| Statement::Label(LabelStatement { label }),
    )
    .context("label statement")
    .parse(input.into())
}
use crate::syntax::span::Span;
use crate::tokens::separators::tok_colon;
