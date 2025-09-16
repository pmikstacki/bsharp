// Tests for parsing namespace declarations

use bsharp::parser::declarations::namespace_declaration_parser::parse_namespace_declaration;

// Local test helper to avoid import issues
fn parse_full_input<'a, O, F>(input: &'a str, parser: F) -> Result<(&'a str, O), String>
where
    F: FnOnce(&'a str) -> bsharp::syntax::errors::BResult<&'a str, O>,
{
    match parser(input) {
        Ok((remaining, result)) => Ok((remaining, result)),
        Err(err) => Err(format!("Parse error: {:?}", err)),
    }
}

#[test]
fn test_simple_namespace_declaration() {
    let input = "namespace MyNamespace { }";
    let result = parse_full_input(input, parse_namespace_declaration);
    assert!(result.is_ok());
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.name.name, "MyNamespace");
}

#[test]
fn test_qualified_namespace_declaration() {
    let input = "namespace System.Collections { }";
    let result = parse_full_input(input, parse_namespace_declaration);
    assert!(result.is_ok());
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.name.name, "System.Collections");
}

#[test]
fn test_parse_namespace() {
    let code = "namespace MyNs { }";
    let result = parse_full_input(code, parse_namespace_declaration);
    assert!(result.is_ok(), "Expected successful parsing of namespace");
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.name.name, "MyNs");
}
