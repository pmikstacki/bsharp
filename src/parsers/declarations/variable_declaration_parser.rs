use nom::{
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, tuple}, // Keep for internal nom usage if any
};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::local_variable_declaration::{LocalVariableDeclaration, VariableDeclarator};
// Import bws, remove local ws. Ensure bchar, keyword etc. are imported.
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword, nom_to_bs};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

// Removed local ws helper

// Parse a single variable declarator: <name> (= <initializer>)?
pub fn parse_variable_declarator(input: &str) -> BResult<&str, VariableDeclarator> {
    bs_context(
        "variable declarator",
        map(
            tuple(( 
                // bws expects its inner parser to return BResult, so wrap parse_identifier (which returns IResult)
                bws(nom_to_bs(parse_identifier)),
                opt(preceded(
                    // bchar returns BResult
                    bws(bchar('=')), 
                    // bws expects its inner parser to return BResult, so wrap parse_expression (which *should* return BResult)
                    // If parse_expression returns IResult, it needs nom_to_bs. Assuming it returns BResult.
                    bws(parse_expression) 
                ))
            )),
            |(name, initializer)| VariableDeclarator { name, initializer }
        )
    )(input)
}

// Parse a local variable declaration: [const] <type> <declarator> (, <declarator>)* ;
pub fn parse_local_variable_declaration(input: &str) -> BResult<&str, LocalVariableDeclaration> {
    bs_context(
        "local variable declaration",
        map(
            tuple((
                opt(keyword("const")), // Optionally parse "const"
                bws(nom_to_bs(parse_type_expression)),
                separated_list1(bws(bchar(',')), parse_variable_declarator),
                bws(bchar(';')) 
            )),
            |(const_modifier, ty, declarators, _)| LocalVariableDeclaration {
                is_const: const_modifier.is_some(), // Check if 'const' was present
                declaration_type: ty,
                declarators
            }
        )
    )(input)
}

// Wrapper function to use in statement parsing
use crate::parser::nodes::statements::statement::Statement;
pub fn parse_local_variable_declaration_statement(input: &str) -> BResult<&str, Statement> {
    map(parse_local_variable_declaration, Statement::Declaration)(input)
}
