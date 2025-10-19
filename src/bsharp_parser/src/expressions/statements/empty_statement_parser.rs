use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

// Parse an empty statement: ;
pub fn parse_empty_statement(input: Span) -> BResult<Statement> {
    map(delimited(ws, tok_semicolon(), ws), |_| Statement::Empty)
        .context("empty statement")
        .parse(input.into())
}
use crate::syntax::span::Span;
use crate::tokens::separators::tok_semicolon;
