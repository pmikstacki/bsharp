use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::parser::expressions::literal_parser::parse_literal;
use crate::parser::identifier_parser::parse_identifier;
use nom::{
    branch::alt,
    character::complete::{char as nom_char},
    combinator::map,
    sequence::delimited,
};
use nom::combinator::cut;
use crate::syntax::parser_helpers::bws;

// Use global comment-aware ws/bws from syntax helpers

// Parse a variable reference (which is just an identifier used as an expression)
fn parse_variable_reference(input: &str) -> BResult<&str, Expression> {
    map(parse_identifier, Expression::Variable)(input)
}

// Parse a parenthesized expression: (expr)
fn parse_parenthesized_expression(input: &str) -> BResult<&str, Expression> {
    delimited(
        bws(nom_char('(')),
        parse_expression,
        cut(bws(nom_char(')')))
    )(input)
}

// Parse a literal expression
fn parse_literal_expression(input: &str) -> BResult<&str, Expression> {
    map(parse_literal, Expression::Literal)(input)
}

// Parse a primary expression (variable, literal, parenthesized)
pub fn parse_primary_expression(input: &str) -> BResult<&str, Expression> {
    alt((
        parse_literal_expression,
        parse_variable_reference,
        parse_parenthesized_expression,
    ))(input)
}

// This is a forward declaration to resolve the circular dependency
// between parse_expression and parse_parenthesized_expression
fn parse_expression(input: &str) -> BResult<&str, Expression> {
    // For now, we'll just parse primary expressions
    // In a complete implementation, this would handle more complex expressions
    parse_primary_expression(input)
}
