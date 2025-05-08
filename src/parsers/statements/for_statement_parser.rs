use crate::parser::nodes::statements::statement::Statement;
// Parser for for loops

use nom::{
    branch::alt,
    character::complete::multispace0, 
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
};

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::*;
use crate::parser::parser_helpers::{bchar, bs_context, keyword};
use crate::parsers::declarations::variable_declaration_parser::parse_local_variable_declaration;
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::statement_parser::parse_statement_ws;

// Original parse_for_initializer from statement_parser.rs
fn parse_for_initializer(input: &str) -> BResult<&str, ForInitializer> {
    bs_context(
        "for initializer",
        alt((
            map(parse_local_variable_declaration, |decl| {
                ForInitializer::Declaration(decl)
            }),
            map(
                separated_list1(delimited(multispace0, bchar(','), multispace0), parse_expression),
                |exprs| ForInitializer::Expressions(exprs),
            ),
        )),
    )(input)
}

// Original parse_for_statement function from statement_parser.rs
pub fn parse_for_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "for statement",
        map(
            tuple((
                preceded(keyword("for"), multispace0),
                bchar('('),
                opt(parse_for_initializer),
                bchar(';'),
                opt(preceded(multispace0, parse_expression)),
                bchar(';'),
                opt(preceded(multispace0,
                    separated_list1(delimited(multispace0, bchar(','), multispace0), parse_expression)
                )),
                bchar(')'),
                multispace0,
                // Use parse_statement_ws for better whitespace handling
                parse_statement_ws
            )),
            |(_, _, initializer, _, condition, _, iterator, _, _, body_statement)| {
                Statement::For(Box::new(ForStatement {
                    initializer,
                    condition: condition,
                    iterator: iterator.unwrap_or_default(),
                    body: Box::new(body_statement), // body_statement is already a Statement
                }))
            },
        ),
    )(input)
}
