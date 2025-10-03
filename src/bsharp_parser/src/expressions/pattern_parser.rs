use crate::parser::expressions::literal_parser::parse_literal;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::contextual_misc_keywords::kw_var;
use crate::parser::keywords::pattern_keywords::{kw_and, kw_not, kw_or};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::pattern::*;
use crate::syntax::parser_helpers::{bws, keyword, parse_delimited_list0};

use nom::combinator::cut;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char as nom_char,
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
};

/// Parse any pattern - entry point for pattern parsing
pub fn parse_pattern(input: &str) -> BResult<&str, Pattern> {
    parse_logical_or_pattern(input)
}

/// Parse logical OR patterns (pattern1 or pattern2)
fn parse_logical_or_pattern(input: &str) -> BResult<&str, Pattern> {
    // Use fold_many0 approach from Nom docs for left-associative parsing
    let (input, first) = parse_logical_and_pattern(input)?;
    let (input, rest) =
        nom::multi::many0(preceded(bws(kw_or()), parse_logical_and_pattern))(input)?;

    // Fold the results into a left-associative tree
    Ok((
        input,
        rest.into_iter().fold(first, |acc, next| {
            Pattern::LogicalOr(Box::new(acc), Box::new(next))
        }),
    ))
}

/// Parse logical AND patterns (pattern1 and pattern2)
fn parse_logical_and_pattern(input: &str) -> BResult<&str, Pattern> {
    // Use fold_many0 approach from Nom docs for left-associative parsing
    let (input, first) = parse_not_pattern(input)?;
    let (input, rest) = nom::multi::many0(preceded(bws(kw_and()), parse_not_pattern))(input)?;

    // Fold the results into a left-associative tree
    Ok((
        input,
        rest.into_iter().fold(first, |acc, next| {
            Pattern::LogicalAnd(Box::new(acc), Box::new(next))
        }),
    ))
}

/// Parse NOT patterns (not pattern)
fn parse_not_pattern(input: &str) -> BResult<&str, Pattern> {
    alt((
        map(
            preceded(kw_not(), bws(parse_relational_pattern)),
            |pattern| Pattern::Not(Box::new(pattern)),
        ),
        parse_relational_pattern,
    ))(input)
}

/// Parse relational patterns (> value, < value, etc.)
fn parse_relational_pattern(input: &str) -> BResult<&str, Pattern> {
    alt((
        // Relational patterns: > 5, <= 10, etc.
        map(
            tuple((
                alt((
                    map(tag(">="), |_| RelationalOperator::GreaterThanOrEqual),
                    map(tag("<="), |_| RelationalOperator::LessThanOrEqual),
                    map(nom_char('>'), |_| RelationalOperator::GreaterThan),
                    map(nom_char('<'), |_| RelationalOperator::LessThan),
                    map(tag("=="), |_| RelationalOperator::Equal),
                    map(tag("!="), |_| RelationalOperator::NotEqual),
                )),
                bws(parse_pattern_expression), // Use specialized pattern expression syntax
            )),
            |(op, value)| Pattern::Relational { op, value },
        ),
        parse_primary_pattern,
    ))(input)
}

/// Parse simple expressions for use in patterns (NO RECURSION)
/// This follows Nom's principle of small, specific parser
fn parse_pattern_expression(input: &str) -> BResult<&str, Expression> {
    alt((
        // Literals first (most specific)
        map(parse_literal, Expression::Literal),
        // Identifiers (variables)
        map(parse_identifier, Expression::Variable),
        // Parenthesized pattern expressions
        delimited(
            bws(nom_char('(')),
            parse_pattern_expression,
            cut(bws(nom_char(')'))),
        ),
        // Member access: obj.member (but no further nesting)
        map(
            tuple((parse_identifier, preceded(nom_char('.'), parse_identifier))),
            |(obj, member)| {
                Expression::MemberAccess(Box::new(
                crate::syntax::nodes::expressions::member_access_expression::MemberAccessExpression {
                    object: Box::new(Expression::Variable(obj)),
                    member,
                }
            ))
            },
        ),
    ))(input)
}

/// Parse primary patterns (leaf patterns)
fn parse_primary_pattern(input: &str) -> BResult<&str, Pattern> {
    alt((
        parse_discard_pattern,
        parse_var_pattern,
        parse_list_pattern,
        parse_property_pattern,
        parse_positional_pattern,
        parse_tuple_pattern,
        parse_type_pattern,
        parse_parenthesized_pattern, // Add back parenthesized patterns
        parse_constant_pattern,      // This now uses parse_pattern_expression
    ))(input)
}

/// Parse discard pattern (_)
fn parse_discard_pattern(input: &str) -> BResult<&str, Pattern> {
    map(keyword("_"), |_| Pattern::Discard)(input)
}

/// Parse var pattern (var identifier)
fn parse_var_pattern(input: &str) -> BResult<&str, Pattern> {
    map(preceded(kw_var(), bws(parse_identifier)), Pattern::Var)(input)
}

/// Parse type pattern (Type or Type designation)
fn parse_type_pattern(input: &str) -> BResult<&str, Pattern> {
    let (input, target_type) = parse_type_expression(input)?;
    let (input, designation) = opt(bws(parse_pattern_designation))(input)?;

    Ok((
        input,
        Pattern::Type {
            target_type,
            designation,
        },
    ))
}

/// Parse pattern designation (variable or discard)
fn parse_pattern_designation(input: &str) -> BResult<&str, PatternDesignation> {
    alt((
        map(keyword("_"), |_| PatternDesignation::Discard),
        map(parse_identifier, PatternDesignation::Variable),
        map(
            delimited(
                bws(nom_char('(')),
                parse_pattern_designation,
                bws(nom_char(')')),
            ),
            |des| PatternDesignation::Parenthesized(Box::new(des)),
        ),
    ))(input)
}

/// Parse list pattern [pattern1, pattern2, ..]
fn parse_list_pattern(input: &str) -> BResult<&str, Pattern> {
    map(
        parse_delimited_list0::<_, _, _, _, char, ListPatternElement, char, char, ListPatternElement>(
            nom_char('['),
            parse_list_pattern_element,
            nom_char(','),
            nom_char(']'),
            false,
            true,
        ),
        |patterns| Pattern::List { patterns },
    )(input)
}

/// Parse list pattern element (pattern or slice)
fn parse_list_pattern_element(input: &str) -> BResult<&str, ListPatternElement> {
    alt((
        // Slice pattern: .. or ..pattern
        map(
            preceded(tag(".."), opt(parse_pattern)),
            ListPatternElement::Slice,
        ),
        // Regular pattern
        map(parse_pattern, ListPatternElement::Pattern),
    ))(input)
}

/// Parse tuple pattern - handle carefully to avoid conflicts
fn parse_tuple_pattern(input: &str) -> BResult<&str, Pattern> {
    // Only parse as tuple if we have multiple patterns separated by commas
    map(
        delimited(
            bws(nom_char('(')),
            tuple((
                parse_pattern,
                nom::multi::many1(preceded(bws(nom_char(',')), bws(parse_pattern))),
                opt(bws(nom_char(','))), // Optional trailing comma
            )),
            cut(bws(nom_char(')'))),
        ),
        |(first, mut rest, _)| {
            rest.insert(0, first);
            Pattern::Tuple(rest)
        },
    )(input)
}

/// Parse property pattern { Property1: pattern1, Property2: pattern2 }
fn parse_property_pattern(input: &str) -> BResult<&str, Pattern> {
    map(
        tuple((
            opt(parse_type_expression), // Optional type annotation
            parse_delimited_list0::<
                _,
                _,
                _,
                _,
                char,
                PropertySubpattern,
                char,
                char,
                PropertySubpattern,
            >(
                nom_char('{'),
                parse_property_subpattern,
                nom_char(','),
                nom_char('}'),
                false,
                true,
            ),
        )),
        |(type_name, subpatterns)| Pattern::Property {
            type_name,
            subpatterns,
        },
    )(input)
}

/// Parse property subpattern (PropertyName: pattern)
fn parse_property_subpattern(input: &str) -> BResult<&str, PropertySubpattern> {
    map(
        tuple((
            bws(parse_identifier),
            bws(nom_char(':')),
            bws(parse_pattern),
        )),
        |(member_name, _, pattern)| PropertySubpattern {
            member_name,
            pattern,
        },
    )(input)
}

/// Parse positional pattern Type(pattern1, pattern2, ...)
fn parse_positional_pattern(input: &str) -> BResult<&str, Pattern> {
    map(
        tuple((
            parse_type_expression,
            parse_delimited_list0::<_, _, _, _, char, Pattern, char, char, Pattern>(
                nom_char('('),
                parse_pattern,
                nom_char(','),
                nom_char(')'),
                false,
                false, // avoid cut to preserve upstream recovery here
            ),
        )),
        |(type_name, subpatterns)| Pattern::Positional {
            type_name: Some(type_name),
            subpatterns,
        },
    )(input)
}

/// Parse parenthesized pattern ((pattern))
fn parse_parenthesized_pattern(input: &str) -> BResult<&str, Pattern> {
    // Only parse as parenthesized if it's a single pattern, not a tuple
    map(
        delimited(bws(nom_char('(')), parse_pattern, cut(bws(nom_char(')')))),
        |pattern| Pattern::Parenthesized(Box::new(pattern)),
    )(input)
}

/// Parse constant pattern (literal value) - NO RECURSION
fn parse_constant_pattern(input: &str) -> BResult<&str, Pattern> {
    map(parse_pattern_expression, Pattern::Constant)(input)
}
