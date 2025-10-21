use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::contextual_misc_keywords::kw_dynamic;
use crate::parser::keywords::linq_query_keywords::{
    kw_ascending, kw_by, kw_descending, kw_equals, kw_from, kw_group, kw_into, kw_join, kw_let,
    kw_on, kw_orderby, kw_select, kw_where,
};
use crate::parser::keywords::parameter_modifier_keywords::kw_in;
use crate::parser::keywords::type_keywords::{
    kw_bool, kw_byte, kw_char, kw_decimal, kw_double, kw_int, kw_long, kw_object, kw_sbyte,
    kw_short, kw_string, kw_uint, kw_ulong, kw_ushort,
};
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use nom::sequence::delimited;
use nom::Parser;
use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::{preceded},
};
use syntax::expressions::{
    Expression, FromClause, JoinClause, LetClause, OrderByOrdering, OrderingDirection, QueryClause,
    QueryContinuation, QueryExpression, QueryOrderByClause, QuerySelectOrGroup, QueryWhereClause,
};
use syntax::Identifier;

/// Parse primitive type identifiers for LINQ type annotations
fn parse_primitive_type_identifier(input: Span) -> BResult<Identifier> {
    map(
        alt((
            // Order matters! Put longer keywords first to avoid partial matches
            kw_ushort(),
            kw_uint(),
            kw_ulong(),
            kw_sbyte(),
            kw_short(),
            kw_byte(),
            kw_bool(),
            kw_int(),
            kw_long(),
            kw_double(),
            kw_decimal(),
            kw_string(),
            kw_object(),
            kw_char(),
            kw_dynamic(),
        )),
        Identifier::new,
    )
        .parse(input)
}

/// Parse a complete LINQ query expression
pub fn parse_query_expression(input: Span) -> BResult<Expression> {
    map(
        (
            parse_from_clause,
            many0(parse_query_clause),
            parse_select_or_group_clause,
            opt(parse_query_continuation),
        ),
        |(from, body, select_or_group, continuation)| {
            Expression::Query(Box::new(QueryExpression {
                from,
                body,
                select_or_group,
                continuation,
            }))
        },
    )
        .parse(input)
}

/// Parse the initial 'from' clause
fn parse_from_clause(input: Span) -> BResult<FromClause> {
    map(
        (
            kw_from(),
            delimited(ws, alt((
                // Type annotation case: from PrimitiveType identifier in expression
                map(
                    (
                        parse_primitive_type_identifier, // Parse known type keywords
                        delimited(ws, parse_identifier, ws),           // Then variable identifier
                    ),
                    |(type_annotation, identifier)| (Some(type_annotation), identifier),
                ),
                // No type annotation case: from identifier in expression
                map(parse_identifier, |identifier| (None, identifier)),
            )), ws),
            delimited(ws, kw_in(), ws),
            delimited(ws, parse_expression, ws), // Collection expression
        ),
        |(_, (type_annotation, identifier), _, expression)| FromClause {
            type_annotation,
            identifier,
            expression,
        },
    )
        .parse(input)
}

/// Parse various query clauses (from, let, where, join, orderby)
fn parse_query_clause(input: Span) -> BResult<QueryClause> {
    if let Ok(r) = map(parse_additional_from_clause, QueryClause::From).parse(input) { return Ok(r); }
    if let Ok(r) = map(parse_let_clause, QueryClause::Let).parse(input) { return Ok(r); }
    if let Ok(r) = map(parse_where_clause, QueryClause::Where).parse(input) { return Ok(r); }
    if let Ok(r) = map(parse_join_clause, QueryClause::Join).parse(input) { return Ok(r); }
    map(parse_orderby_clause, QueryClause::OrderBy).parse(input)
}

/// Parse additional 'from' clauses in the query body
fn parse_additional_from_clause(input: Span) -> BResult<FromClause> {
    parse_from_clause(input)
}

/// Parse 'let' clause for introducing new variables
fn parse_let_clause(input: Span) -> BResult<LetClause> {
    map(
        (
            kw_let(),
            delimited(ws, parse_identifier, ws),
            delimited(ws, tok_assign(), ws),
            delimited(ws, parse_expression, ws),
        ),
        |(_, identifier, _, expression)| LetClause {
            identifier,
            expression,
        },
    )
        .parse(input)
}

/// Parse 'where' clause for filtering
fn parse_where_clause(input: Span) -> BResult<QueryWhereClause> {
    map(preceded(kw_where(), delimited(ws, parse_expression, ws)), |condition| {
        QueryWhereClause { condition }
    })
        .parse(input)
}

/// Parse 'join' clause for joining data sources
fn parse_join_clause(input: Span) -> BResult<JoinClause> {
    map(
        (
            kw_join(),
            delimited(ws, alt((
                // Type annotation case: join PrimitiveType identifier in expression
                map(
                    (
                        parse_primitive_type_identifier, // Parse known type keywords
                        delimited(ws, parse_identifier, ws),           // Then variable identifier
                    ),
                    |(type_annotation, identifier)| (Some(type_annotation), identifier),
                ),
                // No type annotation case: join identifier in expression
                map(parse_identifier, |identifier| (None, identifier)),
            )), ws),
            delimited(ws, kw_in(), ws),
            delimited(ws, parse_expression, ws), // Join collection
            delimited(ws, kw_on(), ws),
            delimited(ws, parse_expression, ws), // Join condition left side
            delimited(ws, kw_equals(), ws),
            delimited(ws, parse_expression, ws), // Join condition right side
            opt(preceded(delimited(ws, kw_into(), ws), delimited(ws, parse_identifier, ws))), // Optional 'into' clause
        ),
        |(
             _,
             (type_annotation, identifier),
             _,
             in_expression,
             _,
             on_expression,
             _,
             equals_expression,
             into_identifier,
         )| {
            JoinClause {
                type_annotation,
                identifier,
                in_expression,
                on_expression,
                equals_expression,
                into_identifier,
            }
        },
    )
        .parse(input)
}

/// Parse 'orderby' clause for sorting
fn parse_orderby_clause(input: Span) -> BResult<QueryOrderByClause> {
    map(
        preceded(kw_orderby(), separated_list1(delimited(ws, tok_comma(), ws), parse_ordering)),
        |orderings| QueryOrderByClause { orderings },
    )
        .parse(input)
}

/// Parse a single ordering expression
fn parse_ordering(input: Span) -> BResult<OrderByOrdering> {
    map(
        (
            delimited(ws, parse_expression, ws),
            opt(delimited(ws, alt((
                map(kw_ascending(), |_| OrderingDirection::Ascending),
                map(kw_descending(), |_| OrderingDirection::Descending),
            )), ws)),
        ),
        |(expression, direction)| OrderByOrdering {
            expression,
            direction,
            identifier: Identifier::new(""),
        },
    )
        .parse(input)
}

/// Parse 'select' or 'group' clause
fn parse_select_or_group_clause(input: Span) -> BResult<QuerySelectOrGroup> {
    if let Ok(r) = parse_select_clause(input) { return Ok(r); }
    parse_group_clause(input)
}

/// Parse 'select' clause
fn parse_select_clause(input: Span) -> BResult<QuerySelectOrGroup> {
    map(
        preceded(kw_select(), delimited(ws, parse_expression, ws)),
        QuerySelectOrGroup::Select,
    )
        .parse(input)
}

/// Parse 'group' clause
fn parse_group_clause(input: Span) -> BResult<QuerySelectOrGroup> {
    map(
        (
            kw_group(),
            delimited(ws, parse_expression, ws),
            delimited(ws, kw_by(), ws),
            delimited(ws, parse_expression, ws),
        ),
        |(_, element, _, by)| QuerySelectOrGroup::Group { element, by },
    )
        .parse(input)
}

/// Parse query continuation ('into' clause)
fn parse_query_continuation(input: Span) -> BResult<QueryContinuation> {
    map(
        (
            kw_into(),
            delimited(ws, parse_identifier, ws),
            many0(parse_query_clause),
            parse_select_or_group_clause,
        ),
        |(_, identifier, body, select_or_group)| QueryContinuation {
            identifier,
            body,
            select_or_group,
        },
    )
        .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::assignment::tok_assign;
use crate::tokens::separators::tok_comma;
