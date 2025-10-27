use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::declaration_keywords::{kw_class, kw_struct};
use crate::parser::keywords::linq_query_keywords::kw_where;
use crate::parser::keywords::parameter_modifier_keywords::{kw_in, kw_out, kw_ref};
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use crate::syntax::list_parser::{parse_delimited_list1, parse_list0};
use nom::Parser;
use nom::sequence::delimited;
use nom::{branch::alt, combinator::value};
use nom_supreme::ParserExt;
use syntax::declarations::{TypeParameterConstraint, TypeParameterConstraintClause};
use syntax::types::{TypeParameter, Variance};

// Parse variance keyword (in/out)
fn parse_variance(input: Span) -> BResult<Variance> {
    nom::combinator::map(
        alt((value(Variance::In, kw_in()), value(Variance::Out, kw_out()))),
        |v| v,
    )
    .context("variance modifier")
    .parse(input)
}

/// Public wrapper to parse a single type parameter node
pub fn parse_type_parameter_node(input: Span) -> BResult<TypeParameter> {
    parse_single_type_parameter(input)
}

/// Public wrapper to parse a single type parameter for trait impls
pub fn parse_type_parameter_for_trait_impl(input: Span) -> BResult<TypeParameter> {
    parse_single_type_parameter(input)
}

// Parse a single type parameter, e.g., "T", "in T", "out T"
fn parse_single_type_parameter(input: Span) -> BResult<TypeParameter> {
    // Try parsing optional variance keyword first
    let (input, variance) =
        nom::combinator::opt(|i| delimited(ws, parse_variance, ws).parse(i)).parse(input)?;
    let (input, name) = delimited(ws, parse_identifier, ws)
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
pub fn parse_type_parameter_list(input: Span) -> BResult<Vec<TypeParameter>> {
    parse_delimited_list1::<_, _, _, _, char, char, char, TypeParameter>(
        |i| delimited(ws, tok_lt(), ws).parse(i),
        |i| {
            delimited(ws, parse_single_type_parameter, ws)
                .context("type parameter")
                .parse(i)
        },
        |i| delimited(ws, tok_comma(), ws).parse(i),
        |i| delimited(ws, tok_gt(), ws).parse(i),
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

    // Try "allows ref struct" contextual constraint
    if let Ok((rest, _)) = nom::combinator::map(
        (
            delimited(
                ws,
                crate::parser::keywords::constraint_keywords::kw_allows(),
                ws,
            ),
            delimited(ws, kw_ref(), ws),
            delimited(ws, kw_struct(), ws),
        ),
        |_| (),
    )
    .parse(input)
    {
        return Ok((rest, TypeParameterConstraint::AllowsRefStruct));
    }

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
    if let Ok((rest, _)) = delimited(ws, kw_not(), kw_null()).parse(input) {
        return Ok((rest, TypeParameterConstraint::NotNull));
    }

    // Try "new()" - must consume the parentheses fully
    if let Ok((rest, _)) = nom::combinator::map(
        (
            delimited(ws, kw_new(), ws),
            delimited(ws, crate::tokens::delimiters::tok_l_paren(), ws),
            delimited(ws, crate::tokens::delimiters::tok_r_paren(), ws),
        ),
        |_| (),
    )
    .parse(input)
    {
        return Ok((rest, TypeParameterConstraint::Constructor));
    }

    // Finally try parsing as a type expression
    let (rest, ty) = (|i| parse_type_expression(i))
        .context("type constraint")
        .parse(input)?;
    Ok((rest, TypeParameterConstraint::SpecificType(ty)))
}

/// Public wrapper to parse a single type parameter constraint node
pub fn parse_type_parameter_constraint_node(input: Span) -> BResult<TypeParameterConstraint> {
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
    let (input, _) = delimited(ws, tok_colon(), ws)
        .context("constraint separator  ")
        .parse(input)?;
    let (input, constraints) = parse_list0(
        |i| {
            delimited(ws, parse_type_parameter_constraint, ws)
                .context("type parameter constraint")
                .parse(i)
        },
        |i| delimited(ws, tok_comma(), ws).parse(i),
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
pub fn parse_type_parameter_where_clause(input: Span) -> BResult<TypeParameterConstraintClause> {
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
use syntax::span::Span;


// Local keyword helpers for constraints not covered elsewhere
use crate::keywords::literal_keywords::kw_null;
use crate::keywords::modifier_keywords::{kw_new, kw_unmanaged};
use crate::keywords::pattern_keywords::kw_not;
use crate::tokens::relational::{tok_gt, tok_lt};
use crate::tokens::separators::{tok_colon, tok_comma};
