use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::errors::BResult;

use crate::trivia::comment_parser::ws;
use nom::Parser;
use nom::sequence::delimited;
use nom::{combinator::opt, sequence::preceded};
use nom_supreme::ParserExt;
use syntax::declarations::FieldDeclaration;
use syntax::expressions::Expression;
// Removed local ws helper, using bws from parser_helpers instead

// Parse the optional initializer part: "= Expression"
fn parse_field_initializer(input: Span) -> BResult<Option<Expression>> {
    opt(preceded(
        delimited(ws, tok_assign(), ws).context("field initializer"),
        delimited(ws, parse_expression_spanned, ws)
            .map(|s| s.node)
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
    let (input, _) = delimited(ws, tok_semicolon(), ws)
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
use syntax::span::Span;

use crate::tokens::assignment::tok_assign;
use crate::tokens::separators::tok_semicolon;
