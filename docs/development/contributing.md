
# Contributing to BSharp

Thank you for your interest in contributing to BSharp! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- A text editor or IDE with Rust support

### Building the Project

1. Clone the repository:
```bash
git clone https://github.com/your-repo/bsharp.git
cd bsharp
```

#### Parser Testing Best Practices

- Prefer `expect_ok(input, parse(input))` from `syntax::test_helpers` when asserting successful parses. It prints readable, rustc-like diagnostics on failure via `format_error_tree`.
- Keep tests focused and minimal; add a separate negative test when ambiguity is possible (e.g., ternary vs `?.` vs `??`, range vs dot vs float).
- For lookahead/disambiguation boundaries, add cases to `tests/parser/expressions/lookahead_boundaries2_tests.rs`.
- For complex constructs (e.g., `new` with object/collection initializers), add positive and negative cases near `tests/parser/expressions/new_expression_tests.rs` and `target_typed_new_tests.rs`.
- Invalid-input diagnostics: place small snapshot-style assertions in `tests/parser/expressions/invalid_diagnostics_tests.rs` that check for line/column and caret presence. Avoid overfitting on exact wording.
- When adding delimited constructs (parentheses, brackets, braces), guard the closing delimiter with `cut(...)` once committed to that branch to prevent misleading backtracking.
- Always wrap sub-parsers with `bws(...)` to ensure whitespace/comments are handled consistently.

#### Adding New Parser Test Files

- In `tests/parser/expressions/`, simply add a new `*_tests.rs` file; it will be discovered by the existing integration test harness.
- For declarations/statements/types, follow the existing directory structure under `tests/parser/` and mimic module organization.
- Keep tests deterministic and avoid relying on environment-specific paths or random data.

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Run the CLI tool:
```bash
cargo run -- --help
```

## Project Structure

Understanding the codebase organization:

```
src/
├── parser/           # Parser implementations (expressions, statements, etc.)
├── syntax/           # Parser infrastructure (AST nodes, helpers, errors)
├── analysis/         # Code analysis framework
├── workspace/        # Solution and project file loading
├── cli/              # Command-line interface
└── lib.rs           # Library entry point
```

### Code Style

Follow Rust conventions:

- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common issues
- Follow naming conventions (`snake_case` for functions, `PascalCase` for types)
- Add documentation comments for public APIs

### Testing

All contributions should include appropriate tests:

#### Parser Tests

**IMPORTANT:** All tests must be in external files under `tests/` directory, NOT inline `#[cfg(test)]` modules.

```rust
// ✅ CORRECT: External test file
// tests/parser/declarations/class_declaration_tests.rs

use bsharp::syntax::test_helpers::expect_ok;
use bsharp::parser::expressions::declarations::parse_class_declaration;

#[test]
fn test_parse_simple_class() {
    let input = "public class MyClass { }";
    let class = expect_ok(input, parse_class_declaration(input));
    assert_eq!(class.identifier.name, "MyClass");
}
```

#### Analysis Tests

```rust
// tests/analysis/complexity_tests.rs

use bsharp::syntax::Parser;
use bsharp::analysis::metrics::cyclomatic_complexity;

#[test]
fn test_complexity_analysis() {
    let source = r#"
        public class Test {
            public void Method() {
                if (true) {
                    for (int i = 0; i < 10; i++) {
                        // complexity += 2
                    }
                }
            }
        }
    "#;
    
    let parser = Parser::new();
    let cu = parser.parse(source).unwrap();
    
    // Find the method and calculate complexity
    // (implementation details depend on analysis API)
    
    assert_eq!(complexity, 3);
}
```

### Documentation

- Add rustdoc comments for public functions and types
- Update this documentation when adding new features
- Include examples in documentation

### Adding New Language Features

When adding support for new C# language features:

1. **Define AST Nodes**: Add node definitions in `src/syntax/nodes/`
2. **Implement Parser**: Add parser in appropriate `src/parser/` subdirectory
3. **Add Tests**: Include comprehensive tests in `tests/parser/` directory
4. **Update Traversal**: Prefer the `framework::query::Query` API for AST enumeration; for statement/expression-heavy logic, use shared helpers or a focused walker.
5. **Document**: Add documentation for the new feature

Example process for adding a new expression type:

1. Define the AST node:
```rust
// src/syntax/nodes/expressions/new_expression.rs
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    pub keyword: String,  // "new"
    pub arguments: Vec<Expression>,
}
```

2. Add to Expression enum:
```rust
// src/syntax/nodes/expressions/expression.rs
pub enum Expression {
    // ... existing variants
    New(NewExpression),
}
```

3. Implement parser:
```rust
// src/parser/expressions/new_expression_parser.rs
pub fn parse_new_expression(input: &str) -> BResult<&str, NewExpression> {
    // Parser implementation
}
```

4. Add tests:
```rust
// tests/parser/expressions/new_expression_tests.rs
#[test]
fn test_parse_new_expression() {
    // Test implementation
}
```

## Submitting Changes

### Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run formatting: `cargo fmt`
6. Run clippy: `cargo clippy`
7. Commit changes with clear messages
8. Push to your fork
9. Create a pull request

### Commit Messages

Use clear, descriptive commit messages:

```
feat: add support for C# 11 file-scoped types

- Add parser for file-scoped type declarations
- Update AST to handle new syntax
- Add comprehensive tests
- Update documentation

Fixes #123
```

### Pull Request Requirements

- All tests must pass
- Code must be formatted with `cargo fmt`
- No clippy warnings
- Include tests for new functionality
- Update documentation if needed

## Common Development Tasks

### Adding a New Parser

1. Define the AST node structure
2. Implement the parser function
3. Add the parser to the appropriate module
4. Write comprehensive tests
5. Update integration points

### Extending Analysis

1. Define analysis traits if needed
2. Implement analyzer struct
3. Add configuration options
4. Write tests with various scenarios
5. Update CLI integration

### Debugging Parser Issues

Use these tools for debugging:

```bash
# Test specific parser with debug output
RUST_LOG=debug cargo test test_name -- --nocapture

# Run parser on test file
cargo run -- parse debug_cases/test.cs --output debug.json

# Check parse tree structure
cargo run -- tree debug_cases/test.cs --output debug.svg
```

## Getting Help

- Check existing issues and documentation
- Ask questions in GitHub issues
- Join community discussions

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Maintain a positive environment

Thank you for contributing to BSharp!
