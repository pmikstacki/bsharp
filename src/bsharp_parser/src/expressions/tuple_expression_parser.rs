use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom_supreme::ParserExt;

use nom::combinator::cut;
use nom::{
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, tuple},
    character::complete::char as nom_char,
    Parser,
};
use syntax::expressions::{Expression, TupleElement, TupleExpression};

/// Parse a tuple expression: (expr1, expr2, ...) or (name1: expr1, name2: expr2, ...)
pub fn parse_tuple_expression<'a>(input: Span<'a>) -> BResult<'a, Expression> {
    map(
        delimited(
            delimited(ws, nom_char('('), ws),
            // Must have at least two elements to be a tuple
            // Single element in parentheses is just a parenthesized expression
            tuple((
                parse_tuple_element,
                delimited(ws, nom_char(','), ws),
                separated_list1(
                    delimited(ws, nom_char(','), ws),
                    delimited(ws, parse_tuple_element, ws),
                ),
                opt(delimited(ws, nom_char(','), ws)), // Optional trailing comma
            )),
            cut(delimited(ws, nom_char(')'), ws)),
        ),
        |(first, _, mut rest, _)| {
            rest.insert(0, first);
            Expression::Tuple(TupleExpression { elements: rest })
        },
    )
    .context("tuple expression")
    .parse(input)
}

/// Parse a tuple element: [name:] expression
fn parse_tuple_element(input: Span) -> BResult<TupleElement> {
    // Try to parse named tuple element first: name: expression
    if let Ok((input, (name, _, value))) = tuple((
        delimited(ws, parse_identifier, ws),
        delimited(ws, nom_char(':'), ws),
        delimited(ws, parse_expression, ws),
    ))
    .parse(input)
    {
        return Ok((
            input,
            TupleElement {
                name: Some(name),
                value,
            },
        ));
    }

    // Otherwise, parse as unnamed tuple element: expression
    map(delimited(ws, parse_expression, ws), |value| TupleElement {
        name: None,
        value,
    })
    .parse(input)
}
use crate::syntax::span::Span;
