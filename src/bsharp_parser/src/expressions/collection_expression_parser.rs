use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;

use nom::{
    combinator::map,
    sequence::preceded,
};
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::Parser;
use syntax::expressions::expression::CollectionElement;
use syntax::expressions::Expression;
use crate::syntax::list_parser::parse_delimited_list0;

/// Parse a collection expression: [elem1, ..spread, elem2]
/// Elements can be regular expressions or spread elements starting with `..` followed by an expression
pub fn parse_collection_expression(input: Span) -> BResult<Expression> {
    parse_collection_expression_or_brackets(input)
}

/// Actual entry point with proper bracket handling
pub fn parse_collection_expression_or_brackets(input: Span) -> BResult<Expression> {
    fn parse_elements(i: Span) -> BResult<Vec<CollectionElement>> {
        parse_delimited_list0::<_, _, _, _, char, CollectionElement, char, char, CollectionElement>(
            |i| delimited(ws, nom_char('['), ws).parse(i),
            |i| delimited(ws, parse_collection_element, ws).parse(i),
            |i| delimited(ws, nom_char(','), ws).parse(i),
            |i| delimited(ws, nom_char(']'), ws).parse(i),
            false,
            true,
        )
        .parse(i)
    }
    map(parse_elements, Expression::Collection).parse(input)
}

fn parse_collection_element(input: Span) -> BResult<CollectionElement> {
    // Try spread element: `.. expr`
    if let Ok((rest, _)) = delimited(ws, preceded(nom_char('.'), nom_char('.')), ws).parse(input) {
        let (rest, expr) = delimited(ws, parse_expression, ws).parse(rest)?;
        return Ok((rest, CollectionElement::Spread(expr)));
    }
    // Otherwise a normal expression
    map(delimited(ws, parse_expression, ws), CollectionElement::Expr).parse(input)
}
use crate::syntax::span::Span;
