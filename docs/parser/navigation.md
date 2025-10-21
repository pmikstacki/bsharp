
# Query API for AST traversal

The Query API is provided by the `bsharp_syntax` crate and re-exported by `bsharp_analysis` for convenience. It replaces older navigation traits, but the Query API itself is current and not deprecated.

## Core types

- `NodeRef<'a>`: a thin enum over AST nodes (`CompilationUnit`, `Namespace`, `Class`, `Struct`, `Interface`, `Enum`, `Record`, `Delegate`, `Method`, `Statement`, `Expression`, plus top-level items). Origin: `bsharp_syntax::node::ast_node::NodeRef` (re-exported as `bsharp_analysis::framework::NodeRef`).
- `Query<'a>`: a fluent helper to enumerate descendants and select typed nodes. Origin: `bsharp_syntax::query::Query` (re-exported as `bsharp_analysis::framework::Query`).

```rust
use bsharp_analysis::framework::{NodeRef, Query};
use bsharp_syntax::CompilationUnit;
use bsharp_syntax::{ClassDeclaration, MethodDeclaration};

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
use bsharp_analysis::framework::{NodeRef, Query};
use bsharp_syntax::statements::Statement;

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
- `src/bsharp_syntax/src/query/` for `Children`, `Extract`, `Query`.
- `src/bsharp_syntax/src/node/ast_node.rs` for `NodeRef`.

<!-- Legacy sections removed; this page documents the current Query API only. -->
