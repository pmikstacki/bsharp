use crate::AnalysisConfig;
use crate::framework::{AnalyzerPass, RuleSet};

pub struct AnalyzerRegistry {
    passes: Vec<Box<dyn AnalyzerPass>>,
    rulesets_local: Vec<RuleSet>,
    rulesets_semantic: Vec<RuleSet>,
}

impl AnalyzerRegistry {
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            rulesets_local: Vec::new(),
            rulesets_semantic: Vec::new(),
        }
    }

    pub fn register_pass(&mut self, pass: impl AnalyzerPass) {
        self.passes.push(Box::new(pass));
    }

    pub fn register_ruleset(&mut self, ruleset: RuleSet) {
        self.rulesets_local.push(ruleset);
    }

    pub fn register_semantic_ruleset(&mut self, ruleset: RuleSet) {
        self.rulesets_semantic.push(ruleset);
    }

    pub fn passes(&self) -> &[Box<dyn AnalyzerPass>] {
        &self.passes
    }
    pub fn rulesets_local(&self) -> &[RuleSet] {
        &self.rulesets_local
    }
    pub fn rulesets_semantic(&self) -> &[RuleSet] {
        &self.rulesets_semantic
    }

    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
            && self.rulesets_local.is_empty()
            && self.rulesets_semantic.is_empty()
    }
}

impl Default for AnalyzerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalyzerRegistry {
    /// Build a minimal default registry used for end-to-end runs in early phases.
    pub fn default_registry() -> Self {
        let mut reg = Self::new();
        // Phase B: indexing
        reg.register_pass(crate::passes::indexing::IndexingPass);
        // Phase B: PE loader (external IL)
        reg.register_pass(crate::passes::pe_loader::PeLoaderPass);
        // Phase C: metrics (generic Query-based)
        reg.register_pass(crate::passes::metrics::MetricsPass);
        // Phase C: baseline naming rules
        reg.register_ruleset(crate::rules::naming::ruleset());
        // Phase C: baseline semantic rules (these don't depend on global artifacts yet)
        reg.register_ruleset(crate::rules::semantic::ruleset());
        // Phase D: global passes
        reg.register_pass(crate::passes::control_flow::ControlFlowPass);
        reg.register_pass(crate::passes::dependencies::DependenciesPass);
        // Phase D: rules that consume global artifacts should run in Semantic phase
        reg.register_semantic_ruleset(crate::rules::control_flow_smells::ruleset());
        // Phase E: semantic passes
        reg.register_pass(crate::semantic::symbols::SymbolsPass);
        reg.register_pass(crate::semantic::binding::BindingPass);
        reg.register_pass(crate::semantic::types::TypeCheckPass);
        reg.register_pass(crate::semantic::overload::OverloadPass);
        reg.register_pass(crate::semantic::generics::GenericsPass);
        reg.register_pass(crate::semantic::flow::FlowPass);
        reg.register_pass(crate::semantic::nullability::NullabilityPass);
        reg.register_pass(crate::semantic::attributes::AttributesPass);
        reg.register_pass(crate::semantic::access::AccessPass);
        reg.register_pass(crate::semantic::extensions::ExtensionsPass);
        // Phase E: reporting
        reg.register_pass(crate::passes::reporting::ReportingPass);
        reg
    }

    /// Build a registry filtered by AnalysisConfig toggles.
    pub fn from_config(cfg: &AnalysisConfig) -> Self {
        let mut reg = Self::default_registry();
        // Filter passes
        if !cfg.enable_passes.is_empty() {
            reg.passes
                .retain(|p| cfg.enable_passes.get(p.id()).cloned().unwrap_or(true));
        }
        // Filter local rulesets
        if !cfg.enable_rulesets.is_empty() {
            reg.rulesets_local
                .retain(|rs| cfg.enable_rulesets.get(rs.id).cloned().unwrap_or(true));
            reg.rulesets_semantic
                .retain(|rs| cfg.enable_rulesets.get(rs.id).cloned().unwrap_or(true));
        }
        reg
    }
}
