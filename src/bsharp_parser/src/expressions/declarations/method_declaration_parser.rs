use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::parameter_parser::parse_parameter_list;
use crate::parser::expressions::declarations::type_parameter_parser::{
    parse_type_parameter_constraints_clauses, parse_type_parameter_list,
};
use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::errors::BResult;
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom_supreme::tag::complete::tag;
use syntax::declarations::{
    ConstructorDeclaration, ConstructorInitializer, MemberDeclaration, MethodDeclaration,
};
use syntax::expressions::Expression;
use syntax::statements::statement::Statement;

// use nom_supreme::ParserExt for .context()
use crate::trivia::comment_parser::ws;
use nom::Parser;
use nom::character::complete::satisfy;
use nom::combinator::peek;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
/// Parse an optional constructor initializer: ": base(args)" or ": this(args)"
fn parse_constructor_initializer(input: Span) -> BResult<ConstructorInitializer> {
    // Shared arg-list parser for (expr, expr, ...)
    fn parse_arg_list(i: Span) -> BResult<Vec<Expression>> {
        crate::syntax::list_parser::parse_delimited_list0::<
            _, _, _, _, char, char, char, Expression,
        >(
            |j: Span| delimited(ws, tok_l_paren(), ws).parse(j),
            |j: Span| {
                let (r, s) = delimited(ws, parse_expression_spanned, ws).parse(j)?;
                Ok((r, s.node))
            },
            |j: Span| delimited(ws, tok_comma(), ws).parse(j),
            |j: Span| delimited(ws, tok_r_paren(), ws).parse(j),
            false,
            true,
        )
            .parse(i)
    }

    map(
        (
            delimited(ws, tok_colon(), ws),
            alt((
                map(
                    (delimited(ws, tag("base"), ws), parse_arg_list),
                    |(_, args)| ConstructorInitializer::Base(args),
                ),
                map(
                    (delimited(ws, tag("this"), ws), parse_arg_list),
                    |(_, args)| ConstructorInitializer::This(args),
                ),
            )),
        ),
        |(_, init)| init,
    )
    .context("constructor initializer")
    .parse(input)
}

/// Parse a pure method declaration, erroring if the unified parser determines constructor syntax.
pub fn parse_pure_method_declaration(input: Span) -> BResult<MethodDeclaration> {
    use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
    let (rest, member) = parse_member_declaration(input)?;
    if member.has_constructor_syntax() {
        return Err(nom::Err::Failure(ErrorTree::Base {
            location: input,
            kind: BaseErrorKind::Expected(Expectation::Tag("method declaration")),
        }));
    }
    Ok((
        rest,
        MethodDeclaration {
            modifiers: member.modifiers,
            return_type: member
                .return_type
                .expect("Internal syntax error: method path must have return type"),
            name: member.name,
            type_parameters: member.type_parameters,
            parameters: member.parameters,
            constraints: member.constraints,
            body: member.body,
        },
    ))
}

/// Parse a pure constructor declaration, erroring if the unified parser determines method syntax.
pub fn parse_constructor_declaration(input: Span) -> BResult<ConstructorDeclaration> {
    use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
    let (rest, member) = parse_member_declaration(input)?;
    if !member.has_constructor_syntax() {
        return Err(nom::Err::Failure(ErrorTree::Base {
            location: input,
            kind: BaseErrorKind::Expected(Expectation::Tag("constructor declaration")),
        }));
    }
    Ok((
        rest,
        ConstructorDeclaration {
            modifiers: member.modifiers,
            name: member.name,
            parameters: member.parameters,
            body: member.body,
            initializer: member.initializer,
        },
    ))
}

/// Parse member body using unified logic for both methods and constructors
/// Supports: block body ({ ... }), expression body (=> expr;), and abstract/interface (; only)
fn parse_member_body(input: Span) -> BResult<Option<Statement>> {
    alt((
        // Block body: { ... }
        |i| {
            use nom::combinator::cut;
            // Only commit to the block branch if the next significant character is '{'.
            let _ = peek(delimited(ws, satisfy(|c| c == '{'), ws)).parse(i)?;
            let (i, body_block) = cut(delimited(ws, parse_block_statement, ws)).parse(i)?;
            Ok((i, Some(body_block)))
        },
        // Expression body: => expr;
        |i| {
            use nom::combinator::cut;
            let (i, _) = delimited(ws, tok_lambda(), ws).parse(i)?;
            let (i, (expr, _semi)) = cut((
                delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
                delimited(ws, tok_semicolon(), ws),
            ))
            .parse(i)?;
            Ok((i, Some(Statement::Expression(expr))))
        },
        // Abstract/interface member: ; (no body)
        |i| {
            let (i, _) = delimited(ws, tok_semicolon(), ws).parse(i)?;
            Ok((i, None))
        },
    ))
    .context("member body (expected block '{...}', expression body '=> expr;', or ';')")
    .parse(input)
}

/// **Pure Structural Parser**
/// Parse a member declaration (method or constructor) based purely on parser structure.
/// NO semantic validation - all syntactically valid constructs are parsed successfully.
/// The analyzer determines semantic meaning and validates semantic rules.
pub fn parse_member_declaration(input: Span) -> BResult<MemberDeclaration> {
    // 1. Parse modifiers (ALL modifiers allowed syntactically - no semantic checks here)
    let (input, modifiers) = parse_modifiers(input)?;

    // 2. We need to determine if this is a method (has return type) or constructor (no return type).
    // The challenge: `Type Name()` vs `Name()` - both start with an identifier.
    // Strategy: Try to parse as method first, if that fails, try as constructor.

    // Try method parsing first (Type Name(...))
    if let Ok((after_type, return_type)) = delimited(ws, parse_type_expression, ws).parse(input) {
        // Successfully parsed a type, now try to parse an identifier after it
        if let Ok((after_name_candidate, name)) =
            delimited(ws, parse_identifier, ws).parse(after_type)
        {
            // Attempt to parse optional type parameters <T, U> for the method itself
            // This must happen BEFORE checking for the parameter list '('.
            let (after_type_params, type_parameters) =
                match opt(|i| delimited(ws, parse_type_parameter_list, ws).parse(i))
                    .parse(after_name_candidate)
                {
                    Ok((rest, tp)) => (rest, tp),
                    Err(_) => (after_name_candidate, None), // If type param parsing fails, continue without them
                };

            // Try parsing parameters directly; if it succeeds, it's a method path
            if let Ok((input_after_params, parameters)) =
                delimited(ws, parse_parameter_list, ws).parse(after_type_params)
            {
                // 6. Parse constraints (for generic members)
                let (input_after_constraints, constraints) =
                    opt(|i| delimited(ws, parse_type_parameter_constraints_clauses, ws).parse(i))
                        .parse(input_after_params)?;

                // 7. Parse body - REQUIRED for methods (not optional)
                let (input_after_body, body) =
                    delimited(ws, parse_member_body, ws).parse(input_after_constraints)?;

                // Clean up empty vectors to None for cleaner AST
                let final_constraints = match constraints {
                    Some(constraints_vec) if constraints_vec.is_empty() => None,
                    other => other,
                };

                return Ok((
                    input_after_body,
                    MemberDeclaration {
                        modifiers,
                        return_type: Some(return_type),
                        name,
                        type_parameters,
                        parameters,
                        constraints: final_constraints,
                        body,
                        initializer: None,
                    },
                ));
            }
        }
    }

    // If method parsing failed, try constructor parsing: Name(...)
    // This path is also taken if the structure doesn't match Type Name<...>(...) pattern
    let (input_after_mods, name) = delimited(ws, parse_identifier, ws).parse(input)?;
    // 4. Parse type parameters (for generic constructors - though rare, syntactically possible)
    let (input_after_type_params, type_parameters) =
        opt(|i| delimited(ws, parse_type_parameter_list, ws).parse(i)).parse(input_after_mods)?;

    // 5. Parse parameters (must continue after type parameters when present)
    let (input_after_params, parameters) =
        delimited(ws, parse_parameter_list, ws).parse(input_after_type_params)?;

    // 5.1 Optional constructor initializer
    let (input_after_init, initializer) =
        opt(|i| delimited(ws, parse_constructor_initializer, ws).parse(i))
            .parse(input_after_params)?;

    // 6. Parse constraints (for generic members)
    let (input_after_constraints, constraints) =
        opt(|i| delimited(ws, parse_type_parameter_constraints_clauses, ws).parse(i))
            .parse(input_after_init)?;

    // 7. Parse body
    let (final_input, body) = parse_member_body(input_after_constraints)?;

    // 8. Clean up empty vectors to None for cleaner AST
    let final_constraints = match constraints {
        Some(constraints_vec) if constraints_vec.is_empty() => None,
        other => other,
    };

    let final_type_parameters = match type_parameters {
        Some(type_params_vec) if type_params_vec.is_empty() => None,
        other => other,
    };

    // 9. Create unified member declaration (constructor case - no return type)
    Ok((
        final_input,
        MemberDeclaration {
            modifiers,
            return_type: None, // Explicitly None for constructor path
            name,
            type_parameters: final_type_parameters,
            parameters,
            constraints: final_constraints,
            body,
            initializer,
        },
    ))
}
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use crate::tokens::lambda::tok_lambda;
use crate::tokens::separators::{tok_colon, tok_comma, tok_semicolon};
