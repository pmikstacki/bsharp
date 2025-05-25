use nom::{
    branch::alt,
    bytes::complete::tag as nom_tag,
    character::complete::char as nom_char,
    combinator::map,
};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{OperatorDeclaration, OperatorKind, ConversionKind};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::types::type_parser::parse_type_expression;
use crate::parsers::declarations::parameter_parser::parse_parameter_list;
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::type_declaration_parser::convert_attributes;
use crate::parsers::statements::block_statement_parser::parse_block_statement;

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
    // Parse attributes
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    let attributes = convert_attributes(attribute_lists);
    
    // Parse modifiers (typically public static)
    let (input, modifiers) = parse_modifiers(input)?;
    
    // Check if this is a conversion operator (implicit/explicit operator)
    // If so, we parse differently
    if let Ok((_, _)) = nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("implicit"))(input) {
        parse_conversion_operator(input, attributes, modifiers, ConversionKind::Implicit)
    } else if let Ok((_, _)) = nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("explicit"))(input) {
        parse_conversion_operator(input, attributes, modifiers, ConversionKind::Explicit)
    } else {
        // Regular operator with return type
        let (input, return_type) = bws(nom_to_bs(parse_type_expression))(input)?;
        
        // Parse the "operator" keyword
        let (input, _) = bws(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("operator")))(input)?;
        
        // Parse the operator symbol
        let (input, operator_symbol) = parse_operator_symbol(input)?;
        
        // Parse parameters
        let (input, parameters) = bws(nom_to_bs(parse_parameter_list))(input)?;
        
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
}

/// Parse a conversion operator (implicit/explicit)
fn parse_conversion_operator(
    input: &str,
    attributes: Vec<crate::parser::nodes::declarations::attribute::Attribute>,
    modifiers: Vec<crate::parser::nodes::declarations::Modifier>,
    kind: ConversionKind
) -> BResult<&str, OperatorDeclaration> {
    // Skip the implicit/explicit keyword
    let (input, _) = match kind {
        ConversionKind::Implicit => bws(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("implicit")))(input)?,
        ConversionKind::Explicit => bws(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("explicit")))(input)?,
    };
    
    // Parse the "operator" keyword
    let (input, _) = bws(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("operator")))(input)?;
    
    // Parse the target type
    let (input, target_type) = bws(nom_to_bs(parse_type_expression))(input)?;
    
    // Parse parameters
    let (input, parameters) = bws(nom_to_bs(parse_parameter_list))(input)?;
    
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
    alt((
        // Multi-character operators first (to avoid prefix conflicts)
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("++")), |_| Identifier::new("++")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("--")), |_| Identifier::new("--")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("==")), |_| Identifier::new("==")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("!=")), |_| Identifier::new("!=")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>(">=")), |_| Identifier::new(">=")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("<=")), |_| Identifier::new("<=")),
        // Keywords (these should also come before single characters)
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("true")), |_| Identifier::new("true")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("false")), |_| Identifier::new("false")),
        // Single character operators
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("+")), |_| Identifier::new("+")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("-")), |_| Identifier::new("-")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("*")), |_| Identifier::new("*")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("/")), |_| Identifier::new("/")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("%")), |_| Identifier::new("%")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>(">")), |_| Identifier::new(">")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("<")), |_| Identifier::new("<")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("!")), |_| Identifier::new("!")),
        map(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("~")), |_| Identifier::new("~")),
    ))(input)
}

/// Parse the operator body (either a block statement or semicolon)
fn parse_operator_body(input: &str) -> BResult<&str, String> {
    alt((
        // Block body
        map(
            nom_to_bs(parse_block_statement),
            |_| "{ /* body */ }".to_string() // Simplified for now
        ),
        // Semicolon (abstract/extern)
        map(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(';'))),
            |_| "".to_string()
        ),
    ))(input)
} 