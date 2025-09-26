// We will handle paren-or-tuple disambiguation locally to avoid early commitment
use crate::parser::expressions::default_expression_parser::parse_default_expression;
use crate::parser::expressions::lambda_expression_parser::parse_lambda_or_anonymous_method;
use crate::parser::expressions::literal_parser::parse_literal;
use crate::parser::expressions::nameof_expression_parser::parse_nameof_expression;
use crate::parser::expressions::new_expression_parser::parse_new_expression;
use crate::parser::expressions::paren_tuple_primary_parser::parse_paren_or_tuple_primary;
use crate::parser::expressions::query_expression_parser::parse_query_expression;
use crate::parser::expressions::stackalloc_expression_parser::parse_stackalloc_expression;
use crate::parser::expressions::switch_expression_parser::parse_switch_expression;
use crate::parser::expressions::throw_expression_parser::parse_throw_expression;
use crate::parser::expressions::collection_expression_parser::parse_collection_expression_or_brackets;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::parse_delimited_list0;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{bws, context};
use crate::parser::keywords::contextual_misc_keywords::{kw_this, kw_base};

use crate::parser::expressions::assignment_expression_parser;
use nom::{
    branch::alt,
    combinator::{map, peek}
    ,
};
/// Parse any expression - the main entry point for expression parsing
pub fn parse_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "expression",
        bws(assignment_expression_parser::parse_assignment_expression_or_higher),
    )(input)
}

pub fn parse_primary_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "primary expression",
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
    )(input)
}

/// Parse a generic type name as a primary expression for static member access.
/// Example: `Result<User>` (treated as a name for `Result` so that `Result<User>.Success(...)` parses)
fn parse_generic_name_primary(input: &str) -> BResult<&str, Expression> {
    use nom::character::complete::char as nom_char;
    use nom::sequence::tuple;

    // Parse Identifier '<' type-args '>' and ensure a '.' follows (without consuming it)
    map(
        tuple((
            bws(parse_identifier),
            parse_delimited_list0::<
                _,
                _,
                _,
                _,
                char,
                crate::syntax::nodes::types::Type,
                char,
                char,
                crate::syntax::nodes::types::Type,
            >(
                nom_char('<'),
                parse_type_expression,
                nom_char(','),
                nom_char('>'),
                false,
                false,
            ),
            // Require a '.' next (static member access), but don't consume it
            peek(bws(nom_char('.'))),
        )),
        |(id, _, _)| Expression::Variable(Identifier { name: id.name }),
    )(input)
}
