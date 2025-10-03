use parser::expressions::declarations::type_declaration_parser::{parse_class_declaration, parse_struct_declaration};
use syntax::nodes::declarations::{ClassDeclaration, StructDeclaration};
use syntax::nodes::identifier::Identifier;
use syntax::nodes::types::{Parameter, Type};

#[test]
fn test_class_primary_constructor_parameters() {
    let input = "class Point(int X, int Y) { }";
    let (rest, decl) = parse_class_declaration(input).expect("parse ok");
    assert!(rest.trim().is_empty());
    let ClassDeclaration { name, primary_constructor_parameters, .. } = decl;
    assert_eq!(name, Identifier::new("Point"));
    let params = primary_constructor_parameters.expect("expected parameters");
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].name, Identifier::new("X"));
    assert_eq!(params[1].name, Identifier::new("Y"));
    assert!(matches!(params[0].parameter_type, Type::Primitive(_)));
}

#[test]
fn test_struct_primary_constructor_parameters() {
    let input = "struct Size(int W, int H) { }";
    let (rest, decl) = parse_struct_declaration(input).expect("parse ok");
    assert!(rest.trim().is_empty());
    let StructDeclaration { name, primary_constructor_parameters, .. } = decl;
    assert_eq!(name, Identifier::new("Size"));
    let params = primary_constructor_parameters.expect("expected parameters");
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].name, Identifier::new("W"));
    assert_eq!(params[1].name, Identifier::new("H"));
}
