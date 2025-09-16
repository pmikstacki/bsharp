use crate::syntax::errors::BResult;
use crate::syntax::nodes::types::{Parameter, ParameterModifier};
use crate::syntax::parser_helpers::{context, bseparated_list0, bws, bchar};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use nom::combinator::opt;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::combinator::{not, peek, map};
use nom::character::complete::alpha1;
use nom::combinator::cut;

// Helper to ensure we match complete words, not prefixes
fn word_boundary(input: &str) -> BResult<&str, ()> {
    // Check that the next character is not alphanumeric or underscore, without consuming it
    peek(not(alpha1))(input)
}

// Parse parameter modifiers (ref, out, in, params) and return the actual modifier
fn parse_parameter_modifiers(input: &str) -> BResult<&str, Option<ParameterModifier>> {
    opt(alt((
        map(terminated(tag("ref"), word_boundary), |_| ParameterModifier::Ref),
        map(terminated(tag("out"), word_boundary), |_| ParameterModifier::Out),
        map(terminated(tag("in"), word_boundary), |_| ParameterModifier::In),
        map(terminated(tag("params"), word_boundary), |_| ParameterModifier::Params),
    )))(input)
}

// Parse a single parameter
pub fn parse_parameter(input: &str) -> BResult<&str, Parameter> {
    let (input, modifier) = bws(parse_parameter_modifiers)(input)?;
    let (input, ty) = context("parameter type (expected valid type expression)", bws(parse_type_expression))(input)?;
    let (input, name) = context("parameter name (expected valid identifier)", bws(parse_identifier))(input)?;
    
    Ok((input, Parameter {
        modifier,
        parameter_type: ty,
        name,
    }))
}

// Parse a parameter list enclosed in parentheses
pub fn parse_parameter_list(input: &str) -> BResult<&str, Vec<Parameter>> {
    // Parse opening parenthesis
    let (input, _) = context("parameter list opening (expected '(')", bws(bchar('(')))(input)?;
    
    // Parse parameters separated by commas
    let (input, params) = bseparated_list0(
        move |input: &str| context("parameter separator (expected ',')", bws(bchar(',')))(input),
        move |input: &str| context("parameter (expected type and name)", bws(parse_parameter))(input)
    )(input)?;
    
    // Parse closing parenthesis
    let (input, _) = context("parameter list closing (expected ')')", cut(bws(bchar(')'))))(input)?;
    
    Ok((input, params))
}
