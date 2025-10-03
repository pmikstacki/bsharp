use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::parameter_modifier_keywords::{
    kw_in, kw_out, kw_params, kw_ref, kw_scoped,
};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::types::{Parameter, ParameterModifier};
use crate::syntax::parser_helpers::{bchar, bws, context, parse_delimited_list0};
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::preceded;

// Parse parameter modifiers (ref, out, in, params) and return the actual modifier
fn parse_parameter_modifiers(input: &str) -> BResult<&str, Option<ParameterModifier>> {
    // If 'scoped' is present, only ref/in/out are valid to follow for this feature set
    if let Ok((after_scoped, _)) = bws(kw_scoped())(input) {
        let (rest, m) = alt((
            map(kw_ref(), |_| ParameterModifier::ScopedRef),
            map(kw_out(), |_| ParameterModifier::ScopedOut),
            map(kw_in(), |_| ParameterModifier::ScopedIn),
        ))(after_scoped)?;
        return Ok((rest, Some(m)));
    }
    opt(alt((
        map(kw_ref(), |_| ParameterModifier::Ref),
        map(kw_out(), |_| ParameterModifier::Out),
        map(kw_in(), |_| ParameterModifier::In),
        map(kw_params(), |_| ParameterModifier::Params),
    )))(input)
}

// Parse a single parameter
pub fn parse_parameter(input: &str) -> BResult<&str, Parameter> {
    // Optional attribute lists before modifiers
    let (input, attribute_lists) = bws(parse_attribute_lists)(input)?;
    let attributes = convert_attributes(attribute_lists);

    // Optional parameter modifier
    let (input, modifier) = bws(parse_parameter_modifiers)(input)?;
    let (input, ty) = context(
        "parameter type (expected valid type expression)",
        bws(parse_type_expression),
    )(input)?;
    let (input, name) = context(
        "parameter name (expected valid identifier)",
        bws(parse_identifier),
    )(input)?;
    // Optional default value: = expression
    let (input, default_value) = opt(preceded(bws(bchar('=')), bws(parse_expression)))(input)?;

    Ok((
        input,
        Parameter {
            attributes,
            modifier,
            parameter_type: ty,
            name,
            default_value,
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
