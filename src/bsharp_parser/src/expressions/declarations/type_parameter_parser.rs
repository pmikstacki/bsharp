use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::declaration_keywords::{kw_class, kw_struct};
use crate::parser::keywords::linq_query_keywords::kw_where;
use crate::parser::keywords::parameter_modifier_keywords::{kw_in, kw_out};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::{branch::alt, combinator::value};
use nom::character::complete::char as nom_char;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;
use syntax::declarations::{TypeParameterConstraint, TypeParameterConstraintClause};
use syntax::types::{TypeParameter, Variance};
use crate::syntax::list_parser::{parse_delimited_list1, parse_list0};

// Parse variance keyword (in/out)
fn parse_variance(input: Span) -> BResult<Variance> {
    nom::combinator::map(alt((value(Variance::In, kw_in()), value(Variance::Out, kw_out()))), |v| v)
        .context("variance modifier")
        .parse(input)
}

/// Public wrapper to parse a single type parameter node
pub fn parse_type_parameter_node<'a>(input: Span<'a>) -> BResult<'a, TypeParameter> {
    parse_single_type_parameter(input)
}

/// Public wrapper to parse a single type parameter for trait impls
pub fn parse_type_parameter_for_trait_impl<'a>(input: Span<'a>) -> BResult<'a, TypeParameter> {
    parse_single_type_parameter(input)
}

// Parse a single type parameter, e.g., "T", "in T", "out T"
fn parse_single_type_parameter(input: Span) -> BResult<TypeParameter> {
    // Try parsing optional variance keyword first
    let (input, variance) = nom::combinator::opt(|i| delimited(ws, parse_variance, ws).parse(i))
        .parse(input)?;
    let (input, name) = (|i| parse_identifier(i))
        .context("type parameter name")
        .parse(input)?;

    Ok((
        input,
        TypeParameter {
            name,
            variance: variance.unwrap_or(Variance::None),
        },
    ))
}

// Parse a list of type parameters enclosed in angle brackets, e.g., "<T, in U>"
pub fn parse_type_parameter_list<'a>(input: Span<'a>) -> BResult<'a, Vec<TypeParameter>> {
    parse_delimited_list1::<_, _, _, _, char, TypeParameter, char, char, TypeParameter>(
        |i| delimited(ws, nom_char('<'), ws).parse(i),
        |i| delimited(ws, parse_single_type_parameter, ws)
            .context("type parameter")
            .parse(i),
        |i| delimited(ws, nom_char(','), ws).parse(i),
        |i| delimited(ws, nom_char('>'), ws).parse(i),
        false,
        true,
    )
    .context("type parameter list opening")
    .parse(input)
}

// Optional type parameter list (returns None if no list, Some(Vec) otherwise)
pub fn opt_parse_type_parameter_list(input: Span) -> BResult<Option<Vec<TypeParameter>>> {
    // First use the opt combinator to try parsing type parameters
    let (rest, opt_params) = nom::combinator::opt(parse_type_parameter_list).parse(input)?;

    // Convert Some([]) to None, matching test expectations
    let result = match opt_params {
        Some(params) if params.is_empty() => None,
        other => other,
    };

    Ok((rest, result))
}

// Parse a single constraint (e.g., 'class', 'struct', 'new()', 'BaseType', 'T')
fn parse_type_parameter_constraint(input: Span) -> BResult<TypeParameterConstraint> {
    // Try each constraint type individually

    // Try "class" keyword
    if let Ok((rest, _)) = kw_class().parse(input) {
        return Ok((rest, TypeParameterConstraint::ReferenceType));
    }

    // Try "struct" keyword
    if let Ok((rest, _)) = kw_struct().parse(input) {
        return Ok((rest, TypeParameterConstraint::ValueType));
    }

    // Try "unmanaged" keyword
    if let Ok((rest, _)) = kw_unmanaged().parse(input) {
        return Ok((rest, TypeParameterConstraint::Unmanaged));
    }

    // Try "notnull" keyword
    if let Ok((rest, _)) = kw_notnull().parse(input) {
        return Ok((rest, TypeParameterConstraint::NotNull));
    }

    // Try "new()" - this is special because it has parentheses
    if let Ok((rest, _)) = delimited(ws, tag("new()"), ws).parse(input) {
        return Ok((rest, TypeParameterConstraint::Constructor));
    }

    // Finally try parsing as a type expression
    let (rest, ty) = (|i| parse_type_expression(i))
        .context("type constraint")
        .parse(input)?;
    Ok((rest, TypeParameterConstraint::SpecificType(ty)))
}

/// Public wrapper to parse a single type parameter constraint node
pub fn parse_type_parameter_constraint_node<'a>(input: Span<'a>) -> BResult<'a, TypeParameterConstraint> {
    parse_type_parameter_constraint(input)
}

// Parse a 'where' clause for a specific type parameter
fn parse_where_clause(input: Span) -> BResult<TypeParameterConstraintClause> {
    let (input, _) = delimited(ws, kw_where(), ws)
        .context("where clause keyword")
        .parse(input)?;
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("constrained type parameter name")
        .parse(input)?;
    let (input, _) = delimited(ws, nom_char(':'), ws)
        .context("constraint separator  ")
        .parse(input)?;
    let (input, constraints) = parse_list0(
        |i| delimited(ws, parse_type_parameter_constraint, ws)
            .context("type parameter constraint")
            .parse(i),
        |i| delimited(ws, nom_char(','), ws).parse(i),
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
    input: Span,
) -> BResult<TypeParameterConstraintClause> {
    parse_where_clause(input)
}

// Parse zero or more 'where' clauses
pub fn parse_type_parameter_constraints_clauses(
    input: Span,
) -> BResult<Vec<TypeParameterConstraintClause>> {
    // Use many0 from nom but wrapped for BResult
    let mut clauses = Vec::new();
    let mut current_input = input;

    while let Ok((rest, clause)) = delimited(ws, parse_where_clause, ws).parse(current_input) {
        clauses.push(clause);
        current_input = rest;
    }

    Ok((current_input, clauses))
}
use crate::syntax::span::Span;

// Local keyword helpers for constraints not covered elsewhere
use crate::define_keyword_pair;
define_keyword_pair!(kw_unmanaged, peek_unmanaged, "unmanaged");
define_keyword_pair!(kw_notnull, peek_notnull, "notnull");
