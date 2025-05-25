use crate::parser::nodes::statements::statement::Statement;
// Parser for throw statements

use nom::{
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

use crate::parser::errors::BResult;
use crate::parser::parser_helpers::{bchar, bs_context, btag};
use crate::parsers::expressions::expression_parser::parse_expression;


// Original parse_throw_statement function from statement_parser.rs
pub fn parse_throw_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "throw statement",
        map(
            tuple((
                multispace0, 
                btag("throw"), 
                opt( 
                    preceded(
                        multispace1, 
                        parse_expression
                    )
                ),
                multispace0, 
                bchar(';')   
            )),
            |(_, _, expr_opt, _, _)| Statement::Throw(expr_opt.map(Box::new)),
        ),
    )(input)
}
