use crate::analysis::framework::session::AnalysisSession;
use crate::syntax::ast::CompilationUnit;

/// Thin enum over AST nodes consumed by rules. Start with root-only; expand incrementally.
pub enum NodeRef<'a> {
    CompilationUnit(&'a CompilationUnit),
}

pub trait Visit {
    fn enter(&mut self, _node: &NodeRef, _session: &mut AnalysisSession) {}
    fn exit(&mut self, _node: &NodeRef, _session: &mut AnalysisSession) {}
}

pub struct AstWalker<'a> {
    visitors: Vec<Box<dyn Visit + 'a>>, // All visitors are run in a single traversal
}

impl<'a> AstWalker<'a> {
    pub fn new() -> Self { Self { visitors: Vec::new() } }

    pub fn with_visitor(mut self, v: Box<dyn Visit + 'a>) -> Self {
        self.visitors.push(v);
        self
    }

    pub fn run(&mut self, cu: &'a CompilationUnit, session: &mut AnalysisSession) {
        let node = NodeRef::CompilationUnit(cu);
        // Pre-order enter for all visitors
        for v in self.visitors.iter_mut() {
            v.enter(&node, session);
        }
        // TODO: Expand traversal into declarations/members/statements as NodeRef grows
        for v in self.visitors.iter_mut().rev() {
            v.exit(&node, session);
        }
    }
}
