use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::tuple_expression::{TupleExpression, TupleElement};
use crate::parser::parser_helpers::{bchar, bs_context, bws};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;

use nom::{
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, tuple},
};

/// Parse a tuple expression: (expr1, expr2, ...) or (name1: expr1, name2: expr2, ...)
pub fn parse_tuple_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "tuple expression",
        map(
            delimited(
                bws(bchar('(')),
                // Must have at least two elements to be a tuple
                // Single element in parentheses is just a parenthesized expression
                tuple((
                    parse_tuple_element,
                    bws(bchar(',')),
                    separated_list1(bws(bchar(',')), bws(parse_tuple_element)),
                    opt(bws(bchar(','))), // Optional trailing comma
                )),
                bws(bchar(')')),
            ),
            |(first, _, mut rest, _)| {
                rest.insert(0, first);
                Expression::Tuple(TupleExpression { elements: rest })
            },
        ),
    )(input)
}

/// Parse a tuple element: [name:] expression
fn parse_tuple_element(input: &str) -> BResult<&str, TupleElement> {
    // Try to parse named tuple element first: name: expression
    if let Ok((input, (name, _, value))) = tuple((
        bws(parse_identifier),
        bws(bchar(':')),
        bws(parse_expression),
    ))(input) {
        return Ok((input, TupleElement {
            name: Some(name),
            value,
        }));
    }

    // Otherwise, parse as unnamed tuple element: expression
    map(
        bws(parse_expression),
        |value| TupleElement {
            name: None,
            value,
        },
    )(input)
} 