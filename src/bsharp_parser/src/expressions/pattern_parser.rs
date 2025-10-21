use crate::parser::expressions::literal_parser::parse_literal;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::contextual_misc_keywords::kw_var;
use crate::parser::keywords::pattern_keywords::{kw_and, kw_not, kw_or};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use crate::syntax::list_parser::parse_delimited_list0;
use nom::combinator::cut;
use nom::Parser;
use nom::{
    branch::alt,
    character::complete::char as nom_char,
    combinator::{map, opt},
    sequence::{delimited, preceded},
};
use syntax::expressions::{
    Expression, ListPatternElement, MemberAccessExpression, Pattern, PatternDesignation,
    PropertySubpattern, RelationalOperator,
};

/// Parse any pattern - entry point for pattern parsing
pub fn parse_pattern(input: Span) -> BResult<Pattern> {
    parse_logical_or_pattern(input)
}

/// Parse logical OR patterns (pattern1 or pattern2)
fn parse_logical_or_pattern(input: Span) -> BResult<Pattern> {
    // Use fold_many0 approach from Nom docs for left-associative parsing
    let (input, first) = parse_logical_and_pattern(input)?;
    let (input, rest) =
        nom::multi::many0(preceded(delimited(ws, kw_or(), ws), parse_logical_and_pattern)).parse(input)?;

    // Fold the results into a left-associative tree
    Ok((
        input,
        rest.into_iter().fold(first, |acc, next| {
            Pattern::LogicalOr(Box::new(acc), Box::new(next))
        }),
    ))
}

/// Parse logical AND patterns (pattern1 and pattern2)
fn parse_logical_and_pattern(input: Span) -> BResult<Pattern> {
    // Use fold_many0 approach from Nom docs for left-associative parsing
    let (input, first) = parse_not_pattern(input)?;
    let (input, rest) = nom::multi::many0(preceded(delimited(ws, kw_and(), ws), parse_not_pattern)).parse(input)?;

    // Fold the results into a left-associative tree
    Ok((
        input,
        rest.into_iter().fold(first, |acc, next| {
            Pattern::LogicalAnd(Box::new(acc), Box::new(next))
        }),
    ))
}

/// Parse NOT patterns (not pattern)
fn parse_not_pattern(input: Span) -> BResult<Pattern> {
    if let Ok((i2, pat)) = map(
        preceded(kw_not(), delimited(ws, parse_relational_pattern, ws)),
        |pattern| Pattern::Not(Box::new(pattern)),
    )
        .parse(input)
    {
        return Ok((i2, pat));
    }
    parse_relational_pattern(input)
}

/// Parse relational patterns (> value, < value, etc.)
fn parse_relational_pattern(input: Span) -> BResult<Pattern> {
    if let Ok((i2, out)) = map(
        (
            alt((
                map(tok_ge(), |_| RelationalOperator::GreaterThanOrEqual),
                map(tok_le(), |_| RelationalOperator::LessThanOrEqual),
                map(tok_gt(), |_| RelationalOperator::GreaterThan),
                map(tok_lt(), |_| RelationalOperator::LessThan),
                map(tok_equal(), |_| RelationalOperator::Equal),
                map(tok_not_equal(), |_| RelationalOperator::NotEqual),
            )),
            delimited(ws, parse_pattern_expression, ws), // Use specialized pattern expression syntax
        ),
        |(op, value)| Pattern::Relational { op, value },
    )
        .parse(input)
    {
        return Ok((i2, out));
    }
    parse_primary_pattern(input)
}

/// Parse simple expressions for use in patterns (NO RECURSION)
/// This follows Nom's principle of small, specific parser
fn parse_pattern_expression(input: Span) -> BResult<Expression> {
    if let Ok(r) = map(parse_literal, Expression::Literal).parse(input) { return Ok(r); }
    if let Ok(r) = map(parse_identifier, Expression::Variable).parse(input) { return Ok(r); }
    if let Ok(r) = delimited(
        delimited(ws, tok_l_paren(), ws),
        parse_pattern_expression,
        cut(delimited(ws, tok_r_paren(), ws)),
    )
        .parse(input)
    { return Ok(r); }
    map(
        (
            parse_identifier,
            preceded(delimited(ws, nom_char('.'), ws), parse_identifier),
        ),
        |(obj, member)| {
            Expression::MemberAccess(Box::new(MemberAccessExpression {
                object: Box::new(Expression::Variable(obj)),
                member,
            }))
        },
    )
        .parse(input)
}

/// Parse primary patterns (leaf patterns)
fn parse_primary_pattern(input: Span) -> BResult<Pattern> {
    if let Ok(r) = parse_discard_pattern(input) { return Ok(r); }
    if let Ok(r) = parse_var_pattern(input) { return Ok(r); }
    if let Ok(r) = parse_list_pattern(input) { return Ok(r); }
    if let Ok(r) = parse_property_pattern(input) { return Ok(r); }
    if let Ok(r) = parse_positional_pattern(input) { return Ok(r); }
    if let Ok(r) = parse_tuple_pattern(input) { return Ok(r); }
    if let Ok(r) = parse_type_pattern(input) { return Ok(r); }
    if let Ok(r) = parse_parenthesized_pattern(input) { return Ok(r); }
    parse_constant_pattern(input)
}

/// Parse discard pattern (_)
fn parse_discard_pattern(input: Span) -> BResult<Pattern> {
    map(delimited(ws, nom_char('_'), ws), |_| Pattern::Discard)
        .parse(input)
}

/// Parse var pattern (var identifier)
fn parse_var_pattern(input: Span) -> BResult<Pattern> {
    map(preceded(kw_var(), delimited(ws, parse_identifier, ws)), Pattern::Var)
        .parse(input)
}

/// Parse type pattern (Type or Type designation)
fn parse_type_pattern(input: Span) -> BResult<Pattern> {
    let (input, target_type) = parse_type_expression(input)?;
    let (input, designation) = opt(|i| delimited(ws, parse_pattern_designation, ws).parse(i)).parse(input)?;

    Ok((
        input,
        Pattern::Type {
            target_type,
            designation,
        },
    ))
}

/// Parse pattern designation (variable or discard)
fn parse_pattern_designation(input: Span) -> BResult<PatternDesignation> {
    if let Ok(r) = map(delimited(ws, nom_char('_'), ws), |_| PatternDesignation::Discard).parse(input) { return Ok(r); }
    if let Ok(r) = map(parse_identifier, PatternDesignation::Variable).parse(input) { return Ok(r); }
    map(
        delimited(
            delimited(ws, tok_l_paren(), ws),
            parse_pattern_designation,
            delimited(ws, tok_r_paren(), ws),
        ),
        |des| PatternDesignation::Parenthesized(Box::new(des)),
    )
        .parse(input)
}

/// Parse list pattern [pattern1, pattern2, ..]
fn parse_list_pattern(input: Span) -> BResult<Pattern> {
    map(
        parse_delimited_list0::<_, _, _, _, char, char, char, ListPatternElement>(
           tok_l_brack(),
            parse_list_pattern_element,
            tok_comma(),
            tok_r_brack(),
            false,
            true,
        ),
        |patterns| Pattern::List { patterns },
    )
        .parse(input)
}

/// Parse list pattern element (pattern or slice)
fn parse_list_pattern_element(input: Span) -> BResult<ListPatternElement> {
    alt((
        // Slice pattern: .. or ..pattern
        map(
            preceded(tok_range(), opt(parse_pattern)),
            ListPatternElement::Slice,
        ),
        // Regular pattern
        map(parse_pattern, ListPatternElement::Pattern),
    ))
        .parse(input)
}

/// Parse tuple pattern - handle carefully to avoid conflicts
fn parse_tuple_pattern(input: Span) -> BResult<Pattern> {
    // Only parse as tuple if we have multiple patterns separated by commas
    map(
        delimited(
            delimited(ws, tok_l_paren(), ws),
            (
                parse_pattern,
                nom::multi::many1(preceded(delimited(ws, tok_comma(), ws), delimited(ws, parse_pattern, ws))),
                opt(delimited(ws, tok_comma(), ws)), // Optional trailing comma
            ),
            cut(delimited(ws, tok_r_paren(), ws)),
        ),
        |(first, mut rest, _)| {
            rest.insert(0, first);
            Pattern::Tuple(rest)
        },
    )
        .parse(input)
}

/// Parse property pattern { Property1: pattern1, Property2: pattern2 }
fn parse_property_pattern(input: Span) -> BResult<Pattern> {
    map(
        (
            opt(parse_type_expression), // Optional type annotation
            parse_delimited_list0::<
                _,
                _,
                _,
                _,
                char,
                char,
                char,
                PropertySubpattern,
            >(
                tok_l_brace(),
                parse_property_subpattern,
                tok_comma(),
                tok_r_brace(),
                false,
                true,
            ),
        ),
        |(type_name, subpatterns)| Pattern::Property {
            type_name,
            subpatterns,
        },
    )
        .parse(input)
}

/// Parse property subpattern (PropertyName: pattern)
fn parse_property_subpattern(input: Span) -> BResult<PropertySubpattern> {
    map(
        (
            delimited(ws, parse_identifier, ws),
            delimited(ws, tok_colon(), ws),
            delimited(ws, parse_pattern, ws),
        ),
        |(member_name, _, pattern)| PropertySubpattern {
            member_name,
            pattern,
        },
    )
        .parse(input)
}

/// Parse positional pattern Type(pattern1, pattern2, ...)
fn parse_positional_pattern(input: Span) -> BResult<Pattern> {
    map(
        (
            parse_type_expression,
            parse_delimited_list0::<_, _, _, _, char, char, char, Pattern>(
                tok_l_paren(),
                parse_pattern,
                tok_comma(),
                tok_r_paren(),
                false,
                false, // avoid cut to preserve upstream recovery here
            ),
        ),
        |(type_name, subpatterns)| Pattern::Positional {
            type_name: Some(type_name),
            subpatterns,
        },
    )
        .parse(input)
}

/// Parse parenthesized pattern ((pattern))
fn parse_parenthesized_pattern(input: Span) -> BResult<Pattern> {
    // Only parse as parenthesized if it's a single pattern, not a tuple
    map(
        delimited(delimited(ws, tok_l_paren(), ws), parse_pattern, cut(delimited(ws, tok_r_paren(), ws))),
        |pattern| Pattern::Parenthesized(Box::new(pattern)),
    )
        .parse(input)
}

/// Parse constant pattern (literal value) - NO RECURSION
fn parse_constant_pattern(input: Span) -> BResult<Pattern> {
    map(parse_pattern_expression, Pattern::Constant).parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::delimiters::{tok_l_brace, tok_l_brack, tok_l_paren, tok_r_brace, tok_r_brack, tok_r_paren};
use crate::tokens::equality::{tok_equal, tok_not_equal};
use crate::tokens::range::tok_range;
use crate::tokens::relational::{tok_ge, tok_gt, tok_le, tok_lt};
use crate::tokens::separators::{tok_colon, tok_comma};
