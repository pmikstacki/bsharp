use crate::syntax::nodes::statements::statement::Statement;
// Parser for expression statements (e.g., x = 5;, DoSomething();)

use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, context};
use crate::parser::expressions::expression_parser::parse_expression;
use nom::{
    combinator::map,
    sequence::terminated,
};

// Parse an expression statement: expression;
pub fn parse_expression_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "expression statement (expected valid C# expression followed by semicolon)",
        map(
            terminated(
                context("expression (expected valid C# expression like assignment, method call, etc.)", parse_expression), 
                context("semicolon after expression statement (expected ';')", bchar(';'))
            ),
            |expr| Statement::Expression(expr),
        ),
    )(input)
}
