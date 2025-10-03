use analysis::artifacts::symbols::{SymbolIndex, SymbolKind};
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use parser::facade::Parser;

#[test]
fn symbols_index_contains_class_and_method() {
    let source = r#"
namespace N {
  public class A {
    public void M() { }
  }
}
"#;
    let parser = Parser::new();
    let (cu, spans) = parser.parse_with_spans(source).expect("parse error");
    let ctx = AnalysisContext::new("test.cs", source);
    let mut session = AnalysisSession::new(ctx, spans);

    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    let idx = session
        .artifacts
        .get::<SymbolIndex>()
        .expect("symbol index missing");
    // Class A
    let class_ids = idx.get_ids_by_name("A").cloned().unwrap_or_default();
    assert!(!class_ids.is_empty(), "expected class 'A' in symbol index");
    let class_sym = idx.get(class_ids[0]).expect("class symbol by id");
    assert_eq!(class_sym.kind, SymbolKind::Class);
    assert_eq!(class_sym.fqn.as_deref(), Some("N.A"));
    assert_eq!(class_sym.file.as_deref(), Some("test.cs"));

    // Method M
    let method_ids = idx.get_ids_by_name("M").cloned().unwrap_or_default();
    assert!(
        !method_ids.is_empty(),
        "expected method 'M' in symbol index"
    );
    let method_sym = idx.get(method_ids[0]).expect("method symbol by id");
    assert_eq!(method_sym.kind, SymbolKind::Method);
    assert_eq!(method_sym.fqn.as_deref(), Some("N.A::M"));
    assert_eq!(method_sym.file.as_deref(), Some("test.cs"));
}
