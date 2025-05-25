// Parser for for loops

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::local_variable_declaration::LocalVariableDeclaration;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::statements::{ForInitializer, ForStatement};
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword, nom_to_bs};
use crate::parsers::declarations::variable_declaration_parser::parse_variable_declarator;
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::statement_parser::parse_statement_ws;
use crate::parsers::types::type_parser::parse_type_expression;

// Parse the initializer part of a for loop statement - can be a variable declaration
// or a comma-separated list of expressions
fn parse_for_initializer(input: &str) -> BResult<&str, ForInitializer> {
    bs_context(
        "for initializer",
        alt((
            // Try to parse a variable declaration first (e.g., "int i = 0")
            // Note: For a for loop, don't expect a semicolon at the end of the variable declaration
            map(
                tuple((
                    // Optionally parse "const"
                    opt(keyword("const")),
                    // Parse type name
                    bws(nom_to_bs(parse_type_expression)),
                    // Parse declarators (name and initializer)
                    separated_list1(bws(bchar(',')), parse_variable_declarator)
                )),
                |(const_modifier, ty, declarators)| {
                    ForInitializer::Declaration(LocalVariableDeclaration {
                        is_const: const_modifier.is_some(),
                        declaration_type: ty,
                        declarators,
                    })
                }
            ),
            // If that fails, try to parse expressions (e.g., "i = 0, j = 1")
            map(
                separated_list1(bws(bchar(',')), bws(parse_expression)),
                |exprs| ForInitializer::Expressions(exprs),
            ),
        )),
    )(input)
}

// Original parse_for_statement function from statement_parser.rs
// Parse a for loop statement using Roslyn-like structure
pub fn parse_for_statement(input: &str) -> BResult<&str, Statement> {
    // Base approach addressing all the test cases
    let (input, _) = keyword("for")(input)?;
    let (input, _) = bws(bchar('('))(input)?;
    
    // 1. Parse initializer (optional)
    let (input, initializer) = opt(bws(parse_for_initializer))(input)?;
    let (input, _) = bws(bchar(';'))(input)?;
    
    // 2. Parse condition (optional)
    let (input, condition) = opt(bws(parse_expression))(input)?;
    let (input, _) = bws(bchar(';'))(input)?;
    
    // 3. Parse iterators - the critical part for the test case
    // We need to handle multiple comma-separated expressions
    let (input, iterators) = separated_list0(bws(bchar(',')), bws(parse_expression))(input)?;
    
    // 4. Parse closing parenthesis
    let (input, _) = bws(bchar(')'))(input)?;
    
    // 5. Parse body statement, ensuring we handle comments correctly
    let (input, body) = bws(parse_statement_ws)(input)?;
    
    // Create and return the ForStatement node
    Ok((input, Statement::For(Box::new(ForStatement {
        initializer,
        condition,
        iterator: iterators,
        body: Box::new(body),
    }))))
}


