use parser::expressions::declarations::delegate_declaration_parser::parse_delegate_declaration;
use syntax::declarations::Modifier;
use syntax::types::{PrimitiveType, Type};

#[test]
fn test_simple_delegate() {
    let input = "public delegate void MyDelegate();";
    let (rest, delegate) = parse_delegate_declaration(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(delegate.name.name, "MyDelegate");
    assert_eq!(delegate.modifiers.len(), 1);
    assert_eq!(delegate.modifiers[0], Modifier::Public);
    assert_eq!(delegate.parameters.len(), 0);
    assert!(delegate.type_parameters.is_empty());
    assert!(delegate.constraints.is_none());
}

#[test]
fn test_delegate_with_parameters() {
    let input = "public delegate int Calculate(int x, int y);";
    let (rest, delegate) = parse_delegate_declaration(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(delegate.name.name, "Calculate");
    assert_eq!(delegate.parameters.len(), 2);
    assert_eq!(delegate.parameters[0].name.name, "x");
    assert_eq!(delegate.parameters[1].name.name, "y");

    // Check return type is int
    if let Type::Primitive(PrimitiveType::Int) = delegate.return_type {
        // Expected
    } else {
        panic!("Expected int return type");
    }
}

#[test]
fn test_generic_delegate() {
    let input = "public delegate T MyGenericDelegate<T>(T input);";
    let (rest, delegate) = parse_delegate_declaration(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(delegate.name.name, "MyGenericDelegate");
    assert_eq!(delegate.type_parameters.len(), 1);
    assert_eq!(delegate.type_parameters[0].name.name, "T");
    assert_eq!(delegate.parameters.len(), 1);
    assert_eq!(delegate.parameters[0].name.name, "input");
}

#[test]
fn test_delegate_with_constraints() {
    let input = "public delegate T MyDelegate<T>(T input) where T : class;";
    let (rest, delegate) = parse_delegate_declaration(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(delegate.name.name, "MyDelegate");
    assert_eq!(delegate.type_parameters.len(), 1);
    assert_eq!(delegate.type_parameters[0].name.name, "T");
    assert_eq!(delegate.parameters.len(), 1);
    assert_eq!(delegate.parameters[0].name.name, "input");
    assert!(delegate.constraints.is_some());
    let constraints = delegate.constraints.unwrap();
    assert_eq!(constraints.len(), 1);
    assert_eq!(constraints[0].type_param.name, "T");
    assert_eq!(constraints[0].constraints.len(), 1);
}

#[test]
fn test_delegate_with_multiple_modifiers() {
    let input = "public unsafe delegate void UnsafeDelegate();";
    let (rest, delegate) = parse_delegate_declaration(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(delegate.name.name, "UnsafeDelegate");
    assert_eq!(delegate.modifiers.len(), 2);
    assert!(delegate.modifiers.contains(&Modifier::Public));
    assert!(delegate.modifiers.contains(&Modifier::Unsafe));
}

#[test]
fn test_delegate_with_attributes() {
    let input = "[Serializable] public delegate void AttributedDelegate();";
    let (rest, delegate) = parse_delegate_declaration(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(delegate.name.name, "AttributedDelegate");
    assert_eq!(delegate.attributes.len(), 1);
    assert_eq!(delegate.attributes[0].attributes.len(), 1);
    assert_eq!(
        delegate.attributes[0].attributes[0].name.name,
        "Serializable"
    );
}

#[test]
fn test_internal_delegate() {
    let input = "internal delegate string ProcessString(string input);";
    let (rest, delegate) = parse_delegate_declaration(input).unwrap();
    assert_eq!(rest, "");
    assert_eq!(delegate.name.name, "ProcessString");
    assert_eq!(delegate.modifiers.len(), 1);
    assert_eq!(delegate.modifiers[0], Modifier::Internal);
    assert_eq!(delegate.parameters.len(), 1);
    assert_eq!(delegate.parameters[0].name.name, "input");
}
