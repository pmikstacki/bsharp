use crate::keywords::contextual_misc_keywords::kw_global;
use crate::parser::expressions::declarations::delegate_declaration_parser::parse_delegate_declaration;
use crate::parser::expressions::declarations::enum_declaration_parser::parse_enum_declaration;
use crate::parser::expressions::declarations::type_declaration_parser::{
    parse_class_declaration, parse_interface_declaration, parse_record_declaration,
    parse_struct_declaration_span,
};
use crate::parser::expressions::declarations::using_directive_parser::parse_using_directive;
use crate::parser::identifier_parser::parse_qualified_name;
use crate::parser::keywords::declaration_keywords::{kw_namespace, kw_using};
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use log::trace;
use nom::Parser;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::peek;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use syntax::Identifier;
use syntax::declarations::{
    FileScopedNamespaceDeclaration, MemberDeclaration, NamespaceBodyDeclaration,
};
// using directive parsing moved to declarations/using_directive_parser.rs

/// Parse a file-scoped namespace declaration
pub fn parse_file_scoped_namespace_declaration(
    input: Span,
) -> BResult<FileScopedNamespaceDeclaration> {
    trace!(
        "[DEBUG] parse_file_scoped_namespace_declaration: input = {:?}",
        &input[..std::cmp::min(100, input.len())]
    );

    let (input, _) = delimited(ws, kw_namespace(), ws)
        .context("namespace keyword")
        .parse(input)?;
    trace!("[DEBUG] parse_file_scoped_namespace_declaration: after namespace keyword");

    let (input, name) = delimited(ws, parse_qualified_name, ws)
        .context("namespace name")
        .parse(input)?;
    trace!(
        "[DEBUG] parse_file_scoped_namespace_declaration: parsed name = {:?}",
        name
    );

    // Parse the semicolon (this is what makes it file-scoped)
    let (input, _) = delimited(ws, tok_semicolon(), ws)
        .context("file-scoped namespace semicolon")
        .parse(input)?;
    trace!("[DEBUG] parse_file_scoped_namespace_declaration: after semicolon");

    // Parse using directives and type declarations with a manual loop for precise control
    let mut current = input;
    let mut using_directives = Vec::new();
    let mut type_declarations = Vec::new();

    loop {
        // Consume whitespace/comments between items
        let (rest, _) = crate::trivia::comment_parser::parse_whitespace_or_comments(current)?;
        current = rest;

        if current.is_empty() {
            break;
        }

        // Normalize lookahead for keywords (handles intervening comments already via parse_whitespace_or_comments)
        if peek(delimited(ws, kw_using(), ws)).parse(current).is_ok() {
            let (rest, using) = delimited(ws, parse_using_directive, ws).parse(current)?;
            using_directives.push(using);
            current = rest;
            continue;
        }

        // Attempt to parse a type declaration; stop if nothing matches without consuming input
        match alt((
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
            map(
                parse_delegate_declaration,
                NamespaceBodyDeclaration::Delegate,
            ),
            map(parse_record_declaration, NamespaceBodyDeclaration::Record),
        ))
        .parse(current)
        {
            Ok((rest, decl)) => {
                type_declarations.push(decl);
                current = rest;
                continue;
            }
            Err(nom::Err::Error(_)) => break,
            Err(nom::Err::Failure(e)) => return Err(nom::Err::Failure(e)),
            Err(nom::Err::Incomplete(needed)) => return Err(nom::Err::Incomplete(needed)),
        }
    }

    // Convert Vec<Identifier> to segments, then build Identifier as Simple or Qualified
    let namespace_segments: Vec<String> = name
        .into_iter()
        .map(|id| match id {
            Identifier::Simple(s) => s,
            Identifier::QualifiedIdentifier(v) => v.join("."),
            Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
        })
        .collect();
    let ns_ident = if namespace_segments.len() == 1 {
        Identifier::Simple(namespace_segments[0].clone())
    } else {
        Identifier::QualifiedIdentifier(namespace_segments)
    };

    Ok((
        current,
        FileScopedNamespaceDeclaration {
            name: ns_ident,
            declarations: type_declarations,
            using_directives,
        },
    ))
}

/// Simplified structure for file-scoped namespace
pub struct FileScoped {
    pub name: Vec<Identifier>,
    pub members: Vec<MemberDeclaration>,
}

/// Parse a global using declaration within a file-scoped namespace
/// Example: global using System.Collections.Generic;
pub fn parse_global_using(input: Span) -> BResult<GlobalUsing> {
    use nom_supreme::ParserExt;

    use nom::sequence::delimited;

    (|i| {
        // 'global' 'using'
        let (i, _) = delimited(ws, kw_global(), ws)
            .context("global keyword")
            .parse(i)?;
        let (i, _) = delimited(ws, kw_using(), ws)
            .context("using keyword")
            .parse(i)?;
        // namespace
        let (i, namespace) = delimited(ws, parse_qualified_name, ws)
            .context("namespace name")
            .parse(i)?;
        // semicolon
        let (i, _) = delimited(ws, tok_semicolon(), ws)
            .context("using semicolon")
            .parse(i)?;
        Ok((i, GlobalUsing { namespace }))
    })
    .context("global using declaration")
    .parse(input)
}

/// Simplified structure for global using
pub struct GlobalUsing {
    pub namespace: Vec<Identifier>,
}
use syntax::span::Span;

use crate::tokens::separators::tok_semicolon;
