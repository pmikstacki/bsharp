# Configuration Overview

BSharp analysis can be configured via TOML or JSON files and by CLI flags that map to config fields.

---

## Locations

- Project root: `.bsharp.toml` or `.bsharp.json`
- Custom path via `bsharp analyze <INPUT> --config <FILE>`

---

## AnalysisConfig (fields)

Source: `src/bsharp_analysis/src/context.rs`

```rust
pub struct AnalysisConfig {
    // Control flow thresholds
    pub cf_high_complexity_threshold: usize, // default: 10
    pub cf_deep_nesting_threshold: usize,    // default: 4

    // Toggles and severities
    pub enable_rulesets: HashMap<String, bool>,
    pub enable_passes: HashMap<String, bool>,
    pub rule_severities: HashMap<String, DiagnosticSeverity>,

    // Workspace filters
    pub workspace: WorkspaceConfig,

    // Optional churn/PE settings (reserved/future)
    pub churn_enable: bool,
    pub churn_period_days: u32,
    pub churn_include_merges: bool,
    pub churn_max_commits: Option<u32>,
    pub pe_reference_paths: Vec<String>,
    pub pe_references: Vec<String>,
}

pub struct WorkspaceConfig {
    pub follow_refs: bool,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}
```

---

## TOML Example

```toml
[analysis]
cf_high_complexity_threshold = 10
cf_deep_nesting_threshold = 4

[enable_rulesets]
naming = true
semantic = true
control_flow_smells = true

[enable_passes]
passes.metrics = true
passes.control_flow = true
passes.dependencies = true

[rule_severities]
CF002 = "warning"
CF003 = "warning"

[workspace]
follow_refs = true
include = ["src/**/*.cs"]
exclude = ["**/obj/**", "**/bin/**"]
```

## JSON Example

```json
{
  "cf_high_complexity_threshold": 10,
  "cf_deep_nesting_threshold": 4,
  "enable_rulesets": {
    "naming": true,
    "semantic": true,
    "control_flow_smells": true
  },
  "enable_passes": {
    "passes.metrics": true,
    "passes.control_flow": true,
    "passes.dependencies": true
  },
  "rule_severities": {
    "CF002": "warning",
    "CF003": "warning"
  },
  "workspace": {
    "follow_refs": true,
    "include": ["src/**/*.cs"],
    "exclude": ["**/obj/**", "**/bin/**"]
  }
}
```

---

## CLI Mapping

- `--enable-ruleset <ID>` / `--disable-ruleset <ID>` → `enable_rulesets[ID] = true|false`
- `--enable-pass <ID>` / `--disable-pass <ID>` → `enable_passes[ID] = true|false`
- `--severity CODE=level` → `rule_severities[CODE] = level` (`error|warning|info|hint`)
- `--follow-refs <BOOL>` → `workspace.follow_refs`
- `--include <GLOB>...` → `workspace.include`
- `--exclude <GLOB>...` → `workspace.exclude`

---

## Tips

- Prefer TOML for readability; JSON is supported for tool integration.
- Thresholds influence `CfgSummary` counts in the final report.
- Use unique IDs for passes/rulesets consistent with registry (see Passes & Rules).
