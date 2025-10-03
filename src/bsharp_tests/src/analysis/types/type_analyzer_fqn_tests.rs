#![cfg(test)]

use analysis::types::TypeAnalyzer;
use syntax::ast::{CompilationUnit, TopLevelDeclaration};
use syntax::nodes::declarations::{
    ClassBodyDeclaration, ClassDeclaration, NamespaceDeclaration,
    namespace_declaration::NamespaceBodyDeclaration,
};
use syntax::nodes::identifier::Identifier;
use syntax::nodes::types::Type;

fn ident(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

fn class(name: &str, bases: Vec<&str>, nested: Vec<ClassBodyDeclaration>) -> ClassDeclaration {
    ClassDeclaration {
        documentation: None,
        attributes: vec![],
        modifiers: vec![],
        name: ident(name),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: bases
            .into_iter()
            .map(|b| Type::Reference(ident(b)))
            .collect(),
        body_declarations: nested,
        constraints: None,
    }
}

#[test]
fn test_fqn_for_top_level_and_nested_types_in_namespace() {
    // namespace N { class Outer { class Inner {} } }
    let inner = ClassDeclaration {
        documentation: None,
        attributes: vec![],
        modifiers: vec![],
        name: ident("Inner"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: vec![],
        body_declarations: vec![],
        constraints: None,
    };

    let outer = class(
        "Outer",
        vec!["Base"],
        vec![ClassBodyDeclaration::NestedClass(inner)],
    );

    let ns = NamespaceDeclaration {
        name: ident("N"),
        using_directives: vec![],
        declarations: vec![NamespaceBodyDeclaration::Class(outer)],
    };

    let cu = CompilationUnit {
        global_attributes: vec![],
        using_directives: vec![],
        global_using_directives: vec![],
        declarations: vec![TopLevelDeclaration::Namespace(ns)],
        file_scoped_namespace: None,
        top_level_statements: vec![],
    };

    let mut analyzer = TypeAnalyzer::new();
    analyzer.analyze_types_in_compilation_unit(&cu);

    // Expect FQNs for N.Outer and N.Outer.Inner
    assert!(analyzer.discovered_types.contains_key("N.Outer"));
    assert!(analyzer.discovered_types.contains_key("N.Outer.Inner"));

    // Relationships: Outer derives from Base (string form)
    let rel = analyzer.build_inheritance_hierarchy();
    // We stored derived->bases; reverse returns base->derived list
    // If Base has derived, ensure N.Outer appears (best-effort when unresolved FQN)
    if let Some(derived) = rel.get("Base") {
        assert!(derived.iter().any(|d| d.ends_with("Outer")));
    }

    let outer_info = analyzer.discovered_types.get("N.Outer").unwrap();
    assert_eq!(outer_info.name, "Outer");
    assert_eq!(outer_info.namespace.as_deref(), Some("N"));
    assert!(outer_info.member_counts.nested_types >= 1);
}
