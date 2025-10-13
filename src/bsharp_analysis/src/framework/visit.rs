use crate::framework::{AnalysisSession, NodeRef};

pub trait Visit {
    fn enter(&mut self, _node: &NodeRef, _session: &mut AnalysisSession) {}
    fn exit(&mut self, _node: &NodeRef, _session: &mut AnalysisSession) {}
}
