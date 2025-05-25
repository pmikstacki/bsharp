use nom::{
    bytes::complete::tag,
    character::complete::char as nom_char,
    combinator::opt,
};
use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::DelegateDeclaration;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;
use crate::parsers::declarations::type_parameter_parser::{opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses};
use crate::parsers::declarations::parameter_parser::parse_parameter_list;
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;

/// Parse a delegate declaration
/// Example: public delegate void MyDelegate(int x, string y);
/// Example: public delegate T MyGenericDelegate<T>(T input) where T : class;
pub fn parse_delegate_declaration(input: &str) -> BResult<&str, DelegateDeclaration> {
    // Parse optional attributes
    let (input, attributes) = bws(parse_attribute_lists)(input)?;
    
    // Parse modifiers
    let (input, modifiers) = parse_modifiers_for_decl_type(input, "delegate")?;
    
    // Parse 'delegate' keyword
    let (input, _) = bws(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("delegate")))(input)?;
    
    // Parse return type
    let (input, return_type) = bws(parse_type_expression)(input)?;
    
    // Parse delegate name
    let (input, name) = bws(parse_identifier)(input)?;
    
    // Parse optional type parameters
    let (input, type_parameters) = bws(nom_to_bs(opt_parse_type_parameter_list))(input)?;
    
    // Parse parameter list
    let (input, parameters) = bws(nom_to_bs(parse_parameter_list))(input)?;
    
    // Parse optional constraints
    let (input, constraints) = opt(bws(nom_to_bs(parse_type_parameter_constraints_clauses)))(input)?;
    
    // Parse semicolon
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(';')))(input)?;

    // Convert Some([]) to None for constraints and type_parameters
    let constraints = match constraints {
        Some(constraints_vec) if constraints_vec.is_empty() => None,
        other => other,
    };

    let type_parameters = match type_parameters {
        Some(type_params_vec) if type_params_vec.is_empty() => None,
        other => other,
    };

    Ok((
        input,
        DelegateDeclaration {
            attributes,
            modifiers,
            return_type,
            name,
            type_parameters: type_parameters.unwrap_or_default(),
            parameters,
            constraints,
        },
    ))
} 