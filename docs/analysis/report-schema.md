# Analysis Report Schema

The `AnalysisReport` summarizes diagnostics and artifacts produced by the analysis pipeline.

---

## Struct

Source: `src/bsharp_analysis/src/report/mod.rs`

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgSummary {
    pub total_methods: usize,
    pub high_complexity_methods: usize,
    pub deep_nesting_methods: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub schema_version: u32,
    pub diagnostics: DiagnosticCollection,
    pub metrics: Option<AstAnalysis>,
    pub cfg: Option<CfgSummary>,
    pub deps: Option<DependencySummary>,
    pub workspace_warnings: Vec<String>,
    pub workspace_errors: Vec<String>,
}
```

---

## Field Details

- `schema_version` – current schema version (1)
- `diagnostics` – all emitted diagnostics with codes, severities, locations
- `metrics` – aggregated `AstAnalysis` when `MetricsPass` runs
- `cfg` – summarized control flow stats when `ControlFlowPass` runs
- `deps` – dependency summary when `DependenciesPass` runs
- `workspace_warnings` – non-fatal workspace-level messages
- `workspace_errors` – reserved for future use

---

## Example (pretty JSON)

```json
{
  "schema_version": 1,
  "diagnostics": {
    "diagnostics": [
      {
        "code": "CF002",
        "severity": "warning",
        "message": "High cyclomatic complexity",
        "file": "src/OrderProcessor.cs",
        "line": 42,
        "column": 17
      }
    ]
  },
  "metrics": {
    "total_classes": 15,
    "total_interfaces": 3,
    "total_structs": 2,
    "total_enums": 1,
    "total_records": 0,
    "total_delegates": 0,
    "total_methods": 87,
    "total_properties": 21,
    "total_fields": 12,
    "total_events": 0,
    "total_constructors": 15,
    "total_if_statements": 20,
    "total_for_loops": 5,
    "total_while_loops": 2,
    "total_switch_statements": 3,
    "total_try_statements": 1,
    "total_using_statements": 2,
    "cyclomatic_complexity": 245,
    "lines_of_code": 980,
    "max_nesting_depth": 5,
    "documented_methods": 0,
    "documented_classes": 0
  },
  "cfg": {
    "total_methods": 87,
    "high_complexity_methods": 5,
    "deep_nesting_methods": 3
  },
  "deps": {
    "nodes": 42,
    "edges": 120
  },
  "workspace_warnings": [],
  "workspace_errors": []
}
```

---

## Where It Comes From

`AnalysisReport::from_session(&session)` collects:
- `metrics` from `session.artifacts.get::<AstAnalysis>()`
- `cfg` by summarizing the `ControlFlowIndex` artifact against thresholds
- `deps` by summarizing `DependencyGraph`
- `diagnostics` copied from `session.diagnostics`

---

## Related

- `docs/cli/analyze.md` – CLI options and examples
- `docs/analysis/pipeline.md` – Where in the pipeline artifacts are produced
