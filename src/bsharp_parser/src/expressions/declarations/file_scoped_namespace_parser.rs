use crate::parser::expressions::declarations::delegate_declaration_parser::parse_delegate_declaration;
use crate::parser::expressions::declarations::enum_declaration_parser::parse_enum_declaration;
use crate::parser::expressions::declarations::type_declaration_parser::{
    parse_class_declaration, parse_interface_declaration, parse_record_declaration,
    parse_struct_declaration,
};
use crate::parser::expressions::declarations::using_directive_parser::parse_using_directive;
use crate::parser::identifier_parser::parse_qualified_name;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{
    FileScopedNamespaceDeclaration, MemberDeclaration,
    namespace_declaration::NamespaceBodyDeclaration,
};
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword, peek_keyword};
use log::trace;
use nom::branch::alt;
use nom::combinator::map;

// using directive parsing moved to declarations/using_directive_parser.rs

/// Parse a file-scoped namespace declaration
pub fn parse_file_scoped_namespace_declaration(
    input: &str,
) -> BResult<&str, FileScopedNamespaceDeclaration> {
    trace!(
        "[DEBUG] parse_file_scoped_namespace_declaration: input = {:?}",
        &input[..std::cmp::min(100, input.len())]
    );

    let (input, _) = context(
        "namespace keyword (expected 'namespace')",
        bws(keyword("namespace")),
    )(input)?;
    trace!("[DEBUG] parse_file_scoped_namespace_declaration: after namespace keyword");

    let (input, name) = context(
        "namespace name (expected qualified identifier)",
        bws(parse_qualified_name),
    )(input)?;
    trace!(
        "[DEBUG] parse_file_scoped_namespace_declaration: parsed name = {:?}",
        name
    );

    // Parse the semicolon (this is what makes it file-scoped)
    let (input, _) = context(
        "file-scoped namespace semicolon (expected ';' after namespace name)",
        bws(bchar(';')),
    )(input)?;
    trace!("[DEBUG] parse_file_scoped_namespace_declaration: after semicolon");

    // Parse using directives and type declarations with a manual loop for precise control
    let mut current = input;
    let mut using_directives = Vec::new();
    let mut type_declarations = Vec::new();

    loop {
        // Consume whitespace/comments between items
        let (rest, _) = crate::syntax::comment_parser::parse_whitespace_or_comments(current)?;
        current = rest;

        if current.is_empty() {
            break;
        }

        // Normalize lookahead for keywords (handles intervening comments already via parse_whitespace_or_comments)
        if peek_keyword("using")(current).is_ok() {
            let (rest, using) = parse_using_directive(current)?;
            using_directives.push(using);
            current = rest;
            continue;
        }

        // Attempt to parse a type declaration; stop if nothing matches without consuming input
        match alt((
            map(parse_class_declaration, NamespaceBodyDeclaration::Class),
            map(parse_struct_declaration, NamespaceBodyDeclaration::Struct),
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
        ))(current)
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

    // Convert Vec<Identifier> to a single namespace string
    let namespace_str = name
        .iter()
        .map(|id| id.name.clone())
        .collect::<Vec<_>>()
        .join(".");

    Ok((
        current,
        FileScopedNamespaceDeclaration {
            name: Identifier::new(&namespace_str),
            using_directives,
            declarations: type_declarations,
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
pub fn parse_global_using(input: &str) -> BResult<&str, GlobalUsing> {
    context(
        "global using declaration (expected 'global using' followed by namespace and semicolon)",
        |input| {
            // Parse 'global' keyword
            let (input, _) =
                context("global keyword (expected 'global')", bws(keyword("global")))(input)?;

            // Parse 'using' keyword
            let (input, _) = context(
                "using keyword (expected 'using' after 'global')",
                bws(keyword("using")),
            )(input)?;

            // Parse namespace name
            let (input, namespace) = context(
                "namespace name (expected qualified identifier)",
                bws(parse_qualified_name),
            )(input)?;

            // Parse semicolon
            let (input, _) = context(
                "using semicolon (expected ';' to end global using declaration)",
                bws(bchar(';')),
            )(input)?;

            Ok((input, GlobalUsing { namespace }))
        },
    )(input)
}

/// Simplified structure for global using
pub struct GlobalUsing {
    pub namespace: Vec<Identifier>,
}
