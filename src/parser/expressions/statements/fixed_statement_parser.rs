use crate::parser::expressions::declarations::variable_declaration_parser::parse_variable_declaration;
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::statements::fixed_statement::FixedStatement;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::parser_helpers::{bchar, bws};
use crate::parser::keywords::exception_and_safety_keywords::kw_fixed;

use nom::combinator::cut;
use nom::Parser;
use nom::{
    combinator::map,
    sequence::{delimited, tuple},
};
use nom_supreme::ParserExt;

/// Parse a fixed statement: fixed (type* ptr = &expr, ...) { ... }
pub fn parse_fixed_statement(input: &str) -> BResult<&str, Statement> {
    map(
        tuple((
            kw_fixed().context("fixed keyword"),
            bws(delimited(
                bchar('(').context("opening parenthesis"),
                // Parse a single variable declaration (no trailing semicolon inside parentheses)
                bws(parse_variable_declaration).context("fixed variable declaration"),
                cut(bchar(')')).context("closing parenthesis"),
            ))
            .context("fixed variable declarations in parentheses"),
            bws(parse_statement_ws).context("fixed body"),
        )),
        |(_, decl, body)| {
            // Take the first declarator to populate FixedStatement fields
            let first = decl
                .declarators
                .first()
                .expect("variable declarator required in fixed statement");
            Statement::Fixed(Box::new(FixedStatement {
                var_type: decl.declaration_type,
                var_name: first.name.clone(),
                initializer: first
                    .initializer
                    .clone()
                    .expect("initializer required in fixed statement"),
                body: Box::new(body),
            }))
        },
    )
    .context("fixed statement")
    .parse(input)
}
