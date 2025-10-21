
# Analysis Framework Overview

The BSharp analysis framework provides a comprehensive suite of tools for analyzing C# code at various levels of detail. It is built on top of the BSharp parser infrastructure and offers insights into code structure, quality, dependencies, and maintainability. These capabilities support standalone analysis tools and editor/CI integrations.

## Analysis Architecture

The analysis framework is organized into specialized modules:

```
src/bsharp_analysis/src/
├── framework/        # pipeline, passes, registry, session, walker, query
├── passes/           # indexing, metrics, control_flow, dependencies, reporting
├── artifacts/        # symbols, cfg, dependencies
├── metrics/          # AstAnalysis data + shared helpers
├── rules/            # naming, semantic, control_flow_smells
├── report/           # AnalysisReport assembly
└── (no quality module)
```

## Analysis Capabilities

### Control Flow Analysis

- **Path Analysis**: Identify all possible execution paths through methods
- **Reachability**: Detect unreachable code sections
- **Complexity Metrics**: Calculate cyclomatic complexity and other flow-based metrics
- **Dead Code Detection**: Find code that can never be executed

### Dependency Analysis

- **Type Dependencies**: Track relationships between types
- **Assembly Dependencies**: Analyze external assembly usage
- **Circular Dependencies**: Detect problematic dependency cycles
- **Coupling Metrics**: Measure afferent and efferent coupling

### Code Metrics

Comprehensive metrics collection across multiple dimensions:

#### Complexity Metrics
- Cyclomatic Complexity
- Cognitive Complexity
- Nesting Depth
- Method Length

#### Size Metrics
- Lines of Code (LOC)
- Source Lines of Code (SLOC)
- Comment Lines
- Method Count per Class

#### Maintainability Metrics
- Maintainability Index
- Technical Debt Indicators
- Code Duplication Detection
- Halstead Metrics

### Rules

- **Naming Rules**: Basic naming convention checks
- **Control Flow Smells**: Simple flow-related smells (e.g., deep nesting warnings)

### Type Analysis

- **Type Usage**: Track how types are used throughout the codebase
- **Generic Analysis**: Analyze generic type usage patterns
- **Inheritance Hierarchies**: Map class and interface hierarchies
- **Interface Compliance**: Validate interface implementations

## Analysis Workflow

### 1. AST Preparation

All analysis begins with a parsed AST:

```rust
let parser = Parser::new();
let compilation_unit = parser.parse(source_code)?;
```

### 2. Pipeline

Use the framework pipeline with registered passes. Per-file runs populate typed artifacts; a final `AnalysisReport` summarizes metrics, control flow, and dependencies.

```rust
use bsharp_analysis::framework::pipeline::AnalyzerPipeline;
use bsharp_analysis::framework::session::AnalysisSession;
use bsharp_analysis::context::AnalysisContext;
use bsharp_analysis::report::AnalysisReport;
use bsharp_parser::facade::Parser;

let parser = Parser::new();
let (cu, spans) = parser.parse_with_spans(source_code)?;
let ctx = AnalysisContext::new("file.cs", source_code);
let mut session = AnalysisSession::new(ctx, spans);
AnalyzerPipeline::run_with_defaults(&cu, &mut session);
let report: AnalysisReport = AnalysisReport::from_session(&session);
```

### 3. Analysis Execution

The pipeline runs passes in phases:

- Index → Metrics (local) → Global (CFG, deps) → Semantic rules → Reporting

Artifacts (e.g., `AstAnalysis`, `ControlFlowIndex`, `DependencyGraph`) are inserted into the `AnalysisSession` and consumed by reporting.

### 4. Results Processing

Analysis results are structured for easy consumption:

```rust
// Metrics results
println!("Cyclomatic Complexity: {}", metrics.cyclomatic_complexity);
println!("Lines of Code: {}", metrics.lines_of_code);

// Diagnostics
for d in &report.diagnostics.diagnostics {
    println!("{}: {}", d.code, d.message);
}
```

## Analysis Registry and Passes

Analyses are implemented as `AnalyzerPass` implementations registered in an `AnalyzerRegistry` and executed by the `AnalyzerPipeline`. Local rulesets and semantic rulesets run alongside passes based on `Phase`.

## Configuration and Customization

### Analysis Configuration

Analyzers can be configured for different scenarios:

```rust
let config = AnalysisConfig {
    max_cyclomatic_complexity: 10,
    max_method_length: 50,
    enforce_naming_conventions: true,
    detect_code_smells: true,
    // ... other configuration options
};

let analyzer = MetricsAnalyzer::with_config(config);
```

### Custom Rules

Extend analysis with custom rules:

```rust
let custom_analyzer = QualityAnalyzer::new()
    .add_rule(CustomRule::new("no-goto-statements"))
    .add_rule(CustomRule::new("max-parameters", 5))
    .add_rule(CustomRule::new("prefer-composition"));
```

### Reporting Options

Flexible reporting formats:

```rust
// JSON output
let json_report = analyzer.analyze(&ast).to_json();

// XML output
let xml_report = analyzer.analyze(&ast).to_xml();

// Custom format
let custom_report = analyzer.analyze(&ast).format_with(custom_formatter);
```

## Integration Points

### CLI Integration

Analysis capabilities are exposed through the `analyze` command and configured via options (format, config, include/exclude, enable/disable passes and rulesets, severity overrides). See `docs/cli/analyze.md` for details.

### Programmatic Usage

Direct integration in tools typically runs the pipeline and pulls artifacts from the session:

```rust
use bsharp_analysis::context::AnalysisContext;
use bsharp_analysis::framework::{AnalyzerPipeline, AnalysisSession};
use bsharp_analysis::metrics::AstAnalysis;
use bsharp_parser::facade::Parser;

let source = fs::read_to_string(path)?;
let (cu, spans) = Parser::new().parse_with_spans(&source)?;
let mut session = AnalysisSession::new(AnalysisContext::new(path, &source), spans);
AnalyzerPipeline::run_with_defaults(&cu, &mut session);
if let Some(ast) = session.artifacts.get::<AstAnalysis>() {
    println!("methods={} complexity={}", ast.total_methods, ast.cyclomatic_complexity);
}
```

## Performance Characteristics

### Analysis Performance

- **Incremental Analysis**: Support for analyzing only changed parts
- **Parallel Processing**: Multi-threaded analysis for large codebases
- **Memory Efficiency**: Minimal memory overhead during analysis
- **Caching**: Results caching for repeated analysis

### Scalability

The framework scales from single files to large enterprise codebases:

- Single file analysis: Sub-second performance
- Medium projects (100+ files): Seconds to minutes
- Large codebases (1000+ files): Minutes with parallel processing

This analysis framework provides the foundation for building sophisticated code quality tools, IDE integrations, and automated code review systems.
