# Code Quality Analysis

The code quality analysis system detects code smells, anti-patterns, and violations of best practices in C# code.

---

## Overview

**Location:** `src/analysis/quality/`

Quality analysis provides:
- Code smell detection
- Best practice validation
- Design pattern recognition
- Maintainability assessment
- Technical debt identification

---

## Code Smells

### Method-Level Smells

#### Long Method

**Description:** Method with too many lines of code

**Threshold:** > 50 lines (configurable)

**Example:**
```csharp
public void ProcessOrder(Order order) {
    // 150 lines of code...
}
```

**Diagnostic:**
```
warning[QUAL001]: Long method detected
  --> src/OrderService.cs:42:17
   |
42 |     public void ProcessOrder(Order order) {
   |                 ^^^^^^^^^^^^ method has 150 lines (threshold: 50)
   |
   = help: Consider breaking this method into smaller, focused methods
```

**Refactoring:**
- Extract method
- Decompose into smaller methods
- Apply Single Responsibility Principle

#### Long Parameter List

**Description:** Method with too many parameters

**Threshold:** > 5 parameters (configurable)

**Example:**
```csharp
public void CreateUser(string firstName, string lastName, string email, 
                      string phone, string address, string city, string zip) {
    // ...
}
```

**Refactoring:**
- Introduce parameter object
- Use builder pattern
- Group related parameters into DTOs

#### Complex Conditional

**Description:** Deeply nested or complex conditional logic

**Example:**
```csharp
if (user != null && user.IsActive && (user.Role == "Admin" || user.Role == "Manager") 
    && user.Department != null && user.Department.Budget > 10000) {
    // ...
}
```

**Refactoring:**
- Extract condition to well-named method
- Use guard clauses
- Simplify boolean logic

### Class-Level Smells

#### Large Class (God Class)

**Description:** Class with too many responsibilities

**Indicators:**
- Too many methods (> 20)
- Too many fields (> 10)
- High cyclomatic complexity
- Low cohesion

**Example:**
```csharp
public class UserManager {
    // 50 methods handling user CRUD, authentication, authorization,
    // email sending, logging, caching, validation, etc.
}
```

**Refactoring:**
- Split into multiple classes
- Apply Single Responsibility Principle
- Extract related functionality

#### Feature Envy

**Description:** Method uses more features of another class than its own

**Example:**
```csharp
public class OrderProcessor {
    public decimal CalculateTotal(Order order) {
        decimal total = 0;
        foreach (var item in order.Items) {
            total += item.Price * item.Quantity;
        }
        total -= order.Discount;
        total += order.Tax;
        return total;
    }
}
```

**Refactoring:**
- Move method to `Order` class
- Method should be where the data is

#### Data Class

**Description:** Class with only fields and getters/setters, no behavior

**Example:**
```csharp
public class User {
    public string Name { get; set; }
    public string Email { get; set; }
    public int Age { get; set; }
    // No methods, just data
}
```

**Note:** Sometimes acceptable for DTOs, but domain objects should have behavior

### Code Organization Smells

#### Duplicate Code

**Description:** Identical or very similar code in multiple places

**Detection:**
- Token-based comparison
- AST structure comparison
- Minimum clone size threshold

**Refactoring:**
- Extract method
- Extract class
- Use inheritance or composition

#### Dead Code

**Description:** Code that is never executed

**Examples:**
- Unreachable statements after `return`
- Unused private methods
- Unused fields
- Conditions that are always true/false

**Diagnostic:**
```
warning[QUAL010]: Unreachable code detected
  --> src/Calculator.cs:15:9
   |
14 |     return result;
15 |     Console.WriteLine("Done");  // Never executed
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable statement
```

#### Magic Numbers

**Description:** Unexplained numeric literals in code

**Example:**
```csharp
if (order.Total > 1000) {  // What does 1000 mean?
    ApplyDiscount(order, 0.1);  // What does 0.1 mean?
}
```

**Refactoring:**
```csharp
const decimal BULK_ORDER_THRESHOLD = 1000m;
const decimal BULK_ORDER_DISCOUNT = 0.1m;

if (order.Total > BULK_ORDER_THRESHOLD) {
    ApplyDiscount(order, BULK_ORDER_DISCOUNT);
}
```

---

## Best Practices

### Naming Conventions

**Rules:**
- Classes: PascalCase
- Methods: PascalCase
- Properties: PascalCase
- Fields: camelCase with `_` prefix for private
- Constants: UPPER_CASE or PascalCase
- Interfaces: PascalCase with `I` prefix

**Violations:**
```
warning[QUAL020]: Naming convention violation
  --> src/UserService.cs:5:17
   |
 5 |     private int UserCount;
   |                 ^^^^^^^^^ private field should use camelCase with _ prefix
   |
   = help: Rename to '_userCount'
```

### Exception Handling

**Anti-patterns:**

**Empty Catch Block:**
```csharp
try {
    RiskyOperation();
} catch (Exception) {
    // Silent failure - BAD!
}
```

**Catching Generic Exception:**
```csharp
try {
    SpecificOperation();
} catch (Exception ex) {  // Too broad
    // ...
}
```

**Best Practices:**
- Catch specific exceptions
- Log exceptions
- Don't swallow exceptions
- Use `finally` for cleanup

### Resource Management

**Using Statement:**
```csharp
// Good
using (var file = File.OpenRead("data.txt")) {
    // Use file
}

// Better (C# 8+)
using var file = File.OpenRead("data.txt");
// Disposed at end of scope
```

**Diagnostic:**
```
warning[QUAL030]: IDisposable not properly disposed
  --> src/FileProcessor.cs:10:9
   |
10 |     var file = File.OpenRead("data.txt");
   |         ^^^^ should be wrapped in using statement
```

---

## Design Patterns and Anti-Patterns

### Detected Patterns

#### Singleton Pattern

**Detection:**
- Private constructor
- Static instance field
- Public static accessor

**Example:**
```csharp
public class Logger {
    private static Logger _instance;
    private Logger() { }
    
    public static Logger Instance {
        get {
            if (_instance == null) {
                _instance = new Logger();
            }
            return _instance;
        }
    }
}
```

#### Factory Pattern

**Detection:**
- Method returning interface or base class
- Creates different concrete types based on parameters

### Anti-Patterns

#### God Object

**Detection:**
- High number of methods and fields
- Low cohesion
- High coupling

#### Spaghetti Code

**Detection:**
- High cyclomatic complexity
- Deep nesting
- Lack of structure

#### Lava Flow

**Detection:**
- Dead code
- Commented-out code
- Unused variables/methods

---

## Quality Metrics

### Code Quality Score

**Composite Score (0-100):**
```rust
pub struct QualityScore {
    pub overall: f64,
    pub maintainability: f64,
    pub complexity: f64,
    pub duplication: f64,
    pub test_coverage: f64,
}
```

**Calculation:**
```
Overall = (Maintainability * 0.3) + 
          (Complexity * 0.3) + 
          (Duplication * 0.2) + 
          (TestCoverage * 0.2)
```

### Technical Debt

**Estimation:**
```rust
pub struct TechnicalDebt {
    pub total_issues: usize,
    pub estimated_hours: f64,
    pub debt_ratio: f64,  // debt / total development time
}
```

**Calculation:**
- Each code smell assigned time cost
- Sum all issues
- Compare to total codebase size

---

## Quality Rules

### Rule System

**Rule Definition:**
```rust
pub trait QualityRule {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn check(&self, node: &NodeRef, session: &mut AnalysisSession);
}
```

**Example Rule:**
```rust
pub struct LongMethodRule {
    max_lines: usize,
}

impl QualityRule for LongMethodRule {
    fn id(&self) -> &'static str { "long_method" }
    fn name(&self) -> &'static str { "Long Method" }
    
    fn check(&self, node: &NodeRef, session: &mut AnalysisSession) {
        if let NodeRef::MethodDeclaration(method) = node {
            let line_count = count_lines(method);
            if line_count > self.max_lines {
                session.diagnostics.add(
                    DiagnosticCode::LongMethod,
                    format!("Method has {} lines (threshold: {})", 
                           line_count, self.max_lines)
                );
            }
        }
    }
}
```

### Rule Categories

**Maintainability Rules:**
- Long method
- Long parameter list
- Large class
- Complex method

**Reliability Rules:**
- Empty catch blocks
- Null reference risks
- Resource leaks
- Unhandled exceptions

**Security Rules:**
- SQL injection risks
- XSS vulnerabilities
- Hardcoded credentials
- Insecure random

**Performance Rules:**
- Inefficient loops
- Unnecessary allocations
- String concatenation in loops
- Boxing/unboxing

---

## Configuration

### Quality Thresholds

```toml
[analysis.quality]
max_method_lines = 50
max_parameters = 5
max_class_methods = 20
max_cyclomatic_complexity = 10
max_nesting_depth = 4

[analysis.quality.rules]
long_method = "warning"
long_parameter_list = "warning"
god_class = "error"
empty_catch = "error"
magic_numbers = "info"
```

### Severity Levels

- **Error:** Must be fixed
- **Warning:** Should be fixed
- **Info:** Consider fixing
- **Hint:** Suggestion for improvement

---

## CLI Usage

### Quality Analysis

```bash
# Analyze code quality
bsharp analyze MyProject.csproj --enable-ruleset quality

# Generate quality report
bsharp analyze MySolution.sln --out quality-report.json

# Filter by severity
bsharp analyze MyFile.cs --severity error,warning
```

### Example Output

```json
{
  "quality_score": {
    "overall": 72.5,
    "maintainability": 68.0,
    "complexity": 75.0,
    "duplication": 80.0
  },
  "technical_debt": {
    "total_issues": 45,
    "estimated_hours": 12.5,
    "debt_ratio": 0.08
  },
  "diagnostics": [
    {
      "code": "QUAL001",
      "severity": "warning",
      "message": "Long method detected",
      "file": "src/OrderService.cs",
      "line": 42,
      "column": 17
    }
  ]
}
```

---

## Integration with Pipeline

### Quality Ruleset

**Registration:**
```rust
// In AnalyzerRegistry
registry.add_ruleset(QualityRuleset {
    id: "quality",
    rules: vec![
        Box::new(LongMethodRule::new()),
        Box::new(LongParameterListRule::new()),
        Box::new(GodClassRule::new()),
        Box::new(EmptyCatchRule::new()),
        // ... more rules
    ],
});
```

**Execution:**
- Rules run during Local or Semantic phase
- Visitor pattern for AST traversal
- Diagnostics collected in session

---

## Programmatic Usage

### Running Quality Analysis

```rust
use bsharp::analysis::quality::QualityAnalyzer;

let parser = Parser::new();
let cu = parser.parse(source_code)?;

let analyzer = QualityAnalyzer::new();
let report = analyzer.analyze(&cu);

println!("Quality Score: {}", report.quality_score.overall);
println!("Issues Found: {}", report.diagnostics.len());
```

### Custom Rules

```rust
use bsharp::analysis::quality::QualityRule;

struct CustomRule;

impl QualityRule for CustomRule {
    fn id(&self) -> &'static str { "custom_rule" }
    fn name(&self) -> &'static str { "Custom Rule" }
    
    fn check(&self, node: &NodeRef, session: &mut AnalysisSession) {
        // Custom logic
    }
}

// Register custom rule
analyzer.add_rule(Box::new(CustomRule));
```

---

## Future Enhancements

### Planned Features

1. **Machine Learning-Based Detection**
   - Learn from codebase patterns
   - Detect project-specific smells

2. **Refactoring Suggestions**
   - Automated refactoring proposals
   - Preview refactoring impact

3. **Quality Trends**
   - Track quality over time
   - Identify degradation
   - Measure improvement

4. **Team Metrics**
   - Per-developer quality metrics
   - Code review insights
   - Best practice adoption

---

## Related Documentation

- [Analysis Pipeline](./pipeline.md) - Pipeline integration
- [Metrics Collection](./metrics.md) - Quality metrics
- [Control Flow Analysis](./control-flow.md) - Complexity analysis
- [Architecture](../development/architecture.md) - Design decisions

---

## References

- **Implementation:** `src/analysis/quality/`
- **Rules:** `src/analysis/rules/`
- **Tests:** `tests/analysis/quality/` (planned)
- **Standards:** Clean Code (Robert C. Martin), Refactoring (Martin Fowler)
