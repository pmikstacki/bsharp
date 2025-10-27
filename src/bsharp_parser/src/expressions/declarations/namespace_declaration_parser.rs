use crate::parser::expressions::declarations::delegate_declaration_parser::parse_delegate_declaration;
use crate::parser::expressions::declarations::enum_declaration_parser::parse_enum_declaration;
use crate::parser::expressions::declarations::type_declaration_parser::{
    parse_class_declaration, parse_interface_declaration, parse_record_declaration,
    parse_struct_declaration_span,
};
use crate::parser::expressions::declarations::using_directive_parser::parse_using_directive;
// use nom::multi::many0; // replaced by manual loop to support directive skipping
use crate::parser::helpers::directives::skip_preprocessor_directives;
use crate::parser::identifier_parser::parse_qualified_name;
use crate::errors::BResult;

use crate::parser::keywords::declaration_keywords::{kw_namespace, kw_using};
use crate::trivia::comment_parser::ws;
use log::trace;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::satisfy;
use nom::combinator::map;
use nom::combinator::{cut, peek};
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use syntax::Identifier;
use syntax::declarations::{NamespaceBodyDeclaration, NamespaceDeclaration, UsingDirective};

/// Parse a namespace member (class, struct, interface, enum, record, or nested namespace)
fn parse_namespace_member_safe(input: Span) -> BResult<NamespaceBodyDeclaration> {
    trace!(
        "[DEBUG] parse_namespace_member_safe: input = {:?}",
        input.chars().take(60).collect::<String>()
    );

    // Use with_ws to handle whitespace and comments before type declarations, and skip directives
    alt((
        // Try class, struct, interface, enum, record, delegate first since they have specific keywords
        map(parse_class_declaration, NamespaceBodyDeclaration::Class),
        map(
            parse_struct_declaration_span,
            NamespaceBodyDeclaration::Struct,
        ),
        map(
            parse_interface_declaration,
            NamespaceBodyDeclaration::Interface,
        ),
        map(parse_enum_declaration, NamespaceBodyDeclaration::Enum),
        map(parse_record_declaration, NamespaceBodyDeclaration::Record),
        map(
            parse_delegate_declaration,
            NamespaceBodyDeclaration::Delegate,
        ),
        // Try nested namespace last since it might be more ambiguous
        map(
            parse_namespace_declaration,
            NamespaceBodyDeclaration::Namespace,
        ),
    ))
    .context("namespace member")
    .parse(input)
}

/// Public wrapper to allow tools and traits to parse a single namespace body declaration.
pub fn parse_namespace_member_for_spans(input: Span) -> BResult<NamespaceBodyDeclaration> {
    parse_namespace_member_safe(input)
}

/// Parse a C# namespace declaration using proper Nom combinators
/// Example in C#:
/// ```csharp
/// namespace MyCompany.MyProject
/// {
///     public class MyClass { }
/// }
/// ```
pub fn parse_namespace_declaration(input: Span) -> BResult<NamespaceDeclaration> {
    trace!(
        "[DEBUG] parse_namespace_declaration: input = {:?}",
        input.chars().take(60).collect::<String>()
    );

    // Parse the "namespace" keyword
    let (input, _) = delimited(ws, kw_namespace(), ws)
        .context("namespace keyword")
        .parse(input)?;

    // Parse qualified name (e.g., "System.Collections")
    let (input, name_parts) = delimited(ws, parse_qualified_name, ws)
        .context("namespace name")
        .parse(input)?;
    let name_segments: Vec<String> = name_parts
        .iter()
        .map(|id| match id {
            Identifier::Simple(s) => s.clone(),
            Identifier::QualifiedIdentifier(v) => v.join("."),
            Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
        })
        .collect();

    // Parse opening brace
    let (input, _) = delimited(ws, satisfy(|c| c == '{'), ws)
        .context("namespace body opening")
        .parse(input)?;
    trace!("[DEBUG] parse_namespace_declaration: after open brace");

    // Parse using directives inside namespace body (namespace-scoped usings)
    let mut cur = input;
    let mut using_directives: Vec<UsingDirective> = Vec::new();
    loop {
        // consume whitespace/comments between usings
        let (r, _) = ws(cur)?;
        cur = r;
        if peek(delimited(ws, kw_using(), ws)).parse(cur).is_ok() {
            let (r2, u) = delimited(ws, parse_using_directive, ws).parse(cur)?;
            using_directives.push(u);
            cur = r2;
            continue;
        }
        break;
    }

    // Parse namespace members with directive skipping between members
    let mut members: Vec<NamespaceBodyDeclaration> = Vec::new();
    loop {
        // Skip whitespace/comments and any preprocessor directives
        let (r, _) = ws(cur)?;
        cur = r;
        cur = skip_preprocessor_directives(cur, false);

        // Stop at closing brace
        if peek(delimited(ws, satisfy(|c| c == '}'), ws))
            .parse(cur)
            .is_ok()
        {
            break;
        }

        // Try parse a member; break if it doesn't parse
        match parse_namespace_member_safe(cur) {
            Ok((rest, m)) => {
                members.push(m);
                cur = rest;
            }
            Err(_) => break,
        }
    }
    let input_after_members = cur;

    trace!(
        "[DEBUG] parse_namespace_declaration: parsed {} members",
        members.len()
    );

    // Parse closing brace (commit once inside namespace body)
    let (input_final, _) = cut(delimited(ws, satisfy(|c| c == '}'), ws))
        .context("namespace body closing")
        .parse(input_after_members)?;
    trace!("[DEBUG] parse_namespace_declaration: successfully parsed closing brace");

    Ok((
        input_final,
        NamespaceDeclaration {
            name: Identifier::QualifiedIdentifier(name_segments),
            using_directives,      // collected namespace-scoped usings
            declarations: members, // parsed members
        },
    ))
}

// Stub for parsing namespace members - replace with actual member parser
// fn parse_namespace_member_stub<'a>(input: &'a str) -> BResult<&'a str, NamespaceBodyDeclaration<'a>> {
//     // This is highly simplified. A real syntax would try to parse class, struct, enum, interface, nested namespace, etc.
//     // For now, let's assume it consumes nothing and returns a dummy member or an error.
//     Err(nom::Err::Error(syntax::errors::BSharpParseError::new(input, crate::syntax::errors::CustomErrorKind::NotYetImplemented("NamespaceBodyDeclaration"))))
// }
use syntax::span::Span;

