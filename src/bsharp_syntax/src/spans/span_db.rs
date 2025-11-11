use crate::node::ast_node::DynNodeRef;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Default, Debug, Clone)]
pub struct SpanDb {
    map: HashMap<usize, Range<usize>>,
}

impl SpanDb {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn insert(&mut self, node: DynNodeRef, range: Range<usize>) {
        let raw: *const dyn crate::node::ast_node::AstNode = node.0;
        let data_ptr = raw as *const ();
        let key = data_ptr as usize;
        self.map.insert(key, range);
    }

    pub fn get(&self, node: &DynNodeRef) -> Option<&Range<usize>> {
        let raw: *const dyn crate::node::ast_node::AstNode = node.0;
        let data_ptr = raw as *const ();
        let key = data_ptr as usize;
        self.map.get(&key)
    }
}
