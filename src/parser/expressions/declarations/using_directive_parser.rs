use crate::parser::identifier_parser::parse_qualified_name;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::UsingDirective;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bchar, bws, context, peek_bchar};
use crate::parser::keywords::declaration_keywords::kw_using;
use crate::parser::keywords::modifier_keywords::{kw_static, peek_static};

/// Parse a using directive (namespace, alias, or static)
pub fn parse_using_directive(input: &str) -> BResult<&str, UsingDirective> {
    context("using directive", |input| {
        // 'using' keyword
        let (input, _) = context("using keyword", bws(kw_using()))(input)?;

        // Try static using first: 'using static TypeName;'
        if peek_static()(input).is_ok() {
            let (input, _) = bws(kw_static())(input)?;
            let (input, type_name_parts) = bws(parse_qualified_name)(input)?;
            let type_name_str = type_name_parts
                .iter()
                .map(|id| id.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            let (input, _) = bws(bchar(';'))(input)?;
            let using_directive = UsingDirective::Static {
                type_name: Identifier { name: type_name_str },
            };
            return Ok((input, using_directive));
        }

        // Otherwise parse a qualified name and decide alias vs namespace using lookahead for '='
        let (input, left_parts) = bws(parse_qualified_name)(input)?;
        let left_str = left_parts
            .iter()
            .map(|id| id.name.clone())
            .collect::<Vec<_>>()
            .join(".");

        if peek_bchar('=')(input).is_ok() {
            let (input, _) = bws(bchar('='))(input)?;
            let (input, right_parts) = bws(parse_qualified_name)(input)?;
            let right_str = right_parts
                .iter()
                .map(|id| id.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            let (input, _) = bws(bchar(';'))(input)?;
            let using_directive = UsingDirective::Alias {
                alias: Identifier { name: left_str },
                namespace_or_type: Identifier { name: right_str },
            };
            Ok((input, using_directive))
        } else {
            let (input, _) = bws(bchar(';'))(input)?;
            let using_directive = UsingDirective::Namespace {
                namespace: Identifier { name: left_str },
            };
            Ok((input, using_directive))
        }
    })(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_namespace_using() {
        let src = "using System.Text;";
        let (rest, ud) = parse_using_directive(src).expect("parse");
        assert!(matches!(ud, UsingDirective::Namespace{..}));
        assert!(rest.is_empty());
    }

    #[test]
    fn parses_static_using() {
        let src = "using static System.Math;";
        let (rest, ud) = parse_using_directive(src).expect("parse");
        assert!(matches!(ud, UsingDirective::Static{..}));
        assert!(rest.is_empty());
    }

    #[test]
    fn parses_alias_using() {
        let src = "using Project = MyCompany.MyProject;";
        let (rest, ud) = parse_using_directive(src).expect("parse");
        assert!(matches!(ud, UsingDirective::Alias{..}));
        assert!(rest.is_empty());
    }
}
