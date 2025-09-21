// Tests for parsing property declarations

use bsharp::parser::expressions::declarations::property_declaration_parser::parse_property_declaration;
use bsharp::syntax::nodes::declarations::{Modifier, PropertyAccessor, PropertyDeclaration};
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};
use bsharp::syntax::nodes::statements::statement::Statement;
use bsharp::syntax::nodes::expressions::expression::Expression;

fn parse_property_decl_test(code: &str) -> Result<PropertyDeclaration, String> {
    match parse_property_declaration(code) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_auto_property() {
    let code = "int Count { get; set; }";
    let expected = PropertyDeclaration {
        attributes: vec![],
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Count".to_string() },
        accessors: vec![
            PropertyAccessor::Get { modifiers: vec![], attributes: vec![], body: None },
            PropertyAccessor::Set { modifiers: vec![], attributes: vec![], body: None },
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_readonly_auto_property() {
    let code = "string Name { get; }";
    let expected = PropertyDeclaration {
        attributes: vec![],
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::String),
        name: Identifier { name: "Name".to_string() },
        accessors: vec![
            PropertyAccessor::Get { modifiers: vec![], attributes: vec![], body: None }
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_getter_with_body() {
    let code = "int Value { get { return _value; } }";
    let parsed = parse_property_decl_test(code).expect("parse ok");
    assert_eq!(parsed.attributes.len(), 0);
    assert_eq!(parsed.modifiers.len(), 0);
    assert_eq!(parsed.ty, Type::Primitive(PrimitiveType::Int));
    assert_eq!(parsed.name.name, "Value");
    assert_eq!(parsed.accessors.len(), 1);
    match &parsed.accessors[0] {
        PropertyAccessor::Get { body, .. } => assert!(matches!(body, Some(bsharp::syntax::nodes::statements::statement::Statement::Block(_)))),
        _ => panic!("expected get accessor"),
    }
}

#[test]
fn test_parse_property_with_bodies() {
    let code = "int Total { get { return _total; } set { _total = value; } }";
    let parsed = parse_property_decl_test(code).expect("parse ok");
    assert_eq!(parsed.attributes.len(), 0);
    assert_eq!(parsed.modifiers.len(), 0);
    assert_eq!(parsed.ty, Type::Primitive(PrimitiveType::Int));
    assert_eq!(parsed.name.name, "Total");
    assert_eq!(parsed.accessors.len(), 2);
    match &parsed.accessors[0] { PropertyAccessor::Get { body, .. } => assert!(body.is_some()), _ => panic!("expected get") }
    match &parsed.accessors[1] { PropertyAccessor::Set { body, .. } => assert!(body.is_some()), _ => panic!("expected set") }
}

#[test]
fn test_parse_init_only_property() {
    let code = "string Id { get; init; }";
    let expected = PropertyDeclaration {
        attributes: vec![],
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::String),
        name: Identifier { name: "Id".to_string() },
        accessors: vec![
            PropertyAccessor::Get { modifiers: vec![], attributes: vec![], body: None },
            PropertyAccessor::Init { modifiers: vec![], attributes: vec![], body: None },
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_property_with_modifier() {
    let code = "public int Count { get; set; }";
    let expected = PropertyDeclaration {
        attributes: vec![],
        modifiers: vec![Modifier::Public],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Count".to_string() },
        accessors: vec![
            PropertyAccessor::Get { modifiers: vec![], attributes: vec![], body: None },
            PropertyAccessor::Set { modifiers: vec![], attributes: vec![], body: None },
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_property_with_multiple_modifiers() {
    let code = "public static int Count { get; set; }";
    let expected = PropertyDeclaration {
        attributes: vec![],
        modifiers: vec![Modifier::Public, Modifier::Static],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Count".to_string() },
        accessors: vec![
            PropertyAccessor::Get { modifiers: vec![], attributes: vec![], body: None },
            PropertyAccessor::Set { modifiers: vec![], attributes: vec![], body: None },
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_accessor_level_attributes_and_modifiers() {
    let code = "[Attr] public int P { [A1] private get; [A2][A3] set; }";
    let parsed = parse_property_decl_test(code).expect("parse ok");
    // Property-level attribute lists preserved
    assert_eq!(parsed.attributes.len(), 1);
    assert_eq!(parsed.attributes[0].attributes.len(), 1);
    assert_eq!(parsed.attributes[0].attributes[0].name.name, "Attr");
    assert_eq!(parsed.modifiers, vec![Modifier::Public]);
    assert_eq!(parsed.ty, Type::Primitive(PrimitiveType::Int));
    assert_eq!(parsed.name.name, "P");

    // Collect accessors
    let mut saw_get = false;
    let mut saw_set = false;
    for acc in &parsed.accessors {
        match acc {
            PropertyAccessor::Get { modifiers, attributes, body } => {
                saw_get = true;
                // Accessor-level attributes and modifiers preserved
                assert_eq!(attributes.len(), 1);
                assert_eq!(attributes[0].attributes.len(), 1);
                assert_eq!(attributes[0].attributes[0].name.name, "A1");
                assert!(modifiers.contains(&Modifier::Private));
                assert!(body.is_none());
            }
            PropertyAccessor::Set { modifiers, attributes, body } => {
                saw_set = true;
                assert!(modifiers.is_empty());
                assert_eq!(attributes.len(), 2);
                // First list [A2]
                assert_eq!(attributes[0].attributes.len(), 1);
                assert_eq!(attributes[0].attributes[0].name.name, "A2");
                // Second list [A3]
                assert_eq!(attributes[1].attributes.len(), 1);
                assert_eq!(attributes[1].attributes[0].name.name, "A3");
                assert!(body.is_none());
            }
            _ => {}
        }
    }
    assert!(saw_get && saw_set, "expected both get and set accessors");
}

#[test]
fn test_expression_bodied_accessors_get_set() {
    let code = "int X { get => _x; set => _x = value; }";
    let parsed = parse_property_decl_test(code).expect("parse ok");
    assert_eq!(parsed.accessors.len(), 2);
    let mut saw_get = false;
    let mut saw_set = false;
    for acc in &parsed.accessors {
        match acc {
            PropertyAccessor::Get { body, .. } => {
                saw_get = true;
                assert!(matches!(body, Some(Statement::Expression(_))));
            }
            PropertyAccessor::Set { body, .. } => {
                saw_set = true;
                assert!(matches!(body, Some(Statement::Expression(_))));
            }
            _ => {}
        }
    }
    assert!(saw_get && saw_set);
}

#[test]
fn test_expression_bodied_init_accessor() {
    let code = "string Id { init => value; }";
    let parsed = parse_property_decl_test(code).expect("parse ok");
    assert_eq!(parsed.accessors.len(), 1);
    match &parsed.accessors[0] {
        PropertyAccessor::Init { body, .. } => {
            assert!(matches!(body, Some(Statement::Expression(_))));
        }
        _ => panic!("expected init accessor"),
    }
}

#[test]
fn test_expression_bodied_property() {
    let code = "int X => _x;";
    let parsed = parse_property_decl_test(code).expect("parse ok");
    assert_eq!(parsed.attributes.len(), 0);
    assert_eq!(parsed.modifiers.len(), 0);
    assert_eq!(parsed.ty, Type::Primitive(PrimitiveType::Int));
    assert_eq!(parsed.name.name, "X");
    assert_eq!(parsed.accessors.len(), 1);
    match &parsed.accessors[0] {
        PropertyAccessor::Get { body, .. } => {
            assert!(matches!(body, Some(Statement::Expression(_))));
        }
        _ => panic!("expected get accessor for expression-bodied property"),
    }
    assert!(parsed.initializer.is_none());
}

#[test]
fn test_property_attribute_contents() {
    let code = "[Prop1(1), PropTwo] int P { get; }";
    let parsed = parse_property_decl_test(code).expect("parse ok");
    assert_eq!(parsed.attributes.len(), 1);
    let list = &parsed.attributes[0];
    assert_eq!(list.attributes.len(), 2);
    assert_eq!(list.attributes[0].name.name, "Prop1");
    assert_eq!(list.attributes[1].name.name, "PropTwo");
    // Check that first attribute has at least one argument
    assert!(!list.attributes[0].arguments.is_empty());
    // The argument should be an expression (typically a literal)
    assert!(matches!(list.attributes[0].arguments[0], Expression::Literal(_)));
}

#[test]
fn test_expression_bodied_property_with_attrs_mods() {
    let code = "[A] public int X => _x;";
    let parsed = parse_property_decl_test(code).expect("parse ok");
    // Property-level attribute A present
    assert_eq!(parsed.attributes.len(), 1);
    assert_eq!(parsed.attributes[0].attributes.len(), 1);
    assert_eq!(parsed.attributes[0].attributes[0].name.name, "A");
    // Modifier Public present
    assert!(parsed.modifiers.contains(&Modifier::Public));
    // Shape checks
    assert_eq!(parsed.ty, Type::Primitive(PrimitiveType::Int));
    assert_eq!(parsed.name.name, "X");
    assert_eq!(parsed.accessors.len(), 1);
    match &parsed.accessors[0] {
        PropertyAccessor::Get { body, .. } => {
            assert!(matches!(body, Some(Statement::Expression(_))));
        }
        _ => panic!("expected get accessor for expression-bodied property"),
    }
}
