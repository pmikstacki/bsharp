# Derive Macros

Procedural macros used by syntax nodes to implement traversal and visualization behavior.

---

## `#[derive(AstNode)]`

- **Crate:** `bsharp_syntax_derive`
- **Implements:** `bsharp_syntax::node::ast_node::AstNode` for your struct/enum
- **Purpose:** Auto-generates `children()` to enable dynamic traversal via `NodeRef`/`DynNodeRef`.

### How it works

For each field, the macro emits code to push children appropriately:
- `Option<T>`: pushes inner `T` if present
- `Vec<T>`: iterates and pushes each `T`
- `Box<T>`: borrows inner `&T` and pushes it
- Other types: treated as AST nodes by default
- Primitive-like types are skipped: `bool`, numbers, `char`, `String`, and internal primitive enums like `PrimitiveType`

Excerpt from implementation (`src/bsharp_syntax_derive/src/lib.rs`):

```rust
#[proc_macro_derive(AstNode)]
pub fn derive_ast_node(input: TokenStream) -> TokenStream {
    // ...
    impl crate::node::ast_node::AstNode for #name {
        fn as_any(&self) -> &dyn ::core::any::Any { self }
        fn children<'a>(&'a self, push: &mut dyn FnMut(crate::node::ast_node::NodeRef<'a>)) {
            // Generated per-type based on fields
        }
    }
}
```

Helper routine decides how to push for common containers:

```rust
fn gen_push_for_type(ty: &Type, access: TokenStream) -> TokenStream {
    // Handles Option<T>, Vec<T>, Box<T>, or default to AST node push
}
```

### Usage

Add the derive to your AST types in `bsharp_syntax`:

```rust
#[derive(bsharp_syntax_derive::AstNode, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Variable(Identifier),
    Invocation(Box<InvocationExpression>),
    // ...
}
```

This enables:
- Graph rendering via `to_text`, `to_mermaid`, `to_dot`
- Traversal via `AstWalker`/`Visit` or `Query` API (by way of `NodeRef` children)

---

## Guidelines

- Ensure child fields are typed as AST nodes or containers of AST nodes for traversal to work.
- Keep primitive data out of traversal (the derive already skips standard primitives).
- Favor `Box<T>` for recursive enum variants to keep sizes reasonable.

---

## See Also

- `docs/syntax/traits.md` – `AstNode`, `NodeRef`
- `docs/analysis/traversal-guide.md` – traversal patterns
- `docs/development/query-cookbook.md` – query examples
