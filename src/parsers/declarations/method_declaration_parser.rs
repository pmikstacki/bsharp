use nom::{
    bytes::complete::take_until,
    character::complete::{char as nom_char, multispace0},
    branch::alt,
    combinator::map,
    sequence::delimited,
};
use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{MethodDeclaration};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;
use crate::parsers::declarations::type_parameter_parser::{opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses};
use crate::parsers::declarations::parameter_parser::parse_parameter_list;
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;

// Helper for optional whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Parser for method body (captures content within braces)
fn parse_method_body(input: &str) -> BResult<&str, Option<String>> {
    // Try parsing a block body { ... }
    let block_parser = map(
        delimited(
            ws(nom_char('{')),
            take_until("}"),
            ws(nom_char('}'))
        ),
        |body_content: &str| Some(body_content.trim().to_string())
    );

    // Try parsing just a semicolon (abstract/interface method)
    let semicolon_parser = map(ws(nom_char(';')), |_| None);

    // Choose between block body or semicolon
    alt((block_parser, semicolon_parser))(input)

    // TODO: Implement expression body parsing `=> expression;` in future
}

// Parse a method declaration
pub fn parse_method_declaration(input: &str) -> BResult<&str, MethodDeclaration> {
    // Parse modifiers with validation for method declarations
    let (input, modifiers) = ws(|i| parse_modifiers_for_decl_type(i, "method"))(input)?;
    
    let (input, return_type) = ws(parse_type_expression)(input)?;
    let (input, name) = ws(parse_identifier)(input)?;
    
    // Parse type parameters (generics)
    let (input, type_parameters) = ws(opt_parse_type_parameter_list)(input)?;
    
    // Parse parameter list
    let (input, parameters) = ws(parse_parameter_list)(input)?;
    
    // Parse type constraints (where clauses)
    let (input, constraints) = ws(parse_type_parameter_constraints_clauses)(input)?;
    
    // Parse method body
    let (input, body_str) = ws(parse_method_body)(input)?;

    Ok((
        input,
        MethodDeclaration {
            modifiers,
            return_type,
            name,
            type_parameters,
            parameters,
            constraints,
            body: body_str,
            _phantom: std::marker::PhantomData,
        },
    ))
}
