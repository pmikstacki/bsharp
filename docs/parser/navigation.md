
# Query API for AST traversal

The analysis framework provides a typed, composable Query API for traversing the AST. It replaces legacy navigation traits.

## Core types

- `NodeRef<'a>`: a thin enum over AST nodes (`CompilationUnit`, `Namespace`, `Class`, `Struct`, `Interface`, `Enum`, `Record`, `Delegate`, `Method`, `Statement`, `Expression`, plus top-level items). See `framework/node_ref.rs`.
- `Query<'a>`: a fluent helper to enumerate descendants and select typed nodes. See `framework/query/`.

```rust
use analysis::framework::{NodeRef, Query};
use analysis::syntax::ast::CompilationUnit;
use analysis::syntax::declarations::{ClassDeclaration, MethodDeclaration};

fn all_classes<'a>(cu: &'a CompilationUnit) -> Vec<&'a ClassDeclaration> {
    Query::from(NodeRef::CompilationUnit(cu))
        .of::<ClassDeclaration>()
        .collect()
}

fn all_methods<'a>(cu: &'a CompilationUnit) -> Vec<&'a MethodDeclaration> {
    Query::from(NodeRef::CompilationUnit(cu))
        .of::<MethodDeclaration>()
        .collect()
}
```

## Descendant enumeration

`Query::descendants()` walks the tree using `Children` implemented for `NodeRef`.

```rust
use analysis::framework::{NodeRef, Query};
use analysis::syntax::statements::statement::Statement;

fn all_statements<'a>(cu: &'a CompilationUnit) -> Vec<&'a Statement> {
    Query::from(NodeRef::CompilationUnit(cu))
        .of::<Statement>()
        .collect()
}
```

## Filtering

Use `filter_typed` to filter by predicate.

```rust
use analysis::syntax::declarations::ClassDeclaration;

let public_classes: Vec<&ClassDeclaration> =
    Query::from(NodeRef::CompilationUnit(&cu))
        .filter_typed::<ClassDeclaration>(|c| c.modifiers.iter().any(|m| m.is_public()))
        .collect();
```

## Best practices

- Prefer `Query` for node enumeration across passes.
- For hot path statement/expression analysis, use shared helpers (`metrics::shared`) or a small local walker when necessary.
- Keep passes stateless and deterministic; feed inputs via `AnalysisSession` artifacts.

## Implementation notes

The `Children`/`Extract` traits are implemented for common AST nodes, enabling `Query::of<T>()` to return strong types. See:
- `framework/query/` for `Children`, `Extract`, `Query`.
- `framework/node_ref.rs` for `NodeRef`.

<!-- Legacy sections removed; this page documents the current Query API only. -->
