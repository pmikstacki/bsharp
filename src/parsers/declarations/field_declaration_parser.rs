use crate::parsers::identifier_parser::parse_identifier;
use crate::parser::nodes::expressions::expression::Expression;
use nom::{
    character::complete::{char as nom_char},
    sequence::{preceded},
    combinator::{map, opt},
};
use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::FieldDeclaration;
use crate::parsers::types::type_parser::parse_type_expression;
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parser::parser_helpers::{bws, nom_to_bs};

// Removed local ws helper, using bws from parser_helpers instead

// Parse the optional initializer part: "= Expression"
fn parse_field_initializer(input: &str) -> BResult<&str, Option<Expression>> {
    opt(
        preceded(
            bws(nom_to_bs(map(nom_char::<&str, nom::error::Error<&str>>('='), |c| c))),
            bws(parse_expression)
        )
    )(input)
}

// Parse a field declaration
// Format: TypeSyntax Identifier [= Initializer];
// Modifiers are not handled yet.
pub fn parse_field_declaration(input: &str) -> BResult<&str, FieldDeclaration> {
    // TODO: Parse modifiers here later
    let (input, ty) = bws(nom_to_bs(parse_type_expression))(input)?;
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    let (input, initializer) = parse_field_initializer(input)?;
    let (input, _) = bws(nom_to_bs(map(nom_char::<&str, nom::error::Error<&str>>(';'), |c| c)))(input)?; // Field declarations must end with a semicolon

    Ok((input, FieldDeclaration {
        ty,
        name,
        initializer,
    }))
}
