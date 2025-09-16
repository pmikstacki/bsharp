use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::tuple_expression::{TupleElement, TupleExpression};
use crate::syntax::parser_helpers::{bchar, bws, context};
use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;

use nom::{
    combinator::{cut, map, opt},
    multi::separated_list1,
    sequence::tuple,
};

/// Unified syntax for either a parenthesized expression or a tuple expression
/// Decides based on whether a comma follows the first element
pub fn parse_paren_or_tuple_primary(input: &str) -> BResult<&str, Expression> {
    context(
        "parenthesized or tuple expression",
        |input| {
            let (input, _) = bws(bchar('('))(input)?;

            // Parse the first element as a potential named tuple element or a plain expression
            let (input, first) = parse_tuple_element_local(input)?;

            // If the next token is a comma, this is a tuple
            if let Ok((input_after_comma, _)) = bws(bchar(','))(input) {
                let (input_after_rest, mut rest) = separated_list1(bws(bchar(',')), bws(parse_tuple_element_local))(input_after_comma)?;
                let (input_after_trailing, _) = opt(bws(bchar(',')))(input_after_rest)?; // optional trailing comma
                let (input_after_close, _) = cut(bws(bchar(')')))(input_after_trailing)?;

                // Build elements list with the first element at the front
                rest.insert(0, first);
                return Ok((input_after_close, Expression::Tuple(TupleExpression { elements: rest })));
            }

            // Otherwise, must be a plain parenthesized expression; consume ')'
            // Note: if the first element was named, we ignore the name here and treat it as a normal expression
            let (input, _) = bws(bchar(')'))(input)?;
            Ok((input, first.value))
        }
    )(input)
}

/// Local tuple element syntax (named or unnamed): [name:] expression
fn parse_tuple_element_local(input: &str) -> BResult<&str, TupleElement> {
    // Try named element first (backtracks cleanly if ':' not present)
    if let Ok((input, (name, _, value))) = tuple((
        bws(parse_identifier),
        bws(bchar(':')),
        bws(parse_expression),
    ))(input) {
        return Ok((input, TupleElement { name: Some(name), value }));
    }

    // Otherwise, parse as unnamed element
    map(
        bws(parse_expression),
        |value| TupleElement { name: None, value },
    )(input)
}
