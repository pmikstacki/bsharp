// Tests for parsing indexer declarations

use bsharp::parser::expressions::declarations::indexer_declaration_parser::parse_indexer_declaration;
use bsharp::syntax::nodes::declarations::{IndexerDeclaration, Modifier};
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::types::Type::Primitive;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};
use bsharp::syntax::nodes::statements::statement::Statement;

fn parse_indexer_declaration_helper(code: &str) -> Result<IndexerDeclaration, String> {
    match parse_indexer_declaration(code) {
        Ok((remaining, declaration)) => {
            if remaining.trim().is_empty() {
                Ok(declaration)
            } else {
                Err(format!("Unexpected remaining input: '{}'", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_indexer_mixed_accessor_modifiers_set_private() {
    let code = "public string this[int i] { get; private set; }";
    let decl = parse_indexer_declaration_helper(code).expect("parse ok");
    // Property-level modifier and type
    assert!(decl.modifiers.contains(&Modifier::Public));
    assert_eq!(decl.indexer_type, Type::Primitive(PrimitiveType::String));
    // Accessor modifiers: get -> none, set -> private
    let get_acc = decl.accessor_list.get_accessor.as_ref().expect("get present");
    assert!(get_acc.modifiers.is_empty());
    assert!(get_acc.body.is_none());
    let set_acc = decl.accessor_list.set_accessor.as_ref().expect("set present");
    assert!(set_acc.modifiers.contains(&Modifier::Private));
    assert_eq!(set_acc.modifiers.len(), 1);
    assert!(set_acc.body.is_none());
}

#[test]
fn test_indexer_mixed_accessor_modifiers_get_private() {
    let code = "public string this[int i] { private get; set; }";
    let decl = parse_indexer_declaration_helper(code).expect("parse ok");
    // Property-level modifier and type
    assert!(decl.modifiers.contains(&Modifier::Public));
    assert_eq!(decl.indexer_type, Type::Primitive(PrimitiveType::String));
    // Accessor modifiers: get -> private, set -> none
    let get_acc = decl.accessor_list.get_accessor.as_ref().expect("get present");
    assert!(get_acc.modifiers.contains(&Modifier::Private));
    assert_eq!(get_acc.modifiers.len(), 1);
    assert!(get_acc.body.is_none());
    let set_acc = decl.accessor_list.set_accessor.as_ref().expect("set present");
    assert!(set_acc.modifiers.is_empty());
    assert!(set_acc.body.is_none());
}

#[test]
fn test_indexer_accessor_attrs_with_mixed_modifiers() {
    let code = "public string this[int i] { [A] get; private set; }";
    let decl = parse_indexer_declaration_helper(code).expect("parse ok");
    // Declaration-level checks
    assert!(decl.modifiers.contains(&Modifier::Public));
    assert_eq!(decl.indexer_type, Type::Primitive(PrimitiveType::String));
    // get accessor: attribute [A], no modifiers, no body
    let get_acc = decl.accessor_list.get_accessor.as_ref().expect("get present");
    assert!(get_acc.modifiers.is_empty());
    assert_eq!(get_acc.attributes.len(), 1);
    assert_eq!(get_acc.attributes[0].name.name, "A");
    assert!(get_acc.body.is_none());
    // set accessor: private modifier, no attributes, no body
    let set_acc = decl.accessor_list.set_accessor.as_ref().expect("set present");
    assert!(set_acc.attributes.is_empty());
    assert!(set_acc.modifiers.contains(&Modifier::Private));
    assert_eq!(set_acc.modifiers.len(), 1);
    assert!(set_acc.body.is_none());
}

#[test]
fn test_indexer_accessor_level_attributes_and_modifiers() {
    let code = "public int this[int index] { [A1] private get; [A2][A3] set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse indexer with accessor attrs/mods: {:?}", result);
    let decl = result.unwrap();
    // get accessor present, private, with one attribute A1
    let get_acc = decl.accessor_list.get_accessor.as_ref().expect("get present");
    assert!(get_acc.modifiers.contains(&Modifier::Private));
    assert_eq!(get_acc.attributes.len(), 1);
    assert_eq!(get_acc.attributes[0].name.name, "A1");
    assert!(get_acc.body.is_none());
    // set accessor present, no modifiers, with attributes A2 and A3
    let set_acc = decl.accessor_list.set_accessor.as_ref().expect("set present");
    assert!(set_acc.modifiers.is_empty());
    assert_eq!(set_acc.attributes.len(), 2);
    let names: Vec<_> = set_acc.attributes.iter().map(|a| a.name.name.as_str()).collect();
    assert!(names.contains(&"A2"));
    assert!(names.contains(&"A3"));
    assert!(set_acc.body.is_none());
}

#[test]
fn test_parse_simple_indexer() {
    let code = "public int this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse simple indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.modifiers, vec![Modifier::Public]);
    assert_eq!(
        declaration.indexer_type,
        Type::Primitive(PrimitiveType::Int)
    );
    assert_eq!(declaration.parameters.len(), 1);

    // Check that both get and set accessors are present
    // Auto-accessors have no bodies -> body None
    assert!(declaration.accessor_list.get_accessor.is_some());
    assert!(declaration.accessor_list
        .get_accessor
        .as_ref()
        .unwrap()
        .body
        .is_none());
    assert!(declaration.accessor_list.set_accessor.is_some());
    assert!(declaration
        .accessor_list
        .set_accessor
        .as_ref()
        .unwrap()
        .body
        .is_none());
}

#[test]
fn test_parse_readonly_indexer() {
    let code = "public string this[int index] { get; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse readonly indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(
        declaration.indexer_type,
        Type::Primitive(PrimitiveType::String)
    );
    // Readonly auto get accessor present with no body
    assert!(declaration.accessor_list.get_accessor.is_some());
    assert!(declaration
        .accessor_list
        .get_accessor
        .as_ref()
        .unwrap()
        .body
        .is_none());
    assert!(declaration.accessor_list.set_accessor.is_none());
}

#[test]
fn test_parse_writeonly_indexer() {
    let code = "public string this[int index] { set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse writeonly indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(
        declaration.indexer_type,
        Type::Primitive(PrimitiveType::String)
    );

    // Writeonly auto set accessor present with no body
    assert!(declaration.accessor_list.get_accessor.is_none());
    assert!(declaration.accessor_list.set_accessor.is_some());
    assert!(declaration
        .accessor_list
        .set_accessor
        .as_ref()
        .unwrap()
        .body
        .is_none());
}

#[test]
fn test_parse_multi_parameter_indexer() {
    let code = "public string this[int row, int col] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse multi-parameter indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.parameters.len(), 2);

    // Check parameter types and names
    assert_eq!(
        declaration.parameters[0].parameter_type,
        Primitive(PrimitiveType::Int)
    );
    assert_eq!(declaration.parameters[0].name, Identifier::new("row"));
    assert_eq!(
        declaration.parameters[1].parameter_type,
        Primitive(PrimitiveType::Int)
    );
    assert_eq!(declaration.parameters[1].name, Identifier::new("col"));
}

#[test]
fn test_parse_indexer_with_body() {
    let code =
        "public int this[int index] { get { return _data[index]; } set { _data[index] = value; } }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse indexer with body: {:?}",
        result
    );

    let declaration = result.unwrap();

    // Check that both accessors have bodies (not just semicolons)
    assert!(declaration.accessor_list.get_accessor.is_some());
    assert!(declaration.accessor_list.set_accessor.is_some());

    let get_body = declaration
        .accessor_list
        .get_accessor
        .as_ref()
        .unwrap()
        .body
        .clone()
        .unwrap();
    let set_body = declaration
        .accessor_list
        .set_accessor
        .as_ref()
        .unwrap()
        .body
        .clone()
        .unwrap();

    // Bodies should be parsed as Statement::Block(...)
    assert!(matches!(get_body, Statement::Block(_)));
    assert!(matches!(set_body, Statement::Block(_)));
}

#[test]
fn test_parse_private_indexer() {
    let code = "private object this[string key] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse private indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.modifiers, vec![Modifier::Private]);
    assert_eq!(
        declaration.indexer_type,
        Type::Primitive(PrimitiveType::Object)
    );
}

#[test]
fn test_parse_protected_indexer() {
    let code = "protected internal T this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse protected internal indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Protected));
    assert!(declaration.modifiers.contains(&Modifier::Internal));
}

#[test]
fn test_parse_virtual_indexer() {
    let code = "public virtual string this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse virtual indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Public));
    assert!(declaration.modifiers.contains(&Modifier::Virtual));
}

#[test]
fn test_parse_abstract_indexer() {
    let code = "public abstract string this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse abstract indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Abstract));
}

#[test]
fn test_parse_override_indexer() {
    let code = "public override string this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse override indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Override));
}

#[test]
fn test_parse_indexer_with_attributes() {
    let code = "[Obsolete] public string this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse indexer with attributes: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert!(
        !declaration.attributes.is_empty(),
        "Expected attributes to be parsed"
    );
}

#[test]
fn test_parse_generic_indexer() {
    let code = "public T this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse generic indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(
        declaration.indexer_type,
        Type::Reference(Identifier::new("T"))
    );
}

#[test]
fn test_parse_complex_type_indexer() {
    let code = "public List<string> this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse complex type indexer: {:?}",
        result
    );

    let declaration = result.unwrap();
    // The exact representation of List<string> depends on how generic types are parsed
    // For now, we just check that it parses successfully
    assert_eq!(declaration.parameters.len(), 1);
}

#[test]
fn test_parse_indexer_mixed_parameter_types() {
    let code = "public object this[string key, int version, bool flag] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse indexer with mixed parameter types: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(declaration.parameters.len(), 3);

    assert_eq!(
        declaration.parameters[0].parameter_type,
        Type::Primitive(PrimitiveType::String)
    );
    assert_eq!(
        declaration.parameters[1].parameter_type,
        Primitive(PrimitiveType::Int)
    );
    assert_eq!(
        declaration.parameters[2].parameter_type,
        Primitive(PrimitiveType::Bool)
    );
}
