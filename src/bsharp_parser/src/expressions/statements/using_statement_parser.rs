use crate::parser::expressions::declarations::variable_declaration_parser::{
    parse_local_variable_declaration, parse_variable_declaration,
};
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::declaration_keywords::kw_using;
use crate::parser::keywords::expression_keywords::kw_await;
use crate::parser::statement_parser::parse_statement_ws;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::combinator::cut;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use syntax::statements::UsingStatement;
use syntax::statements::statement::Statement;

/// Parse a using statement for resource management
/// Format: using (resource_declaration_or_expression) statement
/// Examples:
/// - using (var file = new FileStream(...)) { ... }
/// - using (stream) { ... }
pub fn parse_using_statement(input: Span) -> BResult<Statement> {
    (|input| {
        // Optional 'await'
        let (input, is_await) = match opt(delimited(ws, kw_await(), ws)).parse(input) {
            Ok((rest, Some(_))) => (rest, true),
            _ => (input, false),
        };

        // Mandatory 'using'
        let (input, _) = kw_using().context("using keyword").parse(input)?;

        // Two forms: ( ... ) statement  OR  declaration ;
        if peek(delimited(ws, tok_l_paren(), ws)).parse(input).is_ok() {
            // Try declaration inside parens first, else expression
            let (after_open, _) = delimited(ws, tok_l_paren(), ws).parse(input)?;

            if let Ok((rest_after_decl, decl)) =
                delimited(ws, parse_variable_declaration, ws).parse(after_open)
            {
                let (rest_after_paren, _) = cut(delimited(ws, tok_r_paren(), ws))
                    .context("closing parenthesis after resource")
                    .parse(rest_after_decl)?;
                let (rest_after_body, body) = cut(delimited(ws, parse_statement_ws, ws))
                    .context("using statement body")
                    .parse(rest_after_paren)?;

                return Ok((
                    rest_after_body,
                    Statement::Using(Box::new(UsingStatement {
                        is_await,
                        resource: None,
                        declaration: Some(decl),
                        body: Some(Box::new(body)),
                    })),
                ));
            }

            // Otherwise parse expression resource
            let (after_resource, resource) = delimited(ws, parse_expression, ws)
                .context("resource expression")
                .parse(after_open)?;
            let (after_paren, _) = cut(delimited(ws, tok_r_paren(), ws))
                .context("closing parenthesis after resource")
                .parse(after_resource)?;
            let (rest_after_body, body) = cut(delimited(ws, parse_statement_ws, ws))
                .context("using statement body (expected valid C# statement)")
                .parse(after_paren)?;

            Ok((
                rest_after_body,
                Statement::Using(Box::new(UsingStatement {
                    is_await,
                    resource: Some(resource),
                    declaration: None,
                    body: Some(Box::new(body)),
                })),
            ))
        } else {
            // using declaration form: using <local_variable_declaration>;
            let (rest, decl) = delimited(ws, parse_local_variable_declaration, ws).parse(input)?;
            Ok((
                rest,
                Statement::Using(Box::new(UsingStatement {
                    is_await,
                    resource: None,
                    declaration: Some(decl),
                    body: None,
                })),
            ))
        }
    })
    .context("using statement or declaration")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
