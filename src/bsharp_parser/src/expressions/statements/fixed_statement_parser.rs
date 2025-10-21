use crate::parser::expressions::declarations::variable_declaration_parser::parse_variable_declaration;
use crate::parser::keywords::exception_and_safety_keywords::kw_fixed;
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::cut;
use nom::Parser;
use nom::{
    combinator::map,
    sequence::delimited,
};
use nom_supreme::ParserExt;
use syntax::statements::statement::Statement;
use syntax::statements::FixedStatement;

/// Parse a fixed statement: fixed (type* ptr = &expr, ...) { ... }
pub fn parse_fixed_statement(input: Span) -> BResult<Statement> {
    map(
        (
            kw_fixed().context("fixed keyword"),
            delimited(
                delimited(ws, tok_l_paren(), ws).context("opening parenthesis"),
                // Parse a single variable declaration (no trailing semicolon inside parentheses)
                delimited(ws, parse_variable_declaration, ws)
                    .context("fixed variable declaration"),
                cut(delimited(ws, tok_r_paren(), ws)).context("closing parenthesis"),
            )
                .context("fixed variable declarations in parentheses"),
            cut(delimited(ws, parse_statement_ws, ws)).context("fixed body"),
        ),
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
use crate::syntax::span::Span;
use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
