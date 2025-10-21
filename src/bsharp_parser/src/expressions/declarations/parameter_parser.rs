use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::parameter_modifier_keywords::{
    kw_in, kw_out, kw_params, kw_ref, kw_scoped,
};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;

use crate::syntax::comment_parser::ws;
use crate::syntax::list_parser::parse_delimited_list0;
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, preceded};
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::types::{Parameter, ParameterModifier};

// Parse parameter modifiers (ref, out, in, params) and return the actual modifier
fn parse_parameter_modifiers(input: Span) -> BResult<Option<ParameterModifier>> {
    // If 'scoped' is present, only ref/in/out are valid to follow for this feature set
    if let Ok((after_scoped, _)) = delimited(ws, kw_scoped(), ws).parse(input) {
        let (rest, m) = alt((
            map(kw_ref(), |_| ParameterModifier::ScopedRef),
            map(kw_out(), |_| ParameterModifier::ScopedOut),
            map(kw_in(), |_| ParameterModifier::ScopedIn),
        ))
            .parse(after_scoped)?;
        return Ok((rest, Some(m)));
    }
    opt(alt((
        map(kw_ref(), |_| ParameterModifier::Ref),
        map(kw_out(), |_| ParameterModifier::Out),
        map(kw_in(), |_| ParameterModifier::In),
        map(kw_params(), |_| ParameterModifier::Params),
    )))
        .parse(input)
}

// Parse a single parameter
pub fn parse_parameter(input: Span) -> BResult<Parameter> {
    // Optional attribute lists before modifiers
    let (input, attribute_lists) = delimited(ws, parse_attribute_lists, ws).parse(input)?;
    let attributes = convert_attributes(attribute_lists);

    // Optional parameter modifier
    let (input, modifier) = delimited(ws, parse_parameter_modifiers, ws).parse(input)?;
    let (input, ty) = delimited(ws, parse_type_expression, ws)
        .context("parameter type")
        .parse(input)?;
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("parameter name")
        .parse(input)?;
    // Optional default value: = expression
    let (input, default_value) = opt(preceded(
        |i| delimited(ws, tok_assign(), ws).parse(i),
        |i| delimited(ws, parse_expression, ws).parse(i),
    ))
        .parse(input)?;

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
pub fn parse_parameter_list(input: Span) -> BResult<Vec<Parameter>> {
    parse_delimited_list0::<_, _, _, _, char, char, char, Parameter>(
        |i| delimited(ws, tok_l_paren(), ws).parse(i),
        |i| delimited(ws, parse_parameter, ws)
            .context("parameter")
            .parse(i),
        |i| delimited(ws, tok_comma(), ws).parse(i),
        |i| delimited(ws, tok_r_paren(), ws).parse(i),
        false,
        true,
    )
        .context("parameter list")
        .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::assignment::tok_assign;
use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use crate::tokens::separators::tok_comma;
