use crate::parser::parser_helpers::{bws, nom_to_bs};
use nom::bytes::complete::tag_no_case;
use crate::parser::nodes::expressions::expression::Expression;
use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::{char as nom_char, multispace0},
    combinator::{map, opt},
    multi::many0, 
    sequence::{delimited, preceded, tuple}, // Keep for internal nom usage if any
};
use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{PropertyAccessor, PropertyDeclaration}; 
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;

// Parse get accessor
fn parse_get_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Parse "get" followed by optional body or semicolon
    let (input, _) = bws(tag_no_case("get"))(input)?;
    
    // Parse body or just a semicolon
    let (input, body) = alt((
        // Body with braces: "get { return _value; }"
        map(
            delimited(
                bws(nom_char('{')), 
                take_until("}"), 
                bws(nom_char('}'))
            ),
            |content: &str| Some(content.trim().to_string())
        ),
        // Auto-property with just semicolon: "get;"
        map(
            bws(nom_char(';')),
            |_| None
        )
    ))(input)?;
    
    Ok((input, PropertyAccessor::Get(body)))
}

// Parse set accessor
fn parse_set_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Parse "set" followed by optional body or semicolon
    let (input, _) = bws(tag_no_case("set"))(input)?;
    
    // Parse body or just a semicolon
    let (input, body) = alt((
        // Body with braces: "set { _value = value; }"
        map(
            delimited(
                bws(nom_char('{')), 
                take_until("}"), 
                bws(nom_char('}'))
            ),
            |content: &str| Some(content.trim().to_string())
        ),
        // Auto-property with just semicolon: "set;"
        map(
            bws(nom_char(';')),
            |_| None
        )
    ))(input)?;
    
    Ok((input, PropertyAccessor::Set(body)))
}

// Parse init accessor (C# 9+)
fn parse_init_accessor(input: &str) -> BResult<&str, PropertyAccessor> {
    // Parse "init" followed by optional body or semicolon
    let (input, _) = bws(tag_no_case("init"))(input)?;
    
    // Parse body or just a semicolon
    let (input, body) = alt((
        // Body with braces: "init { _value = value; }"
        map(
            delimited(
                bws(nom_char('{')), 
                take_until("}"), 
                bws(nom_char('}'))
            ),
            |content: &str| Some(content.trim().to_string())
        ),
        // Auto-property with just semicolon: "init;"
        map(
            bws(nom_char(';')),
            |_| None
        )
    ))(input)?;
    
    Ok((input, PropertyAccessor::Init(body)))
}

// Parse property accessors - can be one or more accessors in braces
fn parse_property_accessors(input: &str) -> BResult<&str, Vec<PropertyAccessor>> {
    delimited(
        bws(nom_char('{')),
        many0(
            alt((
                nom_to_bs(parse_get_accessor),
                nom_to_bs(parse_set_accessor),
                nom_to_bs(parse_init_accessor)
            ))
        ),
        bws(nom_char('}'))
    )(input)
}

// Parse optional property initializer: " = expression;"
fn parse_property_initializer(input: &str) -> BResult<&str, Option<Expression>> {
    opt(
        preceded(
            bws(nom_char('=')),
            tuple((
                bws(nom_to_bs(parse_expression)),
                bws(nom_char(';'))
            ))
        )
    )(input)
    .map(|(input, result)| (input, result.map(|(expr, _)| expr)))
}

// Parse a property declaration
pub fn parse_property_declaration(input: &str) -> BResult<&str, PropertyDeclaration> {
    // Parse modifiers specifically for property declarations (they consume trailing space)
    let (input, modifiers) = nom_to_bs(|i| parse_modifiers_for_decl_type(i, "property"))(input)?;
    // Consume any additional optional whitespace before the type
    let (input, _) = multispace0(input)?;
    let (input, ty) = nom_to_bs(parse_type_expression)(input)?;
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    let (input, accessors) = bws(parse_property_accessors)(input)?;
    let (input, initializer) = bws(parse_property_initializer)(input)?;

    Ok((
        input,
        PropertyDeclaration {
            modifiers,
            ty,
            name,
            accessors,
            initializer,
        }
    ))
}
