#![cfg(test)]

use bsharp::analysis::types::TypeAnalyzer;
use bsharp::syntax::nodes::types::{Type, PrimitiveType};
use bsharp::syntax::nodes::identifier::Identifier;

fn ident(name: &str) -> Identifier { Identifier { name: name.to_string() } }

#[test]
fn complexity_arrays_and_generics_and_nullable_pointer() {
    let analyzer = TypeAnalyzer::new();

    // int[][] -> array depth 2
    let ty_arrays = Type::Array {
        element_type: Box::new(Type::Array { element_type: Box::new(Type::Primitive(PrimitiveType::Int)), rank: 1 }),
        rank: 1,
    };
    let d_arrays = analyzer.analyze_type_complexity_detail(&ty_arrays);
    assert_eq!(d_arrays.array_depth, 2);

    // Dictionary<string, List<T?>> -> generic depth >= 2, total args 2 + 1(inner)
    let ty_nested_generics = Type::Generic {
        base: ident("Dictionary"),
        args: vec![
            Type::Reference(ident("string")),
            Type::Generic { base: ident("List"), args: vec![ Type::Nullable(Box::new(Type::Reference(ident("T")))) ] },
        ],
    };
    let d_generics = analyzer.analyze_type_complexity_detail(&ty_nested_generics);
    assert!(d_generics.generic_depth >= 2);
    assert!(d_generics.total_generic_args >= 2);
    assert!(d_generics.is_nullable);

    // int*? -> pointer + nullable
    let ty_ptr_nullable = Type::Nullable(Box::new(Type::Pointer(Box::new(Type::Primitive(PrimitiveType::Int)))));
    let d_ptr_nullable = analyzer.analyze_type_complexity_detail(&ty_ptr_nullable);
    assert!(d_ptr_nullable.is_pointer);
    assert!(d_ptr_nullable.is_nullable);
}
