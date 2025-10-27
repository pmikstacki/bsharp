use crate::parser::identifier_parser::parse_identifier;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::combinator::map;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use syntax::statements::LabelStatement;
use syntax::statements::statement::Statement;

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
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::separators::tok_colon;
