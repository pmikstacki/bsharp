use crate::syntax::comment_parser::ws;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::char as nom_char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::sequence::delimited;
use std::marker::PhantomData;

use crate::syntax::errors::BResult;

use syntax::declarations::{
    Attribute, ClassBodyDeclaration, ClassDeclaration, ConstructorDeclaration, EventDeclaration,
    IndexerDeclaration, InterfaceBodyDeclaration, InterfaceDeclaration, MethodDeclaration,
    Modifier, PropertyAccessor, PropertyDeclaration, RecordDeclaration, StructBodyDeclaration,
    StructDeclaration, TypeDeclaration,
};

use nom_supreme::ParserExt;

// Import specialized parser
use crate::keywords::declaration_keywords::{kw_class, kw_interface, kw_record, kw_struct};
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
pub use crate::parser::expressions::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parser::expressions::declarations::operator_declaration_parser::parse_operator_declaration;
use crate::parser::expressions::declarations::parameter_parser::parse_parameter_list;
use crate::parser::expressions::declarations::property_declaration_parser::parse_property_declaration;
use crate::parser::expressions::declarations::type_parameter_parser::{
    opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses,
};
use crate::parser::helpers::directives::skip_preprocessor_directives;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::parse_mode;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
use syntax::Identifier;
use syntax::declarations::AttributeList;
use syntax::types::{Parameter, Type, TypeParameter};

/// Convert Vec<AttributeList> to Vec<Attribute> by flattening
pub fn convert_attributes(attribute_lists: Vec<AttributeList>) -> Vec<Attribute> {
    attribute_lists
        .into_iter()
        .flat_map(|attr_list| attr_list.attributes)
        .collect()
}

/// Span-native declaration header parser used by class/record Span parsers
fn parse_declaration_header_span<'a, P>(
    input: Span<'a>,
    mut keyword: P,
) -> BResult<'a, DeclarationHeader<'a>>
where
    P: nom::Parser<Span<'a>, Output = &'a str, Error = ErrorTree<Span<'a>>>,
{
    let (i, attributes) = parse_attribute_lists.parse(input)?;
    let (i, modifiers) = parse_modifiers.parse(i)?;
    let (i, _) = delimited(ws, |j| keyword.parse(j), ws).parse(i)?;
    let (i, identifier) = parse_identifier.parse(i)?;
    let (i, type_parameters_opt_opt) = opt(|j| opt_parse_type_parameter_list.parse(j)).parse(i)?;
    let type_parameters = type_parameters_opt_opt.and_then(|x| x);
    let (i, primary_constructor_parameters) = opt(|j| parse_parameter_list.parse(j)).parse(i)?;
    let (i, base_types_opt) = opt(|j| parse_base_type_list.parse(j)).parse(i)?;
    let base_types = base_types_opt.unwrap_or_default();
    Ok((
        i,
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
}

/// Span-native struct declaration
pub fn parse_struct_declaration_span<'a>(input: Span<'a>) -> BResult<'a, StructDeclaration> {
    let (input, header): (Span<'a>, DeclarationHeader<'a>) =
        parse_declaration_header_span(input, kw_struct())?;
    // Optional where-clauses (for generic structs)
    let (input, constraints) =
        opt(|i| delimited(ws, parse_type_parameter_constraints_clauses, ws).parse(i))
            .parse(input)?;
    // Parse the struct body
    let (input, members) = parse_class_body_span(input, parse_struct_member)?;

    Ok((
        input,
        StructDeclaration {
            attributes: header.attributes,
            modifiers: header.modifiers,
            name: header.identifier,
            type_parameters: header.type_parameters,
            primary_constructor_parameters: header.primary_constructor_parameters,
            base_types: header.base_types,
            body_declarations: members,
            constraints: match constraints {
                Some(v) if v.is_empty() => None,
                other => other,
            },
        },
    ))
}

/// Span-native interface declaration
pub fn parse_interface_declaration<'a>(input: Span<'a>) -> BResult<'a, InterfaceDeclaration> {
    // Header
    let (input, header): (Span<'a>, DeclarationHeader<'a>) =
        parse_declaration_header_span(input, kw_interface())?;
    // Optional where-clauses
    let (input, constraints) =
        opt(|i| delimited(ws, parse_type_parameter_constraints_clauses, ws).parse(i))
            .parse(input)?;
    // Body members
    let (input, members) = parse_class_body_span(input, parse_interface_member)?;

    Ok((
        input,
        InterfaceDeclaration {
            attributes: header.attributes,
            modifiers: header.modifiers,
            name: header.identifier,
            type_parameters: header.type_parameters,
            base_types: header.base_types,
            body_declarations: members,
            constraints: match constraints {
                Some(v) if v.is_empty() => None,
                other => other,
            },
        },
    ))
}

/// Public wrapper to allow span-aware tools to parse a single class member.
/// This preserves the internal member parsing order and recovery behavior.
pub fn parse_class_member_for_spans(input: Span) -> BResult<ClassBodyDeclaration> {
    parse_class_member(input)
}

/// Public wrapper to allow span-aware tools to parse a single struct member.
/// Mirrors `parse_struct_member` while keeping its internal visibility.
pub fn parse_struct_member_for_spans(input: Span) -> BResult<StructBodyDeclaration> {
    parse_struct_member(input)
}

/// Public wrapper to allow span-aware tools to parse a single interface member.
/// Mirrors `parse_interface_member` while keeping its internal visibility.
pub fn parse_interface_member_for_spans(input: Span) -> BResult<InterfaceBodyDeclaration> {
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
/// Removed legacy &str-based helper
/// Parse a type declaration (class, struct, interface, record, enum)
pub fn parse_type_declaration(input: Span) -> BResult<TypeDeclaration> {
    alt((
        map(parse_class_declaration, TypeDeclaration::Class),
        map(parse_struct_declaration_span, TypeDeclaration::Struct),
        map(parse_interface_declaration, TypeDeclaration::Interface),
        map(parse_record_declaration, TypeDeclaration::Record),
        map(parse_enum_declaration, TypeDeclaration::Enum),
        map(parse_delegate_declaration, TypeDeclaration::Delegate),
    ))
    .parse(input)
}

/// Span-native class/struct body parser used by new Span-based declaration parsers
pub fn parse_class_body_span<F, M>(input: Span, mut member_parser: F) -> BResult<Vec<M>>
where
    F: FnMut(Span) -> BResult<M>,
{
    let (mut cur, _) = delimited(ws, tok_l_brace(), ws).parse(input)?;
    let mut members: Vec<M> = Vec::new();
    loop {
        // Skip trivia and preprocessor directives inside body
        let (after_ws, _) = ws(cur)?;
        cur = after_ws;
        cur = skip_preprocessor_directives(cur, true);
        // Stop at closing '}'
        if peek(delimited(ws, tok_r_brace(), ws)).parse(cur).is_ok() {
            break;
        }
        // If input exhausted unexpectedly
        if cur.fragment().is_empty() {
            return Err(nom::Err::Failure(ErrorTree::Base {
                location: cur,
                kind: BaseErrorKind::Expected(Expectation::Char('}')),
            }));
        }
        match member_parser(cur) {
            Ok((rest, member)) => {
                members.push(member);
                cur = rest;
            }
            Err(e) => {
                if parse_mode::is_strict() {
                    return Err(e);
                } else {
                    // In lenient mode, attempt to recover by skipping to the next statement terminator or block end
                    // 1) Try to skip up to and including the next ';'
                    if let Ok((after_skipped, _)) =
                        take_until::<&str, Span, ErrorTree<Span>>(";")(cur)
                    {
                        // Consume the semicolon
                        if let Ok((after_semi, _)) =
                            delimited(ws, nom_char(';'), ws).parse(after_skipped)
                        {
                            cur = after_semi;
                            continue;
                        }
                    }
                    // 2) Otherwise, skip up to the next '}' but do not consume it
                    if let Ok((before_rbrace, _)) =
                        take_until::<&str, Span, ErrorTree<Span>>("}")(cur)
                    {
                        cur = before_rbrace;
                        continue;
                    }
                    // 3) As a last resort, break to avoid infinite loop
                    break;
                }
            }
        }
    }
    let (rest, _) = nom::combinator::cut(delimited(ws, tok_r_brace(), ws)).parse(cur)?;
    Ok((rest, members))
}

/// Helper function for parsing class members (fields, methods, properties, constructors, events, indexers, operators, destructors, nested types)
fn parse_class_member(input: Span) -> BResult<ClassBodyDeclaration> {
    use crate::parser::expressions::declarations::method_declaration_parser::parse_member_declaration;

    alt((
        // Try keyword-driven nested type declarations FIRST since they have specific keywords
        // This prevents "record" from being parsed as a return type in parse_member_declaration
        map(parse_record_declaration, ClassBodyDeclaration::NestedRecord),
        map(parse_class_declaration, ClassBodyDeclaration::NestedClass),
        map(parse_struct_declaration_span, ClassBodyDeclaration::NestedStruct),
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
    ))
        .parse(input)
}

/// Parse a member for a struct
fn parse_struct_member(input: Span) -> BResult<StructBodyDeclaration> {
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
            parse_struct_declaration_span,
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
    ))
    .parse(input)
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
pub fn parse_record_class_declaration(input: Span) -> BResult<RecordDeclaration> {
    // attributes, modifiers
    let (input, attributes) = parse_attribute_lists.parse(input)?;
    let (input, modifiers) = parse_modifiers.parse(input)?;
    // 'record' keyword and identifier
    let (input, _) = delimited(ws, kw_record(), ws).parse(input)?;
    let (input, identifier) = parse_identifier.parse(input)?;
    // Optional type parameters
    let (mut input, type_parameters_opt_opt) =
        opt(|i| opt_parse_type_parameter_list.parse(i)).parse(input)?;
    let _type_parameters = type_parameters_opt_opt.and_then(|tp_opt| tp_opt);
    // Optional positional parameters
    let (i_after_params, params_opt) =
        opt(|i| delimited(ws, parse_parameter_list, ws).parse(i)).parse(input)?;
    input = i_after_params;
    let parameters = params_opt.unwrap_or_default();
    // Optional base types
    let (input2, base_opt) =
        opt(|i| delimited(ws, parse_base_type_list, ws).parse(i)).parse(input)?;
    input = input2;
    let base_types = base_opt.unwrap_or_default();
    // Optional where-clauses
    let (input3, constraints_opt) =
        opt(|i| delimited(ws, parse_type_parameter_constraints_clauses, ws).parse(i))
            .parse(input)?;
    input = input3;
    // Decide between ';' or body
    let (input, body_members) = if peek(delimited(ws, tok_semicolon(), ws))
        .parse(input)
        .is_ok()
    {
        let (input, _) = delimited(ws, tok_semicolon(), ws).parse(input)?;
        (input, Vec::<ClassBodyDeclaration>::new())
    } else {
        parse_class_body_span(input, parse_class_member)?
    };

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
        body_declarations: body_members,
        constraints: match constraints_opt {
            Some(v) if v.is_empty() => None,
            other => other,
        },
    };
    Ok((input, record_declaration))
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
pub fn parse_record_struct_declaration(input: Span) -> BResult<RecordDeclaration> {
    // attributes, modifiers, keywords
    let (input, attributes) = parse_attribute_lists.parse(input)?;
    let (input, modifiers) = parse_modifiers.parse(input)?;
    let (input, _) = delimited(ws, kw_record(), ws).parse(input)?;
    let (input, _) = delimited(ws, kw_struct(), ws).parse(input)?;
    // name and optional type params
    let (input, identifier) = parse_identifier.parse(input)?;
    let (mut input, _type_parameters_opt_opt) =
        opt(|i| opt_parse_type_parameter_list.parse(i)).parse(input)?;
    // Optional positional parameters
    let (i_after_params, params_opt) =
        opt(|i| delimited(ws, parse_parameter_list, ws).parse(i)).parse(input)?;
    input = i_after_params;
    let parameters = params_opt.unwrap_or_default();
    // Optional base types
    let (input2, base_opt) =
        opt(|i| delimited(ws, parse_base_type_list, ws).parse(i)).parse(input)?;
    input = input2;
    let base_types = base_opt.unwrap_or_default();
    // Optional where-clauses
    let (input3, constraints_opt) =
        opt(|i| delimited(ws, parse_type_parameter_constraints_clauses, ws).parse(i))
            .parse(input)?;
    input = input3;
    // Decide terminator/body
    let (input, body_members) = if peek(delimited(ws, tok_semicolon(), ws))
        .parse(input)
        .is_ok()
    {
        let (input, _) = delimited(ws, tok_semicolon(), ws).parse(input)?;
        (input, Vec::<ClassBodyDeclaration>::new())
    } else {
        parse_class_body_span(input, parse_class_member)?
    };

    let record_declaration = RecordDeclaration {
        attributes,
        modifiers,
        name: identifier,
        is_struct: true,
        parameters: if parameters.is_empty() {
            None
        } else {
            Some(parameters)
        },
        base_types,
        body_declarations: body_members,
        constraints: match constraints_opt {
            Some(v) if v.is_empty() => None,
            other => other,
        },
    };

    Ok((input, record_declaration))
}

/// Parse a C# record declaration (either record class or record struct)
/// This function tries both forms and returns the first one that matches
pub fn parse_record_declaration(input: Span) -> BResult<RecordDeclaration> {
    // Try parsing as record struct first (more specific)
    if let Ok(result) = parse_record_struct_declaration(input) {
        return Ok(result);
    }

    // If that fails, try parsing as record class
    parse_record_class_declaration(input)
}

/// Parse an interface property declaration
fn parse_interface_property(input: Span) -> BResult<PropertyDeclaration> {
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
                return Err(nom::Err::Error(error_tree));
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
        return Err(nom::Err::Error(error_tree));
    }

    Ok((input, property_decl))
}

/// Parse an interface event declaration
pub fn parse_interface_event(input: Span) -> BResult<EventDeclaration> {
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
        return Err(nom::Err::Error(error_tree));
    }

    Ok((input, event_decl))
}

/// Parse an interface indexer declaration
pub fn parse_interface_indexer(input: Span) -> BResult<IndexerDeclaration> {
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
            return Err(nom::Err::Error(error_tree));
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
            return Err(nom::Err::Error(error_tree));
        }
    }

    Ok((input, indexer_decl))
}

/// Parse an interface member
/// Note: C# 8.0+ allows nested types in interfaces
fn parse_interface_member(input: Span) -> BResult<InterfaceBodyDeclaration> {
    // Try parsing different types of interface members
    // Since alt() from nom uses the first syntax that succeeds,
    // the order matters here - put more specific patterns first
    alt((
        // Nested types (C# 8.0+)
        map(
            parse_class_declaration,
            InterfaceBodyDeclaration::NestedClass,
        ),
        map(
            parse_struct_declaration_span,
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
    ))
        .context("interface member")
        .parse(input)
}

// Removed legacy &str interface wrapper; use `parse_interface_declaration_span`.

/// Parse a C# class declaration
/// This function will be the new implementation, using helpers.
pub fn parse_class_declaration<'a>(input: Span<'a>) -> BResult<'a, ClassDeclaration> {
    let (input, header): (Span<'a>, DeclarationHeader<'a>) =
        parse_declaration_header_span(input, kw_class())?;
    // Optional where-clauses (for generic classes)
    let (input, constraints) =
        opt(|i| delimited(ws, parse_type_parameter_constraints_clauses, ws).parse(i))
            .parse(input)?;
    let (input, members) = parse_class_body_span(input, parse_class_member)?;

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
            constraints: match constraints {
                Some(v) if v.is_empty() => None,
                other => other,
            },
        },
    ))
}
use crate::syntax::span::Span;
use crate::tokens::delimiters::{tok_l_brace, tok_r_brace};
use crate::tokens::separators::tok_semicolon;
