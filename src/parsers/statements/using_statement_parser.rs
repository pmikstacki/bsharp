use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::statements::UsingStatement;

use nom::{
    combinator::map,
    sequence::tuple,
};

use crate::parser::errors::BResult;
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::statement_parser::parse_statement_ws;

/// Parse a using statement for resource management
/// Format: using (resource_declaration_or_expression) statement
/// Examples:
/// - using (var file = new FileStream(...)) { ... }
/// - using (stream) { ... }
pub fn parse_using_statement(input: &str) -> BResult<&str, Statement> {
    bs_context(
        "using statement",
        map(
            tuple((
                // 1. 'using' keyword
                keyword("using"),
                // 2. Opening parenthesis
                bws(bchar('(')),
                // 3. Resource expression (could be declaration or variable)
                bws(parse_expression),
                // 4. Closing parenthesis
                bws(bchar(')')),
                // 5. Body statement
                parse_statement_ws
            )),
            |(_using_kw, _open_paren, resource, _close_paren, body)| {
                Statement::Using(Box::new(UsingStatement {
                    resource,
                    body: Box::new(body),
                }))
            },
        ),
    )(input)
} 