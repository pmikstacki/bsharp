# Writing a Ruleset

This guide shows how to define rules and bundle them into a `RuleSet` to be executed by the analysis pipeline.

---

## Traits and Types

Source: `src/bsharp_analysis/src/framework/rules.rs`

```rust
pub enum RuleTarget { All, Declarations, Members, Statements, Expressions }

pub trait Rule: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn category(&self) -> &'static str;
    fn applies_to(&self) -> RuleTarget { RuleTarget::All }
    fn visit(&self, _node: &NodeRef, _session: &mut AnalysisSession) {}
}

pub struct RuleSet { pub id: &'static str, pub rules: Vec<Box<dyn Rule>> }
```

---

## Minimal Rule

```rust
use bsharp_analysis::framework::{Rule, RuleTarget, NodeRef, AnalysisSession};

pub struct NoEmptyCatch;

impl Rule for NoEmptyCatch {
    fn id(&self) -> &'static str { "QUAL010" }
    fn category(&self) -> &'static str { "quality" }
    fn applies_to(&self) -> RuleTarget { RuleTarget::Statements }

    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        if let NodeRef::Statement(stmt) = node {
            if let bsharp_syntax::statements::statement::Statement::Try(t) = stmt {
                for c in &t.catches {
                    if c.block_is_empty() {
                        session.diagnostics.add(
                            bsharp_analysis::DiagnosticCode::from_static("QUAL010"),
                            "Empty catch block",
                            None,
                        );
                    }
                }
            }
        }
    }
}
```

---

## Building a RuleSet

```rust
use bsharp_analysis::framework::RuleSet;

pub fn ruleset() -> RuleSet {
    RuleSet::new("quality")
        .with_rule(NoEmptyCatch)
        // .with_rule(AnotherRule)
}
```

Register in the default registry (`src/bsharp_analysis/src/framework/registry.rs`) or construct a custom registry.

```rust
reg.register_ruleset(crate::rules::quality::ruleset());         // local rules
reg.register_semantic_ruleset(crate::rules::control_flow_smells::ruleset());
```

Rulesets can be enabled/disabled via `AnalysisConfig.enable_rulesets["quality"] = true|false`.

---

## Tips

- **Choose RuleTarget** thoughtfully to avoid unnecessary visits.
- **Emit diagnostics** with specific codes and helpful messages.
- **Keep rules independent**; accumulate state in `AnalysisSession` artifacts when needed.
- **Honor config toggles**; only run if your ruleset is enabled.
