// Tests for advanced type system features

use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};
use bsharp::parser::types::type_parser::parse_type_expression;
use bsharp::syntax::nodes::types::CallingConvention;

// Helper function for unwrapping syntax results
fn parse_test(code: &str) -> Result<Type, String> {
    match parse_type_expression(code) {
        Ok((remaining, ty)) if remaining.trim().is_empty() => Ok(ty),
        Ok((remaining, _)) => Err(format!("Didn't consume all input. Remaining: '{}'", remaining)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_pointer_types() {
    // Basic pointer types
    let expected_int_ptr = Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Int)));
    assert_eq!(parse_test("int*").unwrap(), expected_int_ptr);
    
    let expected_char_ptr = Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Char)));
    assert_eq!(parse_test("char*").unwrap(), expected_char_ptr);
    
    let expected_void_ptr = Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Void)));
    assert_eq!(parse_test("void*").unwrap(), expected_void_ptr);
}

#[test]
fn test_parse_double_pointer_types() {
    // Double pointer: int**
    let expected = Type::Pointer(Box::new(Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Int)))));
    assert_eq!(parse_test("int**").unwrap(), expected);
    
    // Triple pointer: char***
    let expected_triple = Type::Pointer(Box::new(
        Type::Pointer(Box::new(
            Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Char)))
        ))
    ));
    assert_eq!(parse_test("char***").unwrap(), expected_triple);
}

#[test]
fn test_parse_pointer_to_reference_type() {
    // Pointer to class: MyClass*
    let expected = Type::Pointer(Box::new(Type::Reference(Identifier::new("MyClass"))));
    assert_eq!(parse_test("MyClass*").unwrap(), expected);
    
    // Pointer to qualified type: System.String*
    let expected_qualified = Type::Pointer(Box::new(Type::Reference(Identifier::new("System.String"))));
    assert_eq!(parse_test("System.String*").unwrap(), expected_qualified);
}

#[test]
fn test_parse_pointer_to_array() {
    // Pointer to array: int[]*
    let array_type = Type::Array {
        element_type: Box::new(Type::Primitive(PrimitiveType::Int)),
        rank: 1,
    };
    let expected = Type::Pointer(Box::new(array_type));
    assert_eq!(parse_test("int[]*").unwrap(), expected);
}

#[test]
fn test_parse_array_of_pointers() {
    // Array of pointers: int*[]
    let pointer_type = Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Int)));
    let expected = Type::Array {
        element_type: Box::new(pointer_type),
        rank: 1,
    };
    assert_eq!(parse_test("int*[]").unwrap(), expected);
}

#[test]
fn test_parse_function_pointer_basic() {
    // delegate*<void> - function taking no parameters, returning void
    let expected = Type::FunctionPointer {
        calling_convention: None,
        parameter_types: vec![],
        return_type: Box::new(Type::Primitive(PrimitiveType::Void)),
    };
    assert_eq!(parse_test("delegate*<void>").unwrap(), expected);
}

#[test]
fn test_parse_function_pointer_with_parameters() {
    // delegate*<int, string, void> - function taking int and string, returning void
    let expected = Type::FunctionPointer {
        calling_convention: None,
        parameter_types: vec![
            Type::Primitive(PrimitiveType::Int),
            Type::Primitive(PrimitiveType::String),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Void)),
    };
    assert_eq!(parse_test("delegate*<int, string, void>").unwrap(), expected);
}

#[test]
fn test_parse_function_pointer_with_return_type() {
    // delegate*<int, int, int> - function taking two ints, returning int
    let expected = Type::FunctionPointer {
        calling_convention: None,
        parameter_types: vec![
            Type::Primitive(PrimitiveType::Int),
            Type::Primitive(PrimitiveType::Int),
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::Int)),
    };
    assert_eq!(parse_test("delegate*<int, int, int>").unwrap(), expected);
}

#[test]
fn test_parse_managed_function_pointer() {
    // delegate* managed<int, void> - managed function pointer
    let expected = Type::FunctionPointer {
        calling_convention: Some(CallingConvention::Managed),
        parameter_types: vec![Type::Primitive(PrimitiveType::Int)],
        return_type: Box::new(Type::Primitive(PrimitiveType::Void)),
    };
    assert_eq!(parse_test("delegate* managed<int, void>").unwrap(), expected);
}

#[test]
fn test_parse_unmanaged_function_pointer() {
    // delegate* unmanaged<string, bool> - unmanaged function pointer
    let expected = Type::FunctionPointer {
        calling_convention: Some(CallingConvention::Unmanaged),
        parameter_types: vec![Type::Primitive(PrimitiveType::String)],
        return_type: Box::new(Type::Primitive(PrimitiveType::Bool)),
    };
    assert_eq!(parse_test("delegate* unmanaged<string, bool>").unwrap(), expected);
}

#[test]
fn test_parse_function_pointer_with_complex_types() {
    // delegate*<MyClass, int[], string> - function taking custom class and int array, returning string
    let expected = Type::FunctionPointer {
        calling_convention: None,
        parameter_types: vec![
            Type::Reference(Identifier::new("MyClass")),
            Type::Array {
                element_type: Box::new(Type::Primitive(PrimitiveType::Int)),
                rank: 1,
            },
        ],
        return_type: Box::new(Type::Primitive(PrimitiveType::String)),
    };
    assert_eq!(parse_test("delegate*<MyClass, int[], string>").unwrap(), expected);
}

#[test]
fn test_parse_nullable_reference_types() {
    // string? should be parsed as Nullable (simplified implementation)
    let expected = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::String)));
    assert_eq!(parse_test("string?").unwrap(), expected);
    
    // object? should be Nullable
    let expected_object = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::Object)));
    assert_eq!(parse_test("object?").unwrap(), expected_object);
    
    // MyClass? should be Nullable
    let expected_class = Type::Nullable(Box::new(Type::Reference(Identifier::new("MyClass"))));
    assert_eq!(parse_test("MyClass?").unwrap(), expected_class);
}

#[test]
fn test_parse_nullable_value_types() {
    // int? should be parsed as Nullable
    let expected = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::Int)));
    assert_eq!(parse_test("int?").unwrap(), expected);
    
    // bool? should be Nullable
    let expected_bool = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::Bool)));
    assert_eq!(parse_test("bool?").unwrap(), expected_bool);
    
    // double? should be Nullable
    let expected_double = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::Double)));
    assert_eq!(parse_test("double?").unwrap(), expected_double);
}

#[test]
fn test_parse_nullable_generic_types() {
    // List<int>? should be Nullable
    let generic_type = Type::Generic {
        base: Identifier::new("List"),
        args: vec![Type::Primitive(PrimitiveType::Int)],
    };
    let expected = Type::Nullable(Box::new(generic_type));
    assert_eq!(parse_test("List<int>?").unwrap(), expected);
}

#[test]
fn test_parse_dynamic_type() {
    // dynamic type should be parsed correctly
    let expected = Type::Dynamic;
    assert_eq!(parse_test("dynamic").unwrap(), expected);
}

#[test]
fn test_parse_complex_combinations() {
    // int*[] - array of int pointers
    let expected = Type::Array {
        element_type: Box::new(Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Int)))),
        rank: 1,
    };
    assert_eq!(parse_test("int*[]").unwrap(), expected);
    
    // int*? - nullable pointer to int (though this is unusual in practice)
    let expected_nullable_ptr = Type::Nullable(Box::new(Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Int)))));
    assert_eq!(parse_test("int*?").unwrap(), expected_nullable_ptr);
}

#[test]
fn test_parse_whitespace_handling() {
    // Test that whitespace is handled correctly in complex types
    assert_eq!(parse_test("delegate* < int , void >").unwrap(), 
               Type::FunctionPointer {
                   calling_convention: None,
                   parameter_types: vec![Type::Primitive(PrimitiveType::Int)],
                   return_type: Box::new(Type::Primitive(PrimitiveType::Void)),
               });
    
    assert_eq!(parse_test("int * ").unwrap(), 
               Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Int))));
}

#[test]
fn test_parse_errors() {
    // Empty function pointer should fail
    assert!(parse_test("delegate*<>").is_err());
    
    // Invalid function pointer parser should fail
    assert!(parse_test("delegate*").is_err());
    
    // Invalid generic parser should fail
    assert!(parse_test("List<>").is_err());
} 