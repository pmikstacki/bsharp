use nom::{
    character::complete::{char as nom_char, multispace0},
    multi::separated_list0,
    sequence::{delimited, tuple},
};
use crate::parser::errors::BResult;
use crate::parser::nodes::types::Parameter;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

// Helper for optional whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Parse a single parameter
pub fn parse_parameter(input: &str) -> BResult<&str, Parameter> {
    let (input, ty) = ws(parse_type_expression)(input)?;
    let (input, name) = ws(parse_identifier)(input)?;
    
    Ok((input, Parameter {
        ty,
        name,
        _phantom: std::marker::PhantomData,
    }))
}

// Parse a parameter list enclosed in parentheses
pub fn parse_parameter_list(input: &str) -> BResult<&str, Vec<Parameter>> {
    // Parse empty parameter list
    if let Ok((rest, _)) = tuple((ws(nom_char('(')), ws(nom_char(')'))))(input) {
        return Ok((rest, vec![]));
    }
    
    // Parse parameter list with parameters
    delimited(
        ws(nom_char('(')),
        separated_list0(
            ws(nom_char(',')),
            ws(parse_parameter)
        ),
        ws(nom_char(')'))
    )(input)
}
