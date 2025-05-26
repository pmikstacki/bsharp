use crate::parser::nodes::statements::statement::Statement;
// Parser for block statements { ... }

use nom::combinator::map;
use nom::sequence::delimited;

use crate::parser::errors::BResult;
use crate::parser::parser_helpers::{bchar, bs_context, bws};

use crate::parsers::statement_parser::parse_statement_ws;

// Function to extract statements from a Block statement, if that's what it is
// This is a helper for tests and other code that expects a Vec<Statement>
pub fn extract_statements_from_block(statement: Statement) -> Vec<Statement> {
    match statement {
        Statement::Block(statements) => statements,
        _ => vec![statement], // If it's not a block, wrap it in a Vec as a fallback
    }
}

// Parse a block statement: { statements... }
// This uses parse_statement_ws which handles whitespace for us
pub fn parse_block_statement(input: &str) -> BResult<&str, Statement> {
    use crate::parser::comment_parser::parse_whitespace_or_comments;
    
    bs_context(
        "block statement",
        map(
            delimited(
                bws(bchar('{')),
                |inner_input| {
                    // First consume any whitespace/comments
                    let (mut current, _) = parse_whitespace_or_comments(inner_input)?;
                    let mut statements = Vec::new();
                    
                    // Keep parsing statements until we can't parse any more
                    while !current.trim_start().starts_with('}') && !current.is_empty() {
                        match parse_statement_ws(current) {
                            Ok((remaining, stmt)) => {
                                statements.push(stmt);
                                current = remaining;
                                // Consume any trailing whitespace/comments
                                let (new_current, _) = parse_whitespace_or_comments(current)?;
                                current = new_current;
                            }
                            Err(_) => break, // No more statements to parse
                        }
                    }
                    
                    Ok((current, statements))
                },
                bws(bchar('}'))
            ),
            |statements| Statement::Block(statements),
        ),
    )(input)
}
