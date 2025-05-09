use crate::parser::nodes::statements::statement::Statement;
// Parser for foreach loops

use nom::{
    combinator::map,
    sequence::tuple,
};

use crate::parser::errors::BResult;
use crate::parser::nodes::statements::*;
use crate::parser::parser_helpers::{bchar, bs_context, keyword, nom_to_bs, bws};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;
use crate::parsers::statement_parser::parse_statement_ws;

// Parse a foreach statement following Roslyn's structure:
// foreach (<type> <identifier> in <expression>) <statement>
pub fn parse_foreach_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "foreach statement",
        map(
            tuple((
                // 1. Foreach keyword
                keyword("foreach"),
                // 2. Opening parenthesis
                bws(bchar('(')),
                // 3. Variable type
                bws(nom_to_bs(parse_type_expression)),
                // 4. Variable name (identifier)
                bws(nom_to_bs(parse_identifier)),
                // 5. 'in' keyword
                keyword("in"),
                // 6. Collection expression
                bws(parse_expression),
                // 7. Closing parenthesis
                bws(bchar(')')),
                // 8. Body statement
                parse_statement_ws
            )),
            |(_foreach_kw, _open_paren, var_type, var_name, _in_kw, collection, _close_paren, body)| {
                Statement::ForEach(Box::new(ForEachStatement {
                    var_type,
                    var_name,
                    collection: Box::new(collection),
                    body: Box::new(body),
                }))
            },
        ),
    )(input)
}
