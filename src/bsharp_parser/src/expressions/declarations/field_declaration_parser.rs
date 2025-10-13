use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;

use crate::syntax::comment_parser::ws;
use nom::sequence::delimited;
use nom::character::complete::satisfy;
use nom::Parser;
use nom_supreme::ParserExt;
use nom::{combinator::opt, sequence::preceded};
use syntax::declarations::FieldDeclaration;
use syntax::expressions::Expression;
// Removed local ws helper, using bws from parser_helpers instead

// Parse the optional initializer part: "= Expression"
fn parse_field_initializer(input: Span) -> BResult<Option<Expression>> {
    opt(preceded(
        delimited(ws, satisfy(|c| c == '='), ws)
            .context("field initializer"),
        delimited(ws, parse_expression, ws)
            .context("field initializer expression"),
    ))
    .parse(input)
}

// Parse a field declaration
// Format: [Modifiers] TypeSyntax Identifier [= Initializer];
pub fn parse_field_declaration(input: Span) -> BResult<FieldDeclaration> {
    // Parse modifiers (private, readonly, etc.)
    let (input, modifiers) = parse_modifiers(input)?;
    let (input, ty) = delimited(ws, parse_type_expression, ws)
        .context("field type")
        .parse(input)?;
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("field name")
        .parse(input)?;
    let (input, initializer) = parse_field_initializer(input)?;
    let (input, _) = delimited(ws, satisfy(|c| c == ';'), ws)
        .context("field declaration terminator")
        .parse(input)?;

    Ok((
        input,
        FieldDeclaration {
            modifiers,
            field_type: ty,
            name,
            initializer,
        },
    ))
}
use crate::syntax::span::Span;
