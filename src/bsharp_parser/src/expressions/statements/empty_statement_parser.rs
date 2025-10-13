use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::map;
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

// Parse an empty statement: ;
pub fn parse_empty_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(delimited(ws, nom_char(';'), ws), |_| Statement::Empty)
        .context("empty statement")
        .parse(input)
}
use crate::syntax::span::Span;
