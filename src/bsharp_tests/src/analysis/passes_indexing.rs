use analysis::artifacts::symbols::{FqnMap, NameIndex, SymbolIndex, SymbolKind};
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn indexing_pass_populates_symbols_names_and_fqn() {
    let src = r#"
namespace N {
  public class A { public void M() {} }
  public class B { }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let symbols = session
        .artifacts
        .get::<SymbolIndex>()
        .expect("SymbolIndex missing");
    let names = session
        .artifacts
        .get::<NameIndex>()
        .expect("NameIndex missing");
    let fqns = session.artifacts.get::<FqnMap>().expect("FqnMap missing");

    // Names
    assert!(names.0.contains_key("A"));
    assert!(names.0.contains_key("M"));

    // Symbols
    let class_a = symbols
        .get_ids_by_name("A")
        .and_then(|v| v.first().cloned())
        .and_then(|id| symbols.get(id))
        .expect("class A symbol");
    assert_eq!(class_a.kind, SymbolKind::Class);
    assert_eq!(class_a.fqn.as_deref(), Some("N.A"));

    let method_m = symbols
        .get_ids_by_name("M")
        .and_then(|v| v.first().cloned())
        .and_then(|id| symbols.get(id))
        .expect("method M symbol");
    assert_eq!(method_m.kind, SymbolKind::Method);
    assert_eq!(method_m.fqn.as_deref(), Some("N.A::M"));

    // FQN map tracks namespaces (classes are covered via SymbolIndex.fqn)
    assert!(fqns.0.get("N").is_some());
}
