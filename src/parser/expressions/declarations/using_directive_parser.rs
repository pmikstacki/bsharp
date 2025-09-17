use crate::parser::identifier_parser::parse_qualified_name;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::UsingDirective;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

/// Parse a using directive (namespace, alias, or static)
pub fn parse_using_directive(input: &str) -> BResult<&str, UsingDirective> {
    context("using directive", |input| {
        // Skip leading whitespace/comments
        let (input, _) = ws(input)?;

        // Parse the 'using' keyword
        let (input, _) = keyword("using")(input)?;

        // Consume any spacing after 'using'
        let (input, _) = ws(input)?;

        // Check for static using
        if input.trim_start().starts_with("static") {
            let (input, _) = keyword("static")(input)?;
            let (input, _) = ws(input)?;
            let (input, type_name_parts) = parse_qualified_name(input)?;
            let type_name_str = type_name_parts
                .iter()
                .map(|id| id.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            let (input, _) = bws(bchar(';'))(input)?;
            let (input, _) = ws(input)?;
            let using_directive = UsingDirective::Static {
                type_name: Identifier {
                    name: type_name_str,
                },
            };
            return Ok((input, using_directive));
        }

        // Parse the namespace or alias (qualified name)
        let (input, namespace) = parse_qualified_name(input)?;
        let ns_str = namespace
            .iter()
            .map(|id| id.name.clone())
            .collect::<Vec<_>>()
            .join(".");

        // Check if alias using (has '=')
        let (input, _) = ws(input)?;
        if input.trim_start().starts_with("=") {
            let (input, _) = bchar('=')(input)?;
            let (input, _) = ws(input)?;
            let (input, target_parts) = parse_qualified_name(input)?;
            let target_str = target_parts
                .iter()
                .map(|id| id.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            let (input, _) = bws(bchar(';'))(input)?;
            let (input, _) = ws(input)?;
            let using_directive = UsingDirective::Alias {
                alias: Identifier { name: ns_str },
                namespace_or_type: Identifier { name: target_str },
            };
            return Ok((input, using_directive));
        }

        // Normal namespace using
        let (input, _) = bws(bchar(';'))(input)?;
        let (input, _) = ws(input)?;
        let using_directive = UsingDirective::Namespace {
            namespace: Identifier { name: ns_str },
        };
        Ok((input, using_directive))
    })(input)
}
