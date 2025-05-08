use nom::{
    branch::alt,
    bytes::complete::{is_not, tag_no_case, escaped_transform},
    character::complete::{char as nom_char, digit1, multispace0, none_of},
    combinator::{map, value, opt, map_res},
    sequence::delimited,
    IResult,
};
use crate::parser::nodes::expressions::literal::Literal;

// Helper for optional whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Parse a boolean literal (true or false)
pub fn parse_boolean(input: &str) -> IResult<&str, Literal> {
    alt((
        value(Literal::Boolean(true), tag_no_case("true")),
        value(Literal::Boolean(false), tag_no_case("false")),
    ))(input)
}

// Parse an integer literal
pub fn parse_integer(input: &str) -> IResult<&str, Literal> {
    map_res(digit1, |s: &str| s.parse::<i64>().map(Literal::Integer))(input)
}

// Parse a string literal (e.g., "hello", "with \" escape")
pub fn parse_string(input: &str) -> IResult<&str, Literal> {
    map(
        delimited(
            nom_char('"'),
            // Use opt to handle the case of an empty string content ""
            opt(escaped_transform(
                is_not("\"\\"), // Normal characters
                '\\', // Escape character
                alt(( // Transformation parser for escaped chars
                    value("\"", nom_char('"')),
                    value("\\", nom_char('\\')),
                    value("\n", nom_char('n')),
                    value("\t", nom_char('t')),
                    value("\r", nom_char('r')),
                ))
            )),
            nom_char('"')
        ),
        // Map Option<String> to Literal::String
        |opt_s: Option<String>| Literal::String(opt_s.unwrap_or_default()),
    )(input)
}

// Parse a char literal (e.g., 'a', '\n')
pub fn parse_char_literal(input: &str) -> IResult<&str, Literal> {
    map(
        delimited(
            nom_char('\''),
            none_of("'\\"), // Parse a single character that isn't a quote or backslash
            nom_char('\'')
        ),
        Literal::Char
    )(input)
}

// Main literal parser: tries boolean, integer, string, then char
pub fn parse_literal(input: &str) -> IResult<&str, Literal> {
    ws(alt((
        parse_boolean,
        parse_integer,
        parse_string,
        parse_char_literal,
        // Add other literal types here (null, etc.) if needed
    )))(input)
}
