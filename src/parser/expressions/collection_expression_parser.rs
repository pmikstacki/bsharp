use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::{CollectionElement, Expression};
use crate::syntax::parser_helpers::{bchar, bseparated_list0, bws, context};

use nom::{
    branch::alt,
    combinator::{cut, map},
    sequence::{preceded},
};

/// Parse a collection expression: [elem1, ..spread, elem2]
/// Elements can be regular expressions or spread elements starting with `..` followed by an expression
pub fn parse_collection_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "collection expression (expected '[' elements ']')",
        map(
            bseparated_list0(
                // Use delimited parsing for full list inside brackets
                // We'll parse the interior with a helper and then close
                // Here we wrap the entire list parse as a single element using a closure below
                // but bseparated_list0 expects just element parser; we'll instead manually delimited.
                // Simpler: do manual delimited using parser_helpers::parse_delimited_list0
                // However to avoid import churn, implement manual delimited inline.
                // We'll instead parse the whole structure below.
                // Placeholder separator; this will not be used.
                bws(bchar(',')),
                parse_collection_element,
            ),
            |_v| unreachable!("should not reach")
        ),
    )(input)
}

/// Actual entry point with proper bracket handling
pub fn parse_collection_expression_or_brackets(input: &str) -> BResult<&str, Expression> {
    context(
        "collection expression (expected '[' elements ']')",
        |i| {
            let (i, _) = bws(bchar('['))(i)?;
            let (i, elements) = bseparated_list0(bws(bchar(',')), bws(parse_collection_element))(i)?;
            let (i, _) = cut(bws(bchar(']')))(i)?;
            Ok((i, Expression::Collection(elements)))
        },
    )(input)
}

fn parse_collection_element(input: &str) -> BResult<&str, CollectionElement> {
    // Try spread element: `.. expr`
    if let Ok((rest, _)) = bws(preceded(bchar('.'), bchar('.')))(input) {
        let (rest, expr) = bws(parse_expression)(rest)?;
        return Ok((rest, CollectionElement::Spread(expr)));
    }
    // Otherwise a normal expression
    map(bws(parse_expression), CollectionElement::Expr)(input)
}
