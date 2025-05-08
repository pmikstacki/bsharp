use crate::parser::nodes::identifier::Identifier;
use nom::{
    branch::alt,
    character::complete::{char as nom_char, multispace0},
    bytes::complete::tag,
    combinator::{opt, value},
    sequence::delimited,
};
use crate::parser::nodes::types::{PrimitiveType, Type};
use crate::parser::errors::BResult;

// Helper for optional whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Parse primitive types like int, bool, string
fn parse_primitive_type(input: &str) -> BResult<&str, Type> {
    alt((
        value(Type::Primitive(PrimitiveType::Int), tag("int")),
        value(Type::Primitive(PrimitiveType::Bool), tag("bool")),
        value(Type::Primitive(PrimitiveType::String), tag("string")),
        value(Type::Primitive(PrimitiveType::Void), tag("void")),
        value(Type::Dynamic, tag("dynamic")),
        value(Type::Var, tag("var")), // Add support for 'var' keyword
    ))(input)
}

// Parse an identifier (qualified, e.g., System.Console)
fn parse_identifier(input: &str) -> BResult<&str, String> {
    use nom::{bytes::complete::take_while1, multi::separated_list1};
    let (input, parts) = separated_list1(
        ws(nom_char('.')),
        take_while1(|c: char| c.is_alphanumeric() || c == '_')
    )(input)?;
    Ok((input, parts.join(".")))
}

// Parse a generic type: Identifier<type1, type2, ...>
fn parse_generic_type(input: &str) -> BResult<&str, Type> {
    use nom::{multi::separated_list1, character::complete::char as nom_char};
    let (input, base) = parse_identifier(input)?;
    let (input, opt_generics) = opt(
        delimited(
            ws(nom_char('<')),
            separated_list1(ws(nom_char(',')), parse_type_expression),
            ws(nom_char('>')),
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


// Parse an identifier type (potentially qualified)
// fn parse_identifier_type(input: &str) -> IResult<&str, Type> {
//     map(parse_identifier, Type::Identifier)(input)
// }

// Parse array type suffix like [], [,] etc.
fn parse_array_suffix(input: &str) -> BResult<&str, usize> {
    let (input, _) = ws(nom_char('['))(input)?;
    // Count the number of commas between brackets
    let (input, inner) = nom::bytes::complete::take_while(|c: char| c == ',' || c.is_whitespace())(input)?;
    let (input, _) = ws(nom_char(']'))(input)?;
    let rank = inner.chars().filter(|&c| c == ',').count() + 1;
    Ok((input, rank))
}

// Parse a potentially nullable type (e.g., int?)
fn parse_nullable_suffix(input: &str) -> BResult<&str, ()> {
    value((), nom_char('?'))(input)
}

// Main type parser: handles primitives, identifiers, arrays, nullables
pub fn parse_type_expression(input: &str) -> BResult<&str, Type> {
    // Consume leading whitespace - REMOVED AGAIN
    // let (input, _) = multispace0(input)?;

    // Try primitive first, then identifier
    let (input, ty) = match parse_primitive_type(input) {
        Ok((input, ty)) => (input, ty),
        Err(_) => parse_identifier_type(input)?
    };
    let (input, _) = multispace0(input)?;

    // Define a helper function to parse and apply suffixes recursively
    fn parse_suffixes<'a>(input: &'a str, ty: Type<'a>) -> BResult<&'a str, Type<'a>> {
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
