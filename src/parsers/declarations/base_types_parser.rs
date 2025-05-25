use crate::parser::errors::BResult;
use crate::parser::nodes::types::Type;
use crate::parser::parser_helpers::{bchar, bws, nom_to_bs};
use crate::parsers::types::type_parser::parse_type_expression;
use nom::{
    combinator::opt,
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
/// - No colon (returns empty vector)
///
/// # Returns
/// A vector of parsed types, or an empty vector if no base types are specified.
pub fn parse_base_type_list(input: &str) -> BResult<&str, Vec<Type>> {
    // Check if there's a colon. If not, return empty vector.
    let (input, colon_opt) = opt(bws(bchar(':')))(input)?;
    
    if colon_opt.is_none() {
        return Ok((input, Vec::new()));
    }
    
    // If the colon is present, parse one or more types separated by commas.
    let (input, types) = bws(nom_to_bs(separated_list1(
        bws(bchar(',')),
        bws(nom_to_bs(parse_type_expression))
    )))(input)?;
    
    Ok((input, types))
}
