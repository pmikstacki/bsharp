// Parser for foreach loops

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_await;
use crate::parser::keywords::iteration_keywords::kw_foreach;
use crate::parser::keywords::parameter_modifier_keywords::kw_in;
use crate::parser::statement_parser::parse_statement_ws;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::cut;
use nom::{combinator::map, sequence::{tuple, delimited}};
use nom::bytes::complete::tag;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::ForEachStatement;

// Parse a foreach statement following Roslyn's structure:
// foreach (<type> <identifier> in <expression>) <statement>
pub fn parse_foreach_statement<'a>(input: Span<'a>) -> BResult<'a, Statement> {
    map(
        tuple((
            // 0. Optional 'await'
            nom::combinator::opt(delimited(ws, kw_await(), ws)),
            // 1. Foreach keyword
            kw_foreach().context("foreach keyword"),
            // 2. Opening parenthesis
            delimited(ws, tag("("), ws)
                .context("opening parenthesis after foreach"),
            // 3. Variable type
            delimited(ws, parse_type_expression, ws)
                .context("variable type in foreach"),
            // 4. Variable name (identifier)
            delimited(ws, parse_identifier, ws)
                .context("variable name in foreach"),
            // 5. 'in' keyword
            delimited(ws, kw_in(), ws).context("in keyword in foreach"),
            // 6. Collection expression
            delimited(ws, parse_expression, ws)
                .context("collection expression in foreach"),
            // 7. Closing parenthesis
            cut(delimited(ws, tag(")"), ws))
                .context("closing parenthesis after foreach header"),
            // 8. Body statement
            cut(delimited(ws, parse_statement_ws, ws))
                .context("foreach body statement"),
        )),
        |(
            await_opt,
            _foreach_kw,
            _open_paren,
            var_type,
            var_name,
            _in_kw,
            collection,
            _close_paren,
            body,
        )| {
            Statement::ForEach(Box::new(ForEachStatement {
                is_await: await_opt.is_some(),
                var_type,
                var_name,
                collection: Box::new(collection),
                body: Box::new(body),
            }))
        },
    )
    .context("foreach statement (expected 'foreach (type identifier in collection) statement')")
    .parse(input)
}
use crate::syntax::span::Span;
