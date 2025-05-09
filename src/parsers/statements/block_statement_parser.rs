use crate::parser::nodes::statements::statement::Statement;
// Parser for block statements { ... }

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::multispace1;
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, tuple};

use crate::parser::errors::BResult;
use crate::parser::parser_helpers::{bchar, bs_context}; 

use crate::parsers::statement_parser::parse_statement_ws;

// Function to extract statements from a Block statement, if that's what it is
// This is a helper for tests and other code that expects a Vec<Statement>
pub fn extract_statements_from_block<'a>(statement: Statement<'a>) -> Vec<Statement<'a>> {
    match statement {
        Statement::Block(statements) => statements,
        _ => vec![statement], // If it's not a block, wrap it in a Vec as a fallback
    }
}

// Parse a C-style block comment /* ... */
fn parse_block_comment(input: &str) -> BResult<&str, &str> {
    bs_context(
        "block comment",
        recognize(tuple((
            tag("/*"),
            take_until("*/"),
            tag("*/")
        )))
    )(input)
}

// Parse any whitespace or comments
fn parse_ws_comments(input: &str) -> BResult<&str, &str> {
    recognize(
        many0(alt((
            multispace1,
            parse_block_comment
        )))
    )(input)
}

// Helper to wrap a parser with whitespace and comment handling
fn with_ws<'a, O, F: FnMut(&'a str) -> BResult<&'a str, O>>(
    mut parser: F
) -> impl FnMut(&'a str) -> BResult<&'a str, O> {
    move |input: &'a str| {
        let (input, _) = parse_ws_comments(input)?;
        let (input, result) = parser(input)?;
        let (input, _) = parse_ws_comments(input)?;
        Ok((input, result))
    }
}

// Parse a block statement: { statements... }
// This uses parse_statement_ws which handles whitespace for us
pub fn parse_block_statement<'a>(input: &'a str) -> BResult<&'a str, Statement<'a>> {
    bs_context(
        "block statement",
        map(
            delimited(
                with_ws(bchar('{')),  // Make sure we handle whitespace and comments around braces
                many0(parse_statement_ws), // Allow for zero or more statements (handles empty blocks)
                with_ws(bchar('}'))  // Make sure we handle whitespace and comments around braces
            ),
            |statements| Statement::Block(statements), // Wrap in Statement::Block
        ),
    )(input)
}
