# Control Flow Analysis

The control flow analysis system analyzes method control flow to calculate complexity metrics, detect control flow smells, and identify potential issues.

---

## Overview

**Location:** `src/analysis/passes/control_flow.rs`, `src/analysis/artifacts/cfg.rs`

Control flow analysis provides:
- Cyclomatic complexity calculation
- Maximum nesting depth tracking
- Exit point counting
- Statement counting
- Control flow smell detection

---

## Control Flow Metrics

### Cyclomatic Complexity

**Definition:** Number of linearly independent paths through a method

**Calculation:** `CC = 1 + number of decision points`

**Decision Points:**
- `if` statements
- `case` labels in `switch`
- Loop statements (`for`, `foreach`, `while`, `do-while`)
- `catch` clauses
- Logical operators (`&&`, `||`) in conditions
- Ternary operators (`?:`)
- Null-coalescing operators (`??`)

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
    }
}
// Total CC = 4
```

### Maximum Nesting Depth

**Definition:** Deepest level of nested control structures

**Example:**
```csharp
public void Example() {
    if (condition1) {              // Depth 1
        while (condition2) {       // Depth 2
            if (condition3) {      // Depth 3
                DoSomething();
            }
        }
    }
}
// Max Nesting Depth = 3
```

### Exit Points

**Definition:** Number of points where method can return

**Counted:**
- `return` statements
- `throw` statements
- End of void method

**Example:**
```csharp
public int Calculate(int x) {
    if (x < 0) {
        return -1;        // Exit point 1
    }
    if (x == 0) {
        return 0;         // Exit point 2
    }
    return x * 2;         // Exit point 3
}
// Total Exit Points = 3
```

### Statement Count

**Definition:** Total number of statements in method body

Includes all statement types:
- Expression statements
- Declaration statements
- Control flow statements
- Jump statements

---

## Control Flow Artifacts

### MethodControlFlowStats

```rust
pub struct MethodControlFlowStats {
    pub complexity: usize,
    pub max_nesting: usize,
    pub exit_points: usize,
    pub statement_count: usize,
}
```

### ControlFlowIndex

```rust
pub struct ControlFlowIndex {
    // Method identifier -> stats
    methods: HashMap<String, MethodControlFlowStats>,
}
```

### CfgSummary

```rust
pub struct CfgSummary {
    pub total_methods: usize,
    pub high_complexity_count: usize,
    pub deep_nesting_count: usize,
}
```

---

## Control Flow Smells

### High Complexity

**Threshold:** Configurable (default: 10)

**Detection:**
```rust
if stats.complexity > config.cf_high_complexity_threshold {
    session.diagnostics.add(
        DiagnosticCode::HighComplexity,
        format!("Method complexity {} exceeds threshold {}", 
               stats.complexity, threshold)
    );
}
```

**Diagnostic:**
```
warning[CF002]: High cyclomatic complexity
  --> src/OrderProcessor.cs:42:17
   |
42 |     public void ProcessOrder(Order order) {
   |                 ^^^^^^^^^^^^ complexity = 15 (threshold: 10)
   |
   = help: Consider breaking this method into smaller methods
```

### Deep Nesting

**Threshold:** Configurable (default: 4)

**Detection:**
```rust
if stats.max_nesting > config.cf_deep_nesting_threshold {
    session.diagnostics.add(
        DiagnosticCode::DeepNesting,
        format!("Maximum nesting depth {} exceeds threshold {}", 
               stats.max_nesting, threshold)
    );
}
```

**Diagnostic:**
```
warning[CF003]: Deep nesting detected
  --> src/Validator.cs:15:9
   |
15 |         if (condition1) {
   |         ^^ nesting depth = 5 (threshold: 4)
   |
   = help: Consider extracting nested logic into separate methods
```

---

## Implementation

### Analysis Pass

**Location:** `src/analysis/passes/control_flow.rs`

```rust
pub struct ControlFlowPass;

impl AnalyzerPass for ControlFlowPass {
    fn id(&self) -> &'static str { "control_flow" }
    fn phase(&self) -> Phase { Phase::Semantic }
    
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let mut index = ControlFlowIndex::new();
        
        // Analyze all methods in compilation unit
        for decl in &cu.declarations {
            analyze_declaration(decl, &mut index, session);
        }
        
        session.artifacts.cfg = Some(index);
    }
}
```

### Method Analysis

```rust
fn analyze_method(
    method: &MethodDeclaration,
    index: &mut ControlFlowIndex,
    session: &mut AnalysisSession
) {
    let stats = calculate_stats(method.body.as_ref());
    
    // Check thresholds
    if stats.complexity > session.config.cf_high_complexity_threshold {
        session.diagnostics.add(/* high complexity diagnostic */);
    }
    
    if stats.max_nesting > session.config.cf_deep_nesting_threshold {
        session.diagnostics.add(/* deep nesting diagnostic */);
    }
    
    // Store in index
    index.add_method(&method.identifier.name, stats);
}
```

### Stats Calculation

```rust
fn calculate_stats(body: Option<&Statement>) -> MethodControlFlowStats {
    let complexity = match body {
        Some(stmt) => 1 + count_decision_points(stmt),
        None => 1,
    };
    
    let max_nesting = calculate_max_nesting(body, 0);
    let exit_points = count_exit_points(body);
    let statement_count = count_statements(body);
    
    MethodControlFlowStats {
        complexity,
        max_nesting,
        exit_points,
        statement_count,
    }
}
```

---

## Configuration

### Thresholds

```toml
[analysis.control_flow]
cf_high_complexity_threshold = 10
cf_deep_nesting_threshold = 4
```

### CLI Usage

```bash
# Analyze with custom thresholds
bsharp analyze MyProject.csproj --config .bsharp.toml

# Enable control flow pass
bsharp analyze MyProject.csproj --enable-pass control_flow
```

---

## Integration with Pipeline

### Phase: Semantic

Control flow analysis runs in the **Semantic** phase after symbol indexing:

```
Phase::Index    -> Build SymbolIndex
Phase::Local    -> Collect metrics
Phase::Semantic -> Control flow analysis
```

### Artifacts

Results stored in `AnalysisSession`:
```rust
session.artifacts.cfg = Some(ControlFlowIndex { ... });
```

Summarized in `AnalysisReport`:
```rust
report.cfg = Some(CfgSummary {
    total_methods: 87,
    high_complexity_count: 5,
    deep_nesting_count: 3,
});
```

---

## Related Documentation

- [Analysis Pipeline](./pipeline.md) - Pipeline integration
- [Metrics Collection](./metrics.md) - Related metrics
- [Code Quality](./quality.md) - Quality rules
- [Traversal Guide](./traversal-guide.md) - AST traversal

---

## References

- **Implementation:** `src/analysis/passes/control_flow.rs`
- **Artifacts:** `src/analysis/artifacts/cfg.rs`
- **Tests:** `tests/analysis/types/basic_tests.rs`
