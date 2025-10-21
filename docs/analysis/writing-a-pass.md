# Writing an Analyzer Pass

This guide shows how to create a new analysis pass by implementing `AnalyzerPass` and registering it in the analysis pipeline.

---

## Trait

Source: `src/bsharp_analysis/src/framework/passes.rs`

```rust
pub trait AnalyzerPass: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn phase(&self) -> Phase;                 // Index | LocalRules | Global | Semantic | Reporting
    fn depends_on(&self) -> &'static [&'static str] { &[] }
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {}
}
```

---

## Minimal Pass

```rust
use bsharp_analysis::framework::{AnalyzerPass, Phase, AnalysisSession};
use bsharp_syntax::CompilationUnit;

pub struct MyPass;

impl AnalyzerPass for MyPass {
    fn id(&self) -> &'static str { "passes.my_pass" }
    fn phase(&self) -> Phase { Phase::LocalRules }

    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        // Inspect `cu` and write results into `session.artifacts` or `session.diagnostics`
        // Example: count classes and log a note (pseudo)
        let mut count = 0usize;
        for _c in bsharp_analysis::framework::Query::from(cu).of::<bsharp_syntax::ClassDeclaration>() {
            count += 1;
        }
        // session.artifacts.insert(MyArtifact { class_count: count });
        // session.diagnostics.add(...);
    }
}
```

---

## Registration

Add your pass to the default registry in `src/bsharp_analysis/src/framework/registry.rs`:

```rust
reg.register_pass(crate::passes::my_pass::MyPass);
```

Or, build a custom registry for experiments:

```rust
let mut reg = AnalyzerRegistry::default_registry();
reg.register_pass(MyPass);
AnalyzerPipeline::run_for_file(&cu, &mut session, &reg);
```

You can also toggle passes via `AnalysisConfig.enable_passes["passes.my_pass"] = true|false` (see configuration docs).

---

## Tips

- **Keep passes small**: Focus on one responsibility.
- **Prefer Query/AstWalker**: Use `Query` for typed enumeration or `AstWalker` with `Visit` for custom traversal.
- **Write artifacts**: Insert results with `session.artifacts.insert(T)` when they may be consumed later.
- **Determinism**: Avoid non-deterministic ordering; use sorted maps/lists if needed.
