use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use nom::combinator::cut;
use nom::{branch::alt, combinator::map};

use crate::syntax::comment_parser::ws;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;

pub use syntax::declarations::*;
pub use syntax::expressions::expression::*;
pub use syntax::statements::statement::*;
pub use syntax::trivia::*;

/// Parse a C# destructor declaration
///
/// Examples:
/// ```csharp
/// ~MyClass() { ... }
/// ~MyClass() { /* cleanup code */ }
/// ```
pub fn parse_destructor_declaration(input: Span) -> BResult<DestructorDeclaration> {
    // Parse attributes
    let (input, attribute_lists) = parse_attribute_lists(input.into())?;
    let attributes = convert_attributes(attribute_lists);

    // Parse modifiers (destructors typically don't have explicit modifiers)
    let (input, modifiers) = parse_modifiers(input.into())?;

    // Parse the tilde (~) symbol
    let (input, _) = delimited(ws, tok_tilde(), ws)
        .context("destructor tilde")
        .parse(input.into())?;

    // Parse the class name (destructor name must match class name)
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("destructor name")
        .parse(input.into())?;

    // Parse the parameter list (must be empty for destructors)
    let (input, _) = delimited(ws, tok_l_paren(), ws)
        .context("destructor opening parenthesis")
        .parse(input.into())?;
    let (input, _) = cut(delimited(ws, tok_r_paren(), ws))
        .context("destructor closing parenthesis")
        .parse(input.into())?;

    // Parse the body (either block statement or semicolon)
    let (input, body) = parse_destructor_body(input.into())?;

    let destructor_declaration = DestructorDeclaration {
        attributes,
        modifiers,
        name,
        body,
    };

    Ok((input, destructor_declaration))
}

/// Parse the destructor body (either a block statement or semicolon)
fn parse_destructor_body(input: Span) -> BResult<Option<Statement>> {
    alt((
        // Block body
        map(delimited(ws, parse_block_statement, ws), |stmt| Some(stmt)),
        // Semicolon (extern)
        map(delimited(ws, tok_semicolon(), ws), |_| None),
    ))
        .context("destructor body")
        .parse(input.into())
}
use crate::syntax::span::Span;
use crate::tokens::bitwise::tok_tilde;
use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use crate::tokens::separators::tok_semicolon;
