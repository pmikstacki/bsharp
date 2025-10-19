#![allow(unused_variables)]
// Integration tests for parsing using directives (including global)

use parser::expressions::declarations::using_directive_parser::parse_using_directive;
use syntax::declarations::{GlobalUsingDirective, UsingDirective};
use syntax::identifier::Identifier;

// Assuming a syntax function like parse_global_using_directive will exist
// use syntax::syntax::parse_global_using_directive;

// Placeholder for the actual parsing function call
fn parse_global_using(code: &str) -> Result<GlobalUsingDirective, String> {
    // Replace with actual syntax invocation when implemented
    Err(format!(
        "Parser function not yet implemented for global using: {}",
        code
    ))
}

#[test]
fn test_parse_global_using_namespace() {
    let code = "global using System;";
    let expected = GlobalUsingDirective {
        using_directive: UsingDirective::Namespace {
            namespace: Identifier::new("System"),
        },
    };
    // assert_eq!(parse_global_using(code.into()), Ok(expected)); // Uncomment when syntax implemented
    assert!(
        parse_global_using(code.into()).is_err(),
        "Parser should not be implemented yet"
    );
}

#[test]
fn test_parse_global_using_alias() {
    let code = "global using MyAlias = System.Collections.Generic;";
    let expected = GlobalUsingDirective {
        using_directive: UsingDirective::Alias {
            alias: Identifier::new("MyAlias"),
            namespace_or_type: Identifier::new("System.Collections.Generic"), // Assuming IdentifierNameSyntax handles qualified names for now
        },
    };
    // assert_eq!(parse_global_using(code.into()), Ok(expected)); // Uncomment when syntax implemented
    assert!(
        parse_global_using(code.into()).is_err(),
        "Parser should not be implemented yet"
    );
}

#[test]
fn test_parse_global_using_static() {
    let code = "global using static System.Math;";
    let expected = GlobalUsingDirective {
        using_directive: UsingDirective::Static {
            type_name: Identifier::new("System.Math"), // Assuming IdentifierNameSyntax handles qualified names
        },
    };
    // assert_eq!(parse_global_using(code.into()), Ok(expected)); // Uncomment when syntax implemented
    assert!(
        parse_global_using(code.into()).is_err(),
        "Parser should not be implemented yet"
    );
}

// TODO: Add tests for regular (non-global) using directives in a separate function/file if needed.

#[test]
fn test_parse_using_namespace() {
    let code = "using System.Text;";
    let (rest, dir) = parse_using_directive(code.into()).expect("should parse namespace using");
    assert!(rest.fragment().trim().is_empty());
    match dir {
        UsingDirective::Namespace { namespace } => {
            assert_eq!(namespace.to_string(), "System.Text");
        }
        _ => panic!("expected namespace using"),
    }
}

#[test]
fn test_parse_using_alias() {
    let code = "using TCol = System.Collections.Generic;";
    let (_, dir) = parse_using_directive(code.into()).expect("should parse alias using");
    match dir {
        UsingDirective::Alias {
            alias,
            namespace_or_type,
        } => {
            assert_eq!(alias.to_string(), "TCol");
            assert_eq!(namespace_or_type.to_string(), "System.Collections.Generic");
        }
        _ => panic!("expected alias using"),
    }
}

#[test]
fn test_parse_using_static() {
    let code = "using static System.Math;";
    let (_, dir) = parse_using_directive(code.into()).expect("should parse static using");
    match dir {
        UsingDirective::Static { type_name } => {
            assert_eq!(type_name.to_string(), "System.Math");
        }
        _ => panic!("expected static using"),
    }
}
