use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::FieldDeclaration;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;
use nom::{
    character::complete::char as nom_char,
    combinator::{map, opt},
    sequence::preceded,
};

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
// Format: [Modifiers] TypeSyntax Identifier [= Initializer];
pub fn parse_field_declaration(input: &str) -> BResult<&str, FieldDeclaration> {
    // Parse modifiers (private, readonly, etc.)
    let (input, modifiers) = parse_modifiers(input)?;
    let (input, ty) = bws(nom_to_bs(parse_type_expression))(input)?;
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    let (input, initializer) = parse_field_initializer(input)?;
    let (input, _) = bws(nom_to_bs(map(nom_char::<&str, nom::error::Error<&str>>(';'), |c| c)))(input)?; // Field declarations must end with a semicolon

    Ok((input, FieldDeclaration {
        modifiers,
        ty,
        name,
        initializer,
    }))
}
