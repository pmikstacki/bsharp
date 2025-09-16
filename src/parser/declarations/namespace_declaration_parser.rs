use crate::syntax::comment_parser::with_ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{namespace_declaration::NamespaceBodyDeclaration, NamespaceDeclaration};
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{context, bws, keyword, bchar};
use crate::parser::declarations::delegate_declaration_parser::parse_delegate_declaration;
use crate::parser::declarations::enum_declaration_parser::parse_enum_declaration;
use crate::parser::declarations::type_declaration_parser::{parse_class_declaration, parse_interface_declaration, parse_record_declaration, parse_struct_declaration};
use crate::parser::identifier_parser::parse_qualified_name;
use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use log::trace;

/// Parse a namespace member (class, struct, interface, enum, record, or nested namespace)
fn parse_namespace_member_safe(input: &str) -> BResult<&str, NamespaceBodyDeclaration> {
    trace!("[DEBUG] parse_namespace_member_safe: input = {:?}", input.chars().take(60).collect::<String>());
    
    // Use with_ws to handle whitespace and comments before type declarations
    with_ws(context("namespace member (expected class, struct, interface, enum, record, delegate, or nested namespace)", alt((
        // Try class, struct, interface, enum, record, delegate first since they have specific keywords
        map(parse_class_declaration, NamespaceBodyDeclaration::Class),
        map(parse_struct_declaration, NamespaceBodyDeclaration::Struct),
        map(parse_interface_declaration, NamespaceBodyDeclaration::Interface),
        map(parse_enum_declaration, NamespaceBodyDeclaration::Enum),
        map(parse_record_declaration, NamespaceBodyDeclaration::Record),
        map(parse_delegate_declaration, NamespaceBodyDeclaration::Delegate),
        // Try nested namespace last since it might be more ambiguous
        map(parse_namespace_declaration, NamespaceBodyDeclaration::Namespace),
    ))))(input)
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
    trace!("[DEBUG] parse_namespace_declaration: input = {:?}", input.chars().take(60).collect::<String>());
    
    // Parse the "namespace" keyword
    let (input, _) = context("namespace keyword (expected 'namespace')", bws(keyword("namespace")))(input)?;
    
    // Parse qualified name (e.g., "System.Collections")
    let (input, name_parts) = context("namespace name (expected qualified identifier)", bws(parse_qualified_name))(input)?;
    let name_str = name_parts.iter().map(|id| id.name.clone()).collect::<Vec<_>>().join(".");
    
    // Parse opening brace
    let (input, _) = context("namespace body opening (expected '{')", bws(bchar('{')))(input)?;
    trace!("[DEBUG] parse_namespace_declaration: after open brace");
    
    // Parse namespace members using many0 - this will parse 0 or more members
    // and stop when it can't parse any more (which should be at the closing brace)
    let (input, members) = many0(parse_namespace_member_safe)(input)?;
    
    trace!("[DEBUG] parse_namespace_declaration: parsed {} members", members.len());
    
    // Parse closing brace
    let (input, _) = context("namespace body closing (expected '}')", bws(bchar('}')))(input)?;
    trace!("[DEBUG] parse_namespace_declaration: successfully parsed closing brace");

    Ok((input, NamespaceDeclaration {
        name: Identifier { name: name_str },
        using_directives: vec![], // Namespaces in C# don't directly contain 'using' directives in their body
        declarations: members, // Use the actual parsed members instead of empty vector
    }))
}

// Stub for parsing namespace members - replace with actual member parser
// fn parse_namespace_member_stub<'a>(input: &'a str) -> BResult<&'a str, NamespaceBodyDeclaration<'a>> {
//     // This is highly simplified. A real syntax would try to parse class, struct, enum, interface, nested namespace, etc.
//     // For now, let's assume it consumes nothing and returns a dummy member or an error.
//     Err(nom::Err::Error(crate::syntax::errors::BSharpParseError::new(input, crate::syntax::errors::CustomErrorKind::NotYetImplemented("NamespaceBodyDeclaration"))))
// }
