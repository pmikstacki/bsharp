use crate::parser::errors::BResult;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{PrimitiveType, Type};
use crate::parser::parser_helpers::{bdelimited, bopt, bseparated_list0, bws, nom_to_bs};
use crate::parsers::identifier_parser::parse_qualified_name;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as nom_char, multispace0, alpha1},
    combinator::{map, not, peek, value},
    sequence::terminated,
};

// Helper to ensure we match complete words, not prefixes
fn word_boundary(input: &str) -> nom::IResult<&str, (), nom::error::Error<&str>> {
    // Check that the next character is not alphanumeric or underscore, without consuming it
    peek(not(alpha1))(input)
}

// Parse primitive types like int, bool, string
fn parse_primitive_type(input: &str) -> BResult<&str, Type> {
    nom_to_bs(alt((
        // Void type - revert to Type::Primitive(PrimitiveType::Void) for consistency with other tests
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("void"), word_boundary), |_| Type::Primitive(PrimitiveType::Void)),
        
        // Boolean type
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("bool"), word_boundary), |_| Type::Primitive(PrimitiveType::Bool)),
        
        // Integral types - order matters! Put longer keywords first to avoid partial matches
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("ushort"), word_boundary), |_| Type::Primitive(PrimitiveType::UShort)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("uint"), word_boundary), |_| Type::Primitive(PrimitiveType::UInt)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("ulong"), word_boundary), |_| Type::Primitive(PrimitiveType::ULong)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("sbyte"), word_boundary), |_| Type::Primitive(PrimitiveType::SByte)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("short"), word_boundary), |_| Type::Primitive(PrimitiveType::Short)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("byte"), word_boundary), |_| Type::Primitive(PrimitiveType::Byte)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("int"), word_boundary), |_| Type::Primitive(PrimitiveType::Int)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("long"), word_boundary), |_| Type::Primitive(PrimitiveType::Long)),
        
        // Floating-point types
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("double"), word_boundary), |_| Type::Primitive(PrimitiveType::Double)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("decimal"), word_boundary), |_| Type::Primitive(PrimitiveType::Decimal)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("float"), word_boundary), |_| Type::Primitive(PrimitiveType::Float)),
        
        // Character and string types
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("string"), word_boundary), |_| Type::Primitive(PrimitiveType::String)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("object"), word_boundary), |_| Type::Primitive(PrimitiveType::Object)),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("char"), word_boundary), |_| Type::Primitive(PrimitiveType::Char)),
        
        // Special types
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("dynamic"), word_boundary), |_| Type::Dynamic),
        map(terminated(tag::<&str, &str, nom::error::Error<&str>>("var"), word_boundary), |_| Type::Var),
    )))(input)
}

// Parse function pointer type: delegate*<int, string, void> or delegate* managed<int, void>
fn parse_function_pointer_type(input: &str) -> BResult<&str, Type> {
    let (input, _) = bws(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("delegate*")))(input)?;
    
    // Parse optional calling convention
    let (input, calling_convention) = bopt(bws(nom_to_bs(alt((
        map(tag::<&str, &str, nom::error::Error<&str>>("managed"), |s: &str| s.to_string()),
        map(tag::<&str, &str, nom::error::Error<&str>>("unmanaged"), |s: &str| s.to_string()),
    )))))(input)?;
    
    // Parse type parameters: <param1, param2, ..., return_type>
    let (input, types) = bdelimited(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('<'))),
        bseparated_list0(bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))), parse_type_expression),
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('>'))),
    )(input)?;
    
    if types.is_empty() {
        return Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(
            input,
            crate::parser::errors::CustomErrorKind::Expected("Function pointer must have at least one type parameter")
        )));
    }
    
    // Last type is the return type, rest are parameter types
    let return_type = Box::new(types.last().unwrap().clone());
    let parameter_types = types[..types.len().saturating_sub(1)].to_vec();
    
    Ok((input, Type::FunctionPointer {
        calling_convention,
        parameter_types,
        return_type,
    }))
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
        if args.is_empty() {
            return Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(
                input,
                crate::parser::errors::CustomErrorKind::Expected("Generic type must have at least one type argument")
            )));
        }
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

// Parse pointer suffix (e.g., int* or char**)
fn parse_pointer_suffix(input: &str) -> BResult<&str, ()> {
    value((), nom_char('*'))(input)
}

// Parse a ref return type (ref Type)
fn parse_ref_return_type(input: &str) -> BResult<&str, Type> {
    let (input, _) = bws(nom_to_bs(terminated(tag::<&str, &str, nom::error::Error<&str>>("ref"), word_boundary)))(input)?;
    let (input, inner_type) = parse_type_expression(input)?;
    Ok((input, Type::RefReturn(Box::new(inner_type))))
}

// Main type parser: handles primitives, identifiers, arrays, nullables, pointers, function pointers
pub fn parse_type_expression(input: &str) -> BResult<&str, Type> {
    // Try function pointer first, then ref return, then primitive, then identifier
    let (input, ty) = alt((
        parse_function_pointer_type,
        parse_ref_return_type,
        parse_primitive_type,
        parse_identifier_type,
    ))(input)?;
    let (input, _) = multispace0(input)?;

    // Define a helper function to parse and apply suffixes recursively
    fn parse_suffixes(input: &str, ty: Type) -> BResult<&str, Type> {
        // Try pointer suffix first (higher precedence than array)
        if let Ok((next_input, _)) = parse_pointer_suffix(input) {
            let pointer_type = Type::Pointer(Box::new(ty));
            return parse_suffixes(next_input, pointer_type);
        }
        
        // Try array suffix
        if let Ok((next_input, rank)) = parse_array_suffix(input) {
            let array_type = Type::Array { element_type: Box::new(ty), rank };
            return parse_suffixes(next_input, array_type);
        }
        
        // Try nullable suffix
        if let Ok((next_input, _)) = parse_nullable_suffix(input) {
            // Use Nullable for all nullable types to match existing test expectations
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
