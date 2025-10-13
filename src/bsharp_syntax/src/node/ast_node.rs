use std::any::Any;
use crate::node::dyn_node_ref::DynNodeRef;

/// Trait implemented by all AST node types to support dynamic traversal and extraction.
pub trait AstNode: Any {
    /// Returns a `&dyn Any` to enable downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Push all direct child nodes via the provided callback.
    /// Implementations should call `push(DynNodeRef(child))` for each child.
    fn children<'a>(&'a self, _push: &mut dyn FnMut(DynNodeRef<'a>)) {}
}

/// Public ergonomic alias used across the codebase.
pub type NodeRef<'a> = DynNodeRef<'a>;