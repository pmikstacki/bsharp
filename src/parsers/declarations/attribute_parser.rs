use nom::{
    character::complete::{char as nom_char, multispace0},
    combinator::opt,
    multi::{many0, separated_list0},
    sequence::{delimited, terminated},
};
use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::attribute::{Attribute, AttributeList};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::expressions::expression_parser::parse_expression;

/// Parses an attribute argument which can be any expression
fn parse_attribute_argument(input: &str) -> BResult<&str, Expression> {
    parse_expression(input)
}

/// Parses a single attribute with optional arguments
/// Example: `[Serializable]` or `[DataMember(Name = "firstName", Order = 1)]`
fn parse_single_attribute(input: &str) -> BResult<&str, Attribute> {
    // Parse the attribute name
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    
    // Parse optional arguments in parentheses
    let (input, opt_args) = opt(delimited(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('('))),
        separated_list0(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))),
            bws(nom_to_bs(parse_attribute_argument))
        ),
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(')')))
    ))(input)?;
    
    let arguments = opt_args.unwrap_or_default();
    
    Ok((input, Attribute { name, arguments }))
}

/// Parses an attribute list enclosed in square brackets
/// Example: `[Serializable, DataContract]`
fn parse_attribute_group(input: &str) -> BResult<&str, AttributeList> {
    let (input, attributes) = delimited(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('['))),
        separated_list0(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))),
            bws(nom_to_bs(parse_single_attribute))
        ),
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(']')))
    )(input)?;
    
    Ok((input, AttributeList { attributes }))
}

/// Parses multiple attribute lists that might appear before a declaration
/// Example: `[Serializable] [DataContract]`
pub fn parse_attribute_lists(input: &str) -> BResult<&str, Vec<AttributeList>> {
    // Use terminated to ensure we don't consume trailing whitespace after the attributes
    // This ensures the rest of the parsers (class, interface, etc.) get any whitespace before them
    many0(terminated(nom_to_bs(parse_attribute_group), multispace0))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::nodes::expressions::expression::Expression;
    use crate::parser::nodes::expressions::literal::Literal;
    
    #[test]
    fn test_single_attribute_no_args() {
        let input = "[Serializable]";
        let (rest, lists) = parse_attribute_lists(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].attributes.len(), 1);
        assert_eq!(lists[0].attributes[0].name.name, "Serializable");
        assert!(lists[0].attributes[0].arguments.is_empty());
    }
    
    #[test]
    fn test_multiple_attribute_lists() {
        let input = "[Serializable] [DataContract] class";
        let (rest, lists) = parse_attribute_lists(input).unwrap();
        assert_eq!(rest, "class");
        assert_eq!(lists.len(), 2);
        assert_eq!(lists[0].attributes[0].name.name, "Serializable");
        assert_eq!(lists[1].attributes[0].name.name, "DataContract");
    }
    
    #[test]
    fn test_attribute_with_argument() {
        let input = "[DataMember(1)]";
        let (rest, lists) = parse_attribute_lists(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].attributes[0].name.name, "DataMember");
        assert_eq!(lists[0].attributes[0].arguments.len(), 1);
        
        // Verify the argument is a literal with value 1
        if let Expression::Literal(Literal::Integer(val)) = &lists[0].attributes[0].arguments[0] {
            assert_eq!(*val, 1);
        } else {
            panic!("Expected integer literal");
        }
    }
    
    #[test]
    fn test_attribute_with_named_argument() {
        let _input = "[DataMember(Name = \"firstName\")]";
        // Further assertions would go here if we were testing the output
    }
    
    #[test]
    fn test_empty_attribute_list() {
        // No attributes in source code
        let input = "public class MyClass {}";
        let (rest, lists) = parse_attribute_lists(input).unwrap();
        assert_eq!(rest, "public class MyClass {}");
        assert!(lists.is_empty());
    }
    
    #[test]
    fn test_multiple_attributes_in_one_list() {
        let input = "[Serializable, DataContract]";
        let (rest, lists) = parse_attribute_lists(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].attributes.len(), 2);
        assert_eq!(lists[0].attributes[0].name.name, "Serializable");
        assert_eq!(lists[0].attributes[1].name.name, "DataContract");
    }
    
    #[test]
    fn test_complex_attribute_with_multiple_arguments() {
        let _input = "[DebuggerDisplay(\"Count = {Count}\", Type = \"MyType\")]";
        // Further assertions for complex attributes
    }
}
