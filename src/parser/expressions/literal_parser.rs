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
    character::complete::{char as nom_char, none_of},
    combinator::{cut, map, map_opt, map_res, opt, recognize, value},
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

// Parse a raw interpolated string literal: $""" ... {expr} ... """ or with multiple $ and N quotes
pub fn parse_raw_interpolated_string(input: &str) -> BResult<&str, Literal> {
    context(
        "raw interpolated string literal (expected $\"\"\"...\"\"\" with {expr})",
        |i| {
            // Count leading dollars
            let mut dollar_count = 0usize;
            for ch in i.chars() {
                if ch == '$' { dollar_count += 1; } else { break; }
            }
            if dollar_count == 0 { return Err(nom::Err::Error(make_error(i, ErrorKind::Tag))); }
            let after_dollars = &i[dollar_count..];
            // Count opening quotes (must be >= 3)
            let mut qcount = 0usize;
            for ch in after_dollars.chars() {
                if ch == '"' { qcount += 1; } else { break; }
            }
            if qcount < 3 { return Err(nom::Err::Error(make_error(i, ErrorKind::Tag))); }
            let content_start = dollar_count + qcount;
            let closing = "\"".repeat(qcount);
            let after_open = &i[content_start..];
            if let Some(rel_idx) = after_open.find(&closing) {
                let content_raw = &after_open[..rel_idx];
                let rest = &after_open[rel_idx + closing.len()..];
                let content = trim_raw_content_roslyn(content_raw);
                let mut parts: Vec<InterpolatedStringPart> = Vec::new();
                let mut last = 0usize; // byte index into content
                let open_pat = "{".repeat(dollar_count);
                let close_pat = "}".repeat(dollar_count);

                let mut idx = 0usize;
                while idx < content.len() {
                    let rem = &content[idx..];
                    if rem.starts_with(&open_pat) {
                        // Flush preceding text
                        if last < idx {
                            parts.push(InterpolatedStringPart::Text(content[last..idx].to_string()));
                        }
                        let after_open_idx = idx + open_pat.len();
                        let rem_after_open = &content[after_open_idx..];
                        if let Some(end_rel) = rem_after_open.find(&close_pat) {
                            let core = &rem_after_open[..end_rel];
                            // Attempt to parse the interpolation core
                            if let Some(ipart) = parse_raw_interpolation_core(core) {
                                parts.push(ipart);
                                idx = after_open_idx + end_rel + close_pat.len();
                                last = idx;
                                continue;
                            } else {
                                // Fallback: treat whole segment as text if parsing fails
                                idx = after_open_idx + end_rel + close_pat.len();
                                // Do not update 'last' so text is accumulated
                                continue;
                            }
                        } else {
                            // No closing, stop scanning
                            break;
                        }
                    } else {
                        // advance by one char
                        if let Some((adv, _)) = content[idx..].char_indices().nth(1) {
                            idx += adv;
                        } else {
                            idx = content.len();
                        }
                    }
                }
                if last < content.len() {
                    parts.push(InterpolatedStringPart::Text(content[last..].to_string()));
                }

                return Ok((rest, Literal::InterpolatedString(InterpolatedStringLiteral { parts, is_verbatim: true })));
            }
            Err(nom::Err::Error(make_error(i, ErrorKind::Tag)))
        },
    )(input)
}

fn parse_raw_interpolation_core(core: &str) -> Option<InterpolatedStringPart> {
    use crate::syntax::nodes::expressions::expression::Expression;
    let s = core.trim();
    if let Ok((rest0, expression)) = robust_expression_in_interpolation(s) {
        let mut rest = rest0.trim_start();
        // Optional alignment
        let mut alignment: Option<Expression> = None;
        if rest.starts_with(',') {
            let rest2 = rest[1..].trim_start();
            if let Ok((rest_after_align, align_expr)) = robust_expression_in_interpolation(rest2) {
                alignment = Some(align_expr);
                rest = rest_after_align.trim_start();
            }
        }
        // Optional format string introduced by ':'
        let mut format_string: Option<String> = None;
        if let Some(colon_pos) = rest.find(':') {
            let fmt = rest[colon_pos + 1..].trim();
            if !fmt.is_empty() {
                format_string = Some(fmt.to_string());
            }
        }
        Some(InterpolatedStringPart::Interpolation { expression, alignment, format_string })
    } else {
        None
    }
}

// Helpers for digits with underscores
fn strip_underscores(s: &str) -> String {
    s.chars().filter(|&c| c != '_').collect()
}

fn is_hex(c: char) -> bool { c.is_ascii_hexdigit() || c == '_' }
fn is_bin(c: char) -> bool { c == '0' || c == '1' || c == '_' }
fn is_dec(c: char) -> bool { c.is_ascii_digit() || c == '_' }
fn is_int_suffix(c: char) -> bool { matches!(c, 'u'|'U'|'l'|'L') }

// Parse an integer literal: supports decimal, 0x hex, 0b binary, underscores
pub fn parse_integer(input: &str) -> BResult<&str, Literal> {
    context(
        "integer literal (decimal, 0x..., or 0b..., underscores allowed)",
        alt((
            // Hex 0x...
            map_res(
                recognize(tuple((
                    tag_no_case("0x"),
                    take_while1(is_hex),
                    opt(take_while1(is_int_suffix)),
                ))),
                |s: &str| {
                    use crate::syntax::nodes::expressions::literal::IntegerSuffix;
                    // strip prefix and optional suffix letters
                    let core = &s[2..];
                    let trimmed = core.trim_end_matches(is_int_suffix);
                    let removed = core.len() - trimmed.len();
                    let suffix = if removed > 0 { Some(&core[trimmed.len()..]) } else { None };
                    let val = i64::from_str_radix(&strip_underscores(trimmed), 16)?;
                    if let Some(sfx) = suffix {
                        let mut has_u = false;
                        let mut has_l = false;
                        for ch in sfx.chars() { match ch { 'u'|'U' => has_u = true, 'l'|'L' => has_l = true, _ => {} } }
                        let kind = match (has_u, has_l) { (true, true) => IntegerSuffix::UL, (true, false) => IntegerSuffix::U, (false, true) => IntegerSuffix::L, (false, false) => unreachable!() };
                        Ok::<Literal, std::num::ParseIntError>(Literal::IntegerWithSuffix(val, kind))
                    } else {
                        Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                    }
                },
            ),
            // Binary 0b...
            map_res(
                recognize(tuple((
                    tag_no_case("0b"),
                    take_while1(is_bin),
                    opt(take_while1(is_int_suffix)),
                ))),
                |s: &str| {
                    use crate::syntax::nodes::expressions::literal::IntegerSuffix;
                    let core = &s[2..];
                    let trimmed = core.trim_end_matches(is_int_suffix);
                    let removed = core.len() - trimmed.len();
                    let suffix = if removed > 0 { Some(&core[trimmed.len()..]) } else { None };
                    let val = i64::from_str_radix(&strip_underscores(trimmed), 2)?;
                    if let Some(sfx) = suffix {
                        let mut has_u = false;
                        let mut has_l = false;
                        for ch in sfx.chars() { match ch { 'u'|'U' => has_u = true, 'l'|'L' => has_l = true, _ => {} } }
                        let kind = match (has_u, has_l) { (true, true) => IntegerSuffix::UL, (true, false) => IntegerSuffix::U, (false, true) => IntegerSuffix::L, (false, false) => unreachable!() };
                        Ok::<Literal, std::num::ParseIntError>(Literal::IntegerWithSuffix(val, kind))
                    } else {
                        Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                    }
                },
            ),
            // Decimal
            map_res(
                recognize(tuple((
                    take_while1(is_dec),
                    opt(take_while1(is_int_suffix)),
                ))),
                |s: &str| {
                    use crate::syntax::nodes::expressions::literal::IntegerSuffix;
                    let trimmed = s.trim_end_matches(is_int_suffix);
                    let removed = s.len() - trimmed.len();
                    let suffix = if removed > 0 { Some(&s[trimmed.len()..]) } else { None };
                    let val = strip_underscores(trimmed).parse::<i64>()?;
                    if let Some(sfx) = suffix {
                        let mut has_u = false;
                        let mut has_l = false;
                        for ch in sfx.chars() { match ch { 'u'|'U' => has_u = true, 'l'|'L' => has_l = true, _ => {} } }
                        let kind = match (has_u, has_l) { (true, true) => IntegerSuffix::UL, (true, false) => IntegerSuffix::U, (false, true) => IntegerSuffix::L, (false, false) => unreachable!() };
                        Ok::<Literal, std::num::ParseIntError>(Literal::IntegerWithSuffix(val, kind))
                    } else {
                        Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                    }
                },
            ),
        )),
    )(input)
}

// Parse a floating-point literal with underscores and exponent: 1.23, .5, 1e10, 1_2.3_4e-5
pub fn parse_float(input: &str) -> BResult<&str, Literal> {
    context(
        "floating-point literal (decimal with optional exponent, underscores allowed)",
        |i| {
            let (rest, matched) = recognize(tuple((
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
                // optional float/decimal suffix
                opt(map_opt(nom::character::complete::one_of("fFdDmM"), Some)),
            )))(i)?;

            // Determine suffix
            let suffix = matched.chars().last().filter(|c| "fFdDmM".contains(*c));
            match suffix {
                Some('m') | Some('M') => {
                    // Decimal literal -> return Decimal with normalized number (without suffix)
                    let num = matched[..matched.len()-1].trim();
                    let normalized = strip_underscores(num);
                    Ok((rest, Literal::Decimal(normalized)))
                }
                _ => {
                    let normalized = strip_underscores(matched);
                    // If suffix existed and was f/d, parsing as f64 still fine
                    let num = if suffix.is_some() { &normalized[..normalized.len()-1] } else { &normalized[..] };
                    let val = num.parse::<f64>().map(Literal::Float).map_err(|_| nom::Err::Error(make_error(i, ErrorKind::Float)))?;
                    Ok((rest, val))
                }
            }
        },
    )(input)
}

// Parse a decimal literal like 123m or 1.23m (no exponent). Returns Decimal with normalized content.
pub fn parse_decimal_literal(input: &str) -> BResult<&str, Literal> {
    context(
        "decimal literal (expected digits with optional fraction ending with m/M)",
        |i| {
            // digits [ . digits ] m/M
            let (rest, matched) = recognize(tuple((
                take_while1(is_dec),
                opt(tuple((nom_char('.'), take_while1(is_dec)))),
                map_opt(nom::character::complete::one_of("mM"), Some),
            )))(i)?;
            let num = &matched[..matched.len()-1];
            Ok((rest, Literal::Decimal(strip_underscores(num))))
        },
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
            let (rest2, _) = nom_char('"')(rest)?;
            let mut content = String::new();
            let mut chars = rest2.chars().peekable();
            let mut consumed = 0usize;
            // Track last time we inserted a doubled quote into content so that
            // if the input ends immediately after a doubled quote pair, we can
            // reinterpret it as a single escaped quote plus a closing delimiter.
            let mut last_doubled_quote_insert_len: Option<usize> = None;
            while let Some(ch) = chars.next() {
                consumed += ch.len_utf8();
                if ch == '"' {
                    // doubled quote => literal quote
                    if let Some('"') = chars.peek().copied() {
                        // consume the second quote
                        let _ = chars.next();
                        consumed += 1;
                        content.push('"');
                        last_doubled_quote_insert_len = Some(content.len());
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
            // If we reach here, missing closing quote. If the last event was a doubled quote,
            // treat the second of that pair as the closing delimiter.
            if let Some(len) = last_doubled_quote_insert_len {
                if len > 0 { content.truncate(len - 1); }
                return Ok(("", Literal::VerbatimString(content)));
            }
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
                let trimmed = trim_raw_content_roslyn(content);
                return Ok((rest, Literal::RawString(trimmed)));
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
            parse_decimal_literal,     // Decimal literals with m/M must be detected before ints
            parse_float,
            parse_integer,
            parse_raw_interpolated_string, // Raw interpolated strings before non-raw
            parse_interpolated_string, // Try interpolated strings before regular strings
            parse_verbatim_string,     // Try verbatim strings before regular strings
            parse_raw_string,          // Try raw strings before regular strings
            parse_string,
            parse_char_literal,
            // Add other literal types here (null, etc.) if needed
        ))),
    )(input)
}

// Helper: trim multi-line raw string content according to trailing indentation
fn trim_raw_content_roslyn(content_raw: &str) -> String {
    // Determine if closing delimiter is on its own line; if not, no trimming.
    let (before_last_newline, trailing_segment) = match content_raw.rsplit_once('\n') {
        Some(pair) => pair,
        None => return content_raw.to_string(),
    };

    // The trailing segment must be all spaces/tabs to count as indent.
    if !trailing_segment.chars().all(|c| c == ' ' || c == '\t') {
        return content_raw.to_string();
    }
    let closing_indent = trailing_segment;

    // Remove the trailing newline + indent-only closing line from content
    let mut body = before_last_newline.to_string();

    // Remove a single leading newline if present (common multi-line style)
    if body.starts_with('\n') {
        body.remove(0);
    }

    // For each line, remove as much of the exact closing_indent prefix as present (character-by-character match)
    let mut out_lines: Vec<String> = Vec::new();
    for line in body.split('\n') {
        let mut _cut_bytes = 0usize;
        let mut line_chars = line.chars();
        let mut indent_chars = closing_indent.chars();
        let mut consumed_line = 0usize;
        let mut _consumed_indent = 0usize;
        loop {
            match (indent_chars.next(), line_chars.next()) {
                (Some(ic), Some(lc)) if (ic == ' ' || ic == '\t') && lc == ic => {
                    _consumed_indent += ic.len_utf8();
                    consumed_line += lc.len_utf8();
                }
                _ => {
                    _cut_bytes = consumed_line;
                    break;
                }
            }
        }
        out_lines.push(line.get(_cut_bytes..).unwrap_or("").to_string());
    }

    out_lines.join("\n")
}
