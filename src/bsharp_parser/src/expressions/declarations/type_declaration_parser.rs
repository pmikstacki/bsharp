use crate::syntax::comment_parser::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::opt;
use nom::sequence::tuple;
use std::marker::PhantomData;

use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::attribute::AttributeList;
use crate::syntax::nodes::declarations::{
    Attribute, ClassBodyDeclaration, ClassDeclaration, ConstructorDeclaration, EventDeclaration,
    IndexerDeclaration, InterfaceBodyDeclaration, InterfaceDeclaration, MethodDeclaration,
    Modifier, PropertyAccessor, PropertyDeclaration, RecordDeclaration, StructBodyDeclaration,
    StructDeclaration, TypeDeclaration,
};
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::{Parameter, Type, TypeParameter};
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

// Import specialized parser
use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::base_types_parser::parse_base_type_list;
use crate::parser::expressions::declarations::delegate_declaration_parser::parse_delegate_declaration;
use crate::parser::expressions::declarations::destructor_declaration_parser::parse_destructor_declaration;
use crate::parser::expressions::declarations::enum_declaration_parser::parse_enum_declaration;
use crate::parser::expressions::declarations::event_declaration_parser::parse_event_declaration;
use crate::parser::expressions::declarations::field_declaration_parser::parse_field_declaration;
use crate::parser::expressions::declarations::indexer_declaration_parser::parse_indexer_declaration;
use crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::operator_declaration_parser::parse_operator_declaration;
use crate::parser::expressions::declarations::parameter_parser::parse_parameter_list;
use crate::parser::expressions::declarations::property_declaration_parser::parse_property_declaration;
use crate::parser::expressions::declarations::type_declaration_helpers::{
    at_end_of_body, skip_to_member_boundary_top_level,
};
use crate::parser::expressions::declarations::type_parameter_parser::opt_parse_type_parameter_list;
use crate::parser::helpers::directives::skip_preprocessor_directives;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::parse_mode;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};

pub use crate::parser::expressions::declarations::modifier_parser::parse_modifiers_for_decl_type;

/// Convert Vec<AttributeList> to Vec<Attribute> by flattening
pub fn convert_attributes(attribute_lists: Vec<AttributeList>) -> Vec<Attribute> {
    attribute_lists
        .into_iter()
        .flat_map(|attr_list| attr_list.attributes)
        .collect()
}

/// Public wrapper to allow span-aware tools to parse a single class member.
/// This preserves the internal member parsing order and recovery behavior.
pub fn parse_class_member_for_spans(input: &str) -> BResult<&str, ClassBodyDeclaration> {
    parse_class_member(input)
}

/// Public wrapper to allow span-aware tools to parse a single struct member.
/// Mirrors `parse_struct_member` while keeping its internal visibility.
pub fn parse_struct_member_for_spans(input: &str) -> BResult<&str, StructBodyDeclaration> {
    parse_struct_member(input)
}

/// Public wrapper to allow span-aware tools to parse a single interface member.
/// Mirrors `parse_interface_member` while keeping its internal visibility.
pub fn parse_interface_member_for_spans(input: &str) -> BResult<&str, InterfaceBodyDeclaration> {
    parse_interface_member(input)
}

/// Common structure holding the parts of a declaration header for any type
pub struct DeclarationHeader<'a> {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub identifier: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub primary_constructor_parameters: Option<Vec<Parameter>>,
    pub base_types: Vec<Type>,
    pub _phantom: PhantomData<&'a ()>,
}

/// Parse the common parts of a declaration header (attributes, modifiers, identifier, type params, base types)
pub fn parse_declaration_header<'a>(
    input: &'a str,
    declaration_keyword: &'static str,
) -> BResult<&'a str, DeclarationHeader<'a>> {
    context(
        "type declaration header (expected attributes, modifiers, keyword, name, and optional type parameters)",
        |input| {
            // Parse attributes (can be empty)
            let (input, attributes) = bws(parse_attribute_lists)(input)?;

            // Parse optional modifiers (public, private, etc.) but NOT the declaration keyword itself
            let (input, modifiers) = bws(parse_modifiers)(input)?;

            // Parse the declaration keyword with word boundary
            let (input, _) = bws(keyword(declaration_keyword))(input)?;

            // Parse the declaration name (identifier). On failure, promote to a committed, clear expectation.
            let (input, identifier) = match bws(parse_identifier)(input) {
                Ok(ok) => ok,
                Err(_) => {
                    return Err(nom::Err::Failure(ErrorTree::Base {
                        location: input,
                        kind: BaseErrorKind::Expected(Expectation::Tag("identifier")),
                    }));
                }
            };

            // Parse optional type parameters like <T> or <K, V>
            let (input, type_parameters_opt_opt) = bws(opt(opt_parse_type_parameter_list))(input)?;
            let type_parameters = type_parameters_opt_opt.and_then(|tp_opt| tp_opt);

            // Parse optional primary constructor parameter list: ( ... )
            let (input, primary_constructor_parameters) = bws(opt(parse_parameter_list))(input)?;

            let (input, base_types_opt) = bws(opt(parse_base_type_list))(input)?;
            let base_types = base_types_opt.unwrap_or_default(); // Use empty vec if no base types

            Ok((
                input,
                DeclarationHeader {
                    attributes,
                    modifiers,
                    identifier,
                    type_parameters,
                    primary_constructor_parameters,
                    base_types,
                    _phantom: PhantomData,
                },
            ))
        },
    )(input)
}

/// Parse a type declaration (class, struct, interface, record, enum)
pub fn parse_type_declaration(input: &str) -> BResult<&str, TypeDeclaration> {
    alt((
        map(parse_class_declaration, TypeDeclaration::Class),
        map(parse_struct_declaration, TypeDeclaration::Struct),
        map(parse_interface_declaration, TypeDeclaration::Interface),
        map(parse_record_declaration, TypeDeclaration::Record),
        map(parse_enum_declaration, TypeDeclaration::Enum),
        map(parse_delegate_declaration, TypeDeclaration::Delegate),
    ))(input)
}
/// Generic function to parse the body of a class-like declaration
/// This includes parsing members between braces
pub fn parse_class_body<F, M>(input: &str, member_parser: F) -> BResult<&str, Vec<M>>
where
    F: Fn(&str) -> BResult<&str, M>,
{
    // Use bdelimited with cut on the closing brace to commit once '{' is seen
    context("class or struct body", |input| {
        crate::syntax::parser_helpers::bdelimited(
            bws(bchar('{')),
            |mut cur: &str| {
                let mut members: Vec<M> = Vec::new();
                loop {
                    // Skip whitespace and comments
                    let (after_ws, _) =
                        crate::syntax::comment_parser::parse_whitespace_or_comments(cur)?;
                    cur = after_ws;

                    // Skip any preprocessor directives inside the type body
                    cur = skip_preprocessor_directives(cur, false);

                    // If we reached EOF inside a type body without a closing '}', emit a clear unmatched-brace error
                    if cur.is_empty() {
                        return Err(nom::Err::Failure(ErrorTree::Base {
                            location: cur,
                            kind: BaseErrorKind::Expected(Expectation::Char('}')),
                        }));
                    }

                    if at_end_of_body(cur) {
                        break;
                    }

                    // Parse a single member; on failure, recover to the next safe boundary.
                    // Recovery uses `skip_to_member_boundary_top_level` which:
                    // - consumes a top-level ';' and returns the slice after it, or
                    // - stops before a top-level '}' (not consumed), or
                    // - returns "" on EOF
                    match member_parser(cur) {
                        Ok((rest, member)) => {
                            members.push(member);
                            cur = rest;
                        }
                        Err(e) => {
                            if parse_mode::is_strict() {
                                // In strict mode, if we're effectively at EOF in a body, prefer an unmatched '}' error
                                if cur.is_empty() {
                                    return Err(nom::Err::Failure(ErrorTree::Base {
                                        location: cur,
                                        kind: BaseErrorKind::Expected(Expectation::Char('}')),
                                    }));
                                }
                                return Err(e);
                            } else {
                                let next = skip_to_member_boundary_top_level(cur);
                                if next.is_empty() || next == cur {
                                    // Cannot recover further; stop to avoid infinite loop
                                    break;
                                }
                                cur = next;
                            }
                        }
                    }
                }
                Ok((cur, members))
            },
            nom::combinator::cut(bws(bchar('}'))),
        )(input)
    })(input)
}

/// Helper function for parsing class members (fields, methods, properties, constructors, events, indexers, operators, destructors, nested types)
fn parse_class_member(input: &str) -> BResult<&str, ClassBodyDeclaration> {
    use crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration;

    alt((
        // Try keyword-driven nested type declarations FIRST since they have specific keywords
        // This prevents "record" from being parsed as a return type in parse_member_declaration
        map(parse_record_declaration, ClassBodyDeclaration::NestedRecord),
        map(parse_class_declaration, ClassBodyDeclaration::NestedClass),
        map(parse_struct_declaration, ClassBodyDeclaration::NestedStruct),
        map(
            parse_interface_declaration,
            ClassBodyDeclaration::NestedInterface,
        ),
        // Try destructor declaration (keyword-driven with ~)
        map(
            parse_destructor_declaration,
            ClassBodyDeclaration::Destructor,
        ),
        // Try the unified member syntax for methods and constructors BEFORE property parsing.
        // This avoids property parser committing on method signatures due to accessor cut.
        map(parse_member_declaration, |member_decl| {
            // Convert unified member declaration to specific types based on parser
            if member_decl.has_constructor_syntax() {
                ClassBodyDeclaration::Constructor(ConstructorDeclaration {
                    modifiers: member_decl.modifiers,
                    name: member_decl.name,
                    parameters: member_decl.parameters,
                    body: member_decl.body,
                    initializer: member_decl.initializer,
                })
            } else {
                // Has method parser (return type present)
                // If it's not a constructor, it must have a return type.
                // If member_decl.return_type is None here, it's an internal logic error
                // in parse_member_declaration's method parsing path.
                ClassBodyDeclaration::Method(MethodDeclaration {
                    modifiers: member_decl.modifiers,
                    return_type: member_decl.return_type.expect(
                        "Internal syntax error: Method identified but no return type found in MemberDeclaration"
                    ),
                    name: member_decl.name,
                    type_parameters: member_decl.type_parameters,
                    parameters: member_decl.parameters,
                    constraints: member_decl.constraints,
                    body: member_decl.body,
                })
            }
        }),
        // Try specialized member parsers that are keyword-driven and less ambiguous
        map(parse_property_declaration, ClassBodyDeclaration::Property),
        map(parse_indexer_declaration, ClassBodyDeclaration::Indexer),
        map(parse_event_declaration, ClassBodyDeclaration::Event),
        map(parse_operator_declaration, ClassBodyDeclaration::Operator),
        // Try enum declaration AFTER method syntax to prevent interference
        map(parse_enum_declaration, ClassBodyDeclaration::NestedEnum),
        // Fields should be last since they have the most generic parser
        map(parse_field_declaration, ClassBodyDeclaration::Field),
    ))(input)
}

/// Parse a member for a struct
fn parse_struct_member(input: &str) -> BResult<&str, StructBodyDeclaration> {
    use crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration;

    // Try parsing different member types in priority order
    alt((
        // Keyword-driven nested type declarations FIRST
        map(
            parse_record_declaration,
            StructBodyDeclaration::NestedRecord,
        ),
        map(parse_class_declaration, StructBodyDeclaration::NestedClass),
        map(
            parse_struct_declaration,
            StructBodyDeclaration::NestedStruct,
        ),
        map(
            parse_interface_declaration,
            StructBodyDeclaration::NestedInterface,
        ),
        // Unified member syntax for methods/constructors before property
        map(parse_member_declaration, |member_decl| {
            if member_decl.has_constructor_syntax() {
                StructBodyDeclaration::Constructor(ConstructorDeclaration {
                    modifiers: member_decl.modifiers,
                    name: member_decl.name,
                    parameters: member_decl.parameters,
                    body: member_decl.body,
                    initializer: member_decl.initializer,
                })
            } else {
                StructBodyDeclaration::Method(MethodDeclaration {
                    modifiers: member_decl.modifiers,
                    return_type: member_decl
                        .return_type
                        .expect("Internal syntax error: method path must have return type"),
                    name: member_decl.name,
                    type_parameters: member_decl.type_parameters,
                    parameters: member_decl.parameters,
                    constraints: member_decl.constraints,
                    body: member_decl.body,
                })
            }
        }),
        // Specialized parsers
        map(parse_property_declaration, StructBodyDeclaration::Property),
        map(parse_indexer_declaration, StructBodyDeclaration::Indexer),
        map(parse_event_declaration, StructBodyDeclaration::Event),
        map(parse_operator_declaration, StructBodyDeclaration::Operator),
        // Fields last (most generic)
        map(parse_field_declaration, StructBodyDeclaration::Field),
    ))(input)
}

/// Parse a C# struct declaration
///
/// Example in C#:
/// ```csharp
/// public struct Point {
///    private int x;
///    private int y;
///    public void Method() { }
/// }
/// ```
pub fn parse_struct_declaration<'a>(input: &'a str) -> BResult<&'a str, StructDeclaration> {
    // Parse the declaration header with the 'struct' keyword
    let (input, header): (&'a str, DeclarationHeader<'a>) =
        parse_declaration_header(input, "struct")?;

    // Parse the struct body
    let (input, members) = parse_class_body(input, parse_struct_member)?;

    // Create a struct declaration
    let struct_declaration = StructDeclaration {
        attributes: header.attributes,
        modifiers: header.modifiers,
        name: header.identifier,
        type_parameters: header.type_parameters,
        primary_constructor_parameters: header.primary_constructor_parameters,
        base_types: header.base_types,
        body_declarations: members,
    };

    Ok((input, struct_declaration))
}

/// Parse record body content - either parameters for positional record or members for body record
fn parse_record_body(input: &str) -> BResult<&str, (Vec<Parameter>, Vec<ClassBodyDeclaration>)> {
    // Parse one of two forms - positional record or body record
    alt((
        // First try to parse as a positional record (with parameters in parentheses)
        map(
            tuple((
                // Parse parameters
                bws(parse_parameter_list),
                // Parse optional semicolon
                opt(bws(bchar(';'))),
            )),
            |(params, _)| (params, Vec::<ClassBodyDeclaration>::new()),
        ),
        // Then try to parse as a body record (with members in braces)
        map(
            |i| parse_class_body(i, parse_class_member),
            |members| (vec![], members),
        ),
    ))(input)
}

/// Parse a C# record class declaration
///
/// Examples in C#:
/// ```csharp
/// public record Person(string FirstName, string LastName);
/// // or
/// public record Person {
///    public string FirstName { get; init; }
///    public string LastName { get; init; }
/// }
/// ```
pub fn parse_record_class_declaration(input: &str) -> BResult<&str, RecordDeclaration> {
    // Parse attributes (can be empty)
    let (input, attributes) = parse_attribute_lists(input)?;

    // Parse optional modifiers (public, private, etc.) but NOT the declaration keyword itself
    let (input, modifiers) = parse_modifiers(input)?;

    // Parse the declaration keyword
    let (input, _) = ws(input)?;
    let (input, _) = keyword("record")(input)?;

    // Parse the declaration name (identifier)
    let (input, _) = ws(input)?;
    let (input, identifier) = parse_identifier(input)?;

    // Parse optional type parameters like <T> or <K, V>
    let (input, _) = ws(input)?;
    let (input, type_parameters_opt_opt) = opt(opt_parse_type_parameter_list)(input)?;
    let _type_parameters = type_parameters_opt_opt.and_then(|tp_opt| tp_opt);

    // Now parse the record body which can be either:
    // 1. (parameters) : base_types;
    // 2. : base_types { members }
    // 3. (parameters);
    // 4. { members }
    let (input, (parameters, base_types, members)) = parse_record_class_body(input)?;

    let record_declaration = RecordDeclaration {
        attributes,
        modifiers,
        name: identifier,
        is_struct: false,
        parameters: if parameters.is_empty() {
            None
        } else {
            Some(parameters)
        },
        base_types,
        body_declarations: members,
    };
    Ok((input, record_declaration))
}

/// Parse record class body which handles the unique record parser
#[allow(clippy::type_complexity)]
fn parse_record_class_body(
    input: &str,
) -> BResult<&str, (Vec<Parameter>, Vec<Type>, Vec<ClassBodyDeclaration>)> {
    // Parse one of four forms:
    // 1. (parameters) : base_types;
    // 2. : base_types { members }
    // 3. { members } (no parameters, no base types)
    alt((
        // Form 1: (parameters) : base_types;
        map(
            tuple((
                bws(parse_parameter_list),
                bws(opt(parse_base_type_list)),
                opt(bws(bchar(';'))),
            )),
            |(params, base_types_opt, _)| {
                (
                    params,
                    base_types_opt.unwrap_or_default(),
                    Vec::<ClassBodyDeclaration>::new(),
                )
            },
        ),
        // Form 2: : base_types { members }
        map(
            tuple((bws(opt(parse_base_type_list)), |i| {
                parse_class_body(i, parse_class_member)
            })),
            |(base_types_opt, members)| (vec![], base_types_opt.unwrap_or_default(), members),
        ),
        // Form 3: { members } (no parameters, no base types)
        map(
            |i| parse_class_body(i, parse_class_member),
            |members| (vec![], vec![], members),
        ),
    ))(input)
}

/// Parse a C# record struct declaration
///
/// Example in C#:
/// ```csharp
/// public record struct Point(int X, int Y);
/// // or
/// public record struct Point {
///    public int X { get; init; }
///    public int Y { get; init; }
/// }
/// ```
pub fn parse_record_struct_declaration(input: &str) -> BResult<&str, RecordDeclaration> {
    // First parse the 'record' keyword
    let (input, _) = ws(input)?;

    // Parse attributes
    let (input, attributes_list) = parse_attribute_lists(input)?; // attributes_list is Vec<AttributeList>

    // Parse modifiers
    let (input, modifiers) = parse_modifiers(input)?;

    // Parse 'record struct' keywords
    let (input, _) = ws(input)?;
    let (input, _) = keyword("record")(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = keyword("struct")(input)?;

    // Parse name
    let (input, _) = ws(input)?;
    let (input, identifier) = parse_identifier(input)?;

    // Parse optional type parameters - but they are not used in RecordDeclaration struct
    let (input, _) = ws(input)?;
    let (input, _type_parameters_opt_opt) = opt(opt_parse_type_parameter_list)(input)?; // Parsed but not used
    // let _type_parameters = _type_parameters_opt_opt.and_then(|tp_opt| tp_opt); // Not used

    // Parse base types
    let (input, _) = ws(input)?;
    let (input, base_types_opt) = opt(parse_base_type_list)(input)?;
    let base_types = base_types_opt.unwrap_or_default(); // Use empty vec if no base types

    // Parse record body
    let (input, (parameters, members)) = parse_record_body(input)?;

    // Create a record declaration
    let record_declaration = RecordDeclaration {
        attributes: attributes_list,
        modifiers,
        name: identifier,
        is_struct: true,
        parameters: if parameters.is_empty() {
            None
        } else {
            Some(parameters)
        },
        base_types,
        body_declarations: members,
    };

    Ok((input, record_declaration))
}

/// Parse a C# record declaration (either record class or record struct)
/// This function tries both forms and returns the first one that matches
pub fn parse_record_declaration(input: &str) -> BResult<&str, RecordDeclaration> {
    // Try parsing as record struct first (more specific)
    if let Ok(result) = parse_record_struct_declaration(input) {
        return Ok(result);
    }

    // If that fails, try parsing as record class
    parse_record_class_declaration(input)
}

/// Parse an interface property declaration
fn parse_interface_property(input: &str) -> BResult<&str, PropertyDeclaration> {
    // Import PropertyDeclaration syntax if it exists
    use crate::parser::expressions::declarations::property_declaration_parser::parse_property_declaration;

    // Parse property declaration
    let (input, property_decl) = parse_property_declaration(input)?;

    // Interface properties cannot have a body implementation.
    // Check each accessor.
    for accessor in &property_decl.accessors {
        match accessor {
            PropertyAccessor::Get { body: Some(_), .. }
            | PropertyAccessor::Set { body: Some(_), .. }
            | PropertyAccessor::Init { body: Some(_), .. } => {
                use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
                let error_tree = ErrorTree::Base {
                    location: input,
                    kind: BaseErrorKind::Expected(Expectation::Tag(
                        "Interface property accessor cannot have a body",
                    )),
                };
                return Err(nom::Err::Failure(error_tree));
            }
            _ => {}
        }
    }
    // Also, interface properties cannot have an initializer
    if property_decl.initializer.is_some() {
        use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
        let error_tree = ErrorTree::Base {
            location: input,
            kind: BaseErrorKind::Expected(Expectation::Tag(
                "Interface property cannot have an initializer",
            )),
        };
        return Err(nom::Err::Failure(error_tree));
    }

    Ok((input, property_decl))
}

/// Parse an interface event declaration
pub fn parse_interface_event(input: &str) -> BResult<&str, EventDeclaration> {
    let (input, event_decl) = parse_event_declaration(input)?;

    // Interface events are typically field-like and must not have accessor bodies.
    // For simplicity, we'll currently ensure no accessors are defined for interface events.
    if event_decl.accessor_list.is_some() {
        use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
        let error_tree = ErrorTree::Base {
            location: input,
            kind: BaseErrorKind::Expected(Expectation::Tag(
                "Interface event cannot have explicit add/remove accessors",
            )),
        };
        return Err(nom::Err::Failure(error_tree));
    }

    Ok((input, event_decl))
}

/// Parse an interface indexer declaration
pub fn parse_interface_indexer(input: &str) -> BResult<&str, IndexerDeclaration> {
    let (input, indexer_decl) = parse_indexer_declaration(input)?;

    // Interface indexer accessors may be present, but cannot have bodies.
    if let Some(get_acc) = &indexer_decl.accessor_list.get_accessor {
        if get_acc.body.is_some() {
            use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
            let error_tree = ErrorTree::Base {
                location: input,
                kind: BaseErrorKind::Expected(Expectation::Tag(
                    "Interface indexer 'get' accessor cannot have a body",
                )),
            };
            return Err(nom::Err::Failure(error_tree));
        }
    }
    if let Some(set_acc) = &indexer_decl.accessor_list.set_accessor {
        if set_acc.body.is_some() {
            use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
            let error_tree = ErrorTree::Base {
                location: input,
                kind: BaseErrorKind::Expected(Expectation::Tag(
                    "Interface indexer 'set' accessor cannot have a body",
                )),
            };
            return Err(nom::Err::Failure(error_tree));
        }
    }

    Ok((input, indexer_decl))
}

/// Parse an interface member
/// Note: C# 8.0+ allows nested types in interfaces
fn parse_interface_member(input: &str) -> BResult<&str, InterfaceBodyDeclaration> {
    // Try parsing different types of interface members
    // Since alt() from nom uses the first syntax that succeeds,
    // the order matters here - put more specific patterns first
    context(
        "interface member",
        alt((
            // Nested types (C# 8.0+)
            map(
                parse_class_declaration,
                InterfaceBodyDeclaration::NestedClass,
            ),
            map(
                parse_struct_declaration,
                InterfaceBodyDeclaration::NestedStruct,
            ),
            map(
                parse_interface_declaration,
                InterfaceBodyDeclaration::NestedInterface,
            ),
            map(parse_enum_declaration, InterfaceBodyDeclaration::NestedEnum),
            map(
                parse_record_declaration,
                InterfaceBodyDeclaration::NestedRecord,
            ),
            // Try parsing methods using unified syntax
            map(
                |i| {
                    let (remaining, member_decl) = parse_member_declaration(i)?;

                    if member_decl.has_constructor_syntax() {
                        // Interfaces cannot have constructors syntactically within an InterfaceBodyDeclaration context.
                        // The unified parse_member_declaration might succeed structurally,
                        // but it doesn't fit the InterfaceBodyDeclaration::Method variant expectation.
                        use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
                        let error_tree = ErrorTree::Base {
                            location: i,
                            kind: BaseErrorKind::Expected(Expectation::Tag(
                                "interface method (not constructor parser)",
                            )),
                        };
                        Err(nom::Err::Error(error_tree))
                    } else {
                        // Has method parser - convert to MethodDeclaration
                        // For interface methods, we always set body to None (error recovery for invalid bodies)
                        let method_decl = MethodDeclaration {
                    modifiers: member_decl.modifiers,
                    return_type: member_decl.return_type.expect(
                        "Internal syntax error: Method parser identified but no return type found in MemberDeclaration"
                    ),
                    name: member_decl.name,
                    type_parameters: member_decl.type_parameters,
                    parameters: member_decl.parameters,
                    constraints: member_decl.constraints,
                    body: None, // Interface methods cannot have bodies - set to None for error recovery
                };
                        Ok((remaining, method_decl))
                    }
                },
                InterfaceBodyDeclaration::Method,
            ),
            // Try parsing properties
            map(parse_interface_property, InterfaceBodyDeclaration::Property),
            // Try parsing events
            map(parse_interface_event, InterfaceBodyDeclaration::Event),
            // Try parsing indexers
            map(parse_interface_indexer, InterfaceBodyDeclaration::Indexer),
        )),
    )(input)
}

/// Parse an interface declaration
pub fn parse_interface_declaration<'a>(input: &'a str) -> BResult<&'a str, InterfaceDeclaration> {
    // Parse the declaration header with the 'interface' keyword
    let (input, header): (&'a str, DeclarationHeader<'a>) =
        parse_declaration_header(input, "interface")?;

    // Parse the interface body - similar to class body but with interface members
    let (input, members) = parse_class_body(input, parse_interface_member)?;

    // Create the InterfaceDeclaration with the correct field names and flatten attributes
    let interface_declaration = InterfaceDeclaration {
        attributes: header.attributes,
        modifiers: header.modifiers,
        name: header.identifier,
        type_parameters: header.type_parameters,
        base_types: header.base_types,
        body_declarations: members,
    };

    Ok((input, interface_declaration))
}

/// Parse a C# class declaration
/// This function will be the new implementation, using helpers.
pub fn parse_class_declaration<'a>(input: &'a str) -> BResult<&'a str, ClassDeclaration> {
    let (input, header): (&'a str, DeclarationHeader<'a>) =
        parse_declaration_header(input, "class")?;
    let (input, members) = parse_class_body(input, parse_class_member)?;

    Ok((
        input,
        ClassDeclaration {
            attributes: header.attributes,
            modifiers: header.modifiers,
            name: header.identifier,
            type_parameters: header.type_parameters,
            primary_constructor_parameters: header.primary_constructor_parameters,
            base_types: header.base_types,
            body_declarations: members,
            documentation: None,
        },
    ))
}
