#![cfg(test)]
use parser::expressions::declarations::type_declaration_parser::{
    parse_class_declaration, parse_interface_declaration, parse_record_declaration,
    parse_struct_declaration_span as parse_struct_declaration, parse_type_declaration,
};
use syntax::declarations::{InterfaceBodyDeclaration, TypeDeclaration};

#[test]
fn test_simple_class() {
    let input = "class MyClass {}";

    match parse_class_declaration(input.into()) {
        Ok((remaining, decl)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "MyClass");
            assert_eq!(decl.modifiers.len(), 0);
            assert_eq!(decl.body_declarations.len(), 0);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_simple_struct() {
    let input = "struct MyStruct {}";

    match parse_struct_declaration(input.into()) {
        Ok((remaining, decl)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "MyStruct");
            assert_eq!(decl.modifiers.len(), 0);
            assert_eq!(decl.body_declarations.len(), 0);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_record_class() {
    let input = "record Person(string FirstName, string LastName);";

    match parse_record_declaration(input.into()) {
        Ok((remaining, decl)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "Person");
            assert!(!decl.is_struct);
            assert!(decl.parameters.is_some());
            assert_eq!(decl.parameters.unwrap().len(), 2);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_record_struct() {
    let input = "record struct Point(int X, int Y);";

    match parse_record_declaration(input.into()) {
        Ok((remaining, decl)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "Point");
            assert!(decl.is_struct);
            assert!(decl.parameters.is_some());
            assert_eq!(decl.parameters.unwrap().len(), 2);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_simple_interface() {
    let input = "interface IComparable {}";

    match parse_interface_declaration(input.into()) {
        Ok((remaining, decl)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "IComparable");
            assert_eq!(decl.modifiers.len(), 0);
            assert_eq!(decl.body_declarations.len(), 0);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_interface_with_modifiers_and_base() {
    let input = "public interface IList<T> : ICollection<T>, IEnumerable<T> {}";

    match parse_interface_declaration(input.into()) {
        Ok((remaining, decl)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "IList");
            assert_eq!(decl.modifiers.len(), 1); // public
            assert_eq!(decl.base_types.len(), 2); // ICollection<T> and IEnumerable<T>

            // Type parameters check
            assert!(decl.type_parameters.is_some());
            let type_params = decl.type_parameters.unwrap();
            assert_eq!(type_params.len(), 1); // <T>
            assert_eq!(type_params[0].name.to_string(), "T");
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_interface_with_method() {
    let input = "interface IComparable { int CompareTo(object obj); }";

    match parse_interface_declaration(input.into()) {
        Ok((remaining, decl)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "IComparable");
            assert_eq!(decl.modifiers.len(), 0);
            assert_eq!(decl.body_declarations.len(), 1);

            // Check that the method was parsed correctly
            if let InterfaceBodyDeclaration::Method(method) = &decl.body_declarations[0] {
                assert_eq!(method.name.to_string(), "CompareTo");
                assert_eq!(method.parameters.len(), 1);
                assert_eq!(method.body, None); // Interface methods have no body
            } else {
                panic!("Expected a method, found something else");
            }
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_interface_with_method_body_error() {
    // Interface methods cannot have a body, but syntax should use error recovery
    let input = "interface IBad { void BadMethod() { return; } }";

    match parse_interface_declaration(input.into()) {
        Ok((remaining, decl)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "IBad");
            assert_eq!(decl.body_declarations.len(), 1);

            // The method should be parsed but with body set to None (error recovery)
            if let InterfaceBodyDeclaration::Method(method) = &decl.body_declarations[0] {
                assert_eq!(method.name.to_string(), "BadMethod");
                assert_eq!(method.body, None); // Body should be removed for interface methods
            } else {
                panic!("Expected a method, found something else");
            }
        }
        Err(e) => panic!(
            "Expected parsing to succeed with error recovery, but got: {:?}",
            e
        ),
    }
}

#[test]
fn test_type_declaration_interface() {
    let input = "interface IComparable { int CompareTo(object obj); }";

    match parse_type_declaration(input.into()) {
        Ok((remaining, TypeDeclaration::Interface(decl))) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(decl.name.to_string(), "IComparable");
            assert_eq!(decl.body_declarations.len(), 1);
        }
        Ok(_) => panic!("Expected an interface declaration"),
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}
