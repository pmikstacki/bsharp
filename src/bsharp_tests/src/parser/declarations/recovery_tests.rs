use parser::expressions::declarations::type_declaration_parser::parse_class_declaration;

fn parse_class_decl(code: &str) -> Result<syntax::nodes::declarations::ClassDeclaration, String> {
    match parse_class_declaration(code) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn recovery_skips_to_next_member_on_semicolon() {
    let code = r#"
        class K {
            this is not valid ;
            void M() {}
        }
    "#;

    let decl = parse_class_decl(code.trim()).expect("class should parse with recovery");
    // We expect only the valid method to be captured
    assert_eq!(decl.body_declarations.len(), 1, "Expected exactly one valid member after recovery");
}

#[test]
fn recovery_stops_at_closing_brace_when_no_semicolon() {
    let code = r#"
        class K {
            this is not valid
        }
    "#;

    let decl = parse_class_decl(code.trim()).expect("class should parse with recovery to '}'");
    assert!(decl.body_declarations.is_empty(), "No members should be parsed when malformed member has no terminating ';'");
}
