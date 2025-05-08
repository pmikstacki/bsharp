use nom::{
    combinator::{map, opt},
    multi::separated_list1, 
    sequence::{preceded, tuple}, // Keep for internal nom usage if any
};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::local_variable_declaration::{LocalVariableDeclaration, VariableDeclarator};
// Import bws, remove local ws. Ensure bchar, keyword etc. are imported.
use crate::parser::parser_helpers::{bchar, bs_context, bws, nom_to_bs}; 
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

// Parse a local variable declaration: <type> <declarator> (, <declarator>)* ;
pub fn parse_local_variable_declaration(input: &str) -> BResult<&str, LocalVariableDeclaration> {
    bs_context(
        "local variable declaration",
        map(
            tuple((
                // bws expects its inner parser to return BResult, so wrap parse_type_expression (which returns IResult)
                bws(nom_to_bs(parse_type_expression)),
                // parse_variable_declarator returns BResult
                // bchar returns BResult
                separated_list1(bws(bchar(',')), parse_variable_declarator),
                // bchar returns BResult
                bws(bchar(';')) 
            )),
            |(ty, declarators, _)| LocalVariableDeclaration { ty, declarators }
        )
    )(input)
}


