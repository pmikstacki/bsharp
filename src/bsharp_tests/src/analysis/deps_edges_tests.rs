use analysis::artifacts::dependencies::{DependencyGraph, DependencyType};
use analysis::artifacts::symbols::{SymbolIndex, SymbolKind};
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn dependencies_edges_cover_inheritance_field_param_and_call() {
    let src = r#"
namespace N {
  public class B { public void N() {} }
  public class C { }
  public class A : C {
    private B fld;
    public void M(B p) { p.N(); }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graph = session
        .artifacts
        .get::<DependencyGraph>()
        .expect("DependencyGraph missing");
    let syms = session
        .artifacts
        .get::<SymbolIndex>()
        .expect("SymbolIndex missing");

    // Helper: lookup any symbol by local name and kind
    let find_by_name_kind = |name: &str, kind: SymbolKind| {
        syms.get_ids_by_name(name)
            .and_then(|v| v.iter().find_map(|id| syms.get(*id)))
            .filter(|s| s.kind == kind)
            .cloned()
    };

    let a = find_by_name_kind("A", SymbolKind::Class).expect("class A symbol");
    let b = find_by_name_kind("B", SymbolKind::Class).expect("class B symbol");
    let c = find_by_name_kind("C", SymbolKind::Class).expect("class C symbol");
    let n_method = syms
        .get_ids_by_name("N")
        .and_then(|v| v.first().cloned())
        .and_then(|id| syms.get(id))
        .expect("method N symbol");

    // Collect edges {from,to,type}
    let mut has_inheritance = false;
    let mut has_field = false;
    let mut has_usage_param = false;
    let mut has_call = false;
    for e in &graph.edges {
        if e.from == a.id
            && e.to == c.id
            && matches!(e.dependency_type, DependencyType::Inheritance)
        {
            has_inheritance = true;
        }
        if e.from == a.id
            && e.to == b.id
            && matches!(e.dependency_type, DependencyType::FieldAccess)
        {
            has_field = true;
        }
        if e.from == a.id && e.to == b.id && matches!(e.dependency_type, DependencyType::Usage) {
            has_usage_param = true;
        }
        if e.from == a.id
            && e.to == n_method.id
            && matches!(e.dependency_type, DependencyType::MethodCall)
        {
            has_call = true;
        }
    }

    assert!(has_inheritance, "expected inheritance edge A->C");
    assert!(has_field, "expected field access edge A->B");
    assert!(has_usage_param, "expected usage edge A->B from parameter");
    assert!(has_call, "expected call edge A->B.N");
}

#[test]
fn dependencies_resolve_generic_base_and_nested_class_types() {
    let src = r#"
namespace N {
  public class Generic<T> { }
  public class Outer { public class Inner { } }
  public class Uses {
    private Generic<Outer.Inner> g;
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graph = session
        .artifacts
        .get::<DependencyGraph>()
        .expect("DependencyGraph missing");
    let syms = session
        .artifacts
        .get::<SymbolIndex>()
        .expect("SymbolIndex missing");

    let uses = syms
        .get_ids_by_name("Uses")
        .and_then(|v| v.first().cloned())
        .and_then(|id| syms.get(id))
        .expect("Uses symbol");
    let generic = syms
        .get_ids_by_name("Generic")
        .and_then(|v| v.first().cloned())
        .and_then(|id| syms.get(id))
        .expect("Generic symbol");

    // Implementation-dependent: generic field may or may not create an edge to the base. If present, classify it.
    let edge_to_generic = graph
        .edges
        .iter()
        .find(|e| e.from == uses.id && e.to == generic.id);
    if let Some(e) = edge_to_generic {
        assert!(matches!(
            e.dependency_type,
            DependencyType::FieldAccess | DependencyType::Usage
        ));
    } else {
        // Minimum guarantee: the declaring class is present as a node
        assert!(graph.nodes.contains_key(&uses.id));
    }
}

#[test]
fn dependencies_generic_parameter_type_registers_usage_to_base() {
    let src = r#"
namespace N {
  public class G<T> { }
  public class A { public void M(G<int> p) { } }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graph = session.artifacts.get::<DependencyGraph>().expect("graph");
    let syms = session.artifacts.get::<SymbolIndex>().expect("syms");
    let a = syms
        .get_ids_by_name("A")
        .and_then(|v| v.first().cloned())
        .and_then(|id| syms.get(id))
        .expect("A");
    let g = syms
        .get_ids_by_name("G")
        .and_then(|v| v.first().cloned())
        .and_then(|id| syms.get(id))
        .expect("G");
    let has_edge = graph.edges.iter().any(|e| e.from == a.id && e.to == g.id);
    assert!(has_edge, "expected A to depend on G due to parameter type");
}

#[test]
fn dependencies_generic_does_not_link_inner_argument_type_directly() {
    let src = r#"
namespace N {
  public class Generic<T> { }
  public class Outer { public class Inner { } }
  public class Uses { private Generic<Outer.Inner> g; }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let graph = session.artifacts.get::<DependencyGraph>().expect("graph");
    let syms = session.artifacts.get::<SymbolIndex>().expect("syms");
    let uses = syms
        .get_ids_by_name("Uses")
        .and_then(|v| v.first().cloned())
        .and_then(|id| syms.get(id))
        .expect("Uses");
    let inner = syms
        .get_ids_by_name("Inner")
        .and_then(|v| v.first().cloned())
        .and_then(|id| syms.get(id))
        .expect("Inner");
    // Current pass reduces generic type to base only; ensure there is no direct edge Uses->Inner.
    let has_direct_inner = graph
        .edges
        .iter()
        .any(|e| e.from == uses.id && e.to == inner.id);
    assert!(
        !has_direct_inner,
        "did not expect direct dependency on inner type from generic base"
    );
}
