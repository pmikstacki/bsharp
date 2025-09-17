// use crate::syntax::nodes::expressions::expression::Expression; // Unused
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::literal::{
    InterpolatedStringLiteral, InterpolatedStringPart, Literal,
};
// This is used by parse_interpolation
use crate::syntax::parser_helpers::{bws, context};
use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not, tag, tag_no_case, take_until, take_while1},
    character::complete::{char as nom_char, digit1, none_of /* anychar */}, // anychar was unused
    combinator::{cut /* not, peek */, map, map_res, opt, recognize, value}, // not, peek were unused
    multi::many0,                                                           // many1 was unused
    sequence::{delimited, /* pair, */ preceded, tuple},                     // pair was unused
};

// Use the global comment-aware bws wrapper instead of a local whitespace helper

// Parse a boolean literal (true or false)
pub fn parse_boolean(input: &str) -> BResult<&str, Literal> {
    context(
        "boolean literal (expected 'true' or 'false')",
        alt((
            value(Literal::Boolean(true), tag_no_case("true")),
            value(Literal::Boolean(false), tag_no_case("false")),
        )),
    )(input)
}

// Parse an integer literal
pub fn parse_integer(input: &str) -> BResult<&str, Literal> {
    context(
        "integer literal (expected sequence of digits)",
        map_res(digit1, |s: &str| s.parse::<i64>().map(Literal::Integer)),
    )(input)
}

// Parse a floating-point literal (e.g., 3.14, 2.0, .5)
pub fn parse_float(input: &str) -> BResult<&str, Literal> {
    context(
        "floating-point literal (expected decimal number like '3.14' or '.5')",
        map_res(
            recognize(tuple((
                opt(digit1), // Optional digits before decimal point (e.g., "3" in "3.14" or empty in ".5")
                nom_char('.'), // Decimal point
                digit1,      // At least one digit after the decimal (required)
            ))),
            |s: &str| s.parse::<f64>().map(Literal::Float),
        ),
    )(input)
}

// Parse a string literal (e.g., "hello", "with \" escape")
pub fn parse_string(input: &str) -> BResult<&str, Literal> {
    context(
        "string literal (expected text enclosed in double quotes)",
        map(
            delimited(
                nom_char('"'),
                // Use opt to handle the case of an empty string content ""
                opt(escaped_transform(
                    is_not("\"\\"), // Normal characters
                    '\\',           // Escape character
                    alt((
                        // Transformation syntax for escaped chars
                        value("\"", nom_char('"')),
                        value("\\", nom_char('\\')),
                        value("\n", nom_char('n')),
                        value("\t", nom_char('t')),
                        value("\r", nom_char('r')),
                    )),
                )),
                nom_char('"'),
            ),
            // Map Option<String> to Literal::String
            |opt_s: Option<String>| Literal::String(opt_s.unwrap_or_default()),
        ),
    )(input)
}

// Parse a verbatim string literal (@"...")
pub fn parse_verbatim_string(input: &str) -> BResult<&str, Literal> {
    context(
        "verbatim string literal (expected @\"...\" format)",
        map(
            preceded(
                nom_char('@'),
                delimited(
                    nom_char('"'),
                    opt(take_until("\"")), // Take everything until closing quote
                    nom_char('"'),
                ),
            ),
            |opt_s: Option<&str>| Literal::VerbatimString(opt_s.unwrap_or_default().to_string()),
        ),
    )(input)
}

// Parse a raw string literal ("""text""")
pub fn parse_raw_string(input: &str) -> BResult<&str, Literal> {
    context(
        "raw string literal (expected \"\"\"...\"\"\" format)",
        map(
            delimited(
                tag("\"\"\""),
                opt(take_until("\"\"\"")), // Take everything until closing triple quote
                tag("\"\"\""),
            ),
            |opt_s: Option<&str>| Literal::RawString(opt_s.unwrap_or_default().to_string()),
        ),
    )(input)
}

/// Enhanced interpolated string syntax using robust Nom combinators
/// Handles complex patterns like $"Invalid email: {email}" with graceful fallback
pub fn parse_interpolated_string(input: &str) -> BResult<&str, Literal> {
    context(
        "interpolated string literal (expected $\"...\" or $@\"...\" format with {expression} interpolations)",
        |input| {
            // Enhanced prefix recognition with better error handling
            let (input, is_verbatim) = alt((
                map(tag("$@"), |_| true),
                map(tag("@$"), |_| true),
                map(tag("$"), |_| false),
            ))(input)?;

            let (input, parts) = delimited(
                nom_char('"'),
                enhanced_interpolated_parts,
                cut(nom_char('"')), // Use cut for better error reporting
            )(input)?;

            Ok((
                input,
                Literal::InterpolatedString(InterpolatedStringLiteral { parts, is_verbatim }),
            ))
        },
    )(input)
}

/// Enhanced parsing of interpolated string parts with better error recovery
fn enhanced_interpolated_parts(input: &str) -> BResult<&str, Vec<InterpolatedStringPart>> {
    context(
        "interpolated string parts (expected text and {expression} interpolations)",
        many0(alt((enhanced_interpolation, enhanced_interpolated_text))),
    )(input)
}

/// Enhanced text part parsing with better handling of edge cases
fn enhanced_interpolated_text(input: &str) -> BResult<&str, InterpolatedStringPart> {
    context(
        "interpolated string text (expected literal text between interpolations)",
        map(
            take_while1(|c| c != '{' && c != '"' && c != '\\'), // Take characters that aren't interpolation, quote, or escape
            |s: &str| InterpolatedStringPart::Text(s.to_string()),
        ),
    )(input)
}

/// Enhanced interpolation parsing with graceful fallback
fn enhanced_interpolation(input: &str) -> BResult<&str, InterpolatedStringPart> {
    context(
        "string interpolation (expected {expression} with optional formatting)",
        map(
            delimited(
                nom_char('{'),
                cut(tuple((
                    robust_expression_in_interpolation,
                    opt(preceded(nom_char(','), robust_expression_in_interpolation)), // alignment
                    opt(preceded(nom_char(':'), take_until("}"))), // format string
                ))),
                cut(nom_char('}')),
            ),
            |(expression, alignment, format_string)| InterpolatedStringPart::Interpolation {
                expression,
                alignment,
                format_string: format_string.map(|s| s.to_string()),
            },
        ),
    )(input)
}

/// Robust expression parsing within interpolation with fallback
fn robust_expression_in_interpolation(
    input: &str,
) -> BResult<&str, crate::syntax::nodes::expressions::expression::Expression> {
    context(
        "interpolation expression (expected valid C# expression within braces)",
        alt((
            parse_expression,           // Try full expression parsing first
            fallback_simple_expression, // Fallback for simple cases
        )),
    )(input)
}

/// Fallback syntax for simple expressions when complex parsing fails
fn fallback_simple_expression(
    input: &str,
) -> BResult<&str, crate::syntax::nodes::expressions::expression::Expression> {
    use crate::parser::identifier_parser::parse_identifier;
    use crate::syntax::nodes::expressions::expression::Expression;

    context(
        "simple expression (expected identifier as fallback)",
        map(parse_identifier, |id| Expression::Variable(id)),
    )(input)
}

// Parse a single part of an interpolated string (text or interpolation) - LEGACY VERSION
#[allow(dead_code)]
fn parse_interpolated_part(input: &str) -> BResult<&str, InterpolatedStringPart> {
    alt((parse_interpolation, parse_interpolated_text))(input)
}

// Parse text part of interpolated string - LEGACY VERSION
#[allow(dead_code)]
fn parse_interpolated_text(input: &str) -> BResult<&str, InterpolatedStringPart> {
    map(take_while1(|c| c != '{' && c != '"'), |s: &str| {
        InterpolatedStringPart::Text(s.to_string())
    })(input)
}

// Parse interpolation part {expression} - LEGACY VERSION
#[allow(dead_code)]
fn parse_interpolation(input: &str) -> BResult<&str, InterpolatedStringPart> {
    map(
        delimited(
            nom_char('{'),
            tuple((
                parse_expression, // This is where parse_expression is used
                opt(preceded(nom_char(','), parse_expression)), // alignment
                opt(preceded(nom_char(':'), take_until("}"))), // format string
            )),
            nom_char('}'),
        ),
        |(expression, alignment, format_string)| InterpolatedStringPart::Interpolation {
            expression,
            alignment,
            format_string: format_string.map(|s| s.to_string()),
        },
    )(input)
}

// Parse a char literal (e.g., 'a', '\n')
pub fn parse_char_literal(input: &str) -> BResult<&str, Literal> {
    map(
        delimited(
            nom_char('\''),
            none_of("'\\"), // Parse a single character that isn't a quote or backslash
            nom_char('\''),
        ),
        Literal::Char,
    )(input)
}

// Main literal syntax: tries boolean, integer, float, string, then char
pub fn parse_literal(input: &str) -> BResult<&str, Literal> {
    context(
        "literal (expected any valid C# literal: string, number, boolean, character, or null)",
        bws(alt((
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
        ))),
    )(input)
}
