use crate::parser::nodes::statements::statement::Statement;
use nom::{
    branch::alt,
    character::complete::{char as nom_char, multispace0},
    combinator::{map},
    multi::many0,
    sequence::{delimited, terminated},
    IResult,
};
use crate::parsers::expressions::primary_parser::parse_primary_expression;

// Helper for optional whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Parse an expression statement (expression followed by semicolon)
pub fn parse_expression_statement(input: &str) -> IResult<&str, Statement> {
    map(
        terminated(parse_primary_expression, ws(nom_char(';'))),
        Statement::Expression
    )(input)
}

// Parse a block statement: { stmt* }
pub fn parse_block_statement(input: &str) -> IResult<&str, Statement> {
    map(
        delimited(
            ws(nom_char('{')),
            many0(ws(parse_statement)),
            ws(nom_char('}'))
        ),
        Statement::Block
    )(input)
}

// Main statement parser (for now: block or expression statement)
pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        parse_block_statement,
        parse_expression_statement,
    ))(input)
}
