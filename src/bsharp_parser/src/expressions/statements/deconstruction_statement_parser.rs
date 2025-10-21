use crate::parser::expressions::deconstruction_expression_parser::parse_deconstruction_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::sequence::delimited;
use nom::Parser;
use nom::{combinator::map, sequence::terminated};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;

/// Parse a deconstruction statement: (var x, var y) = tuple;
pub fn parse_deconstruction_statement(input: Span) -> BResult<Statement> {
    map(
        terminated(
            delimited(ws, parse_deconstruction_expression, ws)
                .context("deconstruction expression"),
            delimited(ws, tok_semicolon(), ws)
                .context("semicolon after deconstruction statement"),
        ),
        |deconstruction| Statement::Deconstruction(Box::new(deconstruction)),
    )
        .context("deconstruction statement")
        .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::separators::tok_semicolon;
