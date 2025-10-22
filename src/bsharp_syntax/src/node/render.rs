use crate::node::ast_node::AstNode;
use crate::node::dyn_node_ref::DynNodeRef;
use std::collections::HashMap;

fn traverse<'a>(
    root: DynNodeRef<'a>,
    mut on_edge: impl FnMut(usize, usize),
    mut on_node: impl FnMut(usize, &'a dyn AstNode),
) {
    let mut stack: Vec<DynNodeRef<'a>> = vec![root];
    let mut ids: HashMap<*const (), usize> = HashMap::new();
    let mut next_id: usize = 0;

    // First pass assigns IDs and emits nodes; edges will be emitted as we discover children
    while let Some(nr) = stack.pop() {
        let ptr = (nr.0 as *const dyn AstNode) as *const ();
        let id = *ids.entry(ptr).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        on_node(id, nr.0);
        nr.children(|child| {
            let cptr = (child.0 as *const dyn AstNode) as *const ();
            let cid = *ids.entry(cptr).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
            on_edge(id, cid);
            stack.push(child);
        });
    }
}

pub fn to_mermaid(root: &impl AstNode) -> String {
    let mut out = String::new();
    out.push_str("graph TD\n");
    let mut nodes: HashMap<usize, String> = HashMap::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    traverse(
        DynNodeRef(root),
        |a, b| edges.push((a, b)),
        |i, n| {
            nodes.entry(i).or_insert_with(|| n.node_label());
        },
    );

    // Emit nodes
    let mut ids: Vec<_> = nodes.keys().cloned().collect();
    ids.sort_unstable();
    for i in ids {
        let label = nodes.get(&i).cloned().unwrap_or_else(|| format!("n{}", i));
        out.push_str(&format!("n{}[\"{}\"]\n", i, label.replace('"', "\\\"")));
    }
    // Emit edges
    for (a, b) in edges {
        out.push_str(&format!("n{} --> n{}\n", a, b));
    }
    out
}

pub fn to_dot(root: &impl AstNode) -> String {
    let mut out = String::new();
    out.push_str("digraph AST {\n");
    out.push_str("  node [shape=box, fontname=\"Courier New\"];\n");

    let mut nodes: HashMap<usize, String> = HashMap::new();
    let mut edges: Vec<(usize, usize)> = Vec::new();

    traverse(
        DynNodeRef(root),
        |a, b| edges.push((a, b)),
        |i, n| {
            nodes.entry(i).or_insert_with(|| n.node_label());
        },
    );

    let mut ids: Vec<_> = nodes.keys().cloned().collect();
    ids.sort_unstable();
    for i in ids {
        let label = nodes.get(&i).cloned().unwrap_or_else(|| format!("n{}", i));
        out.push_str(&format!(
            "  n{} [label=\"{}\"];\n",
            i,
            label.replace('"', "\\\"")
        ));
    }
    for (a, b) in edges {
        out.push_str(&format!("  n{} -> n{};\n", a, b));
    }
    out.push_str("}\n");
    out
}

pub fn to_text(root: &impl AstNode) -> String {
    fn rec(out: &mut String, depth: usize, node: DynNodeRef) {
        use std::fmt::Write as _;
        let indent = "  ".repeat(depth);
        writeln!(out, "{}{}", indent, node.0.node_label()).ok();
        node.children(|c| rec(out, depth + 1, c));
    }
    let mut out = String::new();
    rec(&mut out, 0, DynNodeRef(root));
    out
}
