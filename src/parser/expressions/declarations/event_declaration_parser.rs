use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::statement_parser::parse_statement;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{EventAccessor, EventAccessorList, EventDeclaration};
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

use nom::combinator::cut;
use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::{delimited, tuple},
};

/// Parse an event accessor (add or remove)
fn parse_event_accessor(input: &str) -> BResult<&str, (String, EventAccessor)> {
    context(
        "event accessor",
        alt((
            map(
                tuple((
                    keyword("add"),
                    alt((
                        map(bws(bchar(';')), |_| None),
                        map(bws(parse_statement), |stmt| Some(stmt)),
                    )),
                )),
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
                tuple((
                    keyword("remove"),
                    alt((
                        map(bws(bchar(';')), |_| None),
                        map(bws(parse_statement), |stmt| Some(stmt)),
                    )),
                )),
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
        )),
    )(input)
}

/// Parse event accessor list: { add; remove; } or { add { ... } remove { ... } }
fn parse_event_accessor_list(input: &str) -> BResult<&str, EventAccessorList> {
    context(
        "event accessor list",
        map(
            delimited(
                bws(bchar('{')),
                tuple((
                    opt(bws(parse_event_accessor)),
                    opt(bws(parse_event_accessor)),
                )),
                cut(bws(bchar('}'))),
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
        ),
    )(input)
}

/// Parse an event declaration
/// Examples:
/// - public event EventHandler MyEvent;
/// - public event EventHandler MyEvent { add; remove; }
/// - public event EventHandler MyEvent { add { ... } remove { ... } }
pub fn parse_event_declaration(input: &str) -> BResult<&str, EventDeclaration> {
    context(
        "event declaration",
        map(
            tuple((
                // 1. Attributes
                parse_attribute_lists,
                // 2. Modifiers
                parse_modifiers,
                // 3. 'event' keyword
                keyword("event"),
                // 4. Event type
                bws(parse_type_expression),
                // 5. Event name
                bws(parse_identifier),
                // 6. Optional accessor list or semicolon
                alt((
                    map(bws(parse_event_accessor_list), |list| Some(list)),
                    map(bws(bchar(';')), |_| None),
                )),
            )),
            |(attributes, modifiers, _event_kw, event_type, name, accessor_list)| {
                EventDeclaration {
                    attributes,
                    modifiers,
                    event_type,
                    name,
                    accessor_list,
                }
            },
        ),
    )(input)
}
