use nom::{branch::alt, combinator::value};

use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::linq_query_keywords::kw_where;
use crate::parser::keywords::parameter_modifier_keywords::{kw_in, kw_out};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{TypeParameterConstraint, TypeParameterConstraintClause};
use crate::syntax::nodes::types::{TypeParameter, Variance};
use crate::syntax::parser_helpers::{
    bchar, bdelimited, bopt, bseparated_list0, bseparated_list1, btag, bws, context, keyword,
};

// Parse variance keyword (in/out)
fn parse_variance(input: &str) -> BResult<&str, Variance> {
    context(
        "variance modifier (expected 'in' or 'out')",
        alt((value(Variance::In, kw_in()), value(Variance::Out, kw_out()))),
    )(input)
}

/// Public wrapper to parse a single type parameter node
pub fn parse_type_parameter_node(input: &str) -> BResult<&str, TypeParameter> {
    parse_single_type_parameter(input)
}

/// Public wrapper to parse a single type parameter for trait impls
pub fn parse_type_parameter_for_trait_impl(input: &str) -> BResult<&str, TypeParameter> {
    parse_single_type_parameter(input)
}

// Parse a single type parameter, e.g., "T", "in T", "out T"
fn parse_single_type_parameter(input: &str) -> BResult<&str, TypeParameter> {
    // Try parsing optional variance keyword first
    let (input, variance) = bopt(bws(parse_variance))(input)?;
    let (input, name) = context(
        "type parameter name (expected valid identifier)",
        parse_identifier,
    )(input)?;

    Ok((
        input,
        TypeParameter {
            name,
            variance: variance.unwrap_or(Variance::None),
        },
    ))
}

// Parse a list of type parameters enclosed in angle brackets, e.g., "<T, in U>"
pub fn parse_type_parameter_list(input: &str) -> BResult<&str, Vec<TypeParameter>> {
    bdelimited(
        context(
            "type parameter list opening (expected '<')",
            bws(bchar('<')),
        ),
        bseparated_list1(
            context("type parameter separator (expected ',')", bws(bchar(','))),
            context(
                "type parameter (expected identifier with optional variance)",
                bws(parse_single_type_parameter),
            ),
        ),
        context(
            "type parameter list closing (expected '>')",
            nom::combinator::cut(bws(bchar('>'))),
        ),
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
    let (rest, ty) = context(
        "type constraint (expected 'class', 'struct', 'unmanaged', 'notnull', 'new()', or type expression)",
        parse_type_expression,
    )(input)?;
    Ok((rest, TypeParameterConstraint::SpecificType(ty)))
}

/// Public wrapper to parse a single type parameter constraint node
pub fn parse_type_parameter_constraint_node(input: &str) -> BResult<&str, TypeParameterConstraint> {
    parse_type_parameter_constraint(input)
}

// Parse a 'where' clause for a specific type parameter
fn parse_where_clause(input: &str) -> BResult<&str, TypeParameterConstraintClause> {
    let (input, _) = context("where clause keyword (expected 'where')", bws(kw_where()))(input)?;
    let (input, name) = context(
        "constrained type parameter name (expected valid identifier)",
        bws(parse_identifier),
    )(input)?;
    let (input, _) = context("constraint separator (expected ':')", bws(bchar(':')))(input)?;
    let (input, constraints) = bseparated_list0(
        context("constraint separator (expected ',')", bws(bchar(','))),
        context(
            "type parameter constraint (expected constraint expression)",
            bws(parse_type_parameter_constraint),
        ),
    )(input)?;

    Ok((
        input,
        TypeParameterConstraintClause {
            type_param: name,
            constraints,
        },
    ))
}

/// Public wrapper to parse a single `where` clause (TypeParameterConstraintClause)
pub fn parse_type_parameter_where_clause(
    input: &str,
) -> BResult<&str, TypeParameterConstraintClause> {
    parse_where_clause(input)
}

// Parse zero or more 'where' clauses
pub fn parse_type_parameter_constraints_clauses(
    input: &str,
) -> BResult<&str, Vec<TypeParameterConstraintClause>> {
    // Use many0 from nom but wrapped for BResult
    let mut clauses = Vec::new();
    let mut current_input = input;

    while let Ok((rest, clause)) = bws(parse_where_clause)(current_input) {
        clauses.push(clause);
        current_input = rest;
    }

    Ok((current_input, clauses))
}
