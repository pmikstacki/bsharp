use crate::parser::errors::BResult;
use crate::parser::nodes::types::Type;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::types::type_parser::parse_type_expression;
use nom::{
    character::complete::char as nom_char,
    multi::separated_list1,
};

/// Parses a base type list for declarations like classes, structs, interfaces, and records.
/// Handles the syntax `: Type1, Type2, ...` where each type is typically an interface
/// or (for classes only) a base class followed by interfaces.
///
/// # Examples
/// - `: IDisposable`
/// - `: IEnumerable<T>, IDisposable`
/// - `: BaseClass, IDisposable` (for classes only)
///
/// # Returns
/// A vector of parsed types, or an empty vector if no base types are specified.
pub fn parse_base_type_list(input: &str) -> BResult<&str, Vec<Type>> {
    // Attempt to parse the colon. If this fails, the whole function fails (it's not an optional colon here).
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(':')))(input)?;
    
    // If the colon is present, parse one or more types separated by commas.
    let (input, types) = bws(nom_to_bs(separated_list1(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))),
        bws(nom_to_bs(parse_type_expression))
    )))(input)?;
    
    Ok((input, types))
}
