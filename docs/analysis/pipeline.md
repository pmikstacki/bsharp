# Analysis Pipeline

This document describes the analysis pipeline architecture, artifacts, rulesets, configuration toggles, and determinism guarantees in the B# analyzer.

## Phases

The pipeline runs in deterministic phases (see `src/analysis/framework/pipeline.rs`):

- **Index**
  - Runs early passes like `IndexingPass` to populate core artifacts (`SymbolIndex`, `NameIndex`, `FqnMap`).
- **Local Rules**
  - Runs per-file passes such as `MetricsPass` (Query-based) to compute artifacts like `AstAnalysis`.
  - Local rulesets run here as well; use `framework::query::Query` for AST enumeration.
- **Global**
  - Passes that aggregate information across the file (or project) after initial indexing.
- **Semantic**
  - Rules and passes that require previously built artifacts (e.g., control flow, dependencies).
- **Reporting**
  - Finalization phase that can synthesize report artifacts.

Each phase is explicitly selected in `AnalyzerPipeline::run_for_file()` using `Phase` discriminants. Pass and ruleset registration is driven by `AnalyzerRegistry`.

## Artifacts

Artifacts are stored in the per-file `AnalysisSession.artifacts` and summarized into an `AnalysisReport`:

- **Symbols** (`src/analysis/artifacts/symbols.rs`)
  - `SymbolIndex` (by id and name), `NameIndex` (name frequencies), `FqnMap` (local name → FQNs).
- **Control Flow** (`src/analysis/artifacts/cfg.rs`)
  - `ControlFlowIndex` keyed per method; summarized to `CfgSummary` with total methods and smell counts.
- **Dependencies** (`src/analysis/artifacts/dependencies.rs`)
  - Graph keyed by symbols; summarized to node/edge counts.
- **Metrics** (`src/analysis/artifacts/metrics.rs` → `AstAnalysis`)
  - Basic metrics gathered during the local traversal.

Artifacts are optional in the final report; missing artifacts simply result in `None` summaries.

## Rulesets and Passes

Rules implement the `Rule` trait and are grouped into logical rulesets. Passes implement `AnalyzerPass` and declare a `Phase`:

- Rulesets are separated into Local vs. Semantic groups and executed during the respective phases.
- Passes can be toggled individually by id.
- The registry is created with `AnalyzerRegistry::from_config(&AnalysisConfig)` to honor config toggles.

## Configuration

`AnalysisConfig` (`src/analysis/context.rs`) controls thresholds and toggles:

- **Control flow thresholds**
  - `cf_high_complexity_threshold` (default 10)
  - `cf_deep_nesting_threshold` (default 4)
- **Toggles**
  - `enable_rulesets: HashMap<String, bool>`
  - `enable_passes: HashMap<String, bool>`
  - `rule_severities: HashMap<String, DiagnosticSeverity>`
- **Workspace filters**
  - `workspace.follow_refs: bool`
  - `workspace.include: Vec<String>` (glob patterns)
  - `workspace.exclude: Vec<String>` (glob patterns)

CLI maps flags to these fields in `src/cli/commands/analyze.rs` and supports TOML/JSON config files.

## Workspace Analysis and Determinism

`AnalyzerPipeline::run_workspace()` and `run_workspace_with_config()`:

- Discover files deterministically by sorting absolute paths and deduping.
- Analyze each file independently, then merge artifacts into a single `AnalysisReport`.
- Diagnostics are sorted by file, line, column, then diagnostic code for stable output.
- Workspace loader warnings/errors are merged into `workspace_warnings` (sorted, deduped).
- When the `parallel_analysis` feature is enabled, files are analyzed in parallel but merged deterministically in path order.

## Report Schema

`AnalysisReport` (`src/analysis/report/mod.rs`) includes:

- `schema_version: u32` (currently 1)
- `diagnostics: DiagnosticCollection`
- `metrics: Option<AstAnalysis>`
- `cfg: Option<CfgSummary>`
- `deps: Option<DependencySummary>`
- `workspace_warnings: Vec<String>`
- `workspace_errors: Vec<String>` (reserved for future use)

The JSON shape is intentionally stable; tests use snapshots with path normalization to ensure cross-platform consistency.

## Testing Guidance

- Prefer deterministic fixtures under `tests/fixtures/`.
- Normalize absolute paths in snapshots (see `tests/integration/workspace_analysis_snapshot.rs`).
- For workspace filtering, use `run_workspace_with_config()` with `include`/`exclude` globs and snapshot the resulting report.

