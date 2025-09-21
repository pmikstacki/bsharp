use nom::{branch::alt, combinator::map};

use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::parameter_parser::parse_parameter_list;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{ConversionKind, OperatorDeclaration, OperatorKind};
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};
use crate::parser::keywords::declaration_keywords::{kw_operator, kw_explicit, kw_implicit};
use crate::parser::keywords::literal_keywords::{kw_true, kw_false};

/// Parse a C# operator declaration
///
/// Examples:
/// ```csharp
/// public static MyType operator +(MyType a, MyType b) { ... }
/// public static explicit operator int(MyType value) { ... }
/// public static implicit operator string(MyType value) { ... }
/// public static MyType operator -(MyType value) { ... }  // unary
/// ```
pub fn parse_operator_declaration(input: &str) -> BResult<&str, OperatorDeclaration> {
    context(
        "operator declaration (expected optional attributes, modifiers, return type, 'operator' keyword, symbol, parameters, and body)",
        |input| {
            // Parse attributes
            let (input, attribute_lists) = parse_attribute_lists(input)?;
            let attributes = convert_attributes(attribute_lists);

            // Parse modifiers (typically public static)
            let (input, modifiers) = parse_modifiers(input)?;

            // Check if this is a conversion operator (implicit/explicit operator)
            // If so, we parse differently
            if let Ok((_, _)) = kw_implicit()(input) {
                parse_conversion_operator(input, attributes, modifiers, ConversionKind::Implicit)
            } else if let Ok((_, _)) = kw_explicit()(input) {
                parse_conversion_operator(input, attributes, modifiers, ConversionKind::Explicit)
            } else {
                // Regular operator with return type
                let (input, return_type) = context(
                    "operator return type (expected valid C# type)",
                    bws(parse_type_expression),
                )(input)?;

                // Parse the "operator" keyword
                let (input, _) = context(
                    "operator keyword (expected 'operator')",
                    bws(kw_operator()),
                )(input)?;

                // Parse the operator symbol
                let (input, operator_symbol) = parse_operator_symbol(input)?;

                // Parse parameters
                let (input, parameters) = context(
                    "operator parameter list (expected '(' followed by parameters and ')')",
                    bws(parse_parameter_list),
                )(input)?;

                // Parse body
                let (input, body) = parse_operator_body(input)?;

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
        },
    )(input)
}

/// Parse a conversion operator (implicit/explicit)
fn parse_conversion_operator(
    input: &str,
    attributes: Vec<crate::syntax::nodes::declarations::attribute::Attribute>,
    modifiers: Vec<crate::syntax::nodes::declarations::Modifier>,
    kind: ConversionKind,
) -> BResult<&str, OperatorDeclaration> {
    // Skip the implicit/explicit keyword
    let (input, _) = match kind {
        ConversionKind::Implicit => context(
            "implicit keyword (expected 'implicit')",
            bws(kw_implicit()),
        )(input)?,
        ConversionKind::Explicit => context(
            "explicit keyword (expected 'explicit')",
            bws(kw_explicit()),
        )(input)?,
    };

    // Parse the "operator" keyword
    let (input, _) = context(
        "operator keyword (expected 'operator')",
        bws(kw_operator()),
    )(input)?;

    // Parse the target type
    let (input, target_type) = context(
        "conversion target type (expected valid C# type)",
        bws(parse_type_expression),
    )(input)?;

    // Parse parameters
    let (input, parameters) = context(
        "conversion operator parameter list (expected '(' followed by parameters and ')')",
        bws(parse_parameter_list),
    )(input)?;

    // Parse body
    let (input, body) = parse_operator_body(input)?;

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
fn parse_operator_symbol(input: &str) -> BResult<&str, Identifier> {
    context(
        "operator symbol (expected valid C# operator like +, -, *, ++, ==, etc.)",
        alt((
            // Multi-character operators first (to avoid prefix conflicts)
            map(keyword("++"), |_| Identifier::new("++")),
            map(keyword("--"), |_| Identifier::new("--")),
            map(keyword("=="), |_| Identifier::new("==")),
            map(keyword("!="), |_| Identifier::new("!=")),
            map(keyword(">="), |_| Identifier::new(">=")),
            map(keyword("<="), |_| Identifier::new("<=")),
            // Keywords (these should also come before single characters)
            map(kw_true(), |_| Identifier::new("true")),
            map(kw_false(), |_| Identifier::new("false")),
            // Single character operators
            map(keyword("+"), |_| Identifier::new("+")),
            map(keyword("-"), |_| Identifier::new("-")),
            map(keyword("*"), |_| Identifier::new("*")),
            map(keyword("/"), |_| Identifier::new("/")),
            map(keyword("%"), |_| Identifier::new("%")),
            map(keyword(">"), |_| Identifier::new(">")),
            map(keyword("<"), |_| Identifier::new("<")),
            map(keyword("!"), |_| Identifier::new("!")),
            map(keyword("~"), |_| Identifier::new("~")),
        )),
    )(input)
}

/// Parse the operator body (either a block statement or semicolon)
fn parse_operator_body(input: &str) -> BResult<&str, String> {
    context(
        "operator body (expected block statement or semicolon for abstract/extern operator)",
        alt((
            // Block body
            map(
                parse_block_statement,
                |_| "{ /* body */ }".to_string(), // Simplified for now
            ),
            // Semicolon (abstract/extern)
            map(bws(bchar(';')), |_| "".to_string()),
        )),
    )(input)
}
