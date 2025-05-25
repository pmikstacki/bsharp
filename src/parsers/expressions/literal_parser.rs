use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::literal::Literal;
use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag_no_case},
    character::complete::{char as nom_char, digit1, multispace0, none_of},
    combinator::{map, map_res, opt, recognize, value},
    sequence::{delimited, tuple},
};

// Helper for optional whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Parse a boolean literal (true or false)
pub fn parse_boolean(input: &str) -> BResult<&str, Literal> {
    alt((
        value(Literal::Boolean(true), tag_no_case("true")),
        value(Literal::Boolean(false), tag_no_case("false")),
    ))(input)
}

// Parse an integer literal
pub fn parse_integer(input: &str) -> BResult<&str, Literal> {
    map_res(digit1, |s: &str| s.parse::<i64>().map(Literal::Integer))(input)
}

// Parse a floating-point literal (e.g., 3.14, 2.0, .5)
pub fn parse_float(input: &str) -> BResult<&str, Literal> {
    map_res(
        recognize(
            tuple((
                opt(digit1),       // Optional digits before decimal point (e.g., "3" in "3.14" or empty in ".5")
                nom_char('.'),     // Decimal point
                digit1,            // At least one digit after the decimal (required)
            ))
        ),
        |s: &str| s.parse::<f64>().map(Literal::Float)
    )(input)
}

// Parse a string literal (e.g., "hello", "with \" escape")
pub fn parse_string(input: &str) -> BResult<&str, Literal> {
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
pub fn parse_char_literal(input: &str) -> BResult<&str, Literal> {
    map(
        delimited(
            nom_char('\''),
            none_of("'\\"), // Parse a single character that isn't a quote or backslash
            nom_char('\'')
        ),
        Literal::Char
    )(input)
}

// Main literal parser: tries boolean, integer, float, string, then char
pub fn parse_literal(input: &str) -> BResult<&str, Literal> {
    ws(alt((
        parse_boolean,
        // Try float before integer to handle cases like "3.14"
        // which would otherwise be partially parsed as integer "3"
        parse_float,
        parse_integer,
        parse_string,
        parse_char_literal,
        // Add other literal types here (null, etc.) if needed
    )))(input)
}
