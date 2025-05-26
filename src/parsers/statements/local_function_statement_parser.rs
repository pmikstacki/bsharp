use crate::parser::errors::BResult;
use crate::parser::nodes::statements::local_function_statement::LocalFunctionStatement;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::parser_helpers::{bs_context, bws, nom_to_bs};
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::declarations::parameter_parser::parse_parameter_list;
use crate::parsers::declarations::type_parameter_parser::{opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::statements::block_statement_parser::parse_block_statement;
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    combinator::{map, opt},
    sequence::tuple,
};

/// Parse a local function body - similar to method body parsing
fn parse_local_function_body(input: &str) -> BResult<&str, Option<Statement>> {
    let trimmed_input = input.trim_start();

    if trimmed_input.is_empty() {
        return Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(
            trimmed_input,
            crate::parser::errors::CustomErrorKind::Nom(nom::error::ErrorKind::Eof)
        )));
    }

    let first_char = trimmed_input.chars().next().unwrap();

    match first_char {
        // Body block style: { ... }
        '{' => {
            match parse_block_statement(trimmed_input) {
                Ok((rest, body_block)) => Ok((rest, Some(body_block))),
                Err(_) => {
                    // Block statement parsing failed, skip the body
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
                    Ok((remainder, None))
                }
            }
        },
        // Expression body style: => expr; 
        '=' => {
            if trimmed_input.len() > 1 && trimmed_input.chars().nth(1) == Some('>') {
                let after_arrow_input = &trimmed_input[2..];
                // Find the semicolon to consume the rest of the expression body
                if let Some(semicolon_pos) = after_arrow_input.find(';') {
                    let remainder = &after_arrow_input[semicolon_pos + 1..];
                    Ok((remainder, None)) // Simplified: No body in AST for now
                } else {
                    Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(
                        after_arrow_input,
                        crate::parser::errors::CustomErrorKind::Expected("semicolon after expression body")
                    )))
                }
            } else {
                Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(
                    trimmed_input,
                    crate::parser::errors::CustomErrorKind::Expected("=> for expression body")
                )))
            }
        },
        // Unexpected character
        _ => {
            Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(
                trimmed_input,
                crate::parser::errors::CustomErrorKind::Expected("local function body ('{}') or expression body ('=>')")
            )))
        }
    }
}

/// Parse a local function statement
/// 
/// Examples:
/// ```csharp
/// int Add(int a, int b) { return a + b; }
/// static void Helper() { Console.WriteLine("Helper"); }
/// async Task<string> FetchAsync() { return await GetDataAsync(); }
/// T GenericHelper<T>(T value) where T : class { return value; }
/// ```
pub fn parse_local_function_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "local function statement",
        map(
            tuple((
                // Optional modifiers (static, async, unsafe)
                parse_modifiers,
                // Return type
                bws(nom_to_bs(parse_type_expression)),
                // Function name
                bws(nom_to_bs(parse_identifier)),
                // Optional type parameters
                bws(nom_to_bs(opt_parse_type_parameter_list)),
                // Parameter list
                bws(parse_parameter_list),
                // Optional constraints
                opt(bws(parse_type_parameter_constraints_clauses)),
                // Function body
                parse_local_function_body,
            )),
            |(modifiers, return_type, name, type_parameters, parameters, constraints, body)| {
                // Convert Some([]) to None for constraints and type_parameters
                let constraints = match constraints {
                    Some(constraints_vec) if constraints_vec.is_empty() => None,
                    other => other,
                };

                let type_parameters = match type_parameters {
                    Some(type_params_vec) if type_params_vec.is_empty() => None,
                    other => other,
                };

                Statement::LocalFunction(Box::new(LocalFunctionStatement {
                    modifiers,
                    return_type,
                    name,
                    type_parameters,
                    parameters,
                    constraints,
                    body: Box::new(body.unwrap_or(Statement::Empty)),
                }))
            },
        ),
    )(input)
} 