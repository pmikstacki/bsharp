use crate::parser::errors::{BResult, BSharpParseError, CustomErrorKind};
use crate::parser::nodes::declarations::MethodDeclaration;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parsers::declarations::parameter_parser::parse_parameter_list;
use crate::parsers::declarations::type_parameter_parser::{opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::statements::block_statement_parser::parse_block_statement;
use crate::parsers::types::type_parser::parse_type_expression;
use nom::combinator::opt;
use nom::error::ErrorKind;

// Updated to parse into Option<Statement>
fn parse_method_body(input: &str) -> BResult<&str, Option<Statement>> {
    let trimmed_input = input.trim_start();

    if trimmed_input.is_empty() {
        return Err(nom::Err::Error(BSharpParseError::new(
            trimmed_input,
            CustomErrorKind::Nom(ErrorKind::Eof)
        )));
    }

    let first_char = trimmed_input.chars().next().unwrap();

    match first_char {
        // Body block style: { ... }
        '{' => {
            // Try to parse the block statement, but if it fails, still parse the method structure
            match parse_block_statement(trimmed_input) {
                Ok((rest, body_block)) => Ok((rest, Some(body_block))),
                Err(_) => {
                    // Block statement parsing failed, but we can still identify the method structure
                    // Find the matching closing brace to consume the method body
                    println!("[DEBUG] Block statement parsing failed, finding matching brace");
                    let mut brace_count = 0;
                    let char_indices = trimmed_input.char_indices();
                    let mut end_pos = trimmed_input.len();
                    
                    for (i, ch) in char_indices {
                        match ch {
                            '{' => brace_count += 1,
                            '}' => {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    end_pos = i + 1;
                                    break;
                                }
                            },
                            _ => {}
                        }
                    }
                    
                    let remainder = &trimmed_input[end_pos..];
                    println!("[DEBUG] Found matching brace, remainder: {:?}", remainder.chars().take(50).collect::<String>());
                    Ok((remainder, None)) // Return None body but continue parsing
                }
            }
        },
        // Expression body style: => expr; (Simplified: treated as no body for now)
        '=' => {
            if trimmed_input.len() > 1 && trimmed_input.chars().nth(1) == Some('>') {
                let after_arrow_input = &trimmed_input[2..];
                // Find the semicolon to consume the rest of the expression body
                if let Some(semicolon_pos) = after_arrow_input.find(';') {
                    let remainder = &after_arrow_input[semicolon_pos + 1..];
                    Ok((remainder, None)) // Simplified: No body in AST for now
                } else {
                    Err(nom::Err::Error(BSharpParseError::new(
                        after_arrow_input,
                        CustomErrorKind::Expected("semicolon after expression body")
                    )))
                }
            } else {
                Err(nom::Err::Error(BSharpParseError::new(
                    trimmed_input,
                    CustomErrorKind::Expected("=> for expression body")
                )))
            }
        },
        // Abstract method style: ; (No body)
        ';' => {
            let remainder = &trimmed_input[1..]; // Skip the semicolon
            Ok((remainder, None))
        },
        // Unexpected character
        _ => {
            Err(nom::Err::Error(BSharpParseError::new(
                trimmed_input,
                CustomErrorKind::Expected("method body ('{}'), expression body ('=>'), or semicolon (';')")
            )))
        }
    }
}

// Parse a method declaration
pub fn parse_method_declaration(input: &str) -> BResult<&str, MethodDeclaration> {
    let (input, modifiers) = parse_modifiers_for_decl_type(input, "method")?;
    
    let (input, return_type) = bws(parse_type_expression)(input)?;
    let (input, name) = bws(parse_identifier)(input)?;
    
    let (input, type_parameters) = bws(nom_to_bs(opt_parse_type_parameter_list))(input)?;
    
    let (input, parameters) = bws(nom_to_bs(parse_parameter_list))(input)?;
    
    let (input, constraints) = opt(bws(nom_to_bs(parse_type_parameter_constraints_clauses)))(input)?;
    
    let (input, body) = parse_method_body(input)?;

    // Convert Some([]) to None for constraints and type_parameters
    let constraints = match constraints {
        Some(constraints_vec) if constraints_vec.is_empty() => None,
        other => other,
    };

    let type_parameters = match type_parameters {
        Some(type_params_vec) if type_params_vec.is_empty() => None,
        other => other,
    };

    Ok((
        input,
        MethodDeclaration {
            modifiers,
            return_type,
            name,
            type_parameters,
            parameters,
            constraints,
            body,
        },
    ))
}
