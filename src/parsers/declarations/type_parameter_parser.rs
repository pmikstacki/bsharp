use nom::{
    branch::alt,
    character::complete::char as nom_char,
    bytes::complete::tag,
    combinator::value,
};
use crate::parser::errors::BResult;
use crate::parser::nodes::types::{TypeParameter, Variance};
use crate::parser::nodes::declarations::{TypeParameterConstraint, TypeParameterConstraintClause};
use crate::parser::parser_helpers::{bws, nom_to_bs, bdelimited, bseparated_list0, bopt};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

// Parse variance keyword (in/out)
fn parse_variance(input: &str) -> BResult<&str, Variance> {
    nom_to_bs(alt((
        value(Variance::In, tag::<&str, &str, nom::error::Error<&str>>("in")),
        value(Variance::Out, tag::<&str, &str, nom::error::Error<&str>>("out")),
    )))(input)
}

// Parse a single type parameter, e.g., "T", "in T", "out T"
fn parse_single_type_parameter(input: &str) -> BResult<&str, TypeParameter> {
    // Try parsing optional variance keyword first
    let (input, variance) = bopt(bws(nom_to_bs(parse_variance)))(input)?;
    let (input, name) = parse_identifier(input)?;
    
    Ok((input, TypeParameter {
        name,
        variance: variance.unwrap_or(Variance::None),
    }))
}

// Parse a list of type parameters enclosed in angle brackets, e.g., "<T, in U>"
pub fn parse_type_parameter_list(input: &str) -> BResult<&str, Vec<TypeParameter>> {
    bdelimited(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('<'))),
        bseparated_list0(bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))), bws(parse_single_type_parameter)),
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('>')))
    )(input)
}

// Optional type parameter list (returns None if no list, Some(Vec) otherwise)
pub fn opt_parse_type_parameter_list(input: &str) -> BResult<&str, Option<Vec<TypeParameter>>> {
    // First use the opt combinator to try parsing type parameters
    let (rest, opt_params) = bopt(parse_type_parameter_list)(input)?;
    
    // Convert Some([]) to None, matching test expectations
    let result = match opt_params {
        Some(params) if params.is_empty() => None,
        other => other,
    };
    
    Ok((rest, result))
}

// Parse a single constraint (e.g., 'class', 'struct', 'new()', 'BaseType', 'T')
fn parse_type_parameter_constraint(input: &str) -> BResult<&str, TypeParameterConstraint> {
    // Try each constraint type individually
    
    // Try "class" keyword
    if let Ok((rest, _)) = nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("class"))(input) {
        return Ok((rest, TypeParameterConstraint::ReferenceType));
    }
    
    // Try "struct" keyword
    if let Ok((rest, _)) = nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("struct"))(input) {
        return Ok((rest, TypeParameterConstraint::ValueType));
    }
    
    // Try "unmanaged" keyword
    if let Ok((rest, _)) = nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("unmanaged"))(input) {
        return Ok((rest, TypeParameterConstraint::Unmanaged));
    }
    
    // Try "notnull" keyword
    if let Ok((rest, _)) = nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("notnull"))(input) {
        return Ok((rest, TypeParameterConstraint::NotNull));
    }
    
    // Try "new()" keyword
    if let Ok((rest, _)) = nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("new()"))(input) {
        return Ok((rest, TypeParameterConstraint::Constructor));
    }
    
    // Finally try parsing as a type expression
    let (rest, ty) = parse_type_expression(input)?;
    Ok((rest, TypeParameterConstraint::SpecificType(ty)))
}

// Parse a 'where' clause for a specific type parameter
fn parse_where_clause(input: &str) -> BResult<&str, TypeParameterConstraintClause> {
    let (input, _) = bws(nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("where")))(input)?;
    let (input, name) = bws(parse_identifier)(input)?;
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(':')))(input)?;
    let (input, constraints) = bseparated_list0(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))), 
        bws(parse_type_parameter_constraint)
    )(input)?;

    Ok((input, TypeParameterConstraintClause { 
        type_param: name, 
        constraints
    }))
}

// Parse zero or more 'where' clauses
pub fn parse_type_parameter_constraints_clauses(input: &str) -> BResult<&str, Vec<TypeParameterConstraintClause>> {
    // Use many0 from nom but wrapped for BResult
    let mut clauses = Vec::new();
    let mut current_input = input;
    
    loop {
        match bws(parse_where_clause)(current_input) {
            Ok((rest, clause)) => {
                clauses.push(clause);
                current_input = rest;
            },
            Err(_) => break,
        }
    }
    
    Ok((current_input, clauses))
}
