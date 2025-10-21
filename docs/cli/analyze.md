# Analyze Command

The `analyze` command performs comprehensive code analysis on C# files, projects, or solutions, generating detailed reports with diagnostics, metrics, and quality assessments.

---

## Usage

```bash
bsharp analyze <INPUT> [OPTIONS]
```

### Arguments

**`<INPUT>`** (required)
- Path to C# source file (`.cs`)
- Path to project file (`.csproj`)
- Path to solution file (`.sln`)
- Path to directory

---

## Options

### Output Options

**`--out <FILE>`**
- Output file path for analysis report (JSON)
- Default: stdout
- Creates parent directories if needed

**`--format <FORMAT>`**
- Output format: `json` (compact) or `pretty-json` (indented)
- Default: `pretty-json`

### Configuration

**`--config <FILE>`**
- Path to analysis configuration file (JSON or TOML)
- Overrides default settings
- CLI flags override config file settings

See: [Configuration Overview](../configuration/overview.md)

### Workspace Options

**`--follow-refs <BOOL>`**
- Follow ProjectReference dependencies transitively
- Default: `true`
- Set to `false` to analyze only specified project

**`--include <GLOB>...`**
- Include only files matching glob patterns
- Multiple patterns allowed
- Example: `--include "**/*Service.cs" "**/*Controller.cs"`

**`--exclude <GLOB>...`**
- Exclude files matching glob patterns
- Multiple patterns allowed
- Example: `--exclude "**/obj/**" "**/bin/**" "**/Tests/**"`

### Analysis Control

**`--enable-ruleset <ID>...`**
- Enable specific rulesets by ID
- Multiple IDs allowed
- Overrides config file
- Example: `--enable-ruleset naming quality`

**`--disable-ruleset <ID>...`**
- Disable specific rulesets by ID
- Multiple IDs allowed
- Example: `--disable-ruleset experimental`

**`--enable-pass <ID>...`**
- Enable specific analysis passes by ID
- Multiple IDs allowed
- Example: `--enable-pass indexing control_flow`

**`--disable-pass <ID>...`**
- Disable specific analysis passes by ID
- Multiple IDs allowed
- Example: `--disable-pass dependencies`

**`--severity <CODE=LEVEL>...`**
- Override diagnostic severity for specific codes
- Format: `CODE=level` where level is `error`, `warning`, `info`, or `hint`
- Multiple overrides allowed
- Example: `--severity MET001=error QUAL010=warning`

### Legacy Options (Single File Mode)

**`--symbol <NAME>`**
- Search for specific symbol by name
- Only works in single-file mode
- Prints symbol locations and information

---

## Examples

### Basic Analysis

```bash
# Analyze single file
bsharp analyze MyFile.cs

# Analyze project
bsharp analyze MyProject.csproj

# Analyze solution
bsharp analyze MySolution.sln
```

### Output to File

```bash
# Save report to file
bsharp analyze MyProject.csproj --out report.json

# Compact JSON format
bsharp analyze MyProject.csproj --out report.json --format json
```

### Using Configuration File

```bash
# Load config from file
bsharp analyze MyProject.csproj --config .bsharp.toml

# Config file with CLI overrides
bsharp analyze MyProject.csproj \
    --config .bsharp.toml \
    --enable-ruleset quality \
    --severity MET001=error
```

### Workspace Filtering

```bash
# Analyze only service files
bsharp analyze MySolution.sln --include "**/*Service.cs"

# Exclude test files
bsharp analyze MySolution.sln --exclude "**/Tests/**"

# Multiple filters
bsharp analyze MySolution.sln \
    --include "src/**/*.cs" \
    --exclude "**/obj/**" "**/bin/**" "**/Tests/**"
```

### Controlling Analysis

```bash
# Enable specific rulesets
bsharp analyze MyProject.csproj \
    --enable-ruleset naming quality control_flow

# Disable experimental features
bsharp analyze MyProject.csproj \
    --disable-ruleset experimental

# Enable/disable specific passes
bsharp analyze MyProject.csproj \
    --enable-pass indexing control_flow \
    --disable-pass dependencies
```

### Severity Overrides

```bash
# Treat specific warnings as errors
bsharp analyze MyProject.csproj \
    --severity MET001=error \
    --severity QUAL001=error

# Downgrade specific errors to warnings
bsharp analyze MyProject.csproj \
    --severity CS0168=warning
```

### Symbol Search (Single File)

```bash
# Find symbol in file
bsharp analyze MyFile.cs --symbol MyClass

# Output:
# Found symbol 'MyClass' at line 10, column 14
```

---

## Analysis Modes

### Single File Mode

**Triggered when:** Input is a `.cs` file

**Behavior:**
- Parses single file
- Runs analysis pipeline on CompilationUnit
- Supports `--symbol` option for symbol search
- Faster for quick checks

**Example:**
```bash
bsharp analyze Program.cs --out analysis.json
```

### Workspace Mode

**Triggered when:** Input is `.sln`, `.csproj`, or directory

**Behavior:**
- Loads entire workspace
- Discovers all source files
- Follows project references (if `--follow-refs true`)
- Applies include/exclude filters
- Analyzes all files deterministically
- Aggregates results into single report

**Example:**
```bash
bsharp analyze MySolution.sln \
    --follow-refs true \
    --exclude "**/Tests/**" \
    --out workspace-analysis.json
```

---

## Configuration File Format

### TOML Format

**`.bsharp.toml`:**
```toml
[analysis]
max_cyclomatic_complexity = 10
max_method_length = 50

[analysis.control_flow]
cf_high_complexity_threshold = 10
cf_deep_nesting_threshold = 4

[analysis.quality]
long_method = "warning"
god_class = "error"
empty_catch = "error"

[workspace]
follow_refs = true
include = ["src/**/*.cs"]
exclude = ["**/obj/**", "**/bin/**", "**/Tests/**"]

[enable_rulesets]
naming = true
quality = true
control_flow = true

[enable_passes]
indexing = true
control_flow = true
dependencies = true

[rule_severities]
MET001 = "error"
QUAL001 = "warning"
```

### JSON Format

**`.bsharp.json`:**
```json
{
  "analysis": {
    "max_cyclomatic_complexity": 10,
    "max_method_length": 50,
    "control_flow": {
      "cf_high_complexity_threshold": 10,
      "cf_deep_nesting_threshold": 4
    }
  },
  "workspace": {
    "follow_refs": true,
    "include": ["src/**/*.cs"],
    "exclude": ["**/obj/**", "**/bin/**"]
  },
  "enable_rulesets": {
    "naming": true,
    "quality": true
  },
  "enable_passes": {
    "indexing": true,
    "control_flow": true
  },
  "rule_severities": {
    "MET001": "error",
    "QUAL001": "warning"
  }
}
```

---

## Output Format

### Analysis Report Structure

```json
{
  "schema_version": 1,
  "diagnostics": {
    "items": [
      {
        "code": "MET001",
        "severity": "warning",
        "message": "Method has high cyclomatic complexity",
        "file": "src/OrderService.cs",
        "line": 42,
        "column": 17,
        "end_line": 85,
        "end_column": 5
      }
    ]
  },
  "metrics": {
    "total_lines": 1250,
    "code_lines": 980,
    "comment_lines": 150,
    "blank_lines": 120,
    "class_count": 15,
    "interface_count": 3,
    "method_count": 87,
    "total_complexity": 245,
    "max_complexity": 18,
    "max_nesting_depth": 5
  },
  "cfg": {
    "total_methods": 87,
    "high_complexity_count": 5,
    "deep_nesting_count": 3
  },
  "deps": {
    "total_nodes": 15,
    "total_edges": 42,
    "circular_dependencies": 0,
    "max_depth": 4
  },
  "workspace_warnings": [
    "Failed to parse project: MyBrokenProject.csproj"
  ]
}
```

### Diagnostic Fields

- **code**: Diagnostic code (e.g., `MET001`, `QUAL010`)
- **severity**: `error`, `warning`, `info`, or `hint`
- **message**: Human-readable description
- **file**: Source file path
- **line/column**: Start position
- **end_line/end_column**: End position (optional)

### Metrics Fields

- **total_lines**: Total lines including blank/comments
- **code_lines**: Lines with actual code
- **comment_lines**: Lines with comments
- **blank_lines**: Empty lines
- **class_count**: Number of classes
- **interface_count**: Number of interfaces
- **method_count**: Number of methods
- **total_complexity**: Sum of all method complexities
- **max_complexity**: Highest method complexity
- **max_nesting_depth**: Deepest nesting level

---

## Available Rulesets

### Built-in Rulesets

**`naming`** - Naming convention rules
- Class names: PascalCase
- Method names: PascalCase
- Field names: camelCase with `_` prefix
- Constant names: UPPER_CASE or PascalCase

**`quality`** - Code quality rules
- Long method detection
- Long parameter list
- God class detection
- Empty catch blocks
- Magic numbers

**`control_flow`** - Control flow rules
- High complexity warnings
- Deep nesting warnings
- Unreachable code detection

**`semantic`** - Semantic rules
- Type checking
- Null reference analysis
- Resource leak detection

---

## Available Passes

### Built-in Passes

**`indexing`** (Phase: Index)
- Builds symbol index
- Creates name index
- Generates FQN map

**`control_flow`** (Phase: Semantic)
- Analyzes control flow
- Calculates complexity metrics
- Detects control flow smells

**`dependencies`** (Phase: Global)
- Builds dependency graph
- Detects circular dependencies
- Calculates coupling metrics

**`reporting`** (Phase: Reporting)
- Generates final report
- Aggregates diagnostics
- Summarizes artifacts

---

## Diagnostic Codes

### Metrics (MET)

- **MET001**: High cyclomatic complexity
- **MET002**: Deep nesting detected
- **MET003**: Long method
- **MET004**: Long parameter list

### Quality (QUAL)

- **QUAL001**: Long method
- **QUAL002**: Long parameter list
- **QUAL010**: Empty catch block
- **QUAL020**: Naming convention violation
- **QUAL030**: Resource not disposed

### Control Flow (CF)

- **CF001**: Unreachable code
- **CF002**: High complexity
- **CF003**: Deep nesting

### Dependencies (DEP)

- **DEP001**: Circular dependency
- **DEP002**: High coupling
- **DEP003**: Unstable dependency

---

## Performance

### Analysis Speed

- **Single file** (< 1000 lines): < 100ms
- **Small project** (< 10 files): < 500ms
- **Medium project** (10-50 files): 500ms-2s
- **Large solution** (100+ files): 2-10s

### Memory Usage

- Scales with codebase size
- Typical: 50-200 MB for medium projects
- Artifacts cached in memory during analysis

### Parallel Analysis

With `parallel_analysis` feature enabled:
```bash
cargo build --release --features parallel_analysis
```

Files analyzed in parallel, significantly faster for large workspaces.

---

## Integration

### CI/CD Pipeline

```yaml
# GitHub Actions
- name: Code Quality Analysis
  run: |
    bsharp analyze MySolution.sln \
      --out analysis.json \
      --format json \
      --severity MET001=error QUAL001=error
    
    # Check for errors
    errors=$(jq '.diagnostics.items | map(select(.severity == "error")) | length' analysis.json)
    if [ "$errors" -gt 0 ]; then
      echo "Quality gate failed: $errors errors"
      exit 1
    fi
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

changed_files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.cs$')

for file in $changed_files; do
    result=$(bsharp analyze "$file" --format json 2>/dev/null)
    errors=$(echo "$result" | jq '.diagnostics.items | map(select(.severity == "error")) | length')
    
    if [ "$errors" -gt 0 ]; then
        echo "Analysis errors in $file"
        exit 1
    fi
done
```

### Quality Gate Script

```bash
#!/bin/bash
# quality-gate.sh

bsharp analyze MySolution.sln \
    --out report.json \
    --format json \
    --enable-ruleset naming quality control_flow \
    --severity MET001=error QUAL001=error

# Extract metrics
errors=$(jq '.diagnostics.items | map(select(.severity == "error")) | length' report.json)
max_complexity=$(jq '.metrics.max_complexity' report.json)

echo "Errors: $errors"
echo "Max Complexity: $max_complexity"

if [ "$errors" -gt 0 ]; then
    echo "❌ Quality gate failed: $errors errors found"
    exit 1
fi

if [ "$max_complexity" -gt 15 ]; then
    echo "❌ Quality gate failed: complexity $max_complexity exceeds threshold 15"
    exit 1
fi

echo "✅ Quality gate passed"
```

---

## Troubleshooting

### Analysis Fails

```bash
$ bsharp analyze MyProject.csproj
Error: Failed to load workspace
```

**Solutions:**
- Check project file is valid XML
- Verify all referenced projects exist
- Use `--follow-refs false` to skip references

### Out of Memory

```bash
Error: memory allocation failed
```

**Solutions:**
- Analyze smaller subsets with `--include`/`--exclude`
- Disable expensive passes with `--disable-pass`
- Increase system memory

### Slow Analysis

**Solutions:**
- Build with `parallel_analysis` feature
- Exclude unnecessary files
- Disable unused rulesets/passes

---

## Related Documentation

- [CLI Overview](./overview.md) - General CLI usage
- [Analysis Pipeline](../analysis/pipeline.md) - Analysis internals
- [Metrics Collection](../analysis/metrics.md) - Metrics details
- [Code Quality](../analysis/quality.md) - Quality rules
- [Report Schema](../analysis/report-schema.md) - Output JSON layout
- [Configuration Overview](../configuration/overview.md) - Config fields and examples

---

## References

- **Implementation:** `src/bsharp_cli/src/commands/analyze.rs`
- **Pipeline:** `src/bsharp_analysis/src/framework/pipeline.rs`
- **Configuration:** `src/bsharp_analysis/src/context.rs`
