use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::statements::UsingStatement;

use nom::combinator::cut;
use nom::{combinator::map, sequence::tuple};

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

/// Parse a using statement for resource management
/// Format: using (resource_declaration_or_expression) statement
/// Examples:
/// - using (var file = new FileStream(...)) { ... }
/// - using (stream) { ... }
pub fn parse_using_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "using statement (expected 'using (resource) statement' for resource management)",
        map(
            tuple((
                // 1. 'using' keyword
                context("using keyword (expected 'using')", keyword("using")),
                // 2. Opening parenthesis
                context(
                    "opening parenthesis after using (expected '(')",
                    bws(bchar('(')),
                ),
                // 3. Resource expression (could be declaration or variable)
                context(
                    "resource expression (expected variable declaration or disposable expression)",
                    bws(parse_expression),
                ),
                // 4. Closing parenthesis
                context(
                    "closing parenthesis after resource (expected ')')",
                    cut(bws(bchar(')'))),
                ),
                // 5. Body statement
                context(
                    "using statement body (expected valid C# statement)",
                    bws(parse_statement_ws),
                ),
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
