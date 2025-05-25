use nom::{
    character::complete::char as nom_char,
};
use crate::parser::errors::BResult;
use crate::parser::nodes::types::Parameter;
use crate::parser::parser_helpers::{bws, nom_to_bs, bseparated_list0};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

// Parse a single parameter
pub fn parse_parameter(input: &str) -> BResult<&str, Parameter> {
    let (input, ty) = bws(parse_type_expression)(input)?;
    let (input, name) = bws(parse_identifier)(input)?;
    
    Ok((input, Parameter {
        ty,
        name,
    }))
}

// Parse a parameter list enclosed in parentheses
pub fn parse_parameter_list(input: &str) -> BResult<&str, Vec<Parameter>> {
    // Parse opening parenthesis
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('(')))(input)?;
    
    // Parse parameters separated by commas
    let (input, params) = bseparated_list0(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))),
        bws(parse_parameter)
    )(input)?;
    
    // Parse closing parenthesis
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(')')))(input)?;
    
    Ok((input, params))
}
