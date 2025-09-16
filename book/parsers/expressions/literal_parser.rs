use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::literal::{Literal, InterpolatedStringLiteral, InterpolatedStringPart};
// use crate::parser::nodes::expressions::expression::Expression; // Unused
use crate::parsers::expressions::expression_parser::parse_expression; // This is used by parse_interpolation
use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, tag_no_case, take_until, take_while1},
    character::complete::{char as nom_char, digit1, multispace0, none_of /* anychar */}, // anychar was unused
    combinator::{map, map_res, opt, recognize, value /* not, peek */}, // not, peek were unused
    sequence::{delimited, tuple, /* pair, */ preceded}, // pair was unused
    multi::{many0 /* many1 */}, // many1 was unused
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

// Parse a verbatim string literal (@"...")
pub fn parse_verbatim_string(input: &str) -> BResult<&str, Literal> {
    map(
        preceded(
            nom_char('@'),
            delimited(
                nom_char('"'),
                opt(take_until("\"")), // Take everything until closing quote
                nom_char('"')
            )
        ),
        |opt_s: Option<&str>| Literal::VerbatimString(opt_s.unwrap_or_default().to_string()),
    )(input)
}

// Parse a raw string literal ("""text""")
pub fn parse_raw_string(input: &str) -> BResult<&str, Literal> {
    map(
        delimited(
            tag("\"\"\""),
            opt(take_until("\"\"\"")), // Take everything until closing triple quote
            tag("\"\"\"")
        ),
        |opt_s: Option<&str>| Literal::RawString(opt_s.unwrap_or_default().to_string()),
    )(input)
}

// Parse interpolated string ($"..." or $@"..." or @$"...")
pub fn parse_interpolated_string(input: &str) -> BResult<&str, Literal> {
    // Try both $@ and @$ prefixes, as well as just $
    let (input, is_verbatim) = alt((
        map(tag("$@"), |_| true),
        map(tag("@$"), |_| true),
        map(tag("$"), |_| false),
    ))(input)?;
    
    let (input, parts) = delimited(
        nom_char('"'),
        many0(parse_interpolated_part),
        nom_char('"')
    )(input)?;
    
    Ok((input, Literal::InterpolatedString(InterpolatedStringLiteral {
        parts,
        is_verbatim,
    })))
}

// Parse a single part of an interpolated string (text or interpolation)
fn parse_interpolated_part(input: &str) -> BResult<&str, InterpolatedStringPart> {
    alt((
        parse_interpolation,
        parse_interpolated_text,
    ))(input)
}

// Parse text part of interpolated string
fn parse_interpolated_text(input: &str) -> BResult<&str, InterpolatedStringPart> {
    map(
        take_while1(|c| c != '{' && c != '"'),
        |s: &str| InterpolatedStringPart::Text(s.to_string())
    )(input)
}

// Parse interpolation part {expression}
fn parse_interpolation(input: &str) -> BResult<&str, InterpolatedStringPart> {
    map(
        delimited(
            nom_char('{'),
            tuple((
                parse_expression, // This is where parse_expression is used
                opt(preceded(nom_char(','), parse_expression)), // alignment
                opt(preceded(nom_char(':'), take_until("}"))), // format string
            )),
            nom_char('}')
        ),
        |(expression, alignment, format_string)| InterpolatedStringPart::Interpolation {
            expression,
            alignment,
            format_string: format_string.map(|s| s.to_string()),
        }
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
        // null keyword - treat as a special literal
        map(tag_no_case("null"), |_| Literal::Null),
        // Try float before integer to handle cases like "3.14"
        // which would otherwise be partially parsed as integer "3"
        parse_float,
        parse_integer,
        parse_interpolated_string, // Try interpolated strings before regular strings
        parse_verbatim_string,     // Try verbatim strings before regular strings
        parse_raw_string,          // Try raw strings before regular strings
        parse_string,
        parse_char_literal,
        // Add other literal types here (null, etc.) if needed
    )))(input)
}
