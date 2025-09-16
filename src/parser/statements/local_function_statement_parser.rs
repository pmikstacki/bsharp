use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::local_function_statement::LocalFunctionStatement;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{context, bws};
use crate::parser::declarations::modifier_parser::parse_modifiers;
use crate::parser::declarations::parameter_parser::parse_parameter_list;
use crate::parser::declarations::type_parameter_parser::{opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::statements::block_statement_parser::parse_block_statement;
use crate::parser::types::type_parser::parse_type_expression;

use nom::{
    combinator::{map, opt},
    sequence::tuple,
};
use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};

/// Parse a local function body - similar to method body parsing
fn parse_local_function_body(input: &str) -> BResult<&str, Statement> {
    let trimmed_input = input.trim_start();

    if trimmed_input.is_empty() {
        let error_tree = ErrorTree::Base {
            location: trimmed_input,
            kind: BaseErrorKind::Expected(Expectation::Eof),
        };
        return Err(nom::Err::Error(error_tree));
    }

    let first_char = trimmed_input.chars().next().unwrap();

    match first_char {
        // Body block style: { ... }
        '{' => {
            match parse_block_statement(trimmed_input) {
                Ok((rest, body_block)) => Ok((rest, body_block)),
                Err(e) => Err(e),
            }
        },
        // Expression body style: => expr; 
        '=' => {
            if trimmed_input.len() > 1 && trimmed_input.chars().nth(1) == Some('>') {
                let after_arrow_input = &trimmed_input[2..];
                // Find the semicolon to consume the rest of the expression body
                if let Some(semicolon_pos) = after_arrow_input.find(';') {
                    let remainder = &after_arrow_input[semicolon_pos + 1..];
                    // For now, just return an empty statement as a placeholder
                    Ok((remainder, Statement::Empty))
                } else {
                    let error_tree = ErrorTree::Base {
                        location: after_arrow_input,
                        kind: BaseErrorKind::Expected(Expectation::Tag("local function expression body (expected ';' after expression)")),
                    };
                    Err(nom::Err::Error(error_tree))
                }
            } else {
                let error_tree = ErrorTree::Base {
                    location: trimmed_input,
                    kind: BaseErrorKind::Expected(Expectation::Tag("local function expression body (expected '=>')")),
                };
                Err(nom::Err::Error(error_tree))
            }
        },
        // Unexpected character
        _ => {
            let error_tree = ErrorTree::Base {
                location: trimmed_input,
                kind: BaseErrorKind::Expected(Expectation::Tag("local function body (expected '{' for block or '=>' for expression body)")),
            };
            Err(nom::Err::Error(error_tree))
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
    context(
        "local function statement (expected modifiers, return type, name, parameters, and body)",
        map(
            tuple((
                // Optional modifiers (static, async, unsafe)
                parse_modifiers,
                // Return type
                bws(parse_type_expression),
                // Function name
                bws(parse_identifier),
                // Optional type parameters
                bws(opt_parse_type_parameter_list),
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
                    body: Box::new(body),
                }))
            },
        ),
    )(input)
} 