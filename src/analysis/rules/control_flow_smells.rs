use crate::analysis::artifacts::cfg::ControlFlowIndex;
use crate::analysis::diagnostics::diagnostic_code::DiagnosticCode;
use crate::analysis::framework::diagnostic_builder::DiagnosticBuilder;
use crate::analysis::framework::rules::{Rule, RuleSet};
use crate::analysis::framework::session::AnalysisSession;
use crate::analysis::framework::walker::NodeRef;

fn find_method_span(session: &AnalysisSession, class_name: &str, method_name: &str) -> Option<(usize, usize)> {
    // Prefer matching by simple class name to tolerate FQN keys in CFG index
    let simple_class = class_name.rsplit('.').next().unwrap_or(class_name);
    for (k, range) in session.spans.iter() {
            if k.starts_with("method::") && k.ends_with(&format!("::{}::{}", simple_class, method_name)) {
            return Some((range.start, range.end - range.start));
        }
    }
    None
}

struct HighCyclomaticComplexity;
impl Rule for HighCyclomaticComplexity {
    fn id(&self) -> &'static str { "cf.high_cyclomatic_complexity" }
    fn category(&self) -> &'static str { "ControlFlowSmell" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        // Only run once per file at CompilationUnit enter
        if !matches!(node, NodeRef::CompilationUnit(_)) { return; }
        let Some(index) = session.artifacts.get::<ControlFlowIndex>() else { return; };
        let threshold = session.config.cf_high_complexity_threshold;
        for (key, stats) in index.iter() {
            if stats.complexity > threshold {
                // Key is "Class::Method"
                let (class_name, method_name) = split_class_method(key);
                let mut b = DiagnosticBuilder::new(DiagnosticCode::BSW01001)
                    .with_message(format!(
                        "Method '{}' has high cyclomatic complexity ({})",
                        key, stats.complexity
                    ));
                if let (Some(c), Some(m)) = (class_name.as_deref(), method_name.as_deref()) {
                    if let Some((start, len)) = find_method_span(session, c, m) { b = b.at_span(session, start, len); }
                }
                b.emit(session);
            }
        }
    }
}

struct DeepNesting;
impl Rule for DeepNesting {
    fn id(&self) -> &'static str { "cf.deep_nesting" }
    fn category(&self) -> &'static str { "ControlFlowSmell" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        if !matches!(node, NodeRef::CompilationUnit(_)) { return; }
        let Some(index) = session.artifacts.get::<ControlFlowIndex>() else { return; };
        let threshold = session.config.cf_deep_nesting_threshold;
        for (key, stats) in index.iter() {
            if stats.max_nesting > threshold {
                let (class_name, method_name) = split_class_method(key);
                let mut b = DiagnosticBuilder::new(DiagnosticCode::BSW01005)
                    .with_message(format!(
                        "Method '{}' has deep nesting (depth={})",
                        key, stats.max_nesting
                    ));
                if let (Some(c), Some(m)) = (class_name.as_deref(), method_name.as_deref()) {
                    if let Some((start, len)) = find_method_span(session, c, m) { b = b.at_span(session, start, len); }
                }
                b.emit(session);
            }
        }
    }
}

fn split_class_method(key: &str) -> (Option<String>, Option<String>) {
    let mut parts = key.split("::");
    let class = parts.next().map(|s| s.to_string());
    let method = parts.next().map(|s| s.to_string());
    (class, method)
}

pub fn ruleset() -> RuleSet {
    RuleSet::new("control_flow_smells")
        .with_rule(HighCyclomaticComplexity)
        .with_rule(DeepNesting)
}
