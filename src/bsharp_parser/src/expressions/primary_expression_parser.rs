// We will handle paren-or-tuple disambiguation locally to avoid early commitment
use crate::parser::expressions::collection_expression_parser::parse_collection_expression_or_brackets;
use crate::parser::expressions::default_expression_parser::parse_default_expression;
use crate::parser::expressions::lambda_expression_parser::parse_lambda_or_anonymous_method;
use crate::parser::expressions::literal_parser::{parse_literal, parse_literal_spanned};
use crate::parser::expressions::nameof_expression_parser::parse_nameof_expression;
use crate::parser::expressions::new_expression_parser::parse_new_expression;
use crate::parser::expressions::paren_tuple_primary_parser::parse_paren_or_tuple_primary;
use crate::parser::expressions::query_expression_parser::parse_query_expression;
use crate::parser::expressions::stackalloc_expression_parser::parse_stackalloc_expression;
use crate::parser::expressions::switch_expression_parser::parse_switch_expression;
use crate::parser::expressions::throw_expression_parser::parse_throw_expression;
use crate::parser::identifier_parser::{parse_identifier, parse_identifier_spanned};
use crate::parser::keywords::contextual_misc_keywords::{kw_base, kw_this};
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom_supreme::ParserExt;

use crate::parser::expressions::assignment_expression_parser;
use crate::syntax::list_parser::parse_delimited_list0;
use syntax::span::Span;
use crate::span::Spanned;
use crate::span_ext::ParserExt as _;

use crate::tokens::relational::{tok_gt, tok_lt};
use crate::tokens::separators::tok_comma;
use nom::{
    Parser,
    branch::alt,
    combinator::{map, peek},
    sequence::delimited,
};
use syntax::expressions::Expression;
use syntax::types::Type;

#[deprecated(note = "Use parse_expression_spanned or Parsable<'a>::parse -> Spanned<_>")]
pub fn parse_expression(input: Span) -> BResult<Expression> {
    delimited(
        ws,
        assignment_expression_parser::parse_assignment_expression_or_higher,
        ws,
    )
    .context("expression")
    .parse(input)
}

pub fn parse_primary_literal_spanned(input: Span) -> BResult<Spanned<Expression>> {
    let (rest, s) = parse_literal_spanned(input)?;
    Ok((rest, s.map(Expression::Literal)))
}

pub fn parse_primary_variable_spanned(input: Span) -> BResult<Spanned<Expression>> {
    let (rest, s) = parse_identifier_spanned(input)?;
    Ok((rest, s.map(Expression::Variable)))
}

pub fn parse_paren_or_tuple_primary_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_paren_or_tuple_primary(i)).spanned().parse(input)
}

pub fn parse_collection_expression_or_brackets_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_collection_expression_or_brackets(i)).spanned().parse(input)
}

pub fn parse_generic_name_primary_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_generic_name_primary(i)).spanned().parse(input)
}

pub fn parse_query_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_query_expression(i)).spanned().parse(input)
}

pub fn parse_switch_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_switch_expression(i)).spanned().parse(input)
}

pub fn parse_throw_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_throw_expression(i)).spanned().parse(input)
}

pub fn parse_nameof_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_nameof_expression(i)).spanned().parse(input)
}

pub fn parse_default_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_default_expression(i)).spanned().parse(input)
}

pub fn parse_new_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_new_expression(i)).spanned().parse(input)
}

pub fn parse_lambda_or_anonymous_method_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_lambda_or_anonymous_method(i)).spanned().parse(input)
}

pub fn parse_stackalloc_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    (|i| parse_stackalloc_expression(i)).spanned().parse(input)
}

pub fn parse_this_spanned(input: Span) -> BResult<Spanned<Expression>> {
    let (rest, s) = kw_this().spanned().parse(input)?;
    Ok((rest, s.map(|_| Expression::This)))
}

pub fn parse_base_spanned(input: Span) -> BResult<Spanned<Expression>> {
    let (rest, s) = kw_base().spanned().parse(input)?;
    Ok((rest, s.map(|_| Expression::Base)))
}

#[allow(deprecated)]
pub fn parse_primary_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    use nom::branch::alt;
    let core = alt((
        // Keep order consistent with non-spanned primary parser
        parse_paren_or_tuple_primary_spanned,
        parse_collection_expression_or_brackets_spanned,
        parse_generic_name_primary_spanned,
        parse_query_expression_spanned,
        parse_switch_expression_spanned,
        parse_throw_expression_spanned,
        parse_nameof_expression_spanned,
        parse_default_expression_spanned,
        parse_primary_literal_spanned,
        parse_this_spanned,
        parse_base_spanned,
        parse_new_expression_spanned,
        parse_lambda_or_anonymous_method_spanned,
        parse_primary_variable_spanned,
        parse_stackalloc_expression_spanned,
        // Fallback
        (|i| parse_primary_expression(i)).spanned(),
    ));
    nom::sequence::delimited(ws, core, ws).parse(input)
}

pub fn parse_expression_spanned(input: Span) -> BResult<Spanned<Expression>> {
    use crate::parser::expressions::assignment_expression_parser::parse_assignment_expression_or_higher_spanned as core_spanned;
    delimited(ws, |i| core_spanned(i), ws).parse(input)
}

#[deprecated(note = "Use parse_primary_expression_spanned or Parsable<'a>::parse -> Spanned<_>")]
pub fn parse_primary_expression(input: Span) -> BResult<Expression> {
    map(
        alt((
            // Parenthesized or tuple must be tried very early to avoid other branches
            // (like switch basic expression) consuming '(' with a cut
            parse_paren_or_tuple_primary,
            // Collection expressions starting with '[' must be before variable/member/indexing
            parse_collection_expression_or_brackets,
            // Generic type name primary (e.g., Result<User>) used for static member access
            parse_generic_name_primary,
            // LINQ Query expressions - must come before variables/identifiers
            parse_query_expression,
            // Switch expressions - must come before variables/identifiers
            parse_switch_expression,
            // Throw expressions - must come before variables/identifiers
            parse_throw_expression,
            // Nameof expressions - must come before variables/identifiers
            parse_nameof_expression,
            // Default expressions - must come before variables/identifiers
            parse_default_expression,
            // Literals
            map(parse_literal, Expression::Literal),
            // this keyword
            map(kw_this(), |_| Expression::This),
            // base keyword
            map(kw_base(), |_| Expression::Base),
            // New expressions (includes anonymous object creation)
            parse_new_expression,
            // Lambda expressions
            parse_lambda_or_anonymous_method,
            // Variables/identifiers
            map(parse_identifier, Expression::Variable),
            // Stackalloc expressions
            parse_stackalloc_expression,
        )),
        |v| v,
    )
    .context("primary expression")
    .parse(input)
}

/// Parse a generic type name as a primary expression for static member access.
/// Example: `Result<User>` (treated as a name for `Result` so that `Result<User>.Success(...)` parses)
fn parse_generic_name_primary(input: Span) -> BResult<Expression> {
    use nom::character::complete::char as nom_char;

    // Parse Identifier '<' type-args '>' and ensure a '.' follows (without consuming it)
    map(
        (
            delimited(ws, parse_identifier, ws),
            parse_delimited_list0::<_, _, _, _, char, char, char, Type>(
                tok_lt(),
                parse_type_expression,
                tok_comma(),
                tok_gt(),
                false,
                false,
            ),
            // Require a '.' next (static member access), but don't consume it
            peek(delimited(ws, nom_char('.'), ws)),
        ),
        |(id, _, _)| Expression::Variable(id),
    )
    .parse(input)
}
