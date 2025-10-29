use crate::node::dyn_node_ref::DynNodeRef;
use std::any::Any;

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
        if let Some(v) = self.node_label_value() {
            v
        } else {
            self.node_kind().to_string()
        }
    }

    /// Optional value text for labeling (combined with a short kind by renderers).
    /// Default is None; derive macro will generate implementations based on common conventions.
    fn node_label_value(&self) -> Option<String> { None }
}

/// Public ergonomic alias used across the codebase.
pub type NodeRef<'a> = DynNodeRef<'a>;

pub fn push_child<'a, T: AstNode + 'a>(push: &mut dyn FnMut(NodeRef<'a>), node: &'a T) {
    push(DynNodeRef(node));
}
