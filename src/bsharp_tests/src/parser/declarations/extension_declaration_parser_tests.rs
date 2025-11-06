#![cfg(test)]
use parser::expressions::declarations::extension_declaration_parser::parse_extension_declaration;
use parser::parse_mode;
use syntax::declarations::{ClassBodyDeclaration, ExtensionDeclaration};
use syntax::types::Type;
use syntax::emitters::emit_trait::{Emitter, EmitCtx};

fn parse_ext_decl(code: &str) -> Result<ExtensionDeclaration, String> {
    match parse_extension_declaration(code.into()) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_extension_recovery_skips_invalid_member_semicolon() {
    // In lenient mode, an invalid member ending with ';' should be skipped and subsequent valid members parsed
    let prev = parse_mode::is_strict();
    parse_mode::set_strict(false);
    let input = r#"
extension int {
    int ; // invalid
    void M() {}
}
"#;
    let result = parse_ext_decl(input).unwrap();
    parse_mode::set_strict(prev);
    assert_eq!(result.members.len(), 1);
    match &result.members[0] {
        ClassBodyDeclaration::Method(m) => assert_eq!(m.name.to_string(), "M"),
        other => panic!("expected method member after recovery, got {:?}", other),
    }
}

#[test]
fn test_extension_recovery_skips_until_rbrace() {
    // In lenient mode, if no ';' found, parser skips until '}' and ends the body
    let prev = parse_mode::is_strict();
    parse_mode::set_strict(false);
    let input = r#"
extension int {
    $$$ this is not valid $$$
}
"#;
    let result = parse_ext_decl(input).unwrap();
    parse_mode::set_strict(prev);
    assert!(result.members.is_empty());
}

#[test]
fn test_extension_empty_body() {
    let input = "extension int { }";
    let result = parse_ext_decl(input).unwrap();
    // receiver type should be primitive int
    match result.receiver {
        Type::Primitive(_) => {}
        other => panic!("expected primitive type, got {:?}", other),
    }
    assert!(result.members.is_empty());
}

#[test]
fn test_extension_with_method_member() {
    let input = r#"
extension int {
    void M() {}
}
"#;
    let result = parse_ext_decl(input).unwrap();
    assert_eq!(result.members.len(), 1);
    match &result.members[0] {
        ClassBodyDeclaration::Method(m) => {
            assert_eq!(m.name.to_string(), "M");
        }
        other => panic!("expected method member, got {:?}", other),
    }
}

#[test]
fn test_extension_generic_receiver() {
    let input = "extension List<int> { }";
    let result = parse_ext_decl(input).unwrap();
    let s = result.receiver.to_string();
    assert_eq!(s, "List<int>", "receiver was {}", s);
}

#[test]
fn test_extension_missing_body_should_fail() {
    // Missing class body
    let input = "extension int";
    let err = parse_ext_decl(input).err();
    assert!(err.is_some(), "expected parse error for missing body");
}

#[test]
fn test_extension_missing_receiver_should_fail() {
    // Missing receiver type
    let input = "extension { }";
    let err = parse_ext_decl(input).err();
    assert!(err.is_some(), "expected parse error for missing receiver");
}

#[test]
fn test_emit_extension_empty_roundtrip() {
    let input = "extension int { }";
    let ast = parse_ext_decl(input).unwrap();

    // Emit
    let out = Emitter::new().write(&ast).unwrap();
    // Parse emitted output again and compare ASTs
    let ast2 = parse_ext_decl(&out).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn test_emit_extension_with_method_roundtrip() {
    let input = r#"
extension int {
    void M() {}
}
"#;
    let ast = parse_ext_decl(input).unwrap();

    // Emit
    let mut cx = EmitCtx::new();
    let out = Emitter::new().write_with_ctx(&ast, &mut cx).unwrap();

    // Parse emitted output again and compare ASTs (formatting may differ)
    let ast2 = parse_ext_decl(&out).unwrap();
    assert_eq!(ast, ast2);
}
