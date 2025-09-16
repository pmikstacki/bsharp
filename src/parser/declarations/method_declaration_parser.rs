use nom::combinator::opt;
use nom_supreme::tag::complete::tag;

use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::MemberDeclaration;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws};
use crate::parser::declarations::modifier_parser::parse_modifiers;
use crate::parser::declarations::parameter_parser::parse_parameter_list;
use crate::parser::declarations::type_parameter_parser::{parse_type_parameter_list, parse_type_parameter_constraints_clauses};
use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::statements::block_statement_parser::parse_block_statement;
use crate::parser::types::type_parser::parse_type_expression;

/// Parse member body using unified logic for both methods and constructors
/// Supports: block body ({ ... }), expression body (=> expr;), and abstract/interface (; only)
fn parse_member_body(input: &str) -> BResult<&str, Option<Statement>> {
    if input.trim_start().is_empty() {
        use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
        let error_tree = ErrorTree::Base {
            location: input,
            kind: BaseErrorKind::Expected(Expectation::Eof),
        };
        return Err(nom::Err::Error(error_tree));
    }

    let trimmed_input = input.trim_start();
    let first_char = trimmed_input.chars().next().unwrap();

    match first_char {
        // Body block style: { ... } - delegate to block statement syntax
        '{' => {
            match parse_block_statement(input) {  // Use original input, not trimmed
                Ok((rest, body_block)) => {
                    Ok((rest, Some(body_block)))
                },
                Err(e) => {
                    Err(e)
                }
            }
        },
        // Expression body style: => expr;
        '=' => {
            if trimmed_input.len() > 1 && trimmed_input.chars().nth(1) == Some('>') {
                // Parse the => token
                let (input, _) = bws(tag("=>"))(trimmed_input)?;
                
                // Parse the expression using the proper expression syntax
                let (input, expr) = bws(parse_expression)(input)?;
                
                // Parse the semicolon
                let (input, _) = bws(bchar(';'))(input)?;
                
                // Wrap the expression in a Statement::Expression
                Ok((input, Some(Statement::Expression(expr))))
            } else {
                use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
                let error_tree = ErrorTree::Base {
                    location: trimmed_input,
                    kind: BaseErrorKind::Expected(Expectation::Tag("=> for expression body")),
                };
                Err(nom::Err::Error(error_tree))
            }
        },
        // Abstract/interface member style: ; (No body)
        ';' => {
            let remainder = &trimmed_input[1..]; // Skip the semicolon
            Ok((remainder, None))
        },
        // Unexpected character
        _ => {
            use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
            let error_tree = ErrorTree::Base {
                location: trimmed_input,
                kind: BaseErrorKind::Expected(Expectation::Tag("member body ('{}'), expression body ('=>'), or semicolon (';')")),
            };
            Err(nom::Err::Error(error_tree))
        }
    }
}

/// **Pure Structural Parser**
/// Parse a member declaration (method or constructor) based purely on parser structure.
/// NO semantic validation - all syntactically valid constructs are parsed successfully.
/// The analyzer determines semantic meaning and validates semantic rules.
pub fn parse_member_declaration(input: &str) -> BResult<&str, MemberDeclaration> {
    // 1. Parse modifiers (ALL modifiers allowed syntactically - no semantic checks here)
    let (input, modifiers) = parse_modifiers(input)?;

    // 2. We need to determine if this is a method (has return type) or constructor (no return type).
    // The challenge: `Type Name()` vs `Name()` - both start with an identifier.
    // Strategy: Try to parse as method first, if that fails, try as constructor.
    
    // Try method parsing first (Type Name(...))
    if let Ok((after_type, return_type)) = bws(parse_type_expression)(input) {
        // Successfully parsed a type, now try to parse an identifier after it
        if let Ok((after_name_candidate, name)) = bws(parse_identifier)(after_type) {
            // Attempt to parse optional type parameters <T, U> for the method itself
            // This must happen BEFORE checking for the parameter list '('.
            let (after_type_params, type_parameters) = 
                match opt(bws(parse_type_parameter_list))(after_name_candidate) {
                    Ok((rest, tp)) => (rest, tp),
                    Err(_) => (after_name_candidate, None), // If type param parsing fails, continue without them
                };

            // Check if we have parentheses after the name (and optional type_parameters) indicating parameters
            let trimmed = after_type_params.trim_start();
            if trimmed.starts_with('(') {
                // This looks like a method: Type Name<TypeParams>(...)
                let input_for_params = after_type_params; // Use input after type_parameters for parameter list
                                
                // 5. Parse parameters
                let (input_after_params, parameters) = bws(parse_parameter_list)(input_for_params)?;
                
                // 6. Parse constraints (for generic members)
                let (input_after_constraints, constraints) = opt(bws(parse_type_parameter_constraints_clauses))(input_after_params)?;
                
                // 7. Parse body - REQUIRED for methods (not optional)
                let (input_after_body, body) = bws(parse_member_body)(input_after_constraints)?;
                
                // Clean up empty vectors to None for cleaner AST
                let final_constraints = match constraints {
                    Some(constraints_vec) if constraints_vec.is_empty() => None,
                    other => other,
                };
                
                return Ok((input_after_body, MemberDeclaration {
                    modifiers,
                    return_type: Some(return_type),
                    name,
                    type_parameters,
                    parameters,
                    constraints: final_constraints,
                    body
                }));
            }
        }
    }
    
    // If method parsing failed, try constructor parsing: Name(...)
    // This path is also taken if the structure doesn't match Type Name<...>(...) pattern
    let (input_after_mods, name) = bws(parse_identifier)(input)?;
    
    // 4. Parse type parameters (for generic constructors - though rare, syntactically possible)
    let (input_after_type_params, type_parameters) = opt(bws(parse_type_parameter_list))(input_after_mods)?;
    
    // 5. Parse parameters
    let (input_after_params, parameters) = bws(parse_parameter_list)(input_after_type_params)?;
    
    // 6. Parse constraints (for generic members)
    let (input_after_constraints, constraints) = opt(bws(parse_type_parameter_constraints_clauses))(input_after_params)?;
    
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
        },
    ))
}
