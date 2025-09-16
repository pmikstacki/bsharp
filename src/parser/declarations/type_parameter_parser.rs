use nom::{
    branch::alt,
    combinator::value,
};

use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{TypeParameterConstraint, TypeParameterConstraintClause};
use crate::syntax::nodes::types::{TypeParameter, Variance};
use crate::syntax::parser_helpers::{context, bdelimited, bopt, bseparated_list0, bseparated_list1, bws, bchar, keyword, btag};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;

// Parse variance keyword (in/out)
fn parse_variance(input: &str) -> BResult<&str, Variance> {
    context("variance modifier (expected 'in' or 'out')", alt((
        value(Variance::In, keyword("in")),
        value(Variance::Out, keyword("out")),
    )))(input)
}

// Parse a single type parameter, e.g., "T", "in T", "out T"
fn parse_single_type_parameter(input: &str) -> BResult<&str, TypeParameter> {
    // Try parsing optional variance keyword first
    let (input, variance) = bopt(bws(parse_variance))(input)?;
    let (input, name) = context("type parameter name (expected valid identifier)", parse_identifier)(input)?;
    
    Ok((input, TypeParameter {
        name,
        variance: variance.unwrap_or(Variance::None),
    }))
}

// Parse a list of type parameters enclosed in angle brackets, e.g., "<T, in U>"
pub fn parse_type_parameter_list(input: &str) -> BResult<&str, Vec<TypeParameter>> {
    bdelimited(
        context("type parameter list opening (expected '<')", bws(bchar('<'))),
        bseparated_list1(
            context("type parameter separator (expected ',')", bws(bchar(','))), 
            context("type parameter (expected identifier with optional variance)", bws(parse_single_type_parameter))
        ),
        context("type parameter list closing (expected '>')", bws(bchar('>')))
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
    if let Ok((rest, _)) = keyword("class")(input) {
        return Ok((rest, TypeParameterConstraint::ReferenceType));
    }
    
    // Try "struct" keyword
    if let Ok((rest, _)) = keyword("struct")(input) {
        return Ok((rest, TypeParameterConstraint::ValueType));
    }
    
    // Try "unmanaged" keyword
    if let Ok((rest, _)) = keyword("unmanaged")(input) {
        return Ok((rest, TypeParameterConstraint::Unmanaged));
    }
    
    // Try "notnull" keyword
    if let Ok((rest, _)) = keyword("notnull")(input) {
        return Ok((rest, TypeParameterConstraint::NotNull));
    }
    
    // Try "new()" - this is special because it has parentheses
    if let Ok((rest, _)) = btag("new()")(input) {
        return Ok((rest, TypeParameterConstraint::Constructor));
    }
    
    // Finally try parsing as a type expression
    let (rest, ty) = context("type constraint (expected 'class', 'struct', 'unmanaged', 'notnull', 'new()', or type expression)", parse_type_expression)(input)?;
    Ok((rest, TypeParameterConstraint::SpecificType(ty)))
}

// Parse a 'where' clause for a specific type parameter
fn parse_where_clause(input: &str) -> BResult<&str, TypeParameterConstraintClause> {
    let (input, _) = context("where clause keyword (expected 'where')", bws(keyword("where")))(input)?;
    let (input, name) = context("constrained type parameter name (expected valid identifier)", bws(parse_identifier))(input)?;
    let (input, _) = context("constraint separator (expected ':')", bws(bchar(':')))(input)?;
    let (input, constraints) = bseparated_list0(
        context("constraint separator (expected ',')", bws(bchar(','))), 
        context("type parameter constraint (expected constraint expression)", bws(parse_type_parameter_constraint))
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
