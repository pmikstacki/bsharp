use crate::parser::errors::BResult;
use crate::parser::nodes::types::{Parameter, ParameterModifier};
use crate::parser::parser_helpers::{bseparated_list0, bws, nom_to_bs};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;
use nom::character::complete::char as nom_char;
use nom::combinator::opt;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::combinator::{not, peek, map};
use nom::character::complete::alpha1;

// Helper to ensure we match complete words, not prefixes
fn word_boundary(input: &str) -> nom::IResult<&str, (), nom::error::Error<&str>> {
    // Check that the next character is not alphanumeric or underscore, without consuming it
    peek(not(alpha1))(input)
}

// Parse parameter modifiers (ref, out, in, params) and return the actual modifier
fn parse_parameter_modifiers(input: &str) -> BResult<&str, Option<ParameterModifier>> {
    opt(nom_to_bs(alt((
        map(terminated(tag("ref"), word_boundary), |_| ParameterModifier::Ref),
        map(terminated(tag("out"), word_boundary), |_| ParameterModifier::Out),
        map(terminated(tag("in"), word_boundary), |_| ParameterModifier::In),
        map(terminated(tag("params"), word_boundary), |_| ParameterModifier::Params),
    ))))(input)
}

// Parse a single parameter
pub fn parse_parameter(input: &str) -> BResult<&str, Parameter> {
    let (input, modifier) = bws(parse_parameter_modifiers)(input)?;
    let (input, ty) = bws(parse_type_expression)(input)?;
    let (input, name) = bws(parse_identifier)(input)?;
    
    Ok((input, Parameter {
        modifier,
        parameter_type: ty,
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
