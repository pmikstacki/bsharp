use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::parameter_parser::parse_parameter_list;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::keywords::declaration_keywords::{kw_explicit, kw_implicit, kw_operator};
use crate::parser::keywords::literal_keywords::{kw_false, kw_true};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use nom::{branch::alt, combinator::map};
use syntax::declarations::{
    Attribute, ConversionKind, Modifier, OperatorDeclaration, OperatorKind,
};
use syntax::Identifier;

use crate::syntax::comment_parser::ws;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;

/// Parse a C# operator declaration
///
/// Examples:
/// ```csharp
/// public static MyType operator +(MyType a, MyType b) { ... }
/// public static explicit operator int(MyType value) { ... }
/// public static implicit operator string(MyType value) { ... }
/// public static MyType operator -(MyType value) { ... }  // unary
/// ```
pub fn parse_operator_declaration(input: Span) -> BResult<OperatorDeclaration> {
    // Parse attributes
    let (input, attribute_lists) = parse_attribute_lists(input.into())?;
    let attributes = convert_attributes(attribute_lists);

    // Parse modifiers (typically public static)
    let (input, modifiers) = parse_modifiers(input.into())?;

    // Check if this is a conversion operator (implicit/explicit operator)
    // If so, we parse differently
    if let Ok((_, _)) = delimited(ws, kw_implicit(), ws).parse(input.into()) {
        return parse_conversion_operator(input, attributes, modifiers, ConversionKind::Implicit);
    } else if let Ok((_, _)) = delimited(ws, kw_explicit(), ws).parse(input.into()) {
        return parse_conversion_operator(input, attributes, modifiers, ConversionKind::Explicit);
    }

    // Regular operator with return type
    let (input, return_type) = delimited(ws, parse_type_expression, ws)
        .context("operator return type")
        .parse(input.into())?;

    // Parse the "operator" keyword
    let (input, _) = delimited(ws, kw_operator(), ws)
        .context("operator keyword")
        .parse(input.into())?;

    // Parse the operator symbol
    let (input, operator_symbol) = parse_operator_symbol(input.into())?;

    // Parse parameters
    let (input, parameters) = delimited(ws, parse_parameter_list, ws)
        .context("operator parameter list")
        .parse(input.into())?;

    // Parse body
    let (input, body) = parse_operator_body(input.into())?;

    let operator_declaration = OperatorDeclaration {
        attributes,
        modifiers,
        return_type,
        operator: OperatorKind::Binary(operator_symbol),
        parameters,
        body,
    };

    Ok((input, operator_declaration))
}

/// Parse a conversion operator (implicit/explicit)
fn parse_conversion_operator(
    input: Span,
    attributes: Vec<Attribute>,
    modifiers: Vec<Modifier>,
    kind: ConversionKind,
) -> BResult<OperatorDeclaration> {
    // Skip the implicit/explicit keyword
    let (input, _) = match kind {
        ConversionKind::Implicit => delimited(ws, kw_implicit(), ws)
            .context("implicit keyword")
            .parse(input.into())?,
        ConversionKind::Explicit => delimited(ws, kw_explicit(), ws)
            .context("explicit keyword")
            .parse(input.into())?,
    };

    // Parse the "operator" keyword
    let (input, _) = delimited(ws, kw_operator(), ws)
        .context("operator keyword")
        .parse(input.into())?;

    // Parse the target type
    let (input, target_type) = delimited(ws, parse_type_expression, ws)
        .context("conversion target type")
        .parse(input.into())?;

    // Parse parameters
    let (input, parameters) = delimited(ws, parse_parameter_list, ws)
        .context("conversion operator parameter list")
        .parse(input.into())?;

    // Parse body
    let (input, body) = parse_operator_body(input.into())?;

    let operator_declaration = OperatorDeclaration {
        attributes,
        modifiers,
        return_type: target_type.clone(),
        operator: OperatorKind::Conversion { kind, target_type },
        parameters,
        body,
    };

    Ok((input, operator_declaration))
}

/// Parse operator symbols (+, -, *, /, etc.)
fn parse_operator_symbol(input: Span) -> BResult<Identifier> {
    alt((
        // Multi-character operators first (to avoid prefix conflicts)
        map(delimited(ws, tok_increment(), ws), |_| Identifier::new("++")),
        map(delimited(ws, tok_decrement(), ws), |_| Identifier::new("--")),
        map(delimited(ws, tok_equal(), ws), |_| Identifier::new("==")),
        map(delimited(ws, tok_not_equal(), ws), |_| Identifier::new("!=")),
        map(delimited(ws, tok_ge(), ws), |_| Identifier::new(">=")),
        map(delimited(ws, tok_le(), ws), |_| Identifier::new("<=")),
        // Keywords (these should also come before single characters)
        map(delimited(ws, kw_true(), ws), |_| Identifier::new("true")),
        map(delimited(ws, kw_false(), ws), |_| Identifier::new("false")),
        // Single character operators
        map(delimited(ws, tok_plus(), ws), |_| Identifier::new("+")),
        map(delimited(ws, tok_minus(), ws), |_| Identifier::new("-")),
        map(delimited(ws, tok_multiply(), ws), |_| Identifier::new("*")),
        map(delimited(ws, tok_divide(), ws), |_| Identifier::new("/")),
        map(delimited(ws, tok_mod(), ws), |_| Identifier::new("%")),
        map(delimited(ws, tok_gt(), ws), |_| Identifier::new(">")),
        map(delimited(ws, tok_lt(), ws), |_| Identifier::new("<")),
        map(delimited(ws, tok_not(), ws), |_| Identifier::new("!")),
        map(delimited(ws, tok_tilde(), ws), |_| Identifier::new("~")),
    ))
        .context("operator symbol")
        .parse(input.into())
}

/// Parse the operator body (either a block statement or semicolon)
fn parse_operator_body(input: Span) -> BResult<String> {
    alt((
        // Block body
        map(
            delimited(ws, parse_block_statement, ws),
            |_| "{ /* body */ }".to_string(),
        ),
        // Semicolon (abstract/extern)
        map(delimited(ws, tok_semicolon(), ws), |_| "".to_string()),
    ))
        .context("operator body")
        .parse(input.into())
}
use crate::syntax::span::Span;
use crate::tokens::arithmetic::{tok_decrement, tok_divide, tok_increment, tok_minus, tok_mod, tok_multiply, tok_plus};
use crate::tokens::bitwise::tok_tilde;
use crate::tokens::equality::{tok_equal, tok_not_equal};
use crate::tokens::nullish::tok_not;
use crate::tokens::relational::{tok_ge, tok_gt, tok_le, tok_lt};
use crate::tokens::separators::tok_semicolon;
