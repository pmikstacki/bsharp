use nom::branch::alt;
use nom::sequence::preceded;

use crate::parser::comment_parser::parse_whitespace_or_comments;
use crate::parser::errors::BResult;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::parser_helpers::bs_context;

use crate::parsers::statements::block_statement_parser::parse_block_statement;
use crate::parsers::statements::for_statement_parser::parse_for_statement;
use crate::parsers::statements::if_statement_parser::parse_if_statement;
use crate::parsers::statements::return_statement_parser::parse_return_statement;
use crate::parsers::statements::expression_statement_parser::parse_expression_statement;
use crate::parsers::declarations::variable_declaration_parser::parse_local_variable_declaration_statement;
use crate::parsers::statements::while_statement_parser::parse_while_statement;
use crate::parsers::statements::do_while_statement_parser::parse_do_while_statement;
use crate::parsers::statements::foreach_statement_parser::parse_foreach_statement;
use crate::parsers::statements::switch_statement_parser::parse_switch_statement;
use crate::parsers::statements::empty_statement_parser::parse_empty_statement;
use crate::parsers::statements::try_catch_finally_parser::parse_try_statement;
use crate::parsers::statements::throw_statement_parser::parse_throw_statement;
use crate::parsers::statements::break_statement_parser::parse_break_statement;
use crate::parsers::statements::continue_statement_parser::parse_continue_statement;
use crate::parsers::statements::using_statement_parser::parse_using_statement;

/// Main statement parser - handles all types of statements
/// This function correctly handles recursion by dispatching to specific statement parsers
pub fn parse_statement(input: &str) -> BResult<&str, Statement> {
    // Order is important here. Try more specific statements first.
    alt((
        parse_block_statement, // Block statement { ... }
        parse_if_statement,    // If statement
        parse_for_statement,   // For loop
        parse_while_statement, // While loop
        parse_do_while_statement, // Do-while loop
        parse_foreach_statement, // Foreach loop
        parse_switch_statement, // Switch statement
        parse_try_statement, // Try-catch-finally statement
        parse_using_statement, // Using statement
        parse_return_statement, // Return statement
        parse_throw_statement, // Throw statement
        // Declaration statement must be tried before expression statement
        // as a simple identifier could be an expression.
        parse_local_variable_declaration_statement,
        parse_empty_statement, // Empty statement ;
        parse_break_statement,
        parse_continue_statement,
        parse_expression_statement, // Must be last due to its broadness
    ))(input)
}

/// Parse a statement, consuming any leading whitespace or comments.
pub fn parse_statement_ws(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "statement_ws",
        preceded(
            parse_whitespace_or_comments,
            parse_statement
        )
    )(input)
}
