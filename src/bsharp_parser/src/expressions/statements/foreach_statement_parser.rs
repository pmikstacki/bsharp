use crate::syntax::nodes::statements::statement::Statement;
// Parser for foreach loops

use nom::combinator::cut;
use nom::{combinator::map, sequence::tuple};

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_await;
use crate::parser::keywords::iteration_keywords::kw_foreach;
use crate::parser::keywords::parameter_modifier_keywords::kw_in;
use crate::parser::statement_parser::parse_statement_ws;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::*;
use crate::syntax::parser_helpers::{bchar, bws, context};

// Parse a foreach statement following Roslyn's structure:
// foreach (<type> <identifier> in <expression>) <statement>
pub fn parse_foreach_statement(input: &str) -> BResult<&str, Statement> {
    context(
        "foreach statement (expected 'foreach (type identifier in collection) statement')",
        map(
            tuple((
                // 0. Optional 'await'
                nom::combinator::opt(bws(kw_await())),
                // 1. Foreach keyword
                context("foreach keyword (expected 'foreach')", kw_foreach()),
                // 2. Opening parenthesis
                context(
                    "opening parenthesis after foreach (expected '(')",
                    bws(bchar('(')),
                ),
                // 3. Variable type
                context(
                    "variable type in foreach (expected valid C# type)",
                    bws(parse_type_expression),
                ),
                // 4. Variable name (identifier)
                context(
                    "variable name in foreach (expected valid identifier)",
                    bws(parse_identifier),
                ),
                // 5. 'in' keyword
                context("in keyword in foreach (expected 'in')", bws(kw_in())),
                // 6. Collection expression
                context(
                    "collection expression in foreach (expected iterable expression)",
                    bws(parse_expression),
                ),
                // 7. Closing parenthesis
                context(
                    "closing parenthesis after foreach header (expected ')')",
                    cut(bws(bchar(')'))),
                ),
                // 8. Body statement
                context(
                    "foreach body statement (expected valid C# statement)",
                    cut(bws(parse_statement_ws)),
                ),
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
        ),
    )(input)
}
