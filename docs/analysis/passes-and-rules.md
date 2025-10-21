# Passes and Rules Registry

This page summarizes the default analysis registry: which passes and rulesets are registered by default and when they run.

---

## Default Registry

Source: `src/bsharp_analysis/src/framework/registry.rs`

```rust
// Simplified summary based on default_registry()
- Pass: indexing::IndexingPass          // indexing/symbols
- Pass: pe_loader::PeLoaderPass         // external PE metadata (if available)
- Pass: metrics::MetricsPass            // local metrics (Query-based)
- Ruleset (local): rules::naming        // naming conventions
- Ruleset (local): rules::semantic      // baseline semantic checks (local)
- Pass: control_flow::ControlFlowPass   // control flow stats and diagnostics
- Pass: dependencies::DependenciesPass  // dependency graph & summary
- Ruleset (semantic): control_flow_smells // consumes global artifacts
- Pass: reporting::ReportingPass        // consolidate artifacts into report
```

Notes:
- Each pass declares its own `Phase` (`AnalyzerPass::phase()`), e.g. `MetricsPass` runs in `Phase::LocalRules`.
- Semantic rulesets (e.g., `control_flow_smells`) run after global artifacts are produced.

---

## Phases

- **Index**: Build indexes (symbols, FQNs) and load external metadata.
- **LocalRules**: Run per-file local analyses (e.g., metrics) and baseline rules.
- **Global/Semantic**: Build global artifacts (control flow, dependencies), then run semantic rules consuming them.
- **Reporting**: Finalize results into `AnalysisReport`.

---

## Configuration: Enabling/Disabling

Toggles are driven by `AnalysisConfig`:

- Passes: `enable_passes[pass_id] = true|false`
- Rulesets: `enable_rulesets[ruleset_id] = true|false`
- Severities: `rule_severities["CODE"] = Error|Warning|Info|Hint`

The CLI maps flags to these fields (see `docs/cli/analyze.md`).

---

## IDs

- Pass IDs (`AnalyzerPass::id()`):
  - `passes.indexing`
  - `passes.pe_loader`
  - `passes.metrics`
  - `passes.control_flow`
  - `passes.dependencies`
  - `passes.reporting`
- Ruleset IDs depend on the ruleset constructors (e.g., `naming`, `semantic`, `control_flow_smells`).

---

## References

- `src/bsharp_analysis/src/framework/registry.rs`
- `src/bsharp_analysis/src/passes/*`
- `src/bsharp_analysis/src/rules/*`
