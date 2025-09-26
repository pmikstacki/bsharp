use crate::syntax::nodes::statements::statement::Statement;
// Parser for expression statements (e.g., x = 5;, DoSomething();)

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, bws, context};
use nom::{combinator::map, sequence::terminated};
use nom::combinator::cut;

// Parse an expression statement: expression;
pub fn parse_expression_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "expression statement (expected valid C# expression followed by semicolon)",
        map(
            terminated(
                context(
                    "expression (expected valid C# expression like assignment, method call, etc.)",
                    parse_expression,
                ),
                context(
                    "semicolon after expression statement (expected ';')",
                    cut(bws(bchar(';'))),
                ),
            ),
            Statement::Expression,
        ),
    )(input)
}
