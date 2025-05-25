use crate::parser::errors::BResult;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{PrimitiveType, Type};
use crate::parser::parser_helpers::{bdelimited, bopt, bseparated_list0, bws, nom_to_bs};
use crate::parsers::identifier_parser::parse_qualified_name;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as nom_char, multispace0},
    combinator::map,
    combinator::value,
};

// Parse primitive types like int, bool, string
fn parse_primitive_type(input: &str) -> BResult<&str, Type> {
    alt((
        // Void type - revert to Type::Primitive(PrimitiveType::Void) for consistency with other tests
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("void")), |_| Type::Primitive(PrimitiveType::Void)),
        
        // Boolean type
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("bool")), |_| Type::Primitive(PrimitiveType::Bool)),
        
        // Integral types
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("byte")), |_| Type::Primitive(PrimitiveType::Byte)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("sbyte")), |_| Type::Primitive(PrimitiveType::SByte)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("short")), |_| Type::Primitive(PrimitiveType::Short)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("ushort")), |_| Type::Primitive(PrimitiveType::UShort)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("int")), |_| Type::Primitive(PrimitiveType::Int)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("uint")), |_| Type::Primitive(PrimitiveType::UInt)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("long")), |_| Type::Primitive(PrimitiveType::Long)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("ulong")), |_| Type::Primitive(PrimitiveType::ULong)),
        
        // Floating-point types
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("float")), |_| Type::Primitive(PrimitiveType::Float)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("double")), |_| Type::Primitive(PrimitiveType::Double)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("decimal")), |_| Type::Primitive(PrimitiveType::Decimal)),
        
        // Character and string types
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("char")), |_| Type::Primitive(PrimitiveType::Char)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("string")), |_| Type::Primitive(PrimitiveType::String)),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("object")), |_| Type::Primitive(PrimitiveType::Object)),
        
        // Special types
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("dynamic")), |_| Type::Dynamic),
        map(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("var")), |_| Type::Var),
    ))(input)
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
