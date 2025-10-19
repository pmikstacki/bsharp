# Testing Guide

This document provides comprehensive guidance on testing in the BSharp project, covering test organization, best practices, and debugging strategies.

---

## Test Organization Philosophy

### External Test Structure

**Critical Principle:** All parser tests are located in the `tests/` directory, **NOT** inline within parser modules.

```
tests/
├── lib.rs                    # Test crate root
├── parser/
│   ├── mod.rs               # Parser test module
│   ├── expressions/
│   │   ├── expression_tests.rs
│   │   ├── lambda_expression_tests.rs
│   │   ├── pattern_matching_tests.rs
│   │   ├── ambiguity_tests.rs
│   │   ├── lookahead_boundaries2_tests.rs
│   │   └── ...
│   ├── statements/
│   │   ├── if_statement_tests.rs
│   │   ├── for_statement_tests.rs
│   │   ├── expression_statement_tests.rs
│   │   └── ...
│   ├── declarations/
│   │   ├── class_declaration_tests.rs
│   │   ├── interface_declaration_parser_tests.rs
│   │   ├── recovery_tests.rs
│   │   └── ...
│   ├── types/
│   │   ├── type_tests.rs
│   │   ├── advanced_type_tests.rs
│   │   └── ...
│   ├── preprocessor/
│   │   └── ...
│   └── keyword_parsers_tests.rs
└── fixtures/
    ├── happy_path/
    │   ├── testApplication/
    │   └── testDependency/
    └── complex/
        ├── testApplication/
        └── testDependency/
```

**Rationale:**
- **Separation of Concerns**: Test code separate from implementation
- **Compilation Efficiency**: Tests don't bloat production binary
- **Organization**: Clear structure mirrors parser organization
- **Maintainability**: Easy to find and update tests

**What NOT to Do:**
```rust
// ❌ NEVER do this in src/parser/ files
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // ...
    }
}
```

**What to Do Instead:**
```rust
// ✅ Create tests/parser/expressions/my_feature_tests.rs
use bsharp::syntax::test_helpers::expect_ok;
use bsharp::parser::expressions::parse_my_feature;

#[test]
fn test_my_feature() {
    let input = "my feature syntax";
    let result = parse_my_feature(input.into());
    let ast = expect_ok(input, result);
    // assertions...
}
```

---

## Test Helpers

### `expect_ok()` - Readable Test Failures

**Location:** `src/syntax/test_helpers.rs`

**Usage:**
```rust
use bsharp::syntax::test_helpers::expect_ok;

#[test]
fn test_parse_class() {
    let input = "public class MyClass { }";
    let result = parse_class_declaration(input.into());
    let class = expect_ok(input, result);
    
    assert_eq!(class.identifier.name, "MyClass");
}
```

**Benefits:**
- **Automatic Error Formatting**: Pretty-prints `ErrorTree` on failure
- **Readable Diagnostics**: Shows parse failure context with caret
- **Panic on Failure**: Test fails with clear error message

**Error Output Example:**
```
0: at line 1, in keyword "class":
public clas MyClass { }
       ^--- expected keyword "class"

1: in context "class declaration"
```

### Other Test Helpers

**`parse_input_unwrap()`** - Unwrap parse result:
```rust
let (remaining, ast) = parse_input_unwrap(parse_expression(input.into()));
assert_eq!(remaining, "");  // Verify full consumption
```

**`assert_parse_error()`** - Verify parse failures:
```rust
assert_parse_error(parse_expression("invalid syntax"));
```

---

## Parser Testing Best Practices

### 1. Prefer `expect_ok()` for Successful Parses

```rust
#[test]
fn test_if_statement() {
    let input = "if (x > 0) { return x; }";
    let stmt = expect_ok(input, parse_if_statement(input.into()));
    
    // Now assert on the AST structure
    match stmt {
        Statement::If(if_stmt) => {
            // Verify condition, consequence, etc.
        }
        _ => panic!("Expected IfStatement"),
    }
}
```

### 2. Keep Tests Focused and Minimal

**Good:**
```rust
#[test]
fn test_simple_lambda() {
    let input = "x => x * 2";
    let expr = expect_ok(input, parse_lambda_expression(input.into()));
    // Test one thing
}

#[test]
fn test_lambda_with_multiple_params() {
    let input = "(x, y) => x + y";
    let expr = expect_ok(input, parse_lambda_expression(input.into()));
    // Test another thing
}
```

**Bad:**
```rust
#[test]
fn test_all_lambda_forms() {
    // Testing too many things in one test
    // Hard to debug when it fails
}
```

### 3. Add Negative Tests for Ambiguity

When disambiguation is possible, add tests for both valid and invalid cases:

```rust
#[test]
fn test_ternary_vs_nullable() {
    // Valid ternary
    let input = "x ? y : z";
    expect_ok(input, parse_conditional_expression(input.into()));
    
    // Valid null-conditional (different test)
}

#[test]
fn test_null_conditional_operator() {
    let input = "obj?.Property";
    expect_ok(input, parse_postfix_expression(input.into()));
}
```

### 4. Test Lookahead/Disambiguation Boundaries

**Location:** `tests/parser/expressions/lookahead_boundaries2_tests.rs`

```rust
#[test]
fn test_range_vs_dot_vs_float() {
    // Range operator
    expect_ok("1..10", parse_range_expression("1..10"));
    
    // Member access
    expect_ok("obj.Method", parse_postfix_expression("obj.Method"));
    
    // Float literal
    expect_ok("3.14", parse_literal("3.14"));
}
```

### 5. Test Complex Constructs

For complex constructs like `new` expressions with initializers:

**Location:** `tests/parser/expressions/new_expression_tests.rs`

```rust
#[test]
fn test_new_with_object_initializer() {
    let input = "new Person { Name = \"John\", Age = 30 }";
    let expr = expect_ok(input, parse_new_expression(input.into()));
    // Verify structure
}

#[test]
fn test_new_with_collection_initializer() {
    let input = "new List<int> { 1, 2, 3 }";
    let expr = expect_ok(input, parse_new_expression(input.into()));
    // Verify structure
}

#[test]
fn test_target_typed_new() {
    let input = "new(42, \"test\")";
    let expr = expect_ok(input, parse_new_expression(input.into()));
    // Verify structure
}
```

### 6. Test Invalid Input Diagnostics

**Location:** `tests/parser/expressions/invalid_diagnostics_tests.rs`

```rust
#[test]
fn test_unclosed_paren_diagnostic() {
    let input = "(x + y";
    let result = parse_expression(input.into());
    
    assert!(result.is_err());
    // Optionally check error contains expected message
}
```

**Guidelines:**
- Keep small snapshot-style assertions
- Check for line/column and caret presence
- Avoid overfitting on exact wording (may change)

### 7. Guard Closing Delimiters with `cut()`

When adding delimited constructs, ensure closing delimiters use `cut()`:

```rust
use nom::combinator::cut;
use crate::syntax::parser_helpers::{bdelimited, bchar};

fn parse_parenthesized(input: &str) -> BResult<&str, Expression> {
    bdelimited(
        bchar('('),
        parse_expression,
        cut(bchar(')'))  // ✅ Prevents misleading backtracking
    )(input.into())
}
```

### 8. Wrap Sub-Parsers with `bws()`

Ensure whitespace/comments are handled consistently:

```rust
use crate::syntax::parser_helpers::bws;

fn parse_if_statement(input: &str) -> BResult<&str, Statement> {
    let (input, _) = bws(keyword("if"))(input.into())?;
    let (input, _) = bws(bchar('('))(input.into())?;
    let (input, condition) = bws(parse_expression)(input.into())?;
    // ...
}
```

---

## Test Discovery and Execution

### Running All Tests

```bash
cargo test
```

### Running Specific Test Suites

```bash
# All parser tests
cargo test --test parser

# Specific module
cargo test --test parser expression_tests

# Specific test
cargo test --test parser test_lambda_expression
```

### Running with Output

```bash
# Show println! output
cargo test -- --nocapture

# Show test names as they run
cargo test -- --nocapture --test-threads=1
```

### Running with Debug Logging

```bash
RUST_LOG=debug cargo test test_name -- --nocapture
```

---

## Test Fixtures

### Fixture Organization

```
tests/fixtures/
├── happy_path/           # Valid, well-formed C# projects
│   ├── testApplication/
│   │   ├── Program.cs
│   │   ├── testApplication.csproj
│   │   └── ...
│   └── testDependency/
│       └── ...
└── complex/              # Complex, real-world scenarios
    ├── testApplication/
    └── testDependency/
```

### Using Fixtures in Tests

```rust
use std::fs;
use std::path::PathBuf;

#[test]
fn test_parse_fixture() {
    let fixture_path = PathBuf::from("tests/fixtures/happy_path/testApplication/Program.cs");
    let source = fs::read_to_string(&fixture_path).unwrap();
    
    let parser = Parser::new();
    let result = parser.parse(&source);
    
    assert!(result.is_ok());
}
```

### Fixture Guidelines

- **Valid Code**: Fixtures should be valid C# that compiles
- **Realistic**: Use real-world patterns, not contrived examples
- **Documented**: Add README.md explaining fixture purpose
- **Minimal**: Keep fixtures as small as possible while testing feature

---

## Snapshot Testing

### Using `insta` for Snapshot Tests

**Installation:** Already included in `Cargo.toml` dev-dependencies

```rust
use insta::assert_json_snapshot;

#[test]
fn test_class_ast_structure() {
    let input = "public class MyClass { public int Field; }";
    let result = parse_class_declaration(input.into());
    let class = expect_ok(input, result);
    
    // Creates snapshot file on first run
    assert_json_snapshot!(class);
}
```

### Reviewing Snapshots

```bash
# Review snapshot changes
cargo insta review

# Accept all changes
cargo insta accept

# Reject all changes
cargo insta reject
```

### Snapshot Guidelines

- **Complex Structures**: Use for complex AST structures
- **Regression Prevention**: Catch unintended changes
- **Review Carefully**: Always review snapshot diffs
- **Commit Snapshots**: Include snapshot files in git

---

## Debugging Test Failures

### Strategy 1: Use `expect_ok()` Error Output

When a test fails, `expect_ok()` shows the parse error:

```
0: at line 1, in keyword "class":
public clas MyClass { }
       ^--- expected keyword "class"
```

### Strategy 2: Add Debug Logging

```rust
#[test]
fn test_with_logging() {
    env_logger::init();  // Initialize logger
    
    let input = "complex syntax";
    log::debug!("Parsing: {}", input);
    
    let result = parse_expression(input.into());
    log::debug!("Result: {:?}", result);
    
    expect_ok(input, result);
}
```

Run with:
```bash
RUST_LOG=debug cargo test test_with_logging -- --nocapture
```

### Strategy 3: Test Smaller Components

If a complex parser fails, test its sub-parsers individually:

```rust
#[test]
fn test_method_declaration() {
    // Fails - too complex
    let input = "public async Task<int> Method(int x) { return x; }";
    expect_ok(input, parse_method_declaration(input.into()));
}

// Break it down:

#[test]
fn test_method_modifiers() {
    let input = "public async";
    expect_ok(input, parse_modifiers(input.into()));
}

#[test]
fn test_method_return_type() {
    let input = "Task<int>";
    expect_ok(input, parse_type(input.into()));
}

#[test]
fn test_method_parameters() {
    let input = "(int x)";
    expect_ok(input, parse_parameter_list(input.into()));
}
```

### Strategy 4: Use Parser Debugging Tools

```bash
# Parse file and output JSON
cargo run -- parse debug_cases/test.cs --output debug.json

# Generate AST visualization
cargo run -- tree debug_cases/test.cs --output debug.svg
```

### Strategy 5: Check Error Recovery

For declaration error recovery tests:

```rust
#[test]
fn test_recovery_from_malformed_member() {
    let input = r#"
    public class MyClass {
        public int ValidField;
        public invalid syntax here;  // Malformed
        public int AnotherValidField;  // Should recover
    }
    "#;
    
    let result = parse_class_declaration(input.into());
    // Should parse despite error
    assert!(result.is_ok());
}
```

---

## Integration Testing

### Workspace Loading Tests

```rust
use bsharp::workspace::WorkspaceLoader;

#[test]
fn test_load_solution() {
    let sln_path = PathBuf::from("tests/fixtures/happy_path/test.sln");
    let workspace = WorkspaceLoader::from_path(&sln_path).unwrap();
    
    assert_eq!(workspace.projects.len(), 2);
    assert!(workspace.solution.is_some());
}

#[test]
fn test_load_csproj() {
    let csproj_path = PathBuf::from("tests/fixtures/happy_path/testApplication/testApplication.csproj");
    let workspace = WorkspaceLoader::from_path(&csproj_path).unwrap();
    
    assert_eq!(workspace.projects.len(), 1);
}
```

### Analysis Pipeline Tests

```rust
use bsharp::analysis::framework::pipeline::AnalyzerPipeline;
use bsharp::analysis::framework::session::AnalysisSession;

#[test]
fn test_analysis_pipeline() {
    let source = "public class Test { public void Method() { } }";
    let parser = Parser::new();
    let cu = parser.parse(source).unwrap();
    
    let mut session = AnalysisSession::new();
    AnalyzerPipeline::run_with_defaults(&cu, &mut session);
    
    let report = session.into_report();
    assert!(report.diagnostics.is_empty());  // No errors
}
```

---

## Performance Testing

### Benchmarking

```rust
#[test]
#[ignore]  // Run with --ignored flag
fn bench_parse_large_file() {
    use std::time::Instant;
    
    let source = fs::read_to_string("tests/fixtures/large_file.cs").unwrap();
    let parser = Parser::new();
    
    let start = Instant::now();
    let result = parser.parse(&source);
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    println!("Parse time: {:?}", duration);
    
    // Assert reasonable performance
    assert!(duration.as_millis() < 1000, "Parse took too long");
}
```

### Running Performance Tests

```bash
cargo test --ignored -- bench_
```

---

## Continuous Integration

### CI Test Strategy

```yaml
# .github/workflows/test.yml (example)
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all-features
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Check formatting
        run: cargo fmt -- --check
```

---

## Test Coverage

### Measuring Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage
```

### Coverage Goals

- **Parser Core**: 90%+ coverage
- **Analysis Framework**: 80%+ coverage
- **CLI Commands**: 70%+ coverage
- **Workspace Loading**: 80%+ coverage

---

## Common Testing Patterns

### Pattern 1: Positive and Negative Tests

```rust
#[test]
fn test_valid_syntax() {
    let input = "valid syntax";
    expect_ok(input, parse_feature(input.into()));
}

#[test]
fn test_invalid_syntax() {
    let input = "invalid syntax";
    assert!(parse_feature(input.into()).is_err());
}
```

### Pattern 2: Boundary Testing

```rust
#[test]
fn test_empty_input() {
    assert!(parse_feature("").is_err());
}

#[test]
fn test_minimal_input() {
    expect_ok("x", parse_feature("x"));
}

#[test]
fn test_maximal_input() {
    let input = "very complex nested structure...";
    expect_ok(input, parse_feature(input.into()));
}
```

### Pattern 3: Equivalence Testing

```rust
#[test]
fn test_whitespace_insensitive() {
    let compact = "if(x){y;}";
    let spaced = "if (x) { y; }";
    
    let ast1 = expect_ok(compact, parse_if_statement(compact));
    let ast2 = expect_ok(spaced, parse_if_statement(spaced));
    
    assert_eq!(ast1, ast2);
}
```

---

## Test Maintenance

### When to Update Tests

1. **API Changes**: Update tests when parser API changes
2. **Bug Fixes**: Add regression tests for fixed bugs
3. **New Features**: Add tests for new language features
4. **Refactoring**: Ensure tests still pass after refactoring

### Test Cleanup

- **Remove Duplicate Tests**: Consolidate similar tests
- **Update Outdated Tests**: Fix tests using deprecated APIs
- **Remove Dead Tests**: Delete tests for removed features
- **Improve Names**: Use descriptive test names

### Test Documentation

```rust
/// Tests that lambda expressions with multiple parameters are parsed correctly.
/// 
/// This test verifies:
/// - Parameter list parsing
/// - Arrow token recognition
/// - Expression body parsing
#[test]
fn test_lambda_with_multiple_params() {
    let input = "(x, y) => x + y";
    let expr = expect_ok(input, parse_lambda_expression(input.into()));
    // ...
}
```

---

## Summary

### Testing Checklist

- [ ] Tests in `tests/` directory, not inline
- [ ] Use `expect_ok()` for readable failures
- [ ] Keep tests focused and minimal
- [ ] Add negative tests for ambiguity
- [ ] Test lookahead/disambiguation boundaries
- [ ] Test complex constructs thoroughly
- [ ] Use `cut()` for closing delimiters
- [ ] Wrap sub-parsers with `bws()`
- [ ] Add fixtures for integration tests
- [ ] Use snapshot tests for complex structures
- [ ] Document test purpose and coverage

### Resources

- **Test Helpers**: `src/syntax/test_helpers.rs`
- **Example Tests**: `tests/parser/expressions/`
- **Fixtures**: `tests/fixtures/`
- **Contributing Guide**: `docs/development/contributing.md`
- **Architecture**: `docs/development/architecture.md`
