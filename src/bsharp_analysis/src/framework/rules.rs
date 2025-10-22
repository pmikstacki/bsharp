use crate::framework::AnalysisSession;
use crate::framework::NodeRef;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RuleTarget {
    All,
    Declarations,
    Members,
    Statements,
    Expressions,
}

pub trait Rule: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn category(&self) -> &'static str;
    fn applies_to(&self) -> RuleTarget {
        RuleTarget::All
    }
    fn visit(&self, _node: &NodeRef, _session: &mut AnalysisSession) {}
}

pub struct RuleSet {
    pub id: &'static str,
    pub rules: Vec<Box<dyn Rule>>, // boxed trait objects for heterogenous rules
}

impl RuleSet {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            rules: Vec::new(),
        }
    }

    pub fn with_rule(mut self, rule: impl Rule) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn Rule> {
        self.rules.iter().map(|b| b.as_ref())
    }
}
