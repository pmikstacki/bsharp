# Metrics Collection

The BSharp metrics system collects comprehensive code metrics during analysis to assess code complexity, size, and maintainability.

---

## Overview

**Location:** `src/bsharp_analysis/src/metrics/`

The metrics system provides:
- **Basic Metrics** - Lines of code, statement counts, declaration counts
- **Complexity Metrics** - Cyclomatic complexity, cognitive complexity, nesting depth
- **Maintainability Metrics** - Maintainability index, Halstead metrics

---

## Architecture

### Core Components

```
src/bsharp_analysis/src/metrics/
├── core.rs     # AstAnalysis data structure (aggregated counts)
└── shared.rs   # Helpers: decision_points, max_nesting_of, count_statements, etc.
```

### How metrics are produced

- `MetricsPass` runs in `Phase::LocalRules` and computes an `AstAnalysis` artifact using the Query API to enumerate declarations, plus lightweight walkers for statement counts.
- Access `AstAnalysis` from `AnalysisSession` after running the pipeline.

---

## Metric Types

### 1. Basic Metrics

**AstAnalysis Structure:**
```rust
pub struct AstAnalysis {
    // Size metrics
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    
    // Declaration counts
    pub namespace_count: usize,
    pub class_count: usize,
    pub interface_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub method_count: usize,
    pub property_count: usize,
    pub field_count: usize,
    
    // Statement counts
    pub statement_count: usize,
    pub expression_count: usize,
    
    // Complexity (aggregated)
    pub total_complexity: usize,
    pub max_complexity: usize,
    pub max_nesting_depth: usize,
}
```

<!-- Collection via trait impl removed; metrics are produced by MetricsPass during the pipeline. -->

### 2. Complexity Metrics

#### Cyclomatic Complexity

**Definition:** Number of linearly independent paths through code

**Formula:** `CC = E - N + 2P`
- E = edges in control flow graph
- N = nodes in control flow graph
- P = connected components (usually 1)

**Simplified:** `CC = 1 + number of decision points`

**Decision Points:**
- `if`, `else if`
- `case` in `switch`
- `for`, `foreach`, `while`, `do-while`
- `&&`, `||` in conditions
- `catch` clauses
- `?:` ternary operator
- `??` null-coalescing operator

**Example:**
```csharp
public void ProcessOrder(Order order) {  // CC = 1 (base)
    if (order == null) {                 // +1 = 2
        throw new ArgumentNullException();
    }
    
    if (order.IsValid) {                 // +1 = 3
        if (order.Amount > 1000) {       // +1 = 4
            ApplyDiscount(order);
        }
        SaveOrder(order);
    } else {                             // else doesn't add
        LogError(order);
    }
}
// Total CC = 4
```

**Implementation:**
```rust
pub fn cyclomatic_complexity(method: &MethodDeclaration) -> usize {
    let mut complexity = 1;  // Base complexity
    
    if let Some(body) = &method.body {
        complexity += count_decision_points(body);
    }
    
    complexity
}

fn count_decision_points(stmt: &Statement) -> usize {
    let mut count = 0;
    
    walk_statements(stmt, &mut |s| {
        match s {
            Statement::If(_) => count += 1,
            Statement::For(_) => count += 1,
            Statement::ForEach(_) => count += 1,
            Statement::While(_) => count += 1,
            Statement::DoWhile(_) => count += 1,
            Statement::Switch(sw) => {
                // Each case is a decision point
                count += sw.sections.len();
            }
            Statement::Try(try_stmt) => {
                // Each catch is a decision point
                count += try_stmt.catch_clauses.len();
            }
            _ => {}
        }
    });
    
    // Also count logical operators in expressions
    // count += count_logical_operators(stmt);
    
    count
}
```

**Thresholds:**
- **1-10:** Simple, low risk
- **11-20:** Moderate complexity, moderate risk
- **21-50:** Complex, high risk
- **50+:** Very complex, very high risk - refactor recommended

#### Cognitive Complexity

**Definition:** Measure of how difficult code is to understand

**Increments:**
- **+1** for each: `if`, `else if`, `switch`, `for`, `foreach`, `while`, `do-while`, `catch`, `?:`, `??`
- **+1** for each level of nesting (nested control structures)
- **+1** for each `break` or `continue` that jumps out of nested structure
- **+1** for each recursive call

**Example:**
```csharp
public void Process(List<int> items) {
    if (items != null) {                 // +1 (if)
        foreach (var item in items) {    // +1 (loop) +1 (nesting) = +2
            if (item > 0) {              // +1 (if) +2 (nesting) = +3
                Process(item);           // +1 (recursion) +3 (nesting) = +4
            }
        }
    }
}
// Total Cognitive Complexity = 1 + 2 + 3 + 4 = 10
```

**Implementation:**
```rust
pub fn cognitive_complexity(method: &MethodDeclaration) -> usize {
    let mut complexity = 0;
    
    if let Some(body) = &method.body {
        complexity = calculate_cognitive_complexity(body, 0);
    }
    
    complexity
}

fn calculate_cognitive_complexity(stmt: &Statement, nesting_level: usize) -> usize {
    let mut complexity = 0;
    
    match stmt {
        Statement::If(if_stmt) => {
            complexity += 1 + nesting_level;  // if + nesting penalty
            complexity += calculate_cognitive_complexity(&if_stmt.consequence, nesting_level + 1);
            if let Some(alt) = &if_stmt.alternative {
                complexity += calculate_cognitive_complexity(alt, nesting_level + 1);
            }
        }
        Statement::For(for_stmt) => {
            complexity += 1 + nesting_level;
            if let Some(body) = &for_stmt.body {
                complexity += calculate_cognitive_complexity(body, nesting_level + 1);
            }
        }
        // ... other statement types
        _ => {}
    }
    
    complexity
}
```

#### Nesting Depth

**Definition:** Maximum depth of nested control structures

**Example:**
```csharp
public void Example() {
    if (condition1) {              // Depth 1
        while (condition2) {       // Depth 2
            if (condition3) {      // Depth 3
                for (int i = 0; i < 10; i++) {  // Depth 4
                    // Code here
                }
            }
        }
    }
}
// Max Nesting Depth = 4
```

**Implementation:**
```rust
pub fn max_nesting_depth(method: &MethodDeclaration) -> usize {
    method.body.as_ref()
        .map(|body| calculate_max_nesting(body, 0))
        .unwrap_or(0)
}

fn calculate_max_nesting(stmt: &Statement, current_depth: usize) -> usize {
    let mut max_depth = current_depth;
    
    match stmt {
        Statement::If(if_stmt) => {
            let then_depth = calculate_max_nesting(&if_stmt.consequence, current_depth + 1);
            max_depth = max_depth.max(then_depth);
            
            if let Some(alt) = &if_stmt.alternative {
                let else_depth = calculate_max_nesting(alt, current_depth + 1);
                max_depth = max_depth.max(else_depth);
            }
        }
        Statement::Block(stmts) => {
            for s in stmts {
                let depth = calculate_max_nesting(s, current_depth);
                max_depth = max_depth.max(depth);
            }
        }
        // ... other nesting statements
        _ => {}
    }
    
    max_depth
}
```

**Thresholds:**
- **1-3:** Acceptable
- **4-5:** Consider refactoring
- **6+:** Refactor recommended

### Planned: Maintainability Metrics

#### Maintainability Index

**Definition:** Composite metric indicating code maintainability

**Formula (Microsoft version):**
```
MI = MAX(0, (171 - 5.2 * ln(HV) - 0.23 * CC - 16.2 * ln(LOC)) * 100 / 171)
```

Where:
- **HV** = Halstead Volume
- **CC** = Cyclomatic Complexity
- **LOC** = Lines of Code

**Scale:**
- **85-100:** Good maintainability (green)
- **65-84:** Moderate maintainability (yellow)
- **0-64:** Difficult to maintain (red)

Note: Maintainability Index is not implemented in the current codebase. This section outlines potential future work.

```rust
pub fn maintainability_index(
    halstead_volume: f64,
    cyclomatic_complexity: usize,
    lines_of_code: usize
) -> f64 {
    let hv_term = 5.2 * halstead_volume.ln();
    let cc_term = 0.23 * (cyclomatic_complexity as f64);
    let loc_term = 16.2 * (lines_of_code as f64).ln();
    
    let mi = 171.0 - hv_term - cc_term - loc_term;
    let normalized = (mi * 100.0 / 171.0).max(0.0);
    
    normalized
}
```

#### Planned: Halstead Metrics

**Operators and Operands:**
- **n1** = number of distinct operators
- **n2** = number of distinct operands
- **N1** = total number of operators
- **N2** = total number of operands

**Derived Metrics:**
- **Program Vocabulary:** `n = n1 + n2`
- **Program Length:** `N = N1 + N2`
- **Calculated Length:** `N' = n1 * log2(n1) + n2 * log2(n2)`
- **Volume:** `V = N * log2(n)`
- **Difficulty:** `D = (n1 / 2) * (N2 / n2)`
- **Effort:** `E = D * V`
- **Time to Program:** `T = E / 18` seconds
- **Bugs Delivered:** `B = V / 3000`

Note: Halstead metrics are not implemented in the current codebase.

```rust
pub struct HalsteadMetrics {
    pub distinct_operators: usize,    // n1
    pub distinct_operands: usize,     // n2
    pub total_operators: usize,       // N1
    pub total_operands: usize,        // N2
    pub vocabulary: usize,            // n
    pub length: usize,                // N
    pub volume: f64,                  // V
    pub difficulty: f64,              // D
    pub effort: f64,                  // E
    pub time_to_program: f64,         // T
    pub bugs_delivered: f64,          // B
}

impl HalsteadMetrics {
    pub fn calculate(operators: &HashSet<String>, operands: &HashSet<String>,
                     op_count: usize, operand_count: usize) -> Self {
        let n1 = operators.len();
        let n2 = operands.len();
        let n = n1 + n2;
        let N = op_count + operand_count;
        
        let volume = (N as f64) * (n as f64).log2();
        let difficulty = (n1 as f64 / 2.0) * (operand_count as f64 / n2 as f64);
        let effort = difficulty * volume;
        let time = effort / 18.0;
        let bugs = volume / 3000.0;
        
        HalsteadMetrics {
            distinct_operators: n1,
            distinct_operands: n2,
            total_operators: op_count,
            total_operands: operand_count,
            vocabulary: n,
            length: N,
            volume,
            difficulty,
            effort,
            time_to_program: time,
            bugs_delivered: bugs,
        }
    }
}
```

---

## Metrics Collection in the Pipeline

`MetricsPass` is registered in the analyzer registry and runs during `Phase::LocalRules`. It enumerates classes/structs/methods via `Query` and uses helpers from `metrics::shared` to compute statement counts, decision points (cyclomatic complexity), and nesting.

```rust
use analysis::context::AnalysisContext;
use analysis::framework::pipeline::AnalyzerPipeline;
use analysis::framework::session::AnalysisSession;
use analysis::metrics::AstAnalysis;
use parser::facade::Parser;

let source = r#"public class C { public void M() { if (true) { } } }"#;
let (cu, spans) = Parser::new().parse_with_spans(source)?;
let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", source), spans);
AnalyzerPipeline::run_with_defaults(&cu, &mut session);
let ast = session.artifacts.get::<AstAnalysis>().expect("AstAnalysis");
println!("classes={}, methods={}, ifs={}", ast.total_classes, ast.total_methods, ast.total_if_statements);
```

---

## CLI Usage

### Analyze Metrics

```bash
# Analyze single file
bsharp analyze MyFile.cs

# Analyze project
bsharp analyze MyProject.csproj --out metrics.json

# Analyze solution
bsharp analyze MySolution.sln --out metrics.json --format pretty-json
```

### Example Output

```json
{
  "schema_version": 1,
  "metrics": {
    "total_lines": 1250,
    "code_lines": 980,
    "comment_lines": 150,
    "blank_lines": 120,
    "class_count": 15,
    "method_count": 87,
    "total_complexity": 245,
    "max_complexity": 18,
    "max_nesting_depth": 5
  }
}
```

---

## Thresholds and Warnings

### Configuration

```toml
[analysis.metrics]
max_cyclomatic_complexity = 10
max_cognitive_complexity = 15
max_nesting_depth = 4
max_method_length = 50
min_maintainability_index = 65
```

### Diagnostics

**High Complexity Warning:**
```
warning[MET001]: Method has high cyclomatic complexity
  --> src/OrderProcessor.cs:42:17
   |
42 |     public void ProcessOrder(Order order) {
   |                 ^^^^^^^^^^^^ complexity = 18 (threshold: 10)
   |
   = help: Consider breaking this method into smaller methods
```

**Deep Nesting Warning:**
```
warning[MET002]: Deep nesting detected
  --> src/Validator.cs:15:9
   |
15 |         if (condition1) {
   |         ^^ nesting depth = 5 (threshold: 4)
   |
   = help: Consider extracting nested logic into separate methods
```

---

## Programmatic Usage

### Analyzing a Method

```rust
use bsharp::analysis::metrics::{cyclomatic_complexity, cognitive_complexity, max_nesting_depth};

let method = parse_method("public void MyMethod() { ... }");

let cc = cyclomatic_complexity(&method);
let cog = cognitive_complexity(&method);
let nesting = max_nesting_depth(&method);

println!("Cyclomatic Complexity: {}", cc);
println!("Cognitive Complexity: {}", cog);
println!("Max Nesting Depth: {}", nesting);
```

### Analyzing a file via the pipeline

```rust
let (cu, spans) = Parser::new().parse_with_spans(source_code)?;
let mut session = AnalysisSession::new(AnalysisContext::new("file.cs", source_code), spans);
AnalyzerPipeline::run_with_defaults(&cu, &mut session);
let metrics = session.artifacts.get::<AstAnalysis>().expect("AstAnalysis");
println!("Classes: {}", metrics.total_classes);
println!("Methods: {}", metrics.total_methods);
println!("Cyclomatic Complexity: {}", metrics.cyclomatic_complexity);
```

---

## Related Documentation

- [Analysis Pipeline](./pipeline.md) - How metrics fit in the pipeline
- [Control Flow Analysis](./control-flow.md) - Related complexity analysis
- [Code Quality](./quality.md) - Quality assessment using metrics
- [Architecture](../development/architecture.md) - Design decisions

---

## References

- **Implementation:** `src/analysis/metrics/`
- **Visitor:** `src/analysis/metrics/visitor.rs`
- **Tests:** `tests/analysis/metrics/` (planned)
- **Standards:** ISO/IEC 25023 (Software Quality Metrics)
