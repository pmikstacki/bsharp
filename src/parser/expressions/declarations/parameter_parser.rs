use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::types::{Parameter, ParameterModifier};
use crate::syntax::parser_helpers::{bchar, bws, context, keyword, parse_delimited_list0};
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::opt;

// Parse parameter modifiers (ref, out, in, params) and return the actual modifier
fn parse_parameter_modifiers(input: &str) -> BResult<&str, Option<ParameterModifier>> {
    opt(alt((
        map(keyword("ref"), |_| ParameterModifier::Ref),
        map(keyword("out"), |_| ParameterModifier::Out),
        map(keyword("in"), |_| ParameterModifier::In),
        map(keyword("params"), |_| ParameterModifier::Params),
    )))(input)
}

// Parse a single parameter
pub fn parse_parameter(input: &str) -> BResult<&str, Parameter> {
    let (input, modifier) = bws(parse_parameter_modifiers)(input)?;
    let (input, ty) = context(
        "parameter type (expected valid type expression)",
        bws(parse_type_expression),
    )(input)?;
    let (input, name) = context(
        "parameter name (expected valid identifier)",
        bws(parse_identifier),
    )(input)?;

    Ok((
        input,
        Parameter {
            modifier,
            parameter_type: ty,
            name,
        },
    ))
}

// Parse a parameter list enclosed in parentheses
pub fn parse_parameter_list(input: &str) -> BResult<&str, Vec<Parameter>> {
    context(
        "parameter list",
        parse_delimited_list0::<_, _, _, _, char, Parameter, char, char, Parameter>(
            bchar('('),
            |i| context("parameter (expected type and name)", bws(parse_parameter))(i),
            bchar(','),
            bchar(')'),
            false, // trailing commas not allowed in parameter list
            true,  // cut on close
        ),
    )(input)
}
