use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::parameter_parser::parse_parameter_list;
use crate::parser::expressions::declarations::type_parameter_parser::{
    opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses,
};
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::delimited,
};
use nom_supreme::ParserExt;
use syntax::statements::LocalFunctionStatement;
use syntax::statements::statement::Statement;

/// Parse a local function body - similar to method body parsing
fn parse_local_function_body(input: Span) -> BResult<Statement> {
    alt((
        // Block body
        delimited(ws, parse_block_statement, ws),
        // Expression body: => expr;
        |i| {
            let (i, _) = delimited(ws, tok_lambda(), ws).parse(i)?;
            use crate::parser::expressions::primary_expression_parser::parse_expression;
            let (i, _expr) = delimited(ws, parse_expression, ws).parse(i)?;
            let (i, _) = delimited(ws, tok_semicolon(), ws).parse(i)?;
            Ok((i, Statement::Empty))
        },
    ))
    .context("local function body")
    .parse(input)
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
pub fn parse_local_function_statement(input: Span) -> BResult<Statement> {
    map(
        (
            // Optional modifiers (static, async, unsafe)
            parse_modifiers,
            // Return type
            delimited(ws, parse_type_expression, ws),
            // Function name
            delimited(ws, parse_identifier, ws),
            // Optional type parameters
            delimited(ws, opt_parse_type_parameter_list, ws),
            // Parameter list
            delimited(ws, parse_parameter_list, ws),
            // Optional constraints
            opt(delimited(ws, parse_type_parameter_constraints_clauses, ws)),
            // Function body
            parse_local_function_body,
        ),
        |(modifiers, return_type, name, type_parameters, parameters, constraints, body)| {
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
    )
    .context("local function statement")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::lambda::tok_lambda;
use crate::tokens::separators::tok_semicolon;
