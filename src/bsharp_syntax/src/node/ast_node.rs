use std::any::Any;
use crate::node::dyn_node_ref::DynNodeRef;

/// Trait implemented by all AST node types to support dynamic traversal and extraction.
pub trait AstNode: Any {
    /// Returns a `&dyn Any` to enable downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Push all direct child nodes via the provided callback.
    /// Implementations should call `push(DynNodeRef(child))` for each child.
    fn children<'a>(&'a self, _push: &mut dyn FnMut(DynNodeRef<'a>)) {}

    /// A stable kind string for this node, defaults to the Rust type name.
    fn node_kind(&self) -> &'static str {
        core::any::type_name::<Self>()
    }

    /// A short human-friendly label for visualization. Defaults to `node_kind()`.
    fn node_label(&self) -> String {
        format!("{} ({})", self.node_kind(), core::any::type_name::<Self>())
    }
}

/// Public ergonomic alias used across the codebase.
pub type NodeRef<'a> = DynNodeRef<'a>;

pub fn push_child<'a, T: AstNode + 'a>(push: &mut dyn FnMut(NodeRef<'a>), node: &'a T) {
    push(DynNodeRef(node));
}