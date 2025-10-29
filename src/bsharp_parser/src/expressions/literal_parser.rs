use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
// This is used by parse_interpolation
use crate::keywords::literal_keywords::{kw_false, kw_null, kw_true};
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Input;
use nom::Parser;
use nom::error::{ErrorKind, make_error};
use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not as b_is_not, tag_no_case},
    character::complete::{char as nom_char, none_of, satisfy},
    combinator::{cut, map, map_opt, map_res, opt, recognize, value},
    multi::{count, many0, many1},
    sequence::{delimited, preceded},
};
use nom_supreme::ParserExt;
use crate::span::Spanned;
use crate::span_ext::ParserExt as _;
use syntax::expressions::literal::InterpolatedStringPart;
use syntax::expressions::literal::{IntegerSuffix, InterpolatedStringLiteral};
use syntax::expressions::{Expression, Literal};
// Use the global comment-aware bws wrapper instead of a local whitespace helper

// Parse a boolean literal (true or false)
pub fn parse_boolean(input: Span) -> BResult<Literal> {
    alt((
        map(kw_true(), |_| Literal::Boolean(true)),
        map(kw_false(), |_| Literal::Boolean(false)),
    ))
    .context("boolean literal (expected 'true' or 'false')")
    .parse(input)
}

pub fn parse_literal_spanned(input: Span) -> BResult<Spanned<Literal>> {
    use nom::{branch::alt, combinator::map};
    let core = alt((
        parse_boolean,
        map(kw_null(), |_| Literal::Null),
        parse_decimal_literal,
        parse_float,
        parse_integer,
        parse_raw_interpolated_string,
        parse_interpolated_string,
        parse_verbatim_string,
        parse_raw_string,
        parse_string,
        parse_char_literal,
    ));
    nom::sequence::delimited(ws, core.spanned(), ws).parse(input)
}

// Parse a raw interpolated string literal: $""" ... {expr} ... """ or with multiple $ and N quotes
pub fn parse_raw_interpolated_string<'a>(input: Span<'a>) -> BResult<'a, Literal> {
    (|i: Span<'a>| {
        let s = i.fragment();
        // Count leading dollars
        let mut dollar_count = 0usize;
        for ch in s.chars() {
            if ch == '$' {
                dollar_count += 1;
            } else {
                break;
            }
        }
        if dollar_count == 0 {
            return Err(nom::Err::Error(make_error(i, ErrorKind::Tag)));
        }
        // Count opening quotes (must be >= 3)
        let mut qcount = 0usize;
        for ch in s[dollar_count..].chars() {
            if ch == '"' {
                qcount += 1;
            } else {
                break;
            }
        }
        if qcount < 3 {
            return Err(nom::Err::Error(make_error(i, ErrorKind::Tag)));
        }
        let content_start = dollar_count + qcount;
        let closing = "\"".repeat(qcount);
        let after_open = &s[content_start..];
        if let Some(rel_idx) = after_open.find(&closing) {
            let content_raw = &after_open[..rel_idx];
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

            let total = content_start + rel_idx + closing.len();
            let (rest_span, _) = i.take_split(total);
            return Ok((
                rest_span,
                Literal::InterpolatedString(InterpolatedStringLiteral {
                    parts,
                    is_verbatim: true,
                }),
            ));
        }
        Err(nom::Err::Error(make_error(i, ErrorKind::Tag)))
    })
    .context("raw interpolated string literal")
    .parse(input)
}

fn parse_raw_interpolation_core(core: &str) -> Option<InterpolatedStringPart> {
    use syntax::expressions::expression::Expression;
    let s = core.trim();
    if let Ok((rest0, expression)) = robust_expression_in_interpolation(s.into()) {
        let mut rest = rest0.trim_start();
        // Optional alignment
        let mut alignment: Option<Expression> = None;
        if rest.starts_with(',') {
            let rest2 = rest[1..].trim_start();
            if let Ok((rest_after_align, align_expr)) =
                robust_expression_in_interpolation(rest2.into())
            {
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
        Some(InterpolatedStringPart::Interpolation {
            expression,
            alignment,
            format_string,
        })
    } else {
        None
    }
}

// Helpers for digits with underscores
fn strip_underscores(s: &str) -> String {
    s.chars().filter(|&c| c != '_').collect()
}

fn is_hex(c: char) -> bool {
    c.is_ascii_hexdigit() || c == '_'
}
fn is_bin(c: char) -> bool {
    c == '0' || c == '1' || c == '_'
}
fn is_dec(c: char) -> bool {
    c.is_ascii_digit() || c == '_'
}
fn is_int_suffix(c: char) -> bool {
    matches!(c, 'u' | 'U' | 'l' | 'L')
}

// Parse an integer literal: supports decimal, 0x hex, 0b binary, underscores
pub fn parse_integer<'a>(input: Span<'a>) -> BResult<'a, Literal> {
    alt((
        // Hex 0x...
        map_res(
            recognize((
                tag_no_case("0x"),
                recognize(many1(satisfy(is_hex))),
                opt(recognize(many1(satisfy(is_int_suffix)))),
            )),
            |s: Span<'a>| {
                let s = s.fragment();
                use syntax::expressions::literal::IntegerSuffix;
                // strip prefix and optional suffix letters
                let core = &s[2..];
                let trimmed = core.trim_end_matches(is_int_suffix);
                let removed = core.len() - trimmed.len();
                let suffix = if removed > 0 {
                    Some(&core[trimmed.len()..])
                } else {
                    None
                };
                let val = i64::from_str_radix(&strip_underscores(trimmed), 16)?;
                if let Some(sfx) = suffix {
                    let mut has_u = false;
                    let mut has_l = false;
                    for ch in sfx.chars() {
                        match ch {
                            'u' | 'U' => has_u = true,
                            'l' | 'L' => has_l = true,
                            _ => {}
                        }
                    }
                    let kind_opt = match (has_u, has_l) {
                        (true, true) => Some(IntegerSuffix::UL),
                        (true, false) => Some(IntegerSuffix::U),
                        (false, true) => Some(IntegerSuffix::L),
                        (false, false) => None,
                    };
                    if let Some(kind) = kind_opt {
                        Ok::<Literal, std::num::ParseIntError>(Literal::IntegerWithSuffix(
                            val, kind,
                        ))
                    } else {
                        Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                    }
                } else {
                    Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                }
            },
        ),
        // Binary 0b...
        map_res(
            recognize((
                tag_no_case("0b"),
                recognize(many1(satisfy(is_bin))),
                opt(recognize(many1(satisfy(is_int_suffix)))),
            )),
            |s: Span<'a>| {
                let s = s.fragment();
                use IntegerSuffix;
                let core = &s[2..];
                let trimmed = core.trim_end_matches(is_int_suffix);
                let removed = core.len() - trimmed.len();
                let suffix = if removed > 0 {
                    Some(&core[trimmed.len()..])
                } else {
                    None
                };
                let val = i64::from_str_radix(&strip_underscores(trimmed), 2)?;
                if let Some(sfx) = suffix {
                    let mut has_u = false;
                    let mut has_l = false;
                    for ch in sfx.chars() {
                        match ch {
                            'u' | 'U' => has_u = true,
                            'l' | 'L' => has_l = true,
                            _ => {}
                        }
                    }
                    let kind_opt = match (has_u, has_l) {
                        (true, true) => Some(IntegerSuffix::UL),
                        (true, false) => Some(IntegerSuffix::U),
                        (false, true) => Some(IntegerSuffix::L),
                        (false, false) => None,
                    };
                    if let Some(kind) = kind_opt {
                        Ok::<Literal, std::num::ParseIntError>(Literal::IntegerWithSuffix(
                            val, kind,
                        ))
                    } else {
                        Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                    }
                } else {
                    Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                }
            },
        ),
        // Decimal
        map_res(
            recognize((
                recognize(many1(satisfy(is_dec))),
                opt(recognize(many1(satisfy(is_int_suffix)))),
            )),
            |s: Span<'a>| {
                let s = s.fragment();
                use IntegerSuffix;
                let trimmed = s.trim_end_matches(is_int_suffix);
                let removed = s.len() - trimmed.len();
                let suffix = if removed > 0 {
                    Some(&s[trimmed.len()..])
                } else {
                    None
                };
                let val = strip_underscores(trimmed).parse::<i64>()?;
                if let Some(sfx) = suffix {
                    let mut has_u = false;
                    let mut has_l = false;
                    for ch in sfx.chars() {
                        match ch {
                            'u' | 'U' => has_u = true,
                            'l' | 'L' => has_l = true,
                            _ => {}
                        }
                    }
                    let kind_opt = match (has_u, has_l) {
                        (true, true) => Some(IntegerSuffix::UL),
                        (true, false) => Some(IntegerSuffix::U),
                        (false, true) => Some(IntegerSuffix::L),
                        (false, false) => None,
                    };
                    if let Some(kind) = kind_opt {
                        Ok::<Literal, std::num::ParseIntError>(Literal::IntegerWithSuffix(
                            val, kind,
                        ))
                    } else {
                        Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                    }
                } else {
                    Ok::<Literal, std::num::ParseIntError>(Literal::Integer(val))
                }
            },
        ),
    ))
    .context("integer literal (decimal, 0x..., or 0b..., underscores allowed)")
    .parse(input)
}

// Parse a floating-point literal with underscores and exponent: 1.23, .5, 1e10, 1_2.3_4e-5
pub fn parse_float<'a>(input: Span<'a>) -> BResult<'a, Literal> {
    (|i: Span<'a>| {
        let (rest, matched): (Span<'a>, Span<'a>) = recognize((
            // integer or empty before dot
            opt(recognize(many1(satisfy(is_dec)))),
            nom_char('.'),
            recognize(many1(satisfy(is_dec))),
            // optional exponent part
            opt((
                alt((nom_char('e'), nom_char('E'))),
                opt(alt((nom_char('+'), nom_char('-')))),
                recognize(many1(satisfy(is_dec))),
            )),
            // optional float/decimal suffix
            opt(map_opt(nom::character::complete::one_of("fFdDmM"), Some)),
        ))
        .parse(i)?;

        // Determine suffix
        let mfrag = matched.fragment();
        let suffix = mfrag.chars().last().filter(|c| "fFdDmM".contains(*c));
        match suffix {
            Some('m') | Some('M') => {
                // Decimal literal -> return Decimal with normalized number (without suffix)
                let num = mfrag[..mfrag.len() - 1].trim();
                let normalized = strip_underscores(num);
                Ok((rest, Literal::Decimal(normalized)))
            }
            _ => {
                let normalized = strip_underscores(mfrag);
                // If suffix existed and was f/d, parsing as f64 still fine
                let num = if suffix.is_some() {
                    &normalized[..normalized.len() - 1]
                } else {
                    &normalized[..]
                };
                let val = num
                    .parse::<f64>()
                    .map(Literal::Float)
                    .map_err(|_| nom::Err::Error(make_error(i, ErrorKind::Float)))?;
                Ok((rest, val))
            }
        }
    })
    .context("floating-point literal")
    .parse(input)
}

// Parse a decimal literal like 123m or 1.23m (no exponent). Returns Decimal with normalized content.
pub fn parse_decimal_literal<'a>(input: Span<'a>) -> BResult<'a, Literal> {
    (|i: Span<'a>| {
        let (rest, matched): (Span<'a>, Span<'a>) = recognize((
            recognize(many1(satisfy(is_dec))),
            opt((nom_char('.'), recognize(many1(satisfy(is_dec))))),
            map_opt(nom::character::complete::one_of("mM"), Some),
        ))
        .parse(i)?;
        let mfrag = matched.fragment();
        let num = &mfrag[..mfrag.len() - 1];
        Ok((rest, Literal::Decimal(strip_underscores(num))))
    })
    .context("decimal literal")
    .parse(input)
}

// Parse a string literal (e.g., "hello", "with \" escape")
pub fn parse_string<'a>(input: Span<'a>) -> BResult<'a, Literal> {
    (|i: Span<'a>| {
        // Parse core string content
        let (rest_after_quote, inner) = delimited(
            tok_double_quote(),
            // Use opt to handle the case of an empty string content ""
            opt(escaped_transform(
                b_is_not("\"\\"), // Normal characters
                '\\',             // Escape character
                alt((
                    // Transformation syntax for escaped chars
                    value("\"", tok_double_quote()),
                    value("\\", tok_double_slash()),
                    value("\n", nom_char('n')),
                    value("\t", nom_char('t')),
                    value("\r", nom_char('r')),
                )),
            )),
            tok_double_quote(),
        )
        .parse(i)?;

        let content = inner.unwrap_or_default();

        // Optional C# 11 u8 suffix immediately after string literal (no whitespace allowed)
        let rest_frag = rest_after_quote.fragment();
        if rest_frag.starts_with("u8") {
            let (rest_after_suffix, _) = rest_after_quote.take_split(2);
            return Ok((rest_after_suffix, Literal::Utf8String(content.into_bytes())));
        }

        Ok((rest_after_quote, Literal::String(content)))
    })
    .context("string literal")
    .parse(input)
}

// Parse a verbatim string literal (@"...")
pub fn parse_verbatim_string<'a>(input: Span<'a>) -> BResult<'a, Literal> {
    (|i: Span<'a>| {
        // Consume @"
        let (i, _) = (tok_at(), tok_double_quote()).parse(i)?;
        let s = i.fragment();
        let mut content = String::new();
        let mut chars = s.chars().peekable();
        let mut consumed = 0usize;
        while let Some(ch) = chars.next() {
            consumed += ch.len_utf8();
            if ch == '"' {
                if let Some('"') = chars.peek().copied() {
                    let _ = chars.next();
                    consumed += 1;
                    content.push('"');
                    continue;
                } else {
                    // closing quote found
                    let (rest_span, _) = i.take_split(consumed);
                    return Ok((rest_span, Literal::VerbatimString(content)));
                }
            } else {
                content.push(ch);
            }
        }
        Err(nom::Err::Error(make_error(i, ErrorKind::Tag)))
    })
    .context("verbatim string literal")
    .parse(input)
}

// Parse a raw string literal ("""text""")
pub fn parse_raw_string<'a>(input: Span<'a>) -> BResult<'a, Literal> {
    (|i: Span<'a>| {
        let s = i.fragment();
        // Count opening quotes
        let mut qcount = 0usize;
        for ch in s.chars() {
            if ch == '"' {
                qcount += 1;
            } else {
                break;
            }
        }
        if qcount < 3 {
            return Err(nom::Err::Error(make_error(i, ErrorKind::Tag)));
        }
        let start = &s[qcount..];
        let closing = "\"".repeat(qcount);
        if let Some(idx) = start.find(&closing) {
            let content = &start[..idx];
            let trimmed = trim_raw_content_roslyn(content);
            let total = qcount + idx + closing.len();
            let (rest_span, _) = i.take_split(total);
            return Ok((rest_span, Literal::RawString(trimmed)));
        }
        Err(nom::Err::Error(make_error(i, ErrorKind::Tag)))
    })
    .context("raw string literal")
    .parse(input)
}

/// Enhanced interpolated string syntax using robust Nom combinators
/// Handles complex patterns like $"Invalid email: {email}" with graceful fallback
pub fn parse_interpolated_string<'a>(input: Span<'a>) -> BResult<'a, Literal> {
    (|input: Span<'a>| {
        // Enhanced prefix recognition with better error handling
        let (input, is_verbatim) = alt((
            map(tok_dollar_at(), |_| true),
            map(tok_at_dollar(), |_| true),
            map(tok_dollar(), |_| false),
        ))
        .parse(input)?;

        let (input, parts) = delimited(
            tok_double_quote(),
            enhanced_interpolated_parts,
            cut(tok_double_quote()), // Use cut for better error reporting
        )
        .parse(input)?;

        Ok((
            input,
            Literal::InterpolatedString(InterpolatedStringLiteral { parts, is_verbatim }),
        ))
    })
    .context("interpolated string literal")
    .parse(input)
}

/// Enhanced parsing of interpolated string parts with better error recovery
fn enhanced_interpolated_parts(input: Span) -> BResult<Vec<InterpolatedStringPart>> {
    many0(|i| {
        if let Ok(r) = enhanced_interpolation(i) {
            return Ok(r);
        }
        enhanced_interpolated_text(i)
    })
    .context("interpolated string parts")
    .parse(input)
}

/// Enhanced text part parsing with better handling of edge cases
fn enhanced_interpolated_text<'a>(input: Span<'a>) -> BResult<'a, InterpolatedStringPart> {
    map(
        recognize(many1(satisfy(|c| c != '{' && c != '"' && c != '\\'))),
        |s: Span<'a>| InterpolatedStringPart::Text(s.fragment().to_string()),
    )
    .context("interpolated string text")
    .parse(input)
}

/// Enhanced interpolation parsing with graceful fallback
fn enhanced_interpolation(input: Span<'_>) -> BResult<'_, InterpolatedStringPart> {
    map(
        delimited(
            tok_l_brace(),
            cut((
                robust_expression_in_interpolation,
                opt(preceded(tok_comma(), robust_expression_in_interpolation)),
                opt(preceded(
                    tok_colon(),
                    recognize(many1(satisfy(|c| c != '}'))),
                )),
            )),
            cut(tok_r_brace()),
        ),
        |(expression, alignment, format_string)| InterpolatedStringPart::Interpolation {
            expression,
            alignment,
            format_string: format_string.map(|s| s.fragment().to_string()),
        },
    )
    .context("string interpolation")
    .parse(input)
}

/// Robust expression parsing within interpolation with fallback
fn robust_expression_in_interpolation(input: Span) -> BResult<Expression> {
    match parse_expression_spanned.parse(input) {
        Ok((rest, s)) => Ok((rest, s.node)),
        Err(_) => fallback_simple_expression(input),
    }
}

/// Fallback syntax for simple expressions when complex parsing fails
fn fallback_simple_expression(input: Span) -> BResult<Expression> {
    use crate::parser::identifier_parser::parse_identifier;
    // `Expression` already imported at module top

    map(parse_identifier, Expression::Variable)
        .context("simple expression")
        .parse(input)
}

// Parse a char literal with escapes: '\\n', '\\t', '\\xFF', '\\u1234', '\\U0001F642'
pub fn parse_char_literal(input: Span) -> BResult<Literal> {
    fn hex_to_char_opt(hex: &str) -> Option<char> {
        let cp = u32::from_str_radix(hex, 16).ok()?;
        char::from_u32(cp)
    }
    map(
        delimited(
            tok_single_quote(),
            alt((
                // simple escape
                map(
                    preceded(
                        nom_char('\\'),
                        alt((
                            value('\n', nom_char('n')),
                            value('\t', nom_char('t')),
                            value('\r', nom_char('r')),
                            value('\\', nom_char('\\')),
                            value('\'', tok_single_quote()),
                            value('"', tok_double_quote()),
                        )),
                    ),
                    |c| c,
                ),
                // hex escape \\xHH.. (1-4 hex digits)
                map_opt(
                    preceded(
                        (nom_char('\\'), nom_char('x')),
                        recognize(many1(nom::character::complete::one_of(
                            "0123456789abcdefABCDEF",
                        ))),
                    ),
                    |hex: Span| hex_to_char_opt(hex.fragment()),
                ),
                // unicode \\uHHHH
                map_opt(
                    preceded(
                        (nom_char('\\'), nom_char('u')),
                        recognize(count(satisfy(|c: char| c.is_ascii_hexdigit()), 4)),
                    ),
                    |hex: Span| hex_to_char_opt(hex.fragment()),
                ),
                // unicode \\UHHHHHHHH (8 hex digits)
                map_opt(
                    preceded(
                        (nom_char('\\'), nom_char('U')),
                        recognize(count(satisfy(|c: char| c.is_ascii_hexdigit()), 8)),
                    ),
                    |hex: Span| hex_to_char_opt(hex.fragment()),
                ),
                // single non-escape character
                map(none_of("'\\"), |c| c),
            )),
            tok_single_quote(),
        ),
        Literal::Char,
    )
    .context("char literal")
    .parse(input)
}

// Main literal syntax: tries boolean, integer, float, string, then char
pub fn parse_literal(input: Span) -> BResult<Literal> {
    (|i| {
        delimited(
            ws,
            alt((
                parse_boolean,
                // null keyword - treat as a special literal
                map(kw_null(), |_| Literal::Null),
                // Try float before integer to handle cases like "3.14"
                // which would otherwise be partially parsed as integer "3"
                parse_decimal_literal, // Decimal literals with m/M must be detected before ints
                parse_float,
                parse_integer,
                parse_raw_interpolated_string, // Raw interpolated strings before non-raw
                parse_interpolated_string,     // Try interpolated strings before regular strings
                parse_verbatim_string,         // Try verbatim strings before regular strings
                parse_raw_string,              // Try raw strings before regular strings
                parse_string,
                parse_char_literal,
                // Add other literal types here (null, etc.) if needed
            )),
            ws,
        )
        .parse(i)
    })
    .context("literal")
    .parse(input)
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
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_brace, tok_r_brace};
use crate::tokens::quotes::{tok_double_quote, tok_single_quote};
use crate::tokens::separators::{tok_colon, tok_comma};
use crate::tokens::sigils::{tok_at, tok_at_dollar, tok_dollar, tok_dollar_at};
use crate::tokens::string::tok_double_slash;
