use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::accessor_keywords::{kw_add, kw_remove};
use crate::parser::keywords::declaration_keywords::kw_event;
use crate::parser::statement_parser::parse_statement_ws_spanned;
use crate::parser::types::type_parser::parse_type_expression;
use crate::errors::BResult;

use crate::trivia::comment_parser::ws;

use nom::Parser;
use nom::character::complete::satisfy;
use nom::combinator::cut;
use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::delimited,
};
use nom_supreme::ParserExt;
use syntax::declarations::{EventAccessor, EventAccessorList, EventDeclaration};

/// Parse an event accessor (add or remove)
fn parse_event_accessor(input: Span) -> BResult<(String, EventAccessor)> {
    alt((
        map(
            (
                delimited(ws, kw_add(), ws),
                alt((
                    map(delimited(ws, tok_semicolon(), ws), |_| None),
                    delimited(ws, parse_statement_ws_spanned, ws)
                        .map(|s| Some(s.node)),
                )),
            ),
            |(_, body)| {
                (
                    "add".to_string(),
                    EventAccessor {
                        attributes: vec![],
                        modifiers: vec![],
                        body,
                    },
                )
            },
        ),
        map(
            (
                delimited(ws, kw_remove(), ws),
                alt((
                    map(delimited(ws, tok_semicolon(), ws), |_| None),
                    delimited(ws, parse_statement_ws_spanned, ws)
                        .map(|s| Some(s.node)),
                )),
            ),
            |(_, body)| {
                (
                    "remove".to_string(),
                    EventAccessor {
                        attributes: vec![],
                        modifiers: vec![],
                        body,
                    },
                )
            },
        ),
    ))
    .context("event accessor")
    .parse(input)
}

/// Parse event accessor list: { add; remove; } or { add { ... } remove { ... } }
fn parse_event_accessor_list(input: Span) -> BResult<EventAccessorList> {
    map(
        delimited(
            delimited(ws, satisfy(|c| c == '{'), ws).context("event accessor list opening"),
            (
                opt(delimited(ws, parse_event_accessor, ws)),
                opt(delimited(ws, parse_event_accessor, ws)),
            ),
            cut(delimited(ws, satisfy(|c| c == '}'), ws)).context("event accessor list closing"),
        ),
        |(first, second)| {
            let mut add_accessor = None;
            let mut remove_accessor = None;

            // Process first accessor
            if let Some((accessor_type, accessor)) = first {
                if accessor_type == "add" {
                    add_accessor = Some(accessor);
                } else if accessor_type == "remove" {
                    remove_accessor = Some(accessor);
                }
            }

            // Process second accessor
            if let Some((accessor_type, accessor)) = second {
                if accessor_type == "add" {
                    add_accessor = Some(accessor);
                } else if accessor_type == "remove" {
                    remove_accessor = Some(accessor);
                }
            }

            EventAccessorList {
                add_accessor,
                remove_accessor,
            }
        },
    )
    .context("event accessor list")
    .parse(input)
}

/// Parse an event declaration
/// Examples:
/// - public event EventHandler MyEvent;
/// - public event EventHandler MyEvent { add; remove; }
/// - public event EventHandler MyEvent { add { ... } remove { ... } }
pub fn parse_event_declaration(input: Span) -> BResult<EventDeclaration> {
    map(
        (
            // 1. Attributes
            delimited(ws, parse_attribute_lists, ws),
            // 2. Modifiers
            delimited(ws, parse_modifiers, ws),
            // 3. 'event' keyword
            delimited(ws, kw_event(), ws),
            // 4. Event type
            delimited(ws, parse_type_expression, ws),
            // 5. Event name
            delimited(ws, parse_identifier, ws),
            // 6. Optional accessor list or semicolon
            alt((
                map(delimited(ws, parse_event_accessor_list, ws), Some),
                map(delimited(ws, tok_semicolon(), ws), |_| None),
            )),
        ),
        |(attributes, modifiers, _event_kw, event_type, name, accessor_list)| EventDeclaration {
            attributes,
            modifiers,
            event_type,
            name,
            accessor_list,
        },
    )
    .context("event declaration")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::separators::tok_semicolon;
