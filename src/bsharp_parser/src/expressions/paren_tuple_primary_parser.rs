use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;

use nom::{combinator::map, sequence::tuple};
use syntax::expressions::{Expression, TupleElement, TupleExpression};
use crate::syntax::list_parser::{parse_delimited_list_or_singleton, OneOrMany};

/// Unified syntax for either a parenthesized expression or a tuple expression
/// Decides based on whether a comma follows the first element (using bpeek)
pub fn parse_paren_or_tuple_primary(input: Span) -> BResult<Expression> {
    (|input| {
        // Use generic helper to disambiguate list vs singleton
        let mut parser = parse_delimited_list_or_singleton(
            |i| nom::sequence::delimited(ws, nom_char('('), ws).parse(i),
            parse_tuple_element_local, // first element (can be named)
            |i| nom::sequence::delimited(ws, nom_char(','), ws).parse(i), // separator
            parse_tuple_element_local, // subsequent elements
            |i| nom::sequence::delimited(ws, nom_char(')'), ws).parse(i), // close delimiter
            true,                      // allow trailing separator
            true,                      // cut on close when it's a list
        );

        let (rest, result) = parser(input)?;
        match result {
            OneOrMany::Many(mut elems) => Ok((
                rest,
                Expression::Tuple(TupleExpression {
                    elements: std::mem::take(&mut elems),
                }),
            )),
            OneOrMany::Single(first) => {
                // Named single element like (a: 1) is invalid in C# tuple syntax
                if first.name.is_some() {
                    use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
                    let error_tree = ErrorTree::Base {
                        location: rest,
                        kind: BaseErrorKind::Expected(Expectation::Tag(
                            "single-element tuple cannot be named; expected ',' after first element",
                        )),
                    };
                    Err(nom::Err::Error(error_tree))
                } else {
                    Ok((rest, first.value))
                }
            }
        }
    })
    .context("parenthesized or tuple expression")
    .parse(input)
}

/// Local tuple element syntax (named or unnamed): [name:] expression
fn parse_tuple_element_local(input: Span) -> BResult<TupleElement> {
    // Try named element first (backtracks cleanly if ':' not present)
    if let Ok((input, (name, _, value))) = tuple((
        nom::sequence::delimited(ws, parse_identifier, ws),
        nom::sequence::delimited(ws, nom_char(':'), ws),
        nom::sequence::delimited(ws, parse_expression, ws),
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

    // Otherwise, parse as unnamed element
    map(nom::sequence::delimited(ws, parse_expression, ws), |value| TupleElement {
        name: None,
        value,
    })
    .parse(input)
}
use crate::syntax::span::Span;
