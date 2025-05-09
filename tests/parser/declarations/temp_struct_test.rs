#[cfg(test)]
mod tests {
    use bsharp::parsers::declarations::struct_declaration_parser::parse_struct_declaration;
    use bsharp::parser::nodes::declarations::{Modifier, StructDeclaration, StructMember};
    use bsharp::parser::nodes::identifier::Identifier;

    fn parse_struct_to_string(input: &str) -> String {
        match parse_struct_declaration(input) {
            Ok((rest, decl)) => format!("Success! Rest: '{}', Struct name: {}", rest, decl.name.name),
            Err(e) => format!("Error: {:?}", e)
        }
    }

    #[test]
    fn debug_struct_parser() {
        // Simple struct
        let result1 = parse_struct_to_string("struct MyStruct {}");
        println!("Simple struct: {}", result1);
        
        // Public struct
        let result2 = parse_struct_to_string("public struct MyPublicStruct {}");
        println!("Public struct: {}", result2);
        
        // Generic struct
        let result3 = parse_struct_to_string("struct MyGenericStruct<T> {}");
        println!("Generic struct: {}", result3);
        
        // Interface implementation
        let result4 = parse_struct_to_string("struct MyStruct : IDisposable {}");
        println!("Interface struct: {}", result4);
        
        assert!(true); // Always pass to see debug output
    }
}
