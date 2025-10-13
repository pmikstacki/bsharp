use crate::AstNode;

/// A dynamic reference to any `AstNode`.
#[derive(Copy, Clone)]
pub struct DynNodeRef<'a>(pub &'a dyn AstNode);

impl<'a> DynNodeRef<'a> {
    /// Visit children of this node by invoking the provided callback for each child.
    pub fn children(self, mut push: impl FnMut(DynNodeRef<'a>)) {
        self.0.children(&mut push);
    }
}

/// Generic typed extractor from a `DynNodeRef` to a concrete node type.
pub trait ExtractDyn<'a, T> {
    fn as_ref(&self) -> Option<&'a T>;
}

impl<'a, T: AstNode + 'static> ExtractDyn<'a, T> for DynNodeRef<'a> {
    fn as_ref(&self) -> Option<&'a T> {
        self.0.as_any().downcast_ref::<T>()
    }
}

impl<'a, T: AstNode + 'a> From<&'a T> for DynNodeRef<'a> {
    fn from(value: &'a T) -> Self {
        DynNodeRef(value)
    }
}

impl<'a> DynNodeRef<'a> {
    /// Downcast to a concrete node type if it matches.
    pub fn of<T: AstNode + 'static>(&self) -> Option<&'a T> {
        <DynNodeRef<'a> as ExtractDyn<'a, T>>::as_ref(self)
    }

    /// Return an iterator over this node's children.
    pub fn children_iter(self) -> impl Iterator<Item = DynNodeRef<'a>> {
        let mut out: Vec<DynNodeRef<'a>> = Vec::new();
        self.children(|c| out.push(c));
        out.into_iter()
    }
}