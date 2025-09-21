use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::parameter_parser::parse_parameter_list;
use crate::parser::expressions::declarations::type_parameter_parser::{
    opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses,
};
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::local_function_statement::LocalFunctionStatement;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws, context};

use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::{preceded, tuple},
};
use nom_supreme::tag::complete::tag;

/// Parse a local function body - similar to method body parsing
fn parse_local_function_body(input: &str) -> BResult<&str, Statement> {
    context(
        "local function body (expected '{...}' or '=> expr;')",
        alt((
            // Block body
            |i| {
                let (i, body) = bws(parse_block_statement)(i)?;
                Ok((i, body))
            },
            // Expression body: => expr;
            |i| {
                let (i, _) = preceded(bws(tag("=>")), bws(|j| Ok((j, ()))))(i)?;
                // Consume the expression using the real expression parser but discard it
                use crate::parser::expressions::primary_expression_parser::parse_expression;
                let (i, _expr) = bws(parse_expression)(i)?;
                let (i, _) = bws(bchar(';'))(i)?;
                Ok((i, Statement::Empty))
            },
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_block_body() {
        let src = "{ return; }";
        let (rest, stmt) = parse_local_function_body(src).expect("parse");
        assert!(rest.trim().is_empty());
        assert!(matches!(stmt, Statement::Block(_)));
    }

    #[test]
    fn parses_expression_body() {
        let src = "=> x;";
        let (rest, stmt) = parse_local_function_body(src).expect("parse");
        assert!(rest.trim().is_empty());
        assert!(matches!(stmt, Statement::Empty));
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
