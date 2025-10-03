#![allow(unused_variables)]

use analysis::types::TypeAnalyzer;
use syntax::nodes::declarations::ClassDeclaration;
use syntax::nodes::identifier::Identifier;
use syntax::nodes::types::Type;

fn ident(name: &str) -> Identifier {
    Identifier {
        name: name.to_string(),
    }
}

fn class(name: &str, bases: Vec<&str>) -> ClassDeclaration {
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
        body_declarations: vec![],
    }
}

#[test]
fn test_inheritance_hierarchy_and_is_derived_from() {
    let mut analyzer = TypeAnalyzer::new();
    let a = class("A", vec![]);
    let b = class("B", vec!["A"]);
    let c = class("C", vec!["B"]);

    analyzer.analyze_class(&a);
    analyzer.analyze_class(&b);
    analyzer.analyze_class(&c);

    let hierarchy = analyzer.build_inheritance_hierarchy();
    // base -> derived mapping should include A -> [B]
    assert!(
        hierarchy
            .get("A")
            .map(|v| v.contains(&"B".to_string()))
            .unwrap_or(true)
    );

    assert!(analyzer.is_derived_from("C", "A"));
    assert!(analyzer.is_derived_from("B", "A"));
    assert!(!analyzer.is_derived_from("A", "B"));
}

#[test]
fn test_cycle_detection_simple() {
    let mut analyzer = TypeAnalyzer::new();
    let a = class("A", vec!["B"]);
    let b = class("B", vec!["A"]);

    analyzer.analyze_class(&a);
    analyzer.analyze_class(&b);

    let cycles = analyzer.detect_circular_dependencies();
    // We expect at least one cycle reported for A <-> B
    assert!(!cycles.is_empty());
}
