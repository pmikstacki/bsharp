use crate::node::ast_node::{AstNode, NodeRef};
use crate::query::descendants::Descendants;

/// High-level query API for traversal and filtered extraction.
pub struct Query<'a> {
    start: NodeRef<'a>,
}

impl<'a> Query<'a> {
    pub fn from<T: Into<NodeRef<'a>>>(start: T) -> Self {
        Self {
            start: start.into(),
        }
    }

    pub fn descendants(self) -> Descendants<'a> {
        // Seed with direct children of the start node
        let v: Vec<NodeRef<'a>> = self.start.children_iter().collect();
        Descendants::new(v)
    }

    pub fn of<T: AstNode + 'static>(self) -> impl Iterator<Item = &'a T> {
        let mut acc: Vec<&'a T> = Vec::new();
        for n in self.descendants() {
            if let Some(t) = n.of::<T>() {
                acc.push(t);
            }
        }
        acc.into_iter()
    }

    pub fn filter(
        self,
        p: impl Fn(&NodeRef<'a>) -> bool + 'a,
    ) -> impl Iterator<Item = NodeRef<'a>> + 'a {
        let v: Vec<NodeRef<'a>> = self.descendants().collect();
        v.into_iter().filter(move |n| p(n))
    }

    pub fn filter_typed<T: AstNode + 'static>(
        self,
        p: impl Fn(&T) -> bool + 'a,
    ) -> impl Iterator<Item = &'a T> + 'a {
        self.of::<T>().filter(move |t| p(*t))
    }
}
