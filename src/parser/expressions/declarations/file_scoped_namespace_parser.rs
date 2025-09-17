use nom::multi::many0;

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
    namespace_declaration::NamespaceBodyDeclaration, FileScopedNamespaceDeclaration,
};
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};
use log::trace;
use nom::branch::alt;

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

    // Parse using directives and type declarations separately
    let (input, (using_directives, type_declarations)) = many0(alt((
        // Parse using directives
        |i| parse_using_directive(i).map(|(rest, using)| (rest, (Some(using), None))),
        // Parse type declarations
        |i| {
            parse_class_declaration(i)
                .map(|(rest, class)| (rest, (None, Some(NamespaceBodyDeclaration::Class(class)))))
        },
        |i| {
            parse_struct_declaration(i).map(|(rest, struct_decl)| {
                (
                    rest,
                    (None, Some(NamespaceBodyDeclaration::Struct(struct_decl))),
                )
            })
        },
        |i| {
            parse_interface_declaration(i).map(|(rest, interface)| {
                (
                    rest,
                    (None, Some(NamespaceBodyDeclaration::Interface(interface))),
                )
            })
        },
        |i| {
            parse_enum_declaration(i).map(|(rest, enum_decl)| {
                (
                    rest,
                    (None, Some(NamespaceBodyDeclaration::Enum(enum_decl))),
                )
            })
        },
        |i| {
            parse_delegate_declaration(i).map(|(rest, delegate)| {
                (
                    rest,
                    (None, Some(NamespaceBodyDeclaration::Delegate(delegate))),
                )
            })
        },
        |i| {
            parse_record_declaration(i).map(|(rest, record)| {
                (rest, (None, Some(NamespaceBodyDeclaration::Record(record))))
            })
        },
    )))(input)
    .map(|(rest, items)| {
        let mut using_dirs = Vec::new();
        let mut type_decls = Vec::new();

        for (using_opt, type_opt) in items {
            if let Some(using) = using_opt {
                using_dirs.push(using);
            }
            if let Some(type_decl) = type_opt {
                type_decls.push(type_decl);
            }
        }

        (rest, (using_dirs, type_decls))
    })?;

    // Convert Vec<Identifier> to a single namespace string
    let namespace_str = name
        .iter()
        .map(|id| id.name.clone())
        .collect::<Vec<_>>()
        .join(".");

    Ok((
        input,
        FileScopedNamespaceDeclaration {
            name: Identifier::new(&namespace_str),
            using_directives,
            declarations: type_declarations,
        },
    ))
}

/// Simplified structure for file-scoped namespace
pub struct FileScoped {
    pub name: Vec<crate::syntax::nodes::identifier::Identifier>,
    pub members: Vec<crate::syntax::nodes::declarations::member_declaration::MemberDeclaration>,
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
    pub namespace: Vec<crate::syntax::nodes::identifier::Identifier>,
}
