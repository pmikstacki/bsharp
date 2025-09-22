// Tests for parsing method/constructor declarations: constructor initializers

use bsharp::parser::expressions::declarations::type_declaration_parser::parse_class_declaration;
use bsharp::syntax::nodes::declarations::{ClassBodyDeclaration, ConstructorDeclaration, ConstructorInitializer};
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;

fn parse_class(code: &str) -> Result<bsharp::syntax::nodes::declarations::ClassDeclaration, String> {
    match parse_class_declaration(code) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn ctor_initializer_base_with_args() {
    let code = r#"class C { public C() : base(1, 2) {} }"#;
    let class_decl = parse_class(code).expect("parse");
    assert_eq!(class_decl.body_declarations.len(), 1);
    match &class_decl.body_declarations[0] {
        ClassBodyDeclaration::Constructor(ConstructorDeclaration { initializer, .. }) => {
            let init = initializer.as_ref().expect("initializer present");
            match init {
                ConstructorInitializer::Base(args) => {
                    assert_eq!(args.len(), 2);
                    assert!(matches!(args[0], Expression::Literal(Literal::Integer(1))));
                    assert!(matches!(args[1], Expression::Literal(Literal::Integer(2))));
                }
                _ => panic!("expected base initializer"),
            }
        }
        other => panic!("expected constructor, got: {:?}", other),
    }
}

#[test]
fn ctor_initializer_this_with_no_args() {
    let code = r#"class C { public C(int x) : this() {} }"#;
    let class_decl = parse_class(code).expect("parse");
    assert_eq!(class_decl.body_declarations.len(), 1);
    match &class_decl.body_declarations[0] {
        ClassBodyDeclaration::Constructor(ConstructorDeclaration { initializer, .. }) => {
            let init = initializer.as_ref().expect("initializer present");
            match init {
                ConstructorInitializer::This(args) => {
                    assert_eq!(args.len(), 0);
                }
                _ => panic!("expected this initializer"),
            }
        }
        other => panic!("expected constructor, got: {:?}", other),
    }
}
