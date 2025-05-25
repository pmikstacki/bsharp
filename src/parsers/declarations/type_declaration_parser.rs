use nom::branch::alt;
use nom::combinator::map;
use nom::character::complete::multispace0;
use nom::bytes::complete::tag as nom_tag;
use nom::combinator::opt;
use nom::sequence::tuple;
use std::marker::PhantomData;

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::Attribute;
use crate::parser::nodes::declarations::attribute::AttributeList;
use crate::parser::nodes::declarations::{ClassDeclaration, StructDeclaration, InterfaceDeclaration, RecordDeclaration, PropertyDeclaration, EventDeclaration, IndexerDeclaration, InterfaceBodyDeclaration, PropertyAccessor};
use crate::parser::nodes::declarations::{TypeDeclaration, ClassBodyDeclaration, StructBodyDeclaration};
use crate::parser::nodes::types::{Type, TypeParameter, Parameter};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::declarations::Modifier;
use crate::parser::parser_helpers::{nom_to_bs, bws};

// Import specialized parsers
// use crate::parsers::declarations::class_declaration_parser::parse_class_declaration; // Removed old import
// use crate::parsers::declarations::interface_declaration_parser::parse_interface_declaration; // Already removed
// use crate::parsers::declarations::record_declaration_parser::{parse_record_declaration, parse_record_class_declaration, parse_record_struct_declaration}; // Removed, handled in this file
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::declarations::type_parameter_parser::opt_parse_type_parameter_list;
use crate::parsers::declarations::base_types_parser::parse_base_type_list;
use crate::parsers::declarations::type_declaration_helpers::{at_end_of_body, parse_close_brace};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::declarations::field_declaration_parser::parse_field_declaration;
use crate::parsers::declarations::method_declaration_parser::parse_method_declaration;
use crate::parsers::declarations::parameter_parser::parse_parameter_list;
use crate::parsers::declarations::property_declaration_parser::parse_property_declaration;
use crate::parsers::declarations::constructor_declaration_parser::parse_constructor_declaration;
use crate::parsers::declarations::event_declaration_parser::parse_event_declaration;
use crate::parsers::declarations::indexer_declaration_parser::parse_indexer_declaration;

// Re-export the specific type parsers that are now consolidated or managed by this module
pub use crate::parsers::declarations::enum_declaration_parser::parse_enum_declaration;
// Add other re-exports here as needed, e.g.:
// pub use crate::parsers::declarations::class_declaration_parser::parse_class_declaration; // (if it were separate)

pub use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;

/// Convert Vec<AttributeList> to Vec<Attribute> by flattening
pub fn convert_attributes(attribute_lists: Vec<AttributeList>) -> Vec<Attribute> {
    attribute_lists
        .into_iter()
        .flat_map(|attr_list| attr_list.attributes)
        .collect()
}

/// Common structure holding the parts of a declaration header for any type
pub struct DeclarationHeader<'a> {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub identifier: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>,
    pub _phantom: PhantomData<&'a ()>,
}

/// Parse the common parts of a declaration header (attributes, modifiers, identifier, type params, base types)
pub fn parse_declaration_header<'a>(input: &'a str, declaration_keyword: &'a str) -> BResult<&'a str, DeclarationHeader<'a>> {
    // Parse attributes (can be empty)
    let (input, attributes) = parse_attribute_lists(input)?;
    
    // Parse optional modifiers (public, private, etc.) but NOT the declaration keyword itself
    let (input, modifiers) = parse_modifiers(input)?;
    
    // Parse the declaration keyword
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, _) = nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>(declaration_keyword))(input)?;
    
    // Parse the declaration name (identifier)
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, identifier) = nom_to_bs(parse_identifier)(input)?;
    
    // Parse optional type parameters like <T> or <K, V>
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, type_parameters_opt_opt) = opt(nom_to_bs(opt_parse_type_parameter_list))(input)?;
    let type_parameters = type_parameters_opt_opt.and_then(|tp_opt| tp_opt);
    
    // Parse optional base type list (interfaces and/or base class)
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, base_types) = parse_base_type_list(input)?;
    
    Ok((input, DeclarationHeader {
        attributes,
        modifiers,
        identifier,
        type_parameters,
        base_types,
        _phantom: PhantomData,
    }))
}

/// Parse a type declaration (class, struct, interface, record, enum)
pub fn parse_type_declaration(input: &str) -> BResult<&str, TypeDeclaration> {
    alt((
        map(parse_class_declaration, TypeDeclaration::Class),
        map(parse_struct_declaration, TypeDeclaration::Struct),
        map(parse_interface_declaration, TypeDeclaration::Interface),
        map(parse_record_declaration, TypeDeclaration::Record),
        map(parse_enum_declaration, TypeDeclaration::Enum),
    ))(input)
}

/// Generic function to parse the body of a class-like declaration
/// This includes parsing members between braces
pub fn parse_class_body<F, M>(input: &str, member_parser: F) -> BResult<&str, Vec<M>>
where
    F: Fn(&str) -> BResult<&str, M>,
{
    // Parse the opening brace
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, _) = nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("{"))(input)?;
    
    // Keep parsing members until we hit the closing brace
    let mut members = Vec::new();
    let mut current = input;
    
    while !at_end_of_body(current) {
        match member_parser(current) {
            Ok((rest, member)) => {
                members.push(member);
                current = rest;
            },
            Err(_) => break, // Break if we can't parse more members
        }
    }
    
    // Parse the closing brace
    let (input, _) = parse_close_brace(current)?;
    
    Ok((input, members))
}

/// Helper function for parsing class members (fields, methods, properties, constructors, events, indexers)
fn parse_class_member(input: &str) -> BResult<&str, ClassBodyDeclaration> {
    alt((
        map(parse_field_declaration, ClassBodyDeclaration::Field),
        map(parse_method_declaration, ClassBodyDeclaration::Method),
        map(parse_property_declaration, ClassBodyDeclaration::Property),
        map(parse_constructor_declaration, ClassBodyDeclaration::Constructor),
        map(parse_event_declaration, ClassBodyDeclaration::Event),
        map(parse_indexer_declaration, ClassBodyDeclaration::Indexer),
        // TODO: Add other class members like operators, nested types etc.
    ))(input)
}

/// Parse a member for a struct
fn parse_struct_member(input: &str) -> BResult<&str, StructBodyDeclaration> {
    // Try parsing different member types in priority order
    alt((
        map(parse_field_declaration, StructBodyDeclaration::Field),
        map(parse_method_declaration, StructBodyDeclaration::Method),
        map(parse_property_declaration, StructBodyDeclaration::Property),
        map(parse_constructor_declaration, StructBodyDeclaration::Constructor),
        // TODO: Add other members like events, indexers, operators, nested types
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
    let (input, header): (&'a str, DeclarationHeader<'a>) = parse_declaration_header(input, "struct")?;
    
    // Parse the struct body
    let (input, members) = parse_class_body(input, parse_struct_member)?;
    
    // Create a struct declaration
    let struct_declaration = StructDeclaration {
        attributes: header.attributes,
        modifiers: header.modifiers,
        name: header.identifier, 
        type_parameters: header.type_parameters, 
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
                bws(nom_to_bs(parse_parameter_list)),
                // Parse optional semicolon
                opt(bws(nom_to_bs(nom::character::complete::char::<&str, nom::error::Error<&str>>(';')))),
            )),
            |(params, _)| (params, Vec::<ClassBodyDeclaration>::new())
        ),
        
        // Then try to parse as a body record (with members in braces)
        map(
            |i| parse_class_body(i, parse_class_member),
            |members| (vec![], members)
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
pub fn parse_record_class_declaration<'a>(input: &'a str) -> BResult<&'a str, RecordDeclaration> {
    let (input, header): (&'a str, DeclarationHeader<'a>) = parse_declaration_header(input, "record")?;
    let (input, (parameters, members)) = parse_record_body(input)?;
    let record_declaration = RecordDeclaration {
        attributes: header.attributes,
        modifiers: header.modifiers,
        name: header.identifier,
        is_struct: false,
        parameters: Some(parameters),
        base_types: header.base_types,
        body_declarations: members,
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
pub fn parse_record_struct_declaration(input: &str) -> BResult<&str, RecordDeclaration> {
    // First parse the 'record' keyword
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    
    // Parse attributes
    let (input, attributes_list) = parse_attribute_lists(input)?; // attributes_list is Vec<AttributeList>
    
    // Parse modifiers
    let (input, modifiers) = parse_modifiers(input)?;
    
    // Parse 'record struct' keywords
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, _) = nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("record"))(input)?;
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, _) = nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("struct"))(input)?;
    
    // Parse name
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, identifier) = nom_to_bs(parse_identifier)(input)?;
    
    // Parse optional type parameters - but they are not used in RecordDeclaration struct
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, _type_parameters_opt_opt) = opt(nom_to_bs(opt_parse_type_parameter_list))(input)?; // Parsed but not used
    // let _type_parameters = _type_parameters_opt_opt.and_then(|tp_opt| tp_opt); // Not used
    
    // Parse base types
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, base_types) = parse_base_type_list(input)?;
    
    // Parse record body
    let (input, (parameters, members)) = parse_record_body(input)?;
    
    // Create a record declaration
    let record_declaration = RecordDeclaration {
        attributes: attributes_list, 
        modifiers,
        name: identifier,
        is_struct: true, 
        parameters: Some(parameters),
        // type_parameters, // Removed, RecordDeclaration has no such field
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
    // Import PropertyDeclaration parser if it exists
    use crate::parsers::declarations::property_declaration_parser::parse_property_declaration;
    
    // Parse property declaration
    let (input, property_decl) = parse_property_declaration(input)?;
    
    // Interface properties cannot have a body implementation.
    // Check each accessor.
    for accessor in &property_decl.accessors {
        match accessor {
            PropertyAccessor::Get(Some(_)) |
            PropertyAccessor::Set(Some(_)) |
            PropertyAccessor::Init(Some(_)) => {
                return Err(nom::Err::Failure(crate::parser::errors::BSharpParseError::new(input, crate::parser::errors::CustomErrorKind::Expected("Interface property accessor cannot have a body"))));
            }
            _ => {}
        }
    }
    // Also, interface properties cannot have an initializer
    if property_decl.initializer.is_some() {
        return Err(nom::Err::Failure(crate::parser::errors::BSharpParseError::new(input, crate::parser::errors::CustomErrorKind::Expected("Interface property cannot have an initializer"))));
    }
    
    Ok((input, property_decl))
}

/// Parse an interface event declaration
pub fn parse_interface_event(input: &str) -> BResult<&str, EventDeclaration> {
    let (input, event_decl) = parse_event_declaration(input)?;

    // Interface events are typically field-like and must not have accessor bodies.
    // For simplicity, we'll currently ensure no accessors are defined for interface events.
    if event_decl.accessor_list.is_some() {
        return Err(nom::Err::Failure(crate::parser::errors::BSharpParseError::new(input, crate::parser::errors::CustomErrorKind::Expected("Interface event cannot have explicit add/remove accessors"))));
    }
    
    Ok((input, event_decl))
}

/// Parse an interface indexer declaration
pub fn parse_interface_indexer(input: &str) -> BResult<&str, IndexerDeclaration> {
    let (input, indexer_decl) = parse_indexer_declaration(input)?;

    // Interface indexer accessors cannot have a body.
    // If get_accessor or set_accessor is Some, it implies a body/signature, which is disallowed for interfaces.
    if indexer_decl.accessor_list.get_accessor.is_some() ||
       indexer_decl.accessor_list.set_accessor.is_some() {
        return Err(nom::Err::Failure(crate::parser::errors::BSharpParseError::new(input, crate::parser::errors::CustomErrorKind::Expected("Interface indexer accessor cannot have a body"))));
    }
    
    Ok((input, indexer_decl))
}

/// Parse an interface member
fn parse_interface_member(input: &str) -> BResult<&str, InterfaceBodyDeclaration> {
    // Try parsing different types of interface members
    // Since alt() from nom uses the first parser that succeeds,
    // the order matters here - put more specific patterns first
    alt((
        // Try parsing methods
        map(|i| {
            let (remaining, method_decl) = parse_method_declaration(i)?;
            
            // Interface methods cannot have a body
            if method_decl.body.is_some() {
                return Err(nom::Err::Failure(crate::parser::errors::BSharpParseError::new(i, crate::parser::errors::CustomErrorKind::Expected("interface method cannot have a body"))));
            }
            
            Ok((remaining, method_decl))
        }, InterfaceBodyDeclaration::Method),
        
        // Try parsing properties
        map(parse_interface_property, InterfaceBodyDeclaration::Property),
        
        // Try parsing events
        map(parse_interface_event, InterfaceBodyDeclaration::Event),
        
        // Try parsing indexers
        map(parse_interface_indexer, InterfaceBodyDeclaration::Indexer),
    ))(input)
}

/// Parse an interface declaration
pub fn parse_interface_declaration<'a>(input: &'a str) -> BResult<&'a str, InterfaceDeclaration> {
    // Parse the declaration header with the 'interface' keyword
    let (input, header): (&'a str, DeclarationHeader<'a>) = parse_declaration_header(input, "interface")?;
    
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
    let (input, header): (&'a str, DeclarationHeader<'a>) = parse_declaration_header(input, "class")?;
    let (input, members) = parse_class_body(input, parse_class_member)?;

    Ok((
        input,
        ClassDeclaration {
            attributes: header.attributes,
            modifiers: header.modifiers,
            name: header.identifier,
            type_parameters: header.type_parameters,
            base_types: header.base_types, 
            body_declarations: members,
            documentation: None, 
        },
    ))
}
