# Syntax Traits

Core traits used by AST types and formatting emitters.

---

## AstNode

- **Path:** `bsharp_syntax::node::ast_node::AstNode`
- Implemented by all syntax node types for traversal and visualization.

```rust
pub trait AstNode: Any {
    fn as_any(&self) -> &dyn Any;
    fn children<'a>(&'a self, _push: &mut dyn FnMut(NodeRef<'a>)) {}
    fn node_kind(&self) -> &'static str { core::any::type_name::<Self>() }
    fn node_label(&self) -> String { format!("{} ({})", self.node_kind(), core::any::type_name::<Self>()) }
}
```

Helpers:
- `NodeRef<'a>` alias to `DynNodeRef<'a>` for dynamic traversal.
- `push_child(push, node)` to push typed children.

---

## Emit and Emitter

- **Path:** `bsharp_syntax::emitters::emit_trait::{Emit, Emitter, EmitCtx}`
- `Emit` is implemented by nodes that can render themselves as C# code.
- `Emitter` writes items to `String` (or writer) using a mutable `EmitCtx`.

```rust
pub trait Emit {
    fn emit<W: std::fmt::Write>(&self, w: &mut W, cx: &mut EmitCtx) -> Result<(), EmitError>;
}
```

`EmitCtx` controls indentation, simple policies, and optional JSONL tracing.

---

## Rendering Helpers

- Graph renderers in `bsharp_syntax::node::render::{to_text, to_mermaid, to_dot}` operate on `&impl AstNode`.

---

## See Also

- `docs/syntax/spans.md`
- `docs/syntax/derive-macros.md`
- `docs/syntax/formatter.md`
