// Tests for parsing event declarations

use parser::expressions::declarations::event_declaration_parser::parse_event_declaration;
use syntax::declarations::Modifier;

#[test]
fn test_simple_event() {
    let input = "public event EventHandler MyEvent;";
    let (rest, event) = parse_event_declaration(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(event.name.to_string(), "MyEvent");
    assert_eq!(event.modifiers.len(), 1);
    assert_eq!(event.modifiers[0], Modifier::Public);
    assert!(event.accessor_list.is_none());
}

#[test]
fn test_event_with_accessors() {
    let input = "public event EventHandler MyEvent { add; remove; }";
    let (rest, event) = parse_event_declaration(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(event.name.to_string(), "MyEvent");
    assert!(event.accessor_list.is_some());
    let accessors = event.accessor_list.unwrap();
    assert!(accessors.add_accessor.is_some());
    assert!(accessors.remove_accessor.is_some());

    // Check that both accessors have no body (semicolon style)
    assert!(accessors.add_accessor.unwrap().body.is_none());
    assert!(accessors.remove_accessor.unwrap().body.is_none());
}

#[test]
fn test_static_event() {
    let input = "public static event EventHandler StaticEvent;";
    let (rest, event) = parse_event_declaration(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(event.name.to_string(), "StaticEvent");
    assert_eq!(event.modifiers.len(), 2);
    assert!(event.modifiers.contains(&Modifier::Public));
    assert!(event.modifiers.contains(&Modifier::Static));
}

#[test]
fn test_virtual_event() {
    let input = "protected virtual event EventHandler VirtualEvent;";
    let (rest, event) = parse_event_declaration(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(event.name.to_string(), "VirtualEvent");
    assert_eq!(event.modifiers.len(), 2);
    assert!(event.modifiers.contains(&Modifier::Protected));
    assert!(event.modifiers.contains(&Modifier::Virtual));
}

#[test]
fn test_event_with_attributes() {
    let input = "[Obsolete] public event EventHandler AttributedEvent;";
    let (rest, event) = parse_event_declaration(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(event.name.to_string(), "AttributedEvent");
    assert_eq!(event.attributes.len(), 1);
    assert_eq!(event.attributes[0].attributes.len(), 1);
    assert_eq!(
        event.attributes[0].attributes[0].name.to_string(),
        "Obsolete"
    );
}

#[test]
fn test_abstract_event() {
    let input = "public abstract event EventHandler AbstractEvent;";
    let (rest, event) = parse_event_declaration(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(event.name.to_string(), "AbstractEvent");
    assert_eq!(event.modifiers.len(), 2);
    assert!(event.modifiers.contains(&Modifier::Public));
    assert!(event.modifiers.contains(&Modifier::Abstract));
}

#[test]
fn test_override_event() {
    let input = "public override event EventHandler OverrideEvent;";
    let (rest, event) = parse_event_declaration(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "");
    assert_eq!(event.name.to_string(), "OverrideEvent");
    assert_eq!(event.modifiers.len(), 2);
    assert!(event.modifiers.contains(&Modifier::Public));
    assert!(event.modifiers.contains(&Modifier::Override));
}
