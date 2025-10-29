use crate::artifacts::control_flow_graph::index::ControlFlowIndex;
use crate::framework::NodeRef;
use crate::framework::{AnalysisSession, Rule, RuleSet};
use crate::{DiagnosticBuilder, DiagnosticCode};

fn find_method_span(
    session: &AnalysisSession,
    class_name: &str,
    method_name: &str,
) -> Option<(usize, usize)> {
    // Prefer matching by simple class name to tolerate FQN keys in CFG index
    let simple_class = class_name.rsplit('.').next().unwrap_or(class_name);
    for (k, range) in session.spans.iter() {
        if k.starts_with("method::") && k.ends_with(&format!("::{}::{}", simple_class, method_name))
        {
            return Some((range.start, range.end - range.start));
        }
    }
    None
}

struct HighCyclomaticComplexity;
impl Rule for HighCyclomaticComplexity {
    fn id(&self) -> &'static str {
        "cf.high_cyclomatic_complexity"
    }
    fn category(&self) -> &'static str {
        "ControlFlowSmell"
    }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        // Only run once per file at CompilationUnit enter
        if node.of::<crate::syntax::ast::CompilationUnit>().is_none() {
            return;
        }
        let Some(index) = session.artifacts.get::<ControlFlowIndex>() else {
            return;
        };
        let threshold = session.config.cf_high_complexity_threshold;
        for (key, stats) in index.iter() {
            if stats.complexity > threshold {
                // Key is "Class::Method"
                let (class_name, method_name) = split_class_method(key);
                let mut b = DiagnosticBuilder::new(DiagnosticCode::BSW01001).with_message(format!(
                    "Method '{}' has high cyclomatic complexity ({})",
                    key, stats.complexity
                ));
                if let (Some(c), Some(m)) = (class_name.as_deref(), method_name.as_deref())
                    && let Some((start, len)) = find_method_span(session, c, m)
                {
                    b = b.at_span(session, start, len);
                }
                b.emit(session);
            }
        }
    }
}

struct DeepNesting;
impl Rule for DeepNesting {
    fn id(&self) -> &'static str {
        "cf.deep_nesting"
    }
    fn category(&self) -> &'static str {
        "ControlFlowSmell"
    }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        if node.of::<crate::syntax::ast::CompilationUnit>().is_none() {
            return;
        }
        let Some(index) = session.artifacts.get::<ControlFlowIndex>() else {
            return;
        };
        let threshold = session.config.cf_deep_nesting_threshold;
        for (key, stats) in index.iter() {
            if stats.max_nesting > threshold {
                let (class_name, method_name) = split_class_method(key);
                let mut b = DiagnosticBuilder::new(DiagnosticCode::BSW01005).with_message(format!(
                    "Method '{}' has deep nesting (depth={})",
                    key, stats.max_nesting
                ));
                if let (Some(c), Some(m)) = (class_name.as_deref(), method_name.as_deref()) {
                    if let Some((start, len)) = find_method_span(session, c, m) {
                        b = b.at_span(session, start, len);
                    }
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
        .with_rule(LongMethodBySpan)
}

struct LongMethodBySpan;
impl Rule for LongMethodBySpan {
    fn id(&self) -> &'static str {
        "cf.long_method_span"
    }
    fn category(&self) -> &'static str {
        "ControlFlowSmell"
    }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        if node.of::<crate::syntax::ast::CompilationUnit>().is_none() {
            return;
        }
        let Some(index) = session.artifacts.get::<ControlFlowIndex>() else {
            return;
        };
        let threshold: usize = 50; // TODO: make configurable
        let src = session.ctx.source().to_string();
        for (key, _stats) in index.iter() {
            if let (Some(class_name), Some(method_name)) =
                super::control_flow_smells::split_class_method(key)
                && let Some((start, len)) = find_method_span(session, &class_name, &method_name)
            {
                let end = start.saturating_add(len);
                let slice = &src.get(start..end).unwrap_or("");
                let line_count = slice.as_bytes().iter().filter(|&&b| b == b'\n').count() + 1;
                if line_count > threshold {
                    let b = DiagnosticBuilder::new(DiagnosticCode::BSW01002)
                        .with_message(format!(
                            "Method '{}' is too long ({} lines > {})",
                            key, line_count, threshold
                        ))
                        .at_span(session, start, len);
                    b.emit(session);
                }
            }
        }
    }
}
