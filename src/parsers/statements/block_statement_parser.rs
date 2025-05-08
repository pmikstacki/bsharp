use crate::parser::nodes::statements::statement::Statement;
// Parser for block statements { ... }

use nom::{
    combinator::map, 
    multi::many0,
    sequence::delimited,
};

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

// Parse a block statement: { statements... }
// This uses parse_statement_ws which handles whitespace for us
pub fn parse_block_statement<'a>(input: &'a str) -> BResult<&'a str, Statement<'a>> {
    bs_context(
        "block statement",
        map(
            delimited(
                bchar('{'),
                many0(parse_statement_ws),
                bchar('}')
            ),
            |statements| Statement::Block(statements), // Wrap in Statement::Block
        ),
    )(input)
}
