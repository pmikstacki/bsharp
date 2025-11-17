use crate::framework::AnalysisSession;
use crate::framework::NodeRef;

pub trait Rule: std::fmt::Debug {
    fn id(&self) -> &'static str;
    fn category(&self) -> &'static str;
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession);
}
