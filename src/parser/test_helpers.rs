// src/parser/test_helpers.rs
use nom::IResult;
use nom::combinator::all_consuming;
use crate::parser::errors::{BSharpParseError, BResult};
use crate::parsers::statement_parser::parse_statement;
use crate::parser::nodes::statements::statement::Statement;

/// Ensures the entire input is parsed by the given parser.
/// Returns an IResult with BSharpParseError as the error type.
pub fn parse_all<'a, O, P>(parser: P, input: &'a str) -> IResult<&'a str, O, BSharpParseError<&'a str>>
where
    P: Fn(&'a str) -> IResult<&'a str, O, BSharpParseError<&'a str>>,
{
    all_consuming(parser)(input)
}

/// Parses a single statement using the `parse_statement` parser and ensures all input is consumed.
pub fn parse_all_statement(input: &str) -> IResult<&str, Statement, BSharpParseError<&str>> {
    parse_all(parse_statement, input)
}

/// Helper function to unwrap a parser result or panic on error.
/// Used in tests to simplify result handling.
pub fn parse_input_unwrap<T>(parser_result: BResult<&str, T>) -> (&str, T) {
    match parser_result {
        Ok(result) => result,
        Err(e) => panic!("Parse error: {:?}", e),
    }
}

// If parse_input_unwrap is also intended to be here, it can be added.
// For now, focusing on making parse_all and parse_all_statement available.
