use crate::parser::nodes::statements::statement::Statement;
// Parser for foreach loops

use nom::{
    character::complete::{multispace0, multispace1}, 
    combinator::map,
    sequence::{preceded, tuple},
};

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::*;
use crate::parser::parser_helpers::{bchar, bs_context, keyword, nom_to_bs};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;
use crate::parsers::statement_parser::parse_statement_ws;

// Parse a foreach statement: foreach (<type> <identifier> in <expression>) <statement>
pub fn parse_foreach_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "foreach statement",
        map(
            tuple((
                preceded(keyword("foreach"), multispace0),
                bchar('('),
                multispace0,
                nom_to_bs(parse_type_expression),
                multispace1,
                nom_to_bs(parse_identifier),
                multispace0,
                keyword("in"),
                multispace0,
                parse_expression,
                multispace0,
                bchar(')'),
                multispace0,
                // Use parse_statement_ws for better whitespace handling
                parse_statement_ws
            )),
            |(_, _, _, var_type, _, var_name, _, _, _, collection, _, _, _, body_statement)| {
                Statement::ForEach(Box::new(ForEachStatement {
                    var_type,
                    var_name,
                    collection: Box::new(collection),
                    body: Box::new(body_statement), 
                }))
            },
        ),
    )(input)
}
