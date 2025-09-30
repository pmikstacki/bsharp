# Analysis Traversal Guide

This guide explains how to traverse BSharp AST statements and expressions in analysis passes.

- Source files:
  - `src/analysis/visitors.rs`
  - `src/analysis/passes/*`

## Statement traversal

Use `walk_statements()` to visit every `Statement` node in depth-first order.

```rust
use bsharp::analysis::visitors::walk_statements;
use bsharp::syntax::nodes::statements::statement::Statement;

fn count_returns(stmt: &Statement) -> usize {
    let mut count = 0;
    walk_statements(stmt, &mut |s| {
        if matches!(s, Statement::Return(_)) { count += 1; }
    });
    count
}
```

`walk_statements()` currently descends into:
- If (then and else), For, ForEach, While, DoWhile
- Using (optional body)
- Switch (all sections' statements)
- Try (try block, each catch block, finally block)
- Block (all nested statements)

Add additional variants as needed when the language surface expands.

## Expression traversal

Use `walk_expressions()` to visit every `Expression` node in depth-first order.

```rust
use bsharp::analysis::visitors::walk_expressions;
use bsharp::syntax::nodes::expressions::expression::Expression;

fn has_await(expr: &Expression) -> bool {
    let mut found = false;
    walk_expressions(expr, &mut |e| {
        if matches!(e, Expression::Await(_)) { found = true; }
    });
    found
}
```

`walk_expressions()` descends into common expression shapes:
- Composite: Tuple, AnonymousObject, Collection, New (arguments, initializer)
- Indexing / Index
- Conditional (condition, then, else)
- MemberAccess, NullConditional, Invocation
- Assignment (left, right)
- Unary/PostfixUnary, Binary
- Lambda (expression body) and switch-like constructs (SwitchExpression)
- Patterns in `is` and query expressions

Like statement traversal, extend the walker conservatively when new expression kinds are added.

## Putting it together

When analyzing methods, you typically:
- Parse the compilation unit and build the analysis session.
- For each method body (a `Statement::Block`), compute metrics by walking statements and expressions.

Example (from `ControlFlowPass` pattern):

```rust
use bsharp::analysis::artifacts::cfg::{ControlFlowIndex, MethodControlFlowStats};
use bsharp::syntax::nodes::statements::statement::Statement;

fn stats_for_method(body: Option<&Statement>) -> MethodControlFlowStats {
    let complexity = match body { Some(s) => 1 + decision_points(s), None => 1 };
    let max_nesting = calc_max_nesting(body, 0);
    let exit_points = count_exit_points(body);
    let statement_count = count_statements(body);
    MethodControlFlowStats { complexity, max_nesting, exit_points, statement_count }
}
```

See `src/analysis/passes/control_flow.rs` for concrete implementations of `decision_points`, `calc_max_nesting`, and exit/statement counters that leverage the structure of `Statement`.

## Tips

- Keep walkers side-effect free; accumulate results in closures.
- Prefer small, focused passes that use the walkers rather than embedding traversal in each pass.
- If a construct is not being traversed, add it to the walker first to avoid duplicated traversal logic.
