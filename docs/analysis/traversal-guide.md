# Analysis Traversal Guide

This guide explains how to traverse BSharp AST statements and expressions in analysis passes using the current framework.

- Source files:
  - `src/bsharp_analysis/src/framework/walker.rs`
  - `src/bsharp_analysis/src/framework/query/`
  - `src/bsharp_analysis/src/passes/*`

## Statement traversal

Use `AstWalker` for single-pass traversal with the `Visit` trait, or the `Query` API for typed filtering.

Example using `AstWalker` + `Visit` to count `if` statements:

```rust
use bsharp_analysis::framework::{AstWalker, Visit, NodeRef};
use bsharp_analysis::framework::AnalysisSession;

struct CountIfs { pub ifs: usize }
impl Visit for CountIfs {
    fn enter(&mut self, node: &NodeRef, _session: &mut AnalysisSession) {
        if let NodeRef::Statement(s) = node {
            if matches!(s, bsharp_analysis::syntax::statements::statement::Statement::If(_)) {
                self.ifs += 1;
            }
        }
    }
}
```

## Expression traversal

Use `Query` for typed expression searches:

```rust
use bsharp_analysis::framework::Query;
use bsharp_analysis::framework::NodeRef;

let await_count = Query::from(NodeRef::CompilationUnit(&cu))
    .of::<bsharp_analysis::syntax::expressions::await_expression::AwaitExpression>()
    .count();
```

## Putting it together

When analyzing methods, you typically:
- Parse the compilation unit and build the analysis session.
- For each method body (a `Statement::Block`), compute metrics by walking statements and expressions.

Example (from `ControlFlowPass` pattern):

```rust
use bsharp::analysis::artifacts::cfg::{ControlFlowIndex, MethodControlFlowStats};
use bsharp::syntax::statements::statement::Statement;

fn stats_for_method(body: Option<&Statement>) -> MethodControlFlowStats {
    let complexity = match body { Some(s) => 1 + decision_points(s), None => 1 };
    let max_nesting = calc_max_nesting(body, 0);
    let exit_points = count_exit_points(body);
    let statement_count = count_statements(body);
    MethodControlFlowStats { complexity, max_nesting, exit_points, statement_count }
}
```

See `src/bsharp_analysis/src/metrics/shared.rs` for helpers like `decision_points`, `max_nesting_of`, `count_statements` and `src/bsharp_analysis/src/passes/control_flow.rs` for usage.

## Tips

- Keep walkers side-effect free; accumulate results in closures.
- Prefer small, focused passes that use the walkers rather than embedding traversal in each pass.
- If a construct is not being traversed, add it to the walker first to avoid duplicated traversal logic.
