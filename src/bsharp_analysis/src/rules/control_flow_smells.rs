use crate::artifacts::control_flow_graph::index::ControlFlowIndex;
use crate::framework::{AnalysisSession, Rule, RuleSet};
use crate::framework::NodeRef;
use crate::{diag, DiagnosticCode, rule, ruleset};

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

fn split_class_method(key: &str) -> (Option<String>, Option<String>) {
    let mut parts = key.split("::");
    let class = parts.next().map(|s| s.to_string());
    let method = parts.next().map(|s| s.to_string());
    (class, method)
}

rule! {
    HighCyclomaticComplexity: "cf.high_cyclomatic_complexity", "ControlFlowSmell", {
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
                if let (Some(c), Some(m)) = (class_name.as_deref(), method_name.as_deref())
                    && let Some((start, len)) = find_method_span(session, c, m)
                {
                    diag!(
                        session,
                        DiagnosticCode::BSW01001,
                        at_span start,
                        len,
                        msg: format!(
                            "Method '{}' has high cyclomatic complexity ({})",
                            key, stats.complexity
                        )
                    );
                } else {
                    diag!(
                        session,
                        DiagnosticCode::BSW01001,
                        msg: format!(
                            "Method '{}' has high cyclomatic complexity ({})",
                            key, stats.complexity
                        )
                    );
                }
            }
        }
    },
    DeepNesting: "cf.deep_nesting", "ControlFlowSmell", {
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
                if let (Some(c), Some(m)) = (class_name.as_deref(), method_name.as_deref())
                    && let Some((start, len)) = find_method_span(session, c, m)
                {
                    diag!(
                        session,
                        DiagnosticCode::BSW01005,
                        at_span start,
                        len,
                        msg: format!(
                            "Method '{}' has deep nesting (depth={})",
                            key, stats.max_nesting
                        )
                    );
                } else {
                    diag!(
                        session,
                        DiagnosticCode::BSW01005,
                        msg: format!(
                            "Method '{}' has deep nesting (depth={})",
                            key, stats.max_nesting
                        )
                    );
                }
            }
        }
    },
    LongMethodBySpan: "cf.long_method_span", "ControlFlowSmell", {
        if node.of::<crate::syntax::ast::CompilationUnit>().is_none() {
            return;
        }
        let Some(index) = session.artifacts.get::<ControlFlowIndex>() else {
            return;
        };
        let threshold: usize = 50; // TODO: make configurable
        let src = session.ctx.source().to_string();
        for (key, _stats) in index.iter() {
            if let (Some(class_name), Some(method_name)) = split_class_method(key)
                && let Some((start, len)) = find_method_span(session, &class_name, &method_name)
            {
                let end = start.saturating_add(len);
                let slice = &src.get(start..end).unwrap_or("");
                let line_count = slice.as_bytes().iter().filter(|&&b| b == b'\n').count() + 1;
                if line_count > threshold {
                    diag!(
                        session,
                        DiagnosticCode::BSW01002,
                        at_span start,
                        len,
                        msg: format!(
                            "Method '{}' is too long ({} lines > {})",
                            key, line_count, threshold
                        )
                    );
                }
            }
        }
    }
}

ruleset! {
    control_flow_smells: HighCyclomaticComplexity, DeepNesting, LongMethodBySpan
}
