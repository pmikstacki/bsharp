use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::FieldDeclaration;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{context, bws, bchar};
use crate::parser::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use nom::{
    combinator::opt,
    sequence::preceded,
};

// Removed local ws helper, using bws from parser_helpers instead

// Parse the optional initializer part: "= Expression"
fn parse_field_initializer(input: &str) -> BResult<&str, Option<Expression>> {
    opt(
        preceded(
            context("field initializer (expected '=' followed by expression)", bchar('=')),
            context("field initializer expression (expected valid C# expression)", bws(parse_expression))
        )
    )(input)
}

// Parse a field declaration
// Format: [Modifiers] TypeSyntax Identifier [= Initializer];
pub fn parse_field_declaration(input: &str) -> BResult<&str, FieldDeclaration> {
    // Parse modifiers (private, readonly, etc.)
    let (input, modifiers) = parse_modifiers(input)?;
    let (input, ty) = context("field type (expected valid type expression)", bws(parse_type_expression))(input)?;
    let (input, name) = context("field name (expected valid identifier)", bws(parse_identifier))(input)?;
    let (input, initializer) = parse_field_initializer(input)?;
    let (input, _) = context("field declaration terminator (expected ';')", bws(bchar(';')))(input)?; // Field declarations must end with a semicolon

    Ok((input, FieldDeclaration {
        modifiers,
        ty,
        name,
        initializer,
    }))
}
