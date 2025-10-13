use analysis::context::AnalysisContext;
use analysis::framework::{AnalysisSession, AnalyzerPipeline};
use analysis::framework::{NodeRef, Query};
use parser::facade::Parser;

#[test]
fn children_and_descendants_cover_classes_methods_and_bodies() {
    let src = r#"
namespace Ns { public class C { public void M() { if (true) { } } } }
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);

    // Children of CU include Namespace
    let root = NodeRef::from(&cu);
    let cu_children: Vec<_> = root.children_iter().collect();
    assert!(cu_children
        .iter()
        .any(|n| n.of::<analysis::syntax::declarations::NamespaceDeclaration>().is_some()));

    // Descendants include Class and Method
    let has_class = Query::from(&cu)
        .of::<analysis::syntax::declarations::ClassDeclaration>()
        .next()
        .is_some();
    let has_method = Query::from(&cu)
        .of::<analysis::syntax::declarations::MethodDeclaration>()
        .next()
        .is_some();
    assert!(has_class && has_method);

    // Children of Method include Statement body
    let method = Query::from(&cu)
        .of::<analysis::syntax::declarations::MethodDeclaration>()
        .next()
        .unwrap();
    let method_node = NodeRef::from(method);
    let method_children: Vec<_> = method_node.children_iter().collect();
    assert!(method_children
        .iter()
        .any(|n| n.of::<analysis::syntax::statements::statement::Statement>().is_some()));
}
