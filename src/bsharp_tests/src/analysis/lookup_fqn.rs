use analysis::framework::fqn::{class_fqn, method_fqn, namespace_fqn};
use analysis::framework::lookup::find_symbols_by_name;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::{context::AnalysisContext, syntax};
use parser::facade::Parser;

#[test]
fn fqn_for_methods_classes_and_namespaces_in_various_layouts() {
    let src = r#"
namespace Ns {
  public class Outer {
    public class Inner {
      public void M() {}
    }
    public void N() {}
  }
}
// No file-scoped namespace here; test top-level class FQN fallback
public class CTop { public void T() {} }
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");

    // Locate nodes
    let ns = cu
        .declarations
        .iter()
        .find_map(|d| match d {
            syntax::ast::TopLevelDeclaration::Namespace(n) => Some(n),
            _ => None,
        })
        .expect("namespace Ns expected");
    // Find Outer
    let outer = ns
        .declarations
        .iter()
        .find_map(|m| match m {
            syntax::declarations::namespace_declaration::NamespaceBodyDeclaration::Class(c)
                if c.name.to_string() == "Outer" =>
            {
                Some(c)
            }
            _ => None,
        })
        .expect("Outer class");
    // Find Inner
    let inner = outer
        .body_declarations
        .iter()
        .find_map(|m| match m {
            syntax::declarations::ClassBodyDeclaration::NestedClass(c)
                if c.name.to_string() == "Inner" =>
            {
                Some(c)
            }
            _ => None,
        })
        .expect("Inner class");
    // Find methods
    let m_in_inner = inner
        .body_declarations
        .iter()
        .find_map(|m| match m {
            syntax::declarations::ClassBodyDeclaration::Method(md) => Some(md),
            _ => None,
        })
        .expect("method M");
    let m_in_outer = outer
        .body_declarations
        .iter()
        .find_map(|m| match m {
            syntax::declarations::ClassBodyDeclaration::Method(md) => Some(md),
            _ => None,
        })
        .expect("method N");

    // Top-level class without namespace
    let top = cu
        .declarations
        .iter()
        .find_map(|d| match d {
            syntax::ast::TopLevelDeclaration::Class(c) if c.name.to_string() == "CTop" => Some(c),
            _ => None,
        })
        .expect("CTop class");

    // Assert FQNs
    assert_eq!(method_fqn(&cu, m_in_inner), "Ns.Outer.Inner::M");
    assert_eq!(method_fqn(&cu, m_in_outer), "Ns.Outer::N");
    assert_eq!(class_fqn(&cu, inner), "Ns.Outer.Inner");
    assert_eq!(class_fqn(&cu, outer), "Ns.Outer");
    assert!(namespace_fqn(&cu, ns).starts_with("Ns"));
    assert_eq!(method_fqn(&cu, m_in_outer), "Ns.Outer::N");
    assert_eq!(class_fqn(&cu, top), "CTop");
}

#[test]
fn lookup_finds_symbols_by_local_name() {
    let src = r#"
namespace A { public class B { public void N() {} public int P { get; set; } } }
public class C { public void N() {} }
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let found = find_symbols_by_name(&session, "N");
    assert!(
        !found.is_empty(),
        "expected to find method symbols named 'N'"
    );
    assert!(found.iter().all(|s| s.name == "N"));
}
