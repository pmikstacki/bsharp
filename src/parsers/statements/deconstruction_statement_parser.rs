use crate::parser::errors::BResult;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::parser_helpers::{bs_context, bws, nom_to_bs};
use crate::parsers::expressions::deconstruction_expression_parser::parse_deconstruction_expression;

use nom::{
    character::complete::char as nom_char,
    combinator::map,
    sequence::terminated,
};

/// Parse a deconstruction statement: (var x, var y) = tuple;
pub fn parse_deconstruction_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "deconstruction statement",
        map(
            terminated(
                bws(parse_deconstruction_expression),
                bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(';'))),
            ),
            |deconstruction| Statement::Deconstruction(Box::new(deconstruction)),
        ),
    )(input)
} 