use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{preceded, terminated},
};
use crate::syntax::comment_parser::ws;
use crate::syntax::parser_helpers::{bdelimited, bopt, bseparated_list0, btag, bws, context};
use crate::syntax::errors::BResult;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::{PrimitiveType, Type, CallingConvention};
use crate::parser::identifier_parser::parse_qualified_name;
use nom::{
    branch::alt,
    character::complete::{char as nom_char, alpha1},
    combinator::{not, peek, value},
};
use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};

// Helper to ensure we match complete words, not prefixes
fn word_boundary(input: &str) -> BResult<&str, ()> {
    // Check that the next character is not alphanumeric or underscore, without consuming it
    peek(not(alpha1))(input)
}

// Parse primitive types like int, bool, string
fn parse_primitive_type(input: &str) -> BResult<&str, Type> {
    context(
        "primitive type (expected built-in C# type like 'int', 'string', 'bool', etc.)",
        alt((
            // Void type - revert to Type::Primitive(PrimitiveType::Void) for consistency with other tests
            map(terminated(tag("void"), word_boundary), |_| Type::Primitive(PrimitiveType::Void)),
            
            // Boolean type
            map(terminated(tag("bool"), word_boundary), |_| Type::Primitive(PrimitiveType::Bool)),
            
            // Integral types - order matters! Put longer keywords first to avoid partial matches
            map(terminated(tag("ushort"), word_boundary), |_| Type::Primitive(PrimitiveType::UShort)),
            map(terminated(tag("uint"), word_boundary), |_| Type::Primitive(PrimitiveType::UInt)),
            map(terminated(tag("ulong"), word_boundary), |_| Type::Primitive(PrimitiveType::ULong)),
            map(terminated(tag("sbyte"), word_boundary), |_| Type::Primitive(PrimitiveType::SByte)),
            map(terminated(tag("short"), word_boundary), |_| Type::Primitive(PrimitiveType::Short)),
            map(terminated(tag("byte"), word_boundary), |_| Type::Primitive(PrimitiveType::Byte)),
            map(terminated(tag("int"), word_boundary), |_| Type::Primitive(PrimitiveType::Int)),
            map(terminated(tag("long"), word_boundary), |_| Type::Primitive(PrimitiveType::Long)),
            
            // Floating-point types
            map(terminated(tag("double"), word_boundary), |_| Type::Primitive(PrimitiveType::Double)),
            map(terminated(tag("decimal"), word_boundary), |_| Type::Primitive(PrimitiveType::Decimal)),
            map(terminated(tag("float"), word_boundary), |_| Type::Primitive(PrimitiveType::Float)),
            
            // Character and string types
            map(terminated(tag("string"), word_boundary), |_| Type::Primitive(PrimitiveType::String)),
            map(terminated(tag("object"), word_boundary), |_| Type::Primitive(PrimitiveType::Object)),
            map(terminated(tag("char"), word_boundary), |_| Type::Primitive(PrimitiveType::Char)),
            
            // Special types
            map(terminated(tag("dynamic"), word_boundary), |_| Type::Dynamic),
            map(terminated(tag("var"), word_boundary), |_| Type::Var),
        )),
    )(input)
}

// Parse function pointer type: delegate*<int, string, void> or delegate* managed<int, void>
fn parse_function_pointer_type(input: &str) -> BResult<&str, Type> {
    context(
        "function pointer type (expected 'delegate*' followed by optional calling convention and type parameters)",
        |input| {
            let (input, _) = preceded(ws, tag("delegate*"))(input)?;
            
            // Parse optional calling convention using standard nom combinators
            let (input, calling_convention) = opt(preceded(
                ws,
                alt((
                    value(CallingConvention::Managed, tag("managed")),
                    value(CallingConvention::Unmanaged, tag("unmanaged")),
                ))
            ))(input)?;
            
            // Parse type parameters: <param1, param2, ..., return_type>
            let (input, types) = bdelimited(
                bws(nom_char('<')),
                bseparated_list0(bws(nom_char(',')), parse_type_expression),
                bws(nom_char('>')),
            )(input)?;
            
            if types.is_empty() {
                let error_tree = ErrorTree::Base {
                    location: input,
                    kind: BaseErrorKind::Expected(Expectation::Tag("function pointer type parameters (expected at least one type)")),
                };
                return Err(nom::Err::Error(error_tree));
            }
            
            // Last type is the return type, rest are parameter types
            let return_type = Box::new(types.last().unwrap().clone());
            let parameter_types = types[..types.len().saturating_sub(1)].to_vec();
            
            Ok((input, Type::FunctionPointer {
                calling_convention,
                parameter_types,
                return_type,
            }))
        },
    )(input)
}

// Parse an identifier (qualified, e.g., System.Console)
fn parse_identifier_string(input: &str) -> BResult<&str, String> {
    let (input, parts) = parse_qualified_name(input)?;
    let name = parts.iter().map(|id| id.name.clone()).collect::<Vec<_>>().join(".");
    Ok((input, name))
}

// Parse a generic type: Identifier<type1, type2, ...>
fn parse_generic_type(input: &str) -> BResult<&str, Type> {
    context(
        "generic or reference type (expected identifier optionally followed by type arguments)",
        |input| {
            let (input, base) = parse_identifier_string(input)?;
            let (input, opt_generics) = bopt(
                bdelimited(
                    bws(nom_char('<')),
                    bseparated_list0(bws(nom_char(',')), parse_type_expression),
                    bws(nom_char('>')),
                )
            )(input)?;
            if let Some(args) = opt_generics {
                if args.is_empty() {
                    let error_tree = ErrorTree::Base {
                        location: input,
                        kind: BaseErrorKind::Expected(Expectation::Tag("generic type arguments (expected at least one type argument)")),
                    };
                    return Err(nom::Err::Error(error_tree));
                }
                Ok((input, Type::Generic { base: Identifier { name: base }, args }))
            } else {
                Ok((input, Type::Reference(Identifier { name: base })))
            }
        },
    )(input)
}

fn parse_identifier_type(input: &str) -> BResult<&str, Type> {
    parse_generic_type(input)
}

// Parse array type suffix like [], [,] etc.
fn parse_array_suffix(input: &str) -> BResult<&str, usize> {
    let (input, _) = bws(nom_char('['))(input)?;
    // Count the number of commas between brackets
    let (input, inner) = nom::bytes::complete::take_while(|c: char| c == ',' || c.is_whitespace())(input)?;
    let (input, _) = bws(nom_char(']'))(input)?;
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

// Helper function for parsing ref keyword with word boundary
fn parse_ref_keyword(input: &str) -> BResult<&str, &str> {
    terminated(btag("ref"), word_boundary)(input)
}

// Parse a ref return type (ref Type)
fn parse_ref_return_type(input: &str) -> BResult<&str, Type> {
    context(
        "ref return type (expected 'ref' followed by a type)",
        |input| {
            let (input, _) = bws(parse_ref_keyword)(input)?;
            let (input, inner_type) = parse_type_expression(input)?;
            Ok((input, Type::RefReturn(Box::new(inner_type))))
        },
    )(input)
}

// Main type syntax: handles primitives, identifiers, arrays, nullables, pointers, function pointers
pub fn parse_type_expression(input: &str) -> BResult<&str, Type> {
    context(
        "type expression (expected any valid C# type)",
        |input| {
            // Try function pointer first, then ref return, then primitive, then identifier
            let (input, ty) = alt((
                parse_function_pointer_type,
                parse_ref_return_type,
                parse_primitive_type,
                parse_identifier_type,
            ))(input)?;
            let (input, _) = ws(input)?;

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
            let (input, _) = ws(input)?;
            Ok((input, ty))
        },
    )(input)
}
