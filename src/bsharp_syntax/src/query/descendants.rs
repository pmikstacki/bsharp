use crate::node::ast_node::NodeRef;

pub struct Descendants<'a> {
    stack: Vec<NodeRef<'a>>,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = NodeRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.stack.pop()?;
        for c in n.children_iter() {
            self.stack.push(c);
        }
        Some(n)
    }
}

impl<'a> Descendants<'a> {
    pub fn new(stack: Vec<NodeRef<'a>>) -> Self {
        Self { stack }
    }
}
