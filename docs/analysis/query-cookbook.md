# Query Cookbook

> Note: This page has moved. The canonical location is now [Development/Query Cookbook](../development/query-cookbook.md). This copy remains temporarily for back-compat and will be removed.

Practical examples for using the `Query` API to traverse the AST.

Imports used throughout:

```rust
use bsharp_analysis::framework::{NodeRef, Query};
use bsharp_syntax::{CompilationUnit, ClassDeclaration, MethodDeclaration};
```

---

## All classes in a file

```rust
fn all_classes(cu: &CompilationUnit) -> Vec<&ClassDeclaration> {
    Query::from(NodeRef::CompilationUnit(cu))
        .of::<ClassDeclaration>()
        .collect()
}
```

## All methods in a class

```rust
fn all_methods_in_class(c: &ClassDeclaration) -> Vec<&MethodDeclaration> {
    Query::from(NodeRef::from(c))
        .of::<MethodDeclaration>()
        .collect()
}
```

## Public methods only

```rust
use bsharp_syntax::modifiers::Modifier;

fn public_methods(cu: &CompilationUnit) -> Vec<&MethodDeclaration> {
    Query::from(NodeRef::CompilationUnit(cu))
        .filter_typed::<MethodDeclaration>(|m| m.modifiers.iter().any(|mm| *mm == Modifier::Public))
        .collect()
}
```

## Count await expressions

```rust
use bsharp_syntax::expressions::AwaitExpression;

fn await_count(cu: &CompilationUnit) -> usize {
    Query::from(NodeRef::CompilationUnit(cu))
        .of::<AwaitExpression>()
        .count()
}
```

## Find invocations of a method name

```rust
use bsharp_syntax::expressions::{InvocationExpression, Expression};

fn invocations_of(cu: &CompilationUnit, name: &str) -> Vec<&InvocationExpression> {
    Query::from(NodeRef::CompilationUnit(cu))
        .filter_typed::<InvocationExpression>(|inv| {
            // Match simple Variable(...) calls; extend for MemberAccess as needed
            match &*inv.expression {
                Expression::Variable(id) => id.name == name,
                _ => false,
            }
        })
        .collect()
}
```

## Methods with deep nesting

```rust
use bsharp_syntax::statements::statement::Statement;

fn deeply_nested_methods(cu: &CompilationUnit, threshold: usize) -> Vec<&MethodDeclaration> {
    Query::from(NodeRef::CompilationUnit(cu))
        .filter_typed::<MethodDeclaration>(|m| {
            if let Some(body) = &m.body {
                max_nesting(body, 0) > threshold
            } else {
                false
            }
        })
        .collect()
}

fn max_nesting(s: &Statement, cur: usize) -> usize {
    match s {
        Statement::If(i) => {
            let then_d = max_nesting(&i.consequence, cur + 1);
            let else_d = i.alternative.as_ref().map(|a| max_nesting(a, cur + 1)).unwrap_or(cur);
            then_d.max(else_d)
        }
        Statement::Block(stmts) => stmts.iter().map(|st| max_nesting(st, cur)).max().unwrap_or(cur),
        Statement::For(f) => max_nesting(&f.body, cur + 1),
        Statement::ForEach(f) => max_nesting(&f.body, cur + 1),
        Statement::While(w) => max_nesting(&w.body, cur + 1),
        Statement::DoWhile(d) => max_nesting(&d.body, cur + 1),
        _ => cur,
    }
}
```

---

## Tips

- **Chain filters sparingly**: Prefer a single `filter_typed` with a clear predicate.
- **Use `NodeRef::from(x)`**: Start from any AST node to scope queries.
- **Profile**: For hot paths, consider a custom walker when you need full control.
