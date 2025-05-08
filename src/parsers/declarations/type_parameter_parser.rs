use nom::{
    branch::alt,
    character::complete::{char as nom_char, multispace0},
    bytes::complete::tag,
    combinator::{value, opt, map},
    multi::{many0, separated_list0},
    sequence::delimited,
    IResult,
};
use crate::parser::nodes::types::{TypeParameter, Variance};
use crate::parser::nodes::declarations::{TypeParameterConstraint, TypeParameterConstraintClause};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

// Helper for optional whitespace
fn ws<'a, F: 'a, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(multispace0, inner, multispace0)
}

// Parse variance keyword (in/out)
fn parse_variance(input: &str) -> IResult<&str, Variance> {
    alt((
        value(Variance::In, tag("in")),
        value(Variance::Out, tag("out")),
    ))(input)
}

// Parse a single type parameter, e.g., "T", "in T", "out T"
fn parse_single_type_parameter(input: &str) -> IResult<&str, TypeParameter> {
    // Try parsing optional variance keyword first
    let (input, variance) = opt(ws(parse_variance))(input)?;
    let (input, name) = parse_identifier(input)?;
    
    Ok((input, TypeParameter {
        name,
        variance: variance.unwrap_or(Variance::None),
        // constraints: vec![], // Constraint parsing not implemented yet
    }))
}

// Parse a list of type parameters enclosed in angle brackets, e.g., "<T, in U>"
pub fn parse_type_parameter_list(input: &str) -> IResult<&str, Vec<TypeParameter>> {
    delimited(
        ws(nom_char('<')),
        separated_list0(ws(nom_char(',')), ws(parse_single_type_parameter)),
        ws(nom_char('>'))
    )(input)
}

// Optional type parameter list (returns empty Vec if no list)
pub fn opt_parse_type_parameter_list(input: &str) -> IResult<&str, Vec<TypeParameter>> {
    opt(parse_type_parameter_list)(input)
    .map(|(rest, opt_list)| (rest, opt_list.unwrap_or_default()))
}

// Parse a single constraint (e.g., 'class', 'struct', 'new()', 'BaseType', 'T')
fn parse_type_parameter_constraint(input: &str) -> IResult<&str, TypeParameterConstraint> {
    // For now, let's just handle simple type constraints (identifiers)
    // Need to add parsing for 'class', 'struct', 'new()'
    alt((
        // Use map for type expression
        map(parse_type_expression, TypeParameterConstraint::SpecificType),
        value(TypeParameterConstraint::ReferenceType, tag("class")),
        value(TypeParameterConstraint::ValueType, tag("struct")),
        value(TypeParameterConstraint::Unmanaged, tag("unmanaged")),
        value(TypeParameterConstraint::NotNull, tag("notnull")),
        value(TypeParameterConstraint::Constructor, tag("new()")) 
    ))(input)
}

// Parse a 'where' clause for a specific type parameter
fn parse_where_clause(input: &str) -> IResult<&str, TypeParameterConstraintClause> {
    let (input, _) = ws(tag("where"))(input)?;
    let (input, name) = ws(parse_identifier)(input)?;
    let (input, _) = ws(nom_char(':'))(input)?;
    let (input, constraints) = separated_list0(
        ws(nom_char(',')), 
        ws(parse_type_parameter_constraint)
    )(input)?;

    Ok((input, TypeParameterConstraintClause { 
        type_param: name, 
        constraints,
        _phantom: std::marker::PhantomData 
    }))
}

// Parse zero or more 'where' clauses
pub fn parse_type_parameter_constraints_clauses(input: &str) -> IResult<&str, Vec<TypeParameterConstraintClause>> {
    many0(ws(parse_where_clause))(input)
}
