use crate::syntax::nodes::statements::statement::Statement;
// Parser for if/else statements

use nom::{
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
};
use nom::combinator::cut;

use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, context, keyword, bws};

use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement_ws;

/// Parse an if statement with optional else branch
/// Format: if (expr) stmt [else stmt]
/// Note: In C#, if statements MUST have block bodies (braces are required)
pub fn parse_if_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "if statement (expected 'if (condition) statement' with optional 'else statement')",
        map(
            tuple((
                context("if keyword (expected 'if')", keyword("if")),
                bws(delimited(
                    context("opening parenthesis for if condition (expected '(')", bchar('(')),
                    bws(parse_expression),
                    cut(context("closing parenthesis for if condition (expected ')')", bchar(')'))),
                )),
                context("if statement body (expected valid C# statement)", bws(parse_statement_ws)),
                context("optional else clause", opt(preceded(
                    bws(keyword("else")),
                    bws(parse_statement_ws),
                ))),
            )),
            |(_, condition, then_branch, else_branch)| {
                Statement::If(Box::new(IfStatement {
                    condition,
                    consequence: Box::new(then_branch),
                    alternative: else_branch.map(Box::new),
                }))
            },
        ),
    )(input)
}
