
# Analysis Framework Overview

The BSharp analysis framework provides a comprehensive suite of tools for analyzing C# code at various levels of detail. Built on top of the robust parser infrastructure, it offers insights into code structure, quality, dependencies, and maintainability.

## Analysis Architecture

The analysis framework is organized into specialized modules:

```
src/analysis/
├── control_flow/     # Control flow analysis
├── dependencies/     # Dependency tracking and analysis
├── metrics/          # Code metrics collection
├── naming/           # Naming convention analysis
├── navigation/       # Advanced AST navigation
├── quality/          # Code quality assessment
└── types/           # Type system analysis
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

### Quality Analysis

- **Code Smells**: Detect common anti-patterns
- **Best Practices**: Validate adherence to coding standards
- **Performance Issues**: Identify potential performance problems
- **Security Concerns**: Basic security-related code analysis

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

### 2. Analysis Selection

Choose specific analyzers based on requirements:

```rust
// Individual analyzers
let metrics_analyzer = MetricsAnalyzer::new();
let dependency_analyzer = DependencyAnalyzer::new();

// Composite analysis
let quality_analyzer = QualityAnalyzer::new()
    .with_metrics()
    .with_dependencies()
    .with_control_flow();
```

### 3. Analysis Execution

Run analysis on the AST:

```rust
let metrics = metrics_analyzer.analyze(&compilation_unit);
let dependencies = dependency_analyzer.analyze(&compilation_unit);
let quality_report = quality_analyzer.analyze(&compilation_unit);
```

### 4. Results Processing

Analysis results are structured for easy consumption:

```rust
// Metrics results
println!("Cyclomatic Complexity: {}", metrics.cyclomatic_complexity);
println!("Lines of Code: {}", metrics.lines_of_code);

// Quality assessment
for issue in quality_report.issues {
    println!("Issue: {} at line {}", issue.message, issue.line_number);
}
```

## Analysis Traits and Interfaces

### Core Analysis Trait

All analyzers implement a common interface:

```rust
pub trait Analyzer<T> {
    type Output;
    
    fn analyze(&self, input: &T) -> Self::Output;
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
}
```

### Specialized Analysis Traits

Domain-specific traits for different analysis types:

```rust
pub trait MetricsAnalyzer {
    fn calculate_complexity(&self, method: &MethodDeclaration) -> ComplexityMetrics;
    fn calculate_size_metrics(&self, class: &ClassDeclaration) -> SizeMetrics;
    fn calculate_maintainability(&self, compilation_unit: &CompilationUnit) -> MaintainabilityMetrics;
}

pub trait QualityAnalyzer {
    fn detect_code_smells(&self, ast: &CompilationUnit) -> Vec<CodeSmell>;
    fn validate_naming_conventions(&self, ast: &CompilationUnit) -> Vec<NamingViolation>;
    fn check_best_practices(&self, ast: &CompilationUnit) -> Vec<BestPracticeViolation>;
}
```

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

Analysis capabilities are exposed through the CLI:

```bash
# Basic metrics
bsharp analyze metrics input.cs

# Quality analysis
bsharp analyze quality input.cs --config quality-config.json

# Dependency analysis
bsharp analyze dependencies input.cs --output dependencies.json
```

### Programmatic Usage

Direct integration in other tools:

```rust
use bsharp::analysis::{MetricsAnalyzer, QualityAnalyzer};

fn analyze_project(files: Vec<&str>) -> AnalysisResults {
    let mut results = AnalysisResults::new();
    
    for file in files {
        let ast = parse_file(file)?;
        let metrics = MetricsAnalyzer::new().analyze(&ast);
        let quality = QualityAnalyzer::new().analyze(&ast);
        
        results.add_file_analysis(file, metrics, quality);
    }
    
    results
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
