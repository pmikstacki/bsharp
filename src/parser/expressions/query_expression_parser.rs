use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::query_expression::*;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bws, keyword};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{many0, separated_list1},
    sequence::{preceded, tuple},
};

/// Parse primitive type identifiers for LINQ type annotations
fn parse_primitive_type_identifier(input: &str) -> BResult<&str, Identifier> {
    map(
        alt((
            // Order matters! Put longer keywords first to avoid partial matches
            keyword("ushort"),
            keyword("uint"),
            keyword("ulong"),
            keyword("sbyte"),
            keyword("short"),
            keyword("byte"),
            keyword("bool"),
            keyword("int"),
            keyword("long"),
            keyword("double"),
            keyword("decimal"),
            keyword("float"),
            keyword("string"),
            keyword("object"),
            keyword("char"),
            keyword("dynamic"),
        )),
        |name| Identifier::new(name),
    )(input)
}

/// Parse a complete LINQ query expression
pub fn parse_query_expression(input: &str) -> BResult<&str, Expression> {
    map(
        tuple((
            parse_from_clause,
            many0(parse_query_clause),
            parse_select_or_group_clause,
            opt(parse_query_continuation),
        )),
        |(from, body, select_or_group, continuation)| {
            Expression::Query(Box::new(QueryExpression {
                from,
                body,
                select_or_group,
                continuation,
            }))
        },
    )(input)
}

/// Parse the initial 'from' clause
fn parse_from_clause(input: &str) -> BResult<&str, FromClause> {
    map(
        tuple((
            keyword("from"),
            bws(alt((
                // Type annotation case: from PrimitiveType identifier in expression
                map(
                    tuple((
                        parse_primitive_type_identifier, // Parse known type keywords
                        bws(parse_identifier),           // Then variable identifier
                    )),
                    |(type_annotation, identifier)| (Some(type_annotation), identifier),
                ),
                // No type annotation case: from identifier in expression
                map(parse_identifier, |identifier| (None, identifier)),
            ))),
            bws(keyword("in")),
            bws(parse_expression), // Collection expression
        )),
        |(_, (type_annotation, identifier), _, expression)| FromClause {
            type_annotation,
            identifier,
            expression,
        },
    )(input)
}

/// Parse various query clauses (from, let, where, join, orderby)
fn parse_query_clause(input: &str) -> BResult<&str, QueryClause> {
    alt((
        map(parse_additional_from_clause, QueryClause::From),
        map(parse_let_clause, QueryClause::Let),
        map(parse_where_clause, QueryClause::Where),
        map(parse_join_clause, QueryClause::Join),
        map(parse_orderby_clause, QueryClause::OrderBy),
    ))(input)
}

/// Parse additional 'from' clauses in the query body
fn parse_additional_from_clause(input: &str) -> BResult<&str, FromClause> {
    parse_from_clause(input)
}

/// Parse 'let' clause for introducing new variables
fn parse_let_clause(input: &str) -> BResult<&str, LetClause> {
    map(
        tuple((
            keyword("let"),
            bws(parse_identifier),
            bws(keyword("=")),
            bws(parse_expression),
        )),
        |(_, identifier, _, expression)| LetClause {
            identifier,
            expression,
        },
    )(input)
}

/// Parse 'where' clause for filtering
fn parse_where_clause(input: &str) -> BResult<&str, QueryWhereClause> {
    map(
        preceded(keyword("where"), bws(parse_expression)),
        |condition| QueryWhereClause { condition },
    )(input)
}

/// Parse 'join' clause for joining data sources
fn parse_join_clause(input: &str) -> BResult<&str, JoinClause> {
    map(
        tuple((
            keyword("join"),
            bws(alt((
                // Type annotation case: join PrimitiveType identifier in expression
                map(
                    tuple((
                        parse_primitive_type_identifier, // Parse known type keywords
                        bws(parse_identifier),           // Then variable identifier
                    )),
                    |(type_annotation, identifier)| (Some(type_annotation), identifier),
                ),
                // No type annotation case: join identifier in expression
                map(parse_identifier, |identifier| (None, identifier)),
            ))),
            bws(keyword("in")),
            bws(parse_expression), // Join collection
            bws(keyword("on")),
            bws(parse_expression), // Join condition left side
            bws(keyword("equals")),
            bws(parse_expression), // Join condition right side
            opt(preceded(bws(keyword("into")), bws(parse_identifier))), // Optional 'into' clause
        )),
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
    )(input)
}

/// Parse 'orderby' clause for sorting
fn parse_orderby_clause(input: &str) -> BResult<&str, QueryOrderByClause> {
    map(
        preceded(
            keyword("orderby"),
            separated_list1(bws(keyword(",")), parse_ordering),
        ),
        |orderings| QueryOrderByClause { orderings },
    )(input)
}

/// Parse a single ordering expression
fn parse_ordering(input: &str) -> BResult<&str, OrderByOrdering> {
    map(
        tuple((
            bws(parse_expression),
            opt(bws(alt((
                map(keyword("ascending"), |_| OrderingDirection::Ascending),
                map(keyword("descending"), |_| OrderingDirection::Descending),
            )))),
        )),
        |(expression, direction)| OrderByOrdering {
            expression,
            direction,
            identifier: Identifier::new(""),
        },
    )(input)
}

/// Parse 'select' or 'group' clause
fn parse_select_or_group_clause(input: &str) -> BResult<&str, QuerySelectOrGroup> {
    alt((parse_select_clause, parse_group_clause))(input)
}

/// Parse 'select' clause
fn parse_select_clause(input: &str) -> BResult<&str, QuerySelectOrGroup> {
    map(
        preceded(keyword("select"), bws(parse_expression)),
        |expression| QuerySelectOrGroup::Select(expression),
    )(input)
}

/// Parse 'group' clause
fn parse_group_clause(input: &str) -> BResult<&str, QuerySelectOrGroup> {
    map(
        tuple((
            keyword("group"),
            bws(parse_expression),
            bws(keyword("by")),
            bws(parse_expression),
        )),
        |(_, element, _, by)| QuerySelectOrGroup::Group { element, by },
    )(input)
}

/// Parse query continuation ('into' clause)
fn parse_query_continuation(input: &str) -> BResult<&str, QueryContinuation> {
    map(
        tuple((
            keyword("into"),
            bws(parse_identifier),
            many0(parse_query_clause),
            parse_select_or_group_clause,
        )),
        |(_, identifier, body, select_or_group)| QueryContinuation {
            identifier,
            body,
            select_or_group,
        },
    )(input)
}
