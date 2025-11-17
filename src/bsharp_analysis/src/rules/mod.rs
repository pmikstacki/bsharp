// Rule groups

pub mod control_flow_smells;
pub mod naming;
pub mod semantic;

use crate::framework::RuleSet;

pub use control_flow_smells::control_flow_smells_ruleset as control_flow_smells;
pub use naming::naming_ruleset as naming;
pub use semantic::semantic_ruleset as semantic;

pub fn all() -> Vec<RuleSet> {
    vec![naming(), semantic(), control_flow_smells()]
}
