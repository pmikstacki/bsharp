use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{NamespaceDeclaration, namespace_declaration::NamespaceBodyDeclaration};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bws, nom_to_bs, keyword};
use crate::parsers::identifier_parser::parse_qualified_name;
use crate::parsers::declarations::type_declaration_parser::{parse_class_declaration, parse_struct_declaration, parse_interface_declaration, parse_record_declaration};
use crate::parsers::declarations::enum_declaration_parser::parse_enum_declaration;
use crate::parsers::declarations::delegate_declaration_parser::parse_delegate_declaration;
use nom::character::complete::{char as nom_char, multispace0};
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::terminated;

/// Parse a namespace member (class, struct, interface, enum, record, or nested namespace)
fn parse_namespace_member_safe(input: &str) -> BResult<&str, NamespaceBodyDeclaration> {
    println!("[DEBUG] parse_namespace_member_safe: input = {:?}", input.chars().take(60).collect::<String>());
    
    alt((
        map(parse_namespace_declaration, NamespaceBodyDeclaration::Namespace),
        map(parse_interface_declaration, NamespaceBodyDeclaration::Interface),
        map(parse_class_declaration, NamespaceBodyDeclaration::Class),
        map(parse_struct_declaration, NamespaceBodyDeclaration::Struct),
        map(parse_enum_declaration, NamespaceBodyDeclaration::Enum),
        map(parse_record_declaration, NamespaceBodyDeclaration::Record),
        map(parse_delegate_declaration, NamespaceBodyDeclaration::Delegate),
    ))(input)
}

/// Parse a C# namespace declaration using proper Nom combinators
/// Example in C#:
/// ```csharp
/// namespace MyCompany.MyProject
/// {
///     public class MyClass { }
/// }
/// ```
pub fn parse_namespace_declaration(input: &str) -> BResult<&str, NamespaceDeclaration> {
    println!("[DEBUG] parse_namespace_declaration: input = {:?}", input.chars().take(60).collect::<String>());
    
    // Parse the "namespace" keyword
    let (input, _) = bws(keyword("namespace"))(input)?;
    
    // Parse qualified name (e.g., "System.Collections")
    let (input, name_parts) = bws(parse_qualified_name)(input)?;
    let name_str = name_parts.iter().map(|id| id.name.clone()).collect::<Vec<_>>().join(".");
    
    // Parse opening brace
    let (input, _) = bws(nom_to_bs(nom_char::<_, nom::error::Error<_>>('{')))(input)?;
    println!("[DEBUG] parse_namespace_declaration: after open brace");
    
    // Parse namespace members using many0 - this will parse 0 or more members
    // and stop when it can't parse any more (which should be at the closing brace)
    let (input, members) = many0(
        terminated(
            bws(parse_namespace_member_safe),
            multispace0 // consume whitespace after each member
        )
    )(input)?;
    
    println!("[DEBUG] parse_namespace_declaration: parsed {} members", members.len());
    
    // Parse closing brace
    let (input, _) = bws(nom_to_bs(nom_char::<_, nom::error::Error<_>>('}')))(input)?;
    println!("[DEBUG] parse_namespace_declaration: successfully parsed closing brace");

    Ok((input, NamespaceDeclaration {
        name: Identifier { name: name_str },
        using_directives: vec![], // Namespaces in C# don't directly contain 'using' directives in their body
        declarations: vec![], // Stub for parsing namespace members - replace with actual member parsers
    }))
}

// Stub for parsing namespace members - replace with actual member parsers
// fn parse_namespace_member_stub<'a>(input: &'a str) -> BResult<&'a str, NamespaceBodyDeclaration<'a>> {
//     // This is highly simplified. A real parser would try to parse class, struct, enum, interface, nested namespace, etc.
//     // For now, let's assume it consumes nothing and returns a dummy member or an error.
//     Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(input, crate::parser::errors::CustomErrorKind::NotYetImplemented("NamespaceBodyDeclaration"))))
// }


#[cfg(test)]
mod tests {
    use super::*;
    
    // Local test helper to avoid import issues
    fn parse_full_input<'a, O, F>(input: &'a str, parser: F) -> Result<(&'a str, O), String>
    where
        F: FnOnce(&'a str) -> crate::parser::errors::BResult<&'a str, O>,
    {
        match parser(input) {
            Ok((remaining, result)) => Ok((remaining, result)),
            Err(err) => Err(format!("Parse error: {:?}", err)),
        }
    }

    #[test]
    fn test_simple_namespace_declaration() {
        let input = "namespace MyNamespace { }";
        let result = parse_full_input(input, parse_namespace_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "MyNamespace");
    }

    #[test]
    fn test_qualified_namespace_declaration() {
        let input = "namespace System.Collections { }";
        let result = parse_full_input(input, parse_namespace_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "System.Collections");
    }
}
