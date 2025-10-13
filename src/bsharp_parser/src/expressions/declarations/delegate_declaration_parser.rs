use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parser::expressions::declarations::parameter_parser::parse_parameter_list;
use crate::parser::expressions::declarations::type_parameter_parser::{
    opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses,
};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use nom::combinator::opt;
use syntax::declarations::DelegateDeclaration;
use crate::syntax::comment_parser::ws;
use nom::sequence::delimited;
use nom::character::complete::satisfy;
use nom::Parser;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

/// Parse a delegate declaration
/// Example: public delegate void MyDelegate(int x, string y);
/// Example: public delegate T MyGenericDelegate<T>(T input) where T : class;
pub fn parse_delegate_declaration<'a>(input: Span<'a>) -> BResult<'a, DelegateDeclaration> {
    // Attributes
    let (input, attributes) = delimited(ws, parse_attribute_lists, ws).parse(input)?;

    // Modifiers
    let (input, modifiers) = parse_modifiers_for_decl_type(input, "delegate")?;

    // 'delegate' keyword
    let (input, _) = delimited(ws, tag("delegate"), ws)
        .context("delegate keyword")
        .parse(input)?;

    // Return type
    let (input, return_type) = delimited(ws, parse_type_expression, ws)
        .context("delegate return type")
        .parse(input)?;

    // Name
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("delegate name")
        .parse(input)?;

    // Optional type parameters
    let (input, type_parameters) = opt(|i| delimited(ws, opt_parse_type_parameter_list, ws).parse(i))
        .parse(input)?;

    // Parameters
    let (input, parameters) = delimited(ws, parse_parameter_list, ws)
        .context("delegate parameter list")
        .parse(input)?;

    // Optional constraints
    let (input, constraints) = opt(|i| delimited(ws, parse_type_parameter_constraints_clauses, ws).parse(i))
        .parse(input)?;

    // Semicolon
    let (input, _) = delimited(ws, satisfy(|c| c == ';'), ws)
        .context("semicolon")
        .parse(input)?;

    // Normalize empty Some([]) -> None
    let constraints = match constraints {
        Some(v) if v.is_empty() => None,
        other => other,
    };
    let type_parameters = match type_parameters {
        Some(Some(v)) if v.is_empty() => None,
        Some(other) => other,
        None => None,
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
use crate::syntax::span::Span;
