use crate::syntax::nodes::statements::statement::Statement;
// Parser for switch statements

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::statement_parser::parse_statement;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};
// Need this for statements within sections

use crate::syntax::comment_parser::ws;
use nom::branch::alt;
use nom::combinator::{cut, not, peek};
use nom::combinator::map;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated, tuple};

// Helper syntax for case labels: case expression:
fn parse_case_label(input: &str) -> BResult<&str, SwitchLabel> {
    context(
        "case label (expected 'case expression:')",
        map(
            preceded(
                context("case keyword (expected 'case')", keyword("case")),
                terminated(
                    context(
                        "case value expression (expected valid C# expression)",
                        bws(parse_expression),
                    ),
                    context("colon after case value (expected ':')", bws(bchar(':'))),
                ),
            ),
            |expr| SwitchLabel::Case(expr),
        ),
    )(input)
}

// Helper syntax for default label: default:
fn parse_default_label(input: &str) -> BResult<&str, SwitchLabel> {
    context(
        "default label (expected 'default:')",
        map(
            terminated(
                context("default keyword (expected 'default')", keyword("default")),
                context("colon after default (expected ':')", bws(bchar(':'))),
            ),
            |_| SwitchLabel::Default,
        ),
    )(input)
}

// Helper syntax for switch sections (one or more labels followed by zero or more statements)
fn parse_switch_section(input: &str) -> BResult<&str, SwitchSection> {
    context(
        "switch section (expected one or more case/default labels followed by statements)",
        map(
            tuple((
                context(
                    "switch section labels (expected one or more 'case' or 'default' labels)",
                    many1(delimited(
                        ws,
                        context(
                            "switch label (expected 'case expression:' or 'default:')",
                            alt((parse_case_label, parse_default_label)),
                        ),
                        ws,
                    )),
                ),
                context(
                    "switch section statements (expected zero or more C# statements)",
                    many0(|i| {
                        // Do not consume statements if the next token starts a new section or closes the switch
                        // Guard against 'case', 'default', or '}'
                        let mut guard = alt((
                            nom::combinator::map(bws(keyword("case")), |_| ()),
                            nom::combinator::map(bws(keyword("default")), |_| ()),
                            nom::combinator::map(bws(bchar('}')), |_| ()),
                        ));
                        peek(not(&mut guard))(i)?;
                        delimited(ws, parse_statement, ws)(i)
                    }),
                ),
            )),
            |(labels, statements)| SwitchSection { labels, statements },
        ),
    )(input)
}

// Original parse_switch_statement function from statement_parser.rs
pub fn parse_switch_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "switch statement (expected 'switch (expression) { case/default sections }')",
        map(
            tuple((
                context(
                    "whitespace after switch keyword",
                    preceded(
                        context("switch keyword (expected 'switch')", keyword("switch")),
                        ws,
                    ),
                ),
                context(
                    "switch expression in parentheses (expected '(expression)')",
                    delimited(
                        context(
                            "opening parenthesis for switch expression (expected '(')",
                            bchar('('),
                        ),
                        context(
                            "switch expression (expected valid C# expression)",
                            delimited(ws, parse_expression, ws),
                        ),
                        context(
                            "closing parenthesis for switch expression (expected ')')",
                            cut(bchar(')')),
                        ),
                    ),
                ),
                context("whitespace after switch expression", ws),
                context(
                    "switch body (expected '{' followed by case/default sections and '}')",
                    delimited(
                        context("opening brace for switch body (expected '{')", bchar('{')),
                        context(
                            "switch sections (expected zero or more case/default sections)",
                            many0(delimited(ws, parse_switch_section, ws)),
                        ),
                        context(
                            "closing brace for switch body (expected '}')",
                            cut(bws(bchar('}'))),
                        ),
                    ),
                ),
            )),
            |(_, switch_expr, _, sections)| {
                Statement::Switch(Box::new(SwitchStatement {
                    expression: *Box::new(switch_expr),
                    sections,
                }))
            },
        ),
    )(input)
}
