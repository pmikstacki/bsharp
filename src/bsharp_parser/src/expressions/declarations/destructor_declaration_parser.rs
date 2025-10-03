use nom::combinator::cut;
use nom::{branch::alt, combinator::map};

use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::DestructorDeclaration;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws, context};

/// Parse a C# destructor declaration
///
/// Examples:
/// ```csharp
/// ~MyClass() { ... }
/// ~MyClass() { /* cleanup code */ }
/// ```
pub fn parse_destructor_declaration(input: &str) -> BResult<&str, DestructorDeclaration> {
    context(
        "destructor declaration (expected optional attributes, modifiers, '~', class name, empty parameter list, and body)",
        |input| {
            // Parse attributes
            let (input, attribute_lists) = parse_attribute_lists(input)?;
            let attributes = convert_attributes(attribute_lists);

            // Parse modifiers (destructors typically don't have explicit modifiers)
            let (input, modifiers) = parse_modifiers(input)?;

            // Parse the tilde (~) symbol
            let (input, _) = context(
                "destructor tilde (expected '~' to start destructor)",
                bws(bchar('~')),
            )(input)?;

            // Parse the class name (destructor name must match class name)
            let (input, name) = context(
                "destructor name (expected class name matching the containing class)",
                bws(parse_identifier),
            )(input)?;

            // Parse the parameter list (must be empty for destructors)
            let (input, _) = context(
                "destructor opening parenthesis (expected '(' for empty parameter list)",
                bws(bchar('(')),
            )(input)?;
            let (input, _) = context(
                "destructor closing parenthesis (expected ')' - destructors cannot have parameters)",
                cut(bws(bchar(')'))),
            )(input)?;

            // Parse the body (either block statement or semicolon)
            let (input, body) = parse_destructor_body(input)?;

            let destructor_declaration = DestructorDeclaration {
                attributes,
                modifiers,
                name,
                body,
            };

            Ok((input, destructor_declaration))
        },
    )(input)
}

/// Parse the destructor body (either a block statement or semicolon)
fn parse_destructor_body(input: &str) -> BResult<&str, Option<Statement>> {
    context(
        "destructor body (expected block statement or semicolon for extern destructor)",
        alt((
            // Block body
            map(parse_block_statement, |stmt| Some(stmt)),
            // Semicolon (extern)
            map(bws(bchar(';')), |_| None),
        )),
    )(input)
}
