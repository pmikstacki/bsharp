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
    bytes::complete::{escaped_transform, is_not, tag, tag_no_case, take_until, take_while1, take_while_m_n},
    character::complete::{char as nom_char, digit1, none_of},
    combinator::{cut, map, map_opt, map_res, opt, peek, recognize, value},
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
};
use nom::error::{make_error, ErrorKind};

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

// Helpers for digits with underscores
fn strip_underscores(s: &str) -> String {
    s.chars().filter(|&c| c != '_').collect()
}

fn is_hex(c: char) -> bool { c.is_ascii_hexdigit() || c == '_' }
fn is_bin(c: char) -> bool { c == '0' || c == '1' || c == '_' }
fn is_dec(c: char) -> bool { c.is_ascii_digit() || c == '_' }

// Parse an integer literal: supports decimal, 0x hex, 0b binary, underscores
pub fn parse_integer(input: &str) -> BResult<&str, Literal> {
    context(
        "integer literal (decimal, 0x..., or 0b..., underscores allowed)",
        alt((
            // Hex 0x...
            map_res(
                recognize(tuple((tag_no_case("0x"), take_while1(is_hex)))),
                |s: &str| {
                    let digits = &s[2..];
                    i64::from_str_radix(&strip_underscores(digits), 16).map(Literal::Integer)
                },
            ),
            // Binary 0b...
            map_res(
                recognize(tuple((tag_no_case("0b"), take_while1(is_bin)))),
                |s: &str| {
                    let digits = &s[2..];
                    i64::from_str_radix(&strip_underscores(digits), 2).map(Literal::Integer)
                },
            ),
            // Decimal
            map_res(take_while1(is_dec), |s: &str| {
                strip_underscores(s).parse::<i64>().map(Literal::Integer)
            }),
        )),
    )(input)
}

// Parse a floating-point literal with underscores and exponent: 1.23, .5, 1e10, 1_2.3_4e-5
pub fn parse_float(input: &str) -> BResult<&str, Literal> {
    context(
        "floating-point literal (decimal with optional exponent, underscores allowed)",
        map_res(
            recognize(tuple((
                // integer or empty before dot
                opt(take_while1(is_dec)),
                nom_char('.'),
                take_while1(is_dec),
                // optional exponent part
                opt(tuple((
                    alt((nom_char('e'), nom_char('E'))),
                    opt(alt((nom_char('+'), nom_char('-')))),
                    take_while1(is_dec),
                ))),
                // optional float suffix f/F/d/D/m/M which we ignore
                opt(map_opt(nom::character::complete::one_of("fFdDmM"), Some)),
            ))),
            |s: &str| strip_underscores(s).parse::<f64>().map(Literal::Float),
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
        "verbatim string literal (expected @\"...\" with doubled quotes)",
        |i| {
            let (mut rest, _) = nom::character::complete::char('@')(i)?;
            let (mut rest2, _) = nom_char('"')(rest)?;
            let mut content = String::new();
            let mut chars = rest2.chars().peekable();
            let mut consumed = 0usize;
            while let Some(ch) = chars.next() {
                consumed += ch.len_utf8();
                if ch == '"' {
                    // doubled quote => literal quote
                    if let Some('"') = chars.peek().copied() {
                        // consume the second quote
                        let _ = chars.next();
                        consumed += 1;
                        content.push('"');
                        continue;
                    } else {
                        // closing quote
                        // build remaining slice
                        let remainder = &rest2[consumed..];
                        rest = remainder;
                        return Ok((rest, Literal::VerbatimString(content)));
                    }
                } else {
                    content.push(ch);
                }
            }
            // If we reach here, missing closing quote
            Err(nom::Err::Error(make_error(i, ErrorKind::Tag)))
        },
    )(input)
}

// Parse a raw string literal ("""text""")
pub fn parse_raw_string(input: &str) -> BResult<&str, Literal> {
    context(
        "raw string literal (expected N quotes \"\"\"...\"\"\" with N >= 3)",
        |i| {
            // Count opening quotes
            let mut chars = i.chars();
            let mut qcount = 0usize;
            while let Some('"') = chars.next() {
                qcount += 1;
            }
            if qcount < 3 {
                return Err(nom::Err::Error(make_error(i, ErrorKind::Tag)));
            }
            // Slice after opening quotes
            let start = &i[qcount..];
            // Find closing N quotes
            let closing = "\"".repeat(qcount);
            if let Some(idx) = start.find(&closing) {
                let content = &start[..idx];
                let rest = &start[idx + qcount..];
                return Ok((rest, Literal::RawString(content.to_string())));
            }
            Err(nom::Err::Error(make_error(i, ErrorKind::Tag)))
        },
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

// Parse a char literal with escapes: '\\n', '\\t', '\\xFF', '\\u1234', '\\U0001F642'
pub fn parse_char_literal(input: &str) -> BResult<&str, Literal> {
    fn hex_to_char_opt(hex: &str) -> Option<char> {
        let cp = u32::from_str_radix(hex, 16).ok()?;
        char::from_u32(cp)
    }
    context(
        "char literal (expected e.g. 'a', '\\n', '\\u0041')",
        map(
            delimited(
                nom_char('\''),
                alt((
                    // simple escape
                    map(preceded(nom_char('\\'), alt((
                        value('\n', nom_char('n')),
                        value('\t', nom_char('t')),
                        value('\r', nom_char('r')),
                        value('\\', nom_char('\\')),
                        value('\'', nom_char('\'')),
                        value('"', nom_char('"')),
                    ))), |c| c),
                    // hex escape \\xHH.. (1-4 hex digits)
                    map_opt(
                        preceded(
                            tuple((nom_char('\\'), nom_char('x'))),
                            recognize(many1(nom::character::complete::one_of(
                                "0123456789abcdefABCDEF",
                            ))),
                        ),
                        |hex: &str| hex_to_char_opt(hex),
                    ),
                    // unicode \\uHHHH
                    map_opt(
                        preceded(
                            tuple((nom_char('\\'), nom_char('u'))),
                            take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit()),
                        ),
                        |hex: &str| hex_to_char_opt(hex),
                    ),
                    // unicode \\UHHHHHHHH (8 hex digits)
                    map_opt(
                        preceded(
                            tuple((nom_char('\\'), nom_char('U'))),
                            take_while_m_n(8, 8, |c: char| c.is_ascii_hexdigit()),
                        ),
                        |hex: &str| hex_to_char_opt(hex),
                    ),
                    // single non-escape character
                    map(none_of("'\\"), |c| c),
                )),
                nom_char('\''),
            ),
            Literal::Char,
        ),
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
