use nom::{
    character::complete::{char as nom_char, multispace0},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::tuple,
};
use crate::parser::errors::BResult;
use crate::parser::nodes::types::Type;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::types::type_parser::parse_type_expression;

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
pub fn parse_base_type_list<'a>(input: &'a str) -> BResult<&'a str, Vec<Type<'a>>> {
    // Parse an optional base type list prefixed with ':' character
    let base_list_parser = tuple((
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(':'))),
        bws(nom_to_bs(separated_list1(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))),
            bws(nom_to_bs(parse_type_expression))
        ))),
    ));
    
    // Make the entire base list optional
    let (input, opt_base_list) = opt(base_list_parser)(input)?;
    
    // Return parsed types or empty vector if no base list
    match opt_base_list {
        Some((_, types)) => Ok((input, types)),
        None => Ok((input, Vec::new())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::nodes::identifier::Identifier;
    
    #[test]
    fn test_no_base_types() {
        let input = "{ }";
        let (rest, types) = parse_base_type_list(input).unwrap();
        assert_eq!(rest, "{ }");
        assert!(types.is_empty());
    }
    
    #[test]
    fn test_single_interface() {
        let input = ": IDisposable { }";
        let (rest, types) = parse_base_type_list(input).unwrap();
        assert_eq!(rest, "{ }");
        assert_eq!(types.len(), 1);
        
        if let Type::Reference(id) = &types[0] {
            assert_eq!(id.name, "IDisposable");
        } else {
            panic!("Expected Reference type but got {:?}", types[0]);
        }
    }
    
    #[test]
    fn test_multiple_interfaces() {
        let input = ": IComparable, IEnumerable, IDisposable { }";
        let (rest, types) = parse_base_type_list(input).unwrap();
        assert_eq!(rest, "{ }");
        assert_eq!(types.len(), 3);
        
        if let Type::Reference(id) = &types[0] {
            assert_eq!(id.name, "IComparable");
        } else {
            panic!("Expected Reference type but got {:?}", types[0]);
        }
        
        if let Type::Reference(id) = &types[1] {
            assert_eq!(id.name, "IEnumerable");
        } else {
            panic!("Expected Reference type but got {:?}", types[1]);
        }
        
        if let Type::Reference(id) = &types[2] {
            assert_eq!(id.name, "IDisposable");
        } else {
            panic!("Expected Reference type but got {:?}", types[2]);
        }
    }
    
    #[test]
    fn test_generic_interface() {
        let input = ": IEnumerable<string> { }";
        let (rest, types) = parse_base_type_list(input).unwrap();
        assert_eq!(rest, "{ }");
        assert_eq!(types.len(), 1);
        
        if let Type::Generic { base, args } = &types[0] {
            assert_eq!(base.name, "IEnumerable");
            assert_eq!(args.len(), 1);
            match &args[0] {
                Type::Primitive(prim) => assert_eq!(format!("{:?}", prim), "String"),
                _ => panic!("Expected string primitive type"),
            }
        } else {
            panic!("Expected Generic type but got {:?}", types[0]);
        }
    }
    
    #[test]
    fn test_whitespace_variations() {
        // Extra whitespace around colon
        let input = "  :  IDisposable { }";
        let (rest, types) = parse_base_type_list(input).unwrap();
        assert_eq!(rest, "{ }");
        assert_eq!(types.len(), 1);
        
        // Extra whitespace around comma
        let input = ": IComparable  ,  IDisposable { }";
        let (rest, types) = parse_base_type_list(input).unwrap();
        assert_eq!(rest, "{ }");
        assert_eq!(types.len(), 2);
    }
    
    #[test]
    fn test_qualified_interface_name() {
        let input = ": System.Collections.IEnumerable { }";
        let (rest, types) = parse_base_type_list(input).unwrap();
        assert_eq!(rest, "{ }");
        assert_eq!(types.len(), 1);
        
        if let Type::Reference(id) = &types[0] {
            assert_eq!(id.name, "System.Collections.IEnumerable");
        } else {
            panic!("Expected Reference type but got {:?}", types[0]);
        }
    }
}
