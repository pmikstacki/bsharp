use crate::syntax::nodes::statements::UsingStatement;
use crate::syntax::nodes::statements::statement::Statement;

use nom::combinator::cut;
use nom::combinator::opt;

use crate::parser::expressions::declarations::variable_declaration_parser::{
    parse_local_variable_declaration, parse_variable_declaration,
};
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::declaration_keywords::kw_using;
use crate::parser::keywords::expression_keywords::kw_await;
use crate::parser::statement_parser::parse_statement_ws;
use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, bws, context};

/// Parse a using statement for resource management
/// Format: using (resource_declaration_or_expression) statement
/// Examples:
/// - using (var file = new FileStream(...)) { ... }
/// - using (stream) { ... }
pub fn parse_using_statement(input: &str) -> BResult<&str, Statement> {
    context("using statement or declaration", |input| {
        // Optional 'await'
        let (input, is_await) = match opt(bws(kw_await()))(input) {
            Ok((rest, Some(_))) => (rest, true),
            _ => (input, false),
        };

        // Mandatory 'using'
        let (input, _) = context("using keyword (expected 'using')", kw_using())(input)?;

        // Two forms: ( ... ) statement  OR  declaration ;

        // If next is '(', parse parenthesized resource then statement body
        if let Ok((after_open, _)) = bws(bchar('('))(input) {
            // Try declaration inside parens first, else expression
            // declaration: type declarators
            if let Ok((rest_after_decl, decl)) = bws(parse_variable_declaration)(after_open) {
                let (rest_after_paren, _) = context(
                    "closing parenthesis after resource (expected ')')",
                    cut(bws(bchar(')'))),
                )(rest_after_decl)?;
                let (rest_after_body, body) = context(
                    "using statement body (expected valid C# statement)",
                    cut(bws(parse_statement_ws)),
                )(rest_after_paren)?;

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
            let (after_resource, resource) = context(
                "resource expression (expected variable declaration or disposable expression)",
                bws(parse_expression),
            )(after_open)?;
            let (after_paren, _) = context(
                "closing parenthesis after resource (expected ')')",
                cut(bws(bchar(')'))),
            )(after_resource)?;
            let (rest_after_body, body) = context(
                "using statement body (expected valid C# statement)",
                cut(bws(parse_statement_ws)),
            )(after_paren)?;

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
            let (rest, decl) = bws(parse_local_variable_declaration)(input)?;
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
    })(input)
}
