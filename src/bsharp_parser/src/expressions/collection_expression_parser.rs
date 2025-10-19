use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use crate::syntax::list_parser::parse_delimited_list0;
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::Parser;
use nom::{
    combinator::map,
    sequence::preceded,
};
use syntax::expressions::expression::CollectionElement;
use syntax::expressions::Expression;

/// Parse a collection expression: [elem1, ..spread, elem2]
/// Elements can be regular expressions or spread elements starting with `..` followed by an expression
pub fn parse_collection_expression(input: Span) -> BResult<Expression> {
    parse_collection_expression_or_brackets(input.into())
}

/// Actual entry point with proper bracket handling
pub fn parse_collection_expression_or_brackets(input: Span) -> BResult<Expression> {
    fn parse_elements(i: Span) -> BResult<Vec<CollectionElement>> {
        parse_delimited_list0::<_, _, _, _, char, char, char, CollectionElement>(
            |i| delimited(ws,tok_l_brack(), ws).parse(i),
            |i| delimited(ws, parse_collection_element, ws).parse(i),
            |i| delimited(ws, tok_comma(), ws).parse(i),
            |i| delimited(ws, tok_r_brack(), ws).parse(i),
            false,
            true,
        )
            .parse(i)
    }
    map(parse_elements, Expression::Collection).parse(input.into())
}

fn parse_collection_element(input: Span) -> BResult<CollectionElement> {
    // Try spread element: `.. expr`
    if let Ok((rest, _)) = delimited(ws, preceded(nom_char('.'), nom_char('.')), ws).parse(input.into()) {
        let (rest, expr) = delimited(ws, parse_expression, ws).parse(rest)?;
        return Ok((rest, CollectionElement::Spread(expr)));
    }
    // Otherwise a normal expression
    map(delimited(ws, parse_expression, ws), CollectionElement::Expr).parse(input.into())
}
use crate::syntax::span::Span;
use crate::tokens::delimiters::{tok_l_brack, tok_r_brack};
use crate::tokens::separators::tok_comma;
