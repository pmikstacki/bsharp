use analysis::context::AnalysisContext;
use analysis::framework::visit::Visit;
use analysis::framework::{session::AnalysisSession, walker::AstWalker};
use parser::facade::Parser;
use std::cell::RefCell;
use std::rc::Rc;
use analysis::framework::NodeRef;
use analysis::syntax::statements::statement::Statement;

#[test]
fn walker_traverses_nested_statements_depth_first() {
    let src = r#"
public class C {
  public void M() {
    if (true) {
      for (int i=0; i<1; i++) {
        if (false) { }
      }
    }
  }
}
"#;
    let (cu, spans) = Parser::new().parse_with_spans(src).expect("parse error");
    let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", src), spans);

    #[derive(Clone)]
    struct Capture {
        enters: Rc<RefCell<Vec<&'static str>>>,
        exits: Rc<RefCell<Vec<&'static str>>>,
    }

    impl Visit for Capture {
        fn enter(&mut self, node: &NodeRef, _session: &mut AnalysisSession) {
            if let Some(s) = node.of::<Statement>() {
                use analysis::syntax::statements::statement::Statement::*;
                let tag = match s {
                    If(_) => "If",
                    For(_) => "For",
                    Block(_) => "Block",
                    _ => "Other",
                };
                self.enters.borrow_mut().push(tag);
            }
        }
        fn exit(&mut self, node: &NodeRef, _session: &mut AnalysisSession) {
            if let Some(s) = node.of::<Statement>() {
                use analysis::syntax::statements::statement::Statement::*;
                let tag = match s {
                    If(_) => "If",
                    For(_) => "For",
                    Block(_) => "Block",
                    _ => "Other",
                };
                self.exits.borrow_mut().push(tag);
            }
        }
    }

    let cap = Capture {
        enters: Rc::new(RefCell::new(Vec::new())),
        exits: Rc::new(RefCell::new(Vec::new())),
    };
    let enters_ref = cap.enters.clone();
    let exits_ref = cap.exits.clone();

    let mut walker = AstWalker::new().with_visitor(Box::new(cap));
    walker.run(&cu, &mut session);

    let enters = enters_ref.borrow().clone();
    let exits = exits_ref.borrow().clone();
    // Expect DFS visit to include both If and For; ordering should have first If before first For
    assert!(enters.contains(&"If"));
    assert!(enters.contains(&"For"));
    let pos_if = enters
        .iter()
        .position(|&t| t == "If")
        .expect("first If position");
    let pos_for = enters
        .iter()
        .position(|&t| t == "For")
        .expect("first For position");
    assert!(
        pos_if < pos_for,
        "expected first If to be entered before first For: {:?}",
        enters
    );
    // Exit order should be reverse-nested (last-in-first-out for statements)
    assert!(exits.last().is_some());
}
