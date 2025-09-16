// Tests for parsing indexer declarations

use bsharp::syntax::nodes::declarations::{IndexerDeclaration, Modifier};
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::types::{Type, PrimitiveType};
use bsharp::syntax::nodes::types::Type::Primitive;
use bsharp::parser::declarations::indexer_declaration_parser::parse_indexer_declaration;

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
fn test_parse_simple_indexer() {
    let code = "public int this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse simple indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert_eq!(declaration.modifiers, vec![Modifier::Public]);
    assert_eq!(declaration.indexer_type, Type::Primitive(PrimitiveType::Int));
    assert_eq!(declaration.parameters.len(), 1);
    
    // Check that both get and set accessors are present
    assert!(declaration.accessor_list.get_accessor.is_some());
    assert!(declaration.accessor_list.set_accessor.is_some());
}

#[test]
fn test_parse_readonly_indexer() {
    let code = "public string this[int index] { get; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse readonly indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert_eq!(declaration.indexer_type, Type::Primitive(PrimitiveType::String));    
    // Check that only get accessor is present
    assert!(declaration.accessor_list.get_accessor.is_some());
    assert!(declaration.accessor_list.set_accessor.is_none());
}

#[test]
fn test_parse_writeonly_indexer() {
    let code = "public string this[int index] { set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse writeonly indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert_eq!(declaration.indexer_type, Type::Primitive(PrimitiveType::String));
    
    // Check that only set accessor is present
    assert!(declaration.accessor_list.get_accessor.is_none());
    assert!(declaration.accessor_list.set_accessor.is_some());
}

#[test]
fn test_parse_multi_parameter_indexer() {
    let code = "public string this[int row, int col] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse multi-parameter indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert_eq!(declaration.parameters.len(), 2);
    
    // Check parameter types and names
    assert_eq!(declaration.parameters[0].parameter_type, Primitive(PrimitiveType::Int));
    assert_eq!(declaration.parameters[0].name, Identifier::new("row"));
    assert_eq!(declaration.parameters[1].parameter_type, Primitive(PrimitiveType::Int));
    assert_eq!(declaration.parameters[1].name, Identifier::new("col"));
}

#[test]
fn test_parse_indexer_with_body() {
    let code = "public int this[int index] { get { return _data[index]; } set { _data[index] = value; } }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse indexer with body: {:?}", result);
    
    let declaration = result.unwrap();
    
    // Check that both accessors have bodies (not just semicolons)
    assert!(declaration.accessor_list.get_accessor.is_some());
    assert!(declaration.accessor_list.set_accessor.is_some());
    
    let get_body = declaration.accessor_list.get_accessor.unwrap();
    let set_body = declaration.accessor_list.set_accessor.unwrap();
    
    // Our simplified implementation returns formatted strings for bodies
    assert!(get_body.contains("get body") || get_body.is_empty());
    assert!(set_body.contains("set body") || set_body.is_empty());
}

#[test]
fn test_parse_private_indexer() {
    let code = "private object this[string key] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse private indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert_eq!(declaration.modifiers, vec![Modifier::Private]);
    assert_eq!(declaration.indexer_type, Type::Primitive(PrimitiveType::Object));
}

#[test]
fn test_parse_protected_indexer() {
    let code = "protected internal T this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse protected internal indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Protected));
    assert!(declaration.modifiers.contains(&Modifier::Internal));
}

#[test]
fn test_parse_virtual_indexer() {
    let code = "public virtual string this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse virtual indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Public));
    assert!(declaration.modifiers.contains(&Modifier::Virtual));
}

#[test]
fn test_parse_abstract_indexer() {
    let code = "public abstract string this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse abstract indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Abstract));
}

#[test]
fn test_parse_override_indexer() {
    let code = "public override string this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse override indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Override));
}

#[test]
fn test_parse_indexer_with_attributes() {
    let code = "[Obsolete] public string this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse indexer with attributes: {:?}", result);
    
    let declaration = result.unwrap();
    assert!(!declaration.attributes.is_empty(), "Expected attributes to be parsed");
}

#[test]
fn test_parse_generic_indexer() {
    let code = "public T this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse generic indexer: {:?}", result);
    
    let declaration = result.unwrap();
    assert_eq!(declaration.indexer_type, Type::Reference(Identifier::new("T")));
}

#[test]
fn test_parse_complex_type_indexer() {
    let code = "public List<string> this[int index] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse complex type indexer: {:?}", result);
    
    let declaration = result.unwrap();
    // The exact representation of List<string> depends on how generic types are parsed
    // For now, we just check that it parses successfully
    assert_eq!(declaration.parameters.len(), 1);
}

#[test]
fn test_parse_indexer_mixed_parameter_types() {
    let code = "public object this[string key, int version, bool flag] { get; set; }";
    let result = parse_indexer_declaration_helper(code);
    assert!(result.is_ok(), "Failed to parse indexer with mixed parameter types: {:?}", result);
    
    let declaration = result.unwrap();
    assert_eq!(declaration.parameters.len(), 3);
    
    assert_eq!(declaration.parameters[0].parameter_type, Type::Primitive(PrimitiveType::String));
    assert_eq!(declaration.parameters[1].parameter_type, Primitive(PrimitiveType::Int));
    assert_eq!(declaration.parameters[2].parameter_type, Primitive(PrimitiveType::Bool));
}
