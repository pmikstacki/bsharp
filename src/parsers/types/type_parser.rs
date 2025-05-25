use crate::parser::nodes::identifier::Identifier;
use nom::{
    branch::alt,
    character::complete::{char as nom_char, multispace0},
    bytes::complete::tag,
    combinator::value,
};
use crate::parser::nodes::types::{PrimitiveType, Type};
use crate::parser::errors::BResult;
use crate::parser::parser_helpers::{bws, nom_to_bs, bdelimited, bseparated_list0, bopt};
use crate::parsers::identifier_parser::parse_qualified_name;

// Parse primitive types like int, bool, string
fn parse_primitive_type(input: &str) -> BResult<&str, Type> {
    nom_to_bs(alt((
        // Void type
        value(Type::Primitive(PrimitiveType::Void), tag::<&str, &str, nom::error::Error<&str>>("void")),
        
        // Boolean type
        value(Type::Primitive(PrimitiveType::Bool), tag::<&str, &str, nom::error::Error<&str>>("bool")),
        
        // Integral types
        value(Type::Primitive(PrimitiveType::Byte), tag::<&str, &str, nom::error::Error<&str>>("byte")),
        value(Type::Primitive(PrimitiveType::SByte), tag::<&str, &str, nom::error::Error<&str>>("sbyte")),
        value(Type::Primitive(PrimitiveType::Short), tag::<&str, &str, nom::error::Error<&str>>("short")),
        value(Type::Primitive(PrimitiveType::UShort), tag::<&str, &str, nom::error::Error<&str>>("ushort")),
        value(Type::Primitive(PrimitiveType::Int), tag::<&str, &str, nom::error::Error<&str>>("int")),
        value(Type::Primitive(PrimitiveType::UInt), tag::<&str, &str, nom::error::Error<&str>>("uint")),
        value(Type::Primitive(PrimitiveType::Long), tag::<&str, &str, nom::error::Error<&str>>("long")),
        value(Type::Primitive(PrimitiveType::ULong), tag::<&str, &str, nom::error::Error<&str>>("ulong")),
        
        // Floating-point types
        value(Type::Primitive(PrimitiveType::Float), tag::<&str, &str, nom::error::Error<&str>>("float")),
        value(Type::Primitive(PrimitiveType::Double), tag::<&str, &str, nom::error::Error<&str>>("double")),
        value(Type::Primitive(PrimitiveType::Decimal), tag::<&str, &str, nom::error::Error<&str>>("decimal")),
        
        // Character and string types
        value(Type::Primitive(PrimitiveType::Char), tag::<&str, &str, nom::error::Error<&str>>("char")),
        value(Type::Primitive(PrimitiveType::String), tag::<&str, &str, nom::error::Error<&str>>("string")),
        
        // Special types
        value(Type::Dynamic, tag::<&str, &str, nom::error::Error<&str>>("dynamic")),
        value(Type::Var, tag::<&str, &str, nom::error::Error<&str>>("var")),
    )))(input)
}

// Parse an identifier (qualified, e.g., System.Console)
fn parse_identifier_string(input: &str) -> BResult<&str, String> {
    let (input, parts) = parse_qualified_name(input)?;
    let name = parts.iter().map(|id| id.name.clone()).collect::<Vec<_>>().join(".");
    Ok((input, name))
}

// Parse a generic type: Identifier<type1, type2, ...>
fn parse_generic_type(input: &str) -> BResult<&str, Type> {
    let (input, base) = parse_identifier_string(input)?;
    let (input, opt_generics) = bopt(
        bdelimited(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('<'))),
            bseparated_list0(bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))), parse_type_expression),
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('>'))),
        )
    )(input)?;
    if let Some(args) = opt_generics {
        Ok((input, Type::Generic { base: Identifier { name: base }, args }))
    } else {
        Ok((input, Type::Reference(Identifier { name: base })))
    }
}

fn parse_identifier_type(input: &str) -> BResult<&str, Type> {
    parse_generic_type(input)
}

// Parse array type suffix like [], [,] etc.
fn parse_array_suffix(input: &str) -> BResult<&str, usize> {
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('[')))(input)?;
    // Count the number of commas between brackets
    let (input, inner) = nom::bytes::complete::take_while(|c: char| c == ',' || c.is_whitespace())(input)?;
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(']')))(input)?;
    let rank = inner.chars().filter(|&c| c == ',').count() + 1;
    Ok((input, rank))
}

// Parse a potentially nullable type (e.g., int?)
fn parse_nullable_suffix(input: &str) -> BResult<&str, ()> {
    value((), nom_char('?'))(input)
}

// Main type parser: handles primitives, identifiers, arrays, nullables
pub fn parse_type_expression(input: &str) -> BResult<&str, Type> {
    // Try primitive first, then identifier
    let (input, ty) = match parse_primitive_type(input) {
        Ok((input, ty)) => (input, ty),
        Err(_) => parse_identifier_type(input)?
    };
    let (input, _) = multispace0(input)?;

    // Define a helper function to parse and apply suffixes recursively
    fn parse_suffixes(input: &str, ty: Type) -> BResult<&str, Type> {
        // Try array suffix
        if let Ok((next_input, rank)) = parse_array_suffix(input) {
            let array_type = Type::Array { element_type: Box::new(ty), rank };
            return parse_suffixes(next_input, array_type);
        }
        
        // Try nullable suffix
        if let Ok((next_input, _)) = parse_nullable_suffix(input) {
            let nullable_type = Type::Nullable(Box::new(ty));
            return parse_suffixes(next_input, nullable_type);
        }
        
        // No more suffixes found, return the current type
        Ok((input, ty))
    }
    
    // Apply the helper function to handle all suffixes
    let (input, ty) = parse_suffixes(input, ty)?;
    let (input, _) = multispace0(input)?;
    Ok((input, ty))
}
