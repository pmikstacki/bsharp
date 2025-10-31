use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;

use crate::syntax::list_parser::parse_delimited_list0;
use nom::Parser;
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::{combinator::map, sequence::preceded};
use syntax::expressions::Expression;
use syntax::expressions::expression::CollectionElement;

/// Parse a collection expression: [elem1, ..spread, elem2]
/// Elements can be regular expressions or spread elements starting with `..` followed by an expression
pub fn parse_collection_expression(input: Span) -> BResult<Expression> {
    parse_collection_expression_or_brackets(input)
}

/// Actual entry point with proper bracket handling
pub fn parse_collection_expression_or_brackets(input: Span) -> BResult<Expression> {
    fn parse_elements(i: Span) -> BResult<Vec<CollectionElement>> {
        parse_delimited_list0::<_, _, _, _, char, char, char, CollectionElement>(
            |i| delimited(ws, tok_l_brack(), ws).parse(i),
            |i| delimited(ws, parse_collection_element, ws).parse(i),
            |i| delimited(ws, tok_comma(), ws).parse(i),
            |i| delimited(ws, tok_r_brack(), ws).parse(i),
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
        let (rest, expr) = delimited(ws, parse_expression_spanned, ws).map(|s| s.node).parse(rest)?;
        return Ok((rest, CollectionElement::Spread(expr)));
    }
    // Otherwise a normal expression
    map(
        delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
        CollectionElement::Expr,
    )
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_brack, tok_r_brack};
use crate::tokens::separators::tok_comma;
