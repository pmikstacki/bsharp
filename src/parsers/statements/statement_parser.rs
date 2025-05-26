use crate::parser::errors::BResult;
use crate::parser::nodes::statements::statement::Statement;
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::statements::deconstruction_statement_parser::parse_deconstruction_statement;
use nom::error::ParseError;
use nom::{
    branch::alt,
    character::complete::{char as nom_char, multispace0},
    combinator::map,
    multi::many0,
    sequence::{delimited, terminated},
};

// Helper for optional whitespace - now generic over error type E
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> nom::IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> nom::IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// Parse an expression statement (expression followed by semicolon)
pub fn parse_expression_statement(input: &str) -> BResult<&str, Statement> {
    map(
        terminated(parse_expression, ws(nom_char(';'))),
        Statement::Expression
    )(input)
}

// Parse a block statement: { stmt* }
pub fn parse_block_statement(input: &str) -> BResult<&str, Statement> {
    map(
        delimited(
            ws(nom_char('{')),
            many0(ws(parse_statement)),
            ws(nom_char('}'))
        ),
        Statement::Block
    )(input)
}

// Main statement parser
pub fn parse_statement(input: &str) -> BResult<&str, Statement> {
    alt((
        parse_block_statement,
        parse_deconstruction_statement, // Try deconstruction before expression statements
        parse_expression_statement,
    ))(input)
}
