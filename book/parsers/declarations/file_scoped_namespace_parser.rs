use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{FileScopedNamespaceDeclaration, namespace_declaration::NamespaceBodyDeclaration, UsingDirective};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bws, keyword, nom_to_bs};
use crate::parsers::declarations::delegate_declaration_parser::parse_delegate_declaration;
use crate::parsers::declarations::enum_declaration_parser::parse_enum_declaration;
use crate::parsers::declarations::type_declaration_parser::{parse_class_declaration, parse_interface_declaration, parse_record_declaration, parse_struct_declaration};
use crate::parsers::identifier_parser::parse_qualified_name;
use nom::branch::alt;
use nom::character::complete::{char as nom_char, multispace0};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::terminated;

/// Parse a file-scoped namespace member (class, struct, interface, enum, record, or nested namespace)
fn parse_file_scoped_namespace_member(input: &str) -> BResult<&str, NamespaceBodyDeclaration> {
    println!("[DEBUG] parse_file_scoped_namespace_member: input = {:?}", input.chars().take(60).collect::<String>());
    
    alt((
        map(parse_interface_declaration, NamespaceBodyDeclaration::Interface),
        map(parse_class_declaration, NamespaceBodyDeclaration::Class),
        map(parse_struct_declaration, NamespaceBodyDeclaration::Struct),
        map(parse_enum_declaration, NamespaceBodyDeclaration::Enum),
        map(parse_record_declaration, NamespaceBodyDeclaration::Record),
        map(parse_delegate_declaration, NamespaceBodyDeclaration::Delegate),
    ))(input)
}

/// Parse a using directive for file-scoped namespaces
fn parse_using_directive_simple(input: &str) -> BResult<&str, UsingDirective> {
    println!("[DEBUG] parse_using_directive_simple: input = {:?}", input.chars().take(60).collect::<String>());
    
    // Parse the 'using' keyword
    let (input, _) = bws(keyword("using"))(input)?;
    
    // Parse the namespace (qualified name)
    let (input, namespace_parts) = bws(parse_qualified_name)(input)?;
    let namespace_str = namespace_parts.iter().map(|id| id.name.clone()).collect::<Vec<_>>().join(".");
    
    // Parse the semicolon
    let (input, _) = bws(nom_to_bs(nom_char::<_, nom::error::Error<_>>(';')))(input)?;
    
    Ok((input, UsingDirective::Namespace { 
        namespace: Identifier { name: namespace_str } 
    }))
}

/// Parse a C# file-scoped namespace declaration (C# 10+)
/// Example in C#:
/// ```csharp
/// namespace MyCompany.MyProject;
/// 
/// using System;
/// 
/// public class MyClass { }
/// ```
pub fn parse_file_scoped_namespace_declaration(input: &str) -> BResult<&str, FileScopedNamespaceDeclaration> {
    println!("[DEBUG] parse_file_scoped_namespace_declaration: input = {:?}", input.chars().take(60).collect::<String>());
    
    // Parse the "namespace" keyword
    let (input, _) = bws(keyword("namespace"))(input)?;
    
    // Parse qualified name (e.g., "System.Collections")
    let (input, name_parts) = bws(parse_qualified_name)(input)?;
    let name_str = name_parts.iter().map(|id| id.name.clone()).collect::<Vec<_>>().join(".");
    
    // Parse the semicolon (this is what makes it file-scoped)
    let (input, _) = bws(nom_to_bs(nom_char::<_, nom::error::Error<_>>(';')))(input)?;
    println!("[DEBUG] parse_file_scoped_namespace_declaration: after semicolon");
    
    // Parse any whitespace after the namespace declaration
    let (input, _) = multispace0(input)?;
    
    // Parse using directives that come after the namespace declaration
    let (input, using_directives) = many0(
        terminated(
            bws(parse_using_directive_simple),
            multispace0
        )
    )(input)?;
    
    println!("[DEBUG] parse_file_scoped_namespace_declaration: parsed {} using directives", using_directives.len());
    
    // Parse namespace members using many0 - this will parse 0 or more members
    let (input, members) = many0(
        terminated(
            bws(parse_file_scoped_namespace_member),
            multispace0 // consume whitespace after each member
        )
    )(input)?;
    
    println!("[DEBUG] parse_file_scoped_namespace_declaration: parsed {} members", members.len());

    Ok((input, FileScopedNamespaceDeclaration {
        name: Identifier { name: name_str },
        using_directives,
        declarations: members,
    }))
} 