use crate::artifacts::control_flow_graph::block::{BasicBlock, BlockId};
use crate::artifacts::control_flow_graph::edge::EdgeKind;
use crate::artifacts::control_flow_graph::terminator::Terminator;

use bsharp_syntax::statements::statement::Statement;
// Control Flow artifacts: per-method control flow stats and Control Flow Graph (CFG)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Control Flow Graph for a single method.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ControlFlowGraph {
    pub blocks: Vec<BasicBlock>,
    pub edges: Vec<(BlockId, BlockId, EdgeKind)>,
    pub entry: Option<BlockId>,
    pub exits: Vec<BlockId>,
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate cyclomatic complexity M = E - N + 2P, where P is number of connected components.
    pub fn cyclomatic_complexity(&self) -> usize {
        let e = self.edges.len() as isize;
        let n = self.blocks.len() as isize;
        // Assume single connected component when entry exists, else P = 0
        let p = if self.entry.is_some() { 1 } else { 0 } as isize;
        let m = e - n + 2 * p;
        m.max(1) as usize
    }

    /// Compute essential complexity using a simple structured reduction.
    /// Heuristics implemented:
    /// - Collapse diamonds: a block with two successors that reconverge at a join.
    /// - Collapse trivial multi-branch reconvergence (switch-like) when all branch nodes flow directly to a common join.
    ///   This is a conservative approximation; in well-structured code it commonly reduces to 1.
    pub fn essential_complexity(&self) -> usize {
        use std::collections::{HashMap, HashSet};

        // Build mutable graph representation
        #[derive(Clone, Copy, Eq, PartialEq, Hash)]
        struct Edge(BlockId, BlockId, EdgeKind);

        let mut nodes: HashSet<BlockId> = self.blocks.iter().map(|b| b.id).collect();
        let mut edges: HashSet<Edge> = self
            .edges
            .iter()
            .map(|(a, b, k)| Edge(*a, *b, *k))
            .collect();

        let mut changed = true;
        while changed {
            changed = false;

            // Rebuild adjacency
            let mut out: HashMap<BlockId, Vec<(BlockId, EdgeKind)>> = HashMap::new();
            let mut inn: HashMap<BlockId, Vec<(BlockId, EdgeKind)>> = HashMap::new();
            for &Edge(u, v, k) in &edges {
                out.entry(u).or_default().push((v, k));
                inn.entry(v).or_default().push((u, k));
            }

            // 1) Collapse diamonds u -> {v,w} -> j where v and w have single in from u and single out to j
            let mut removals: Vec<(BlockId, BlockId, BlockId)> = Vec::new(); // (u, v, w) to collapse into u->j
            for &u in &nodes {
                let succ = out.get(&u).cloned().unwrap_or_default();
                if succ.len() == 2 {
                    let (v, _kv) = succ[0];
                    let (w, _kw) = succ[1];
                    let v_in = inn.get(&v).map(|x| x.len()).unwrap_or(0);
                    let w_in = inn.get(&w).map(|x| x.len()).unwrap_or(0);
                    let v_out = out.get(&v).cloned().unwrap_or_default();
                    let w_out = out.get(&w).cloned().unwrap_or_default();
                    if v_in == 1 && w_in == 1 && v_out.len() == 1 && w_out.len() == 1 {
                        let (j1, _k1) = v_out[0];
                        let (j2, _k2) = w_out[0];
                        if j1 == j2 {
                            // Additionally ensure j's incoming includes exactly these two to avoid over-collapsing
                            let j_in = inn.get(&j1).cloned().unwrap_or_default();
                            let preds: HashSet<BlockId> = j_in.iter().map(|(p, _)| *p).collect();
                            if preds.contains(&v) && preds.contains(&w) {
                                removals.push((u, v, w));
                            }
                        }
                    }
                }
            }
            if !removals.is_empty() {
                changed = true;
                for (u, v, w) in removals {
                    // Determine join j
                    let j = {
                        let v_out = out.get(&v).unwrap();
                        v_out[0].0
                    };
                    // Remove edges u->v, u->w, v->j, w->j
                    edges.retain(|Edge(a, b, _)| {
                        !matches!((*a, *b), (x, y) if
                            (x == u && (y == v || y == w)) ||
                            ((x == v || x == w) && y == j)
                        )
                    });
                    // Remove nodes v and w
                    nodes.remove(&v);
                    nodes.remove(&w);
                    // Add edge u->j
                    edges.insert(Edge(u, j, EdgeKind::Normal));
                }
                continue; // rebuild maps and iterate again
            }

            // 2) Collapse trivial multi-branch reconvergence: multiple preds all with single out to common j and single in from same head
            // This is a weaker heuristic: if a node j has >=2 predecessors X, and every x in X has out-degree 1 to j and in-degree 1,
            // collapse all x into their unique predecessor head (if it exists and is same for all).
            let mut collapsed_any = false;
            // Build reverse map: for each node j with in >= 2
            for &j in &nodes {
                let preds = inn.get(&j).cloned().unwrap_or_default();
                if preds.len() < 2 {
                    continue;
                }
                // Candidates X: each has out-degree 1 to j and in-degree 1
                let mut x_set: Vec<BlockId> = Vec::new();
                let mut head_opt: Option<BlockId> = None;
                let mut ok = true;
                for (x, _kx) in &preds {
                    let x_out = out.get(x).cloned().unwrap_or_default();
                    let x_in = inn.get(x).cloned().unwrap_or_default();
                    if x_out.len() != 1 || x_out[0].0 != j || x_in.len() != 1 {
                        ok = false;
                        break;
                    }
                    let head = x_in[0].0;
                    if let Some(h) = head_opt {
                        if h != head {
                            ok = false;
                            break;
                        }
                    } else {
                        head_opt = Some(head);
                    }
                    x_set.push(*x);
                }
                if ok {
                    if let Some(head) = head_opt {
                        if head != j {
                            // Perform collapse: remove all x in X and wire head->j
                            for x in &x_set {
                                nodes.remove(x);
                            }
                            edges.retain(|Edge(a, b, _)| {
                                !(x_set.iter().any(|x| *a == *x || *b == *x))
                            });
                            edges.insert(Edge(head, j, EdgeKind::Normal));
                            collapsed_any = true;
                            break;
                        }
                    }
                }
            }
            if collapsed_any {
                changed = true;
            }
        }

        // Compute M on reduced graph
        let e = edges.len() as isize;
        let n = nodes.len() as isize;
        let p = if self.entry.is_some() { 1 } else { 0 } as isize; // assume single component per method
        let m = e - n + 2 * p;
        m.max(1) as usize
    }
}

/// Build a minimal CFG for a statement tree.
pub fn build_cfg(stmt: &Statement) -> ControlFlowGraph {
    struct Builder {
        cfg: ControlFlowGraph,
        next_id: u32,
    }
    impl Builder {
        fn new() -> Self {
            Self {
                cfg: ControlFlowGraph::new(),
                next_id: 0,
            }
        }
        fn bb(&mut self) -> BlockId {
            let id = BlockId(self.next_id);
            self.next_id += 1;
            self.cfg.blocks.push(BasicBlock {
                id,
                statements: Vec::new(),
                terminator: Terminator::Unreachable,
            });
            id
        }
        fn edge(&mut self, from: BlockId, to: BlockId, kind: EdgeKind) {
            self.cfg.edges.push((from, to, kind));
        }
        fn build_stmt(&mut self, s: &Statement) -> (BlockId, BlockId) {
            match s {
                Statement::Block(stmts) => {
                    let mut entry = None;
                    let mut last_exit = None;
                    for st in stmts {
                        let (e, x) = self.build_stmt(st);
                        if let Some(prev) = last_exit {
                            self.edge(prev, e, EdgeKind::Normal);
                        } else {
                            entry = Some(e);
                        }
                        last_exit = Some(x);
                    }
                    let e = entry.unwrap_or_else(|| self.bb());
                    let x = last_exit.unwrap_or(e);
                    (e, x)
                }
                Statement::If(if_stmt) => {
                    let cond = self.bb();
                    let (t_entry, t_exit) = self.build_stmt(&if_stmt.consequence);
                    let (f_entry, f_exit) = if let Some(alt) = &if_stmt.alternative {
                        self.build_stmt(alt)
                    } else {
                        let bb = self.bb();
                        (bb, bb)
                    };
                    self.edge(cond, t_entry, EdgeKind::True);
                    self.edge(cond, f_entry, EdgeKind::False);
                    let join = self.bb();
                    self.edge(t_exit, join, EdgeKind::Normal);
                    self.edge(f_exit, join, EdgeKind::Normal);
                    (cond, join)
                }
                Statement::While(w) => {
                    let head = self.bb();
                    let (body_e, body_x) = self.build_stmt(&w.body);
                    self.edge(head, body_e, EdgeKind::True);
                    self.edge(body_x, head, EdgeKind::Normal);
                    let exit = self.bb();
                    self.edge(head, exit, EdgeKind::False);
                    (head, exit)
                }
                Statement::For(f) => {
                    // Treat like while for CFG shape
                    let head = self.bb();
                    let (body_e, body_x) = self.build_stmt(&f.body);
                    self.edge(head, body_e, EdgeKind::True);
                    self.edge(body_x, head, EdgeKind::Normal);
                    let exit = self.bb();
                    self.edge(head, exit, EdgeKind::False);
                    (head, exit)
                }
                Statement::DoWhile(dw) => {
                    let (body_e, body_x) = self.build_stmt(&dw.body);
                    let head = body_e;
                    self.edge(body_x, head, EdgeKind::Normal);
                    let exit = self.bb();
                    self.edge(head, exit, EdgeKind::False);
                    (head, exit)
                }
                Statement::Switch(sw) => {
                    let head = self.bb();
                    let mut exits: Vec<BlockId> = Vec::new();
                    for section in &sw.sections {
                        let (e, x) = self.build_stmt(&Statement::Block(section.statements.clone()));
                        self.edge(head, e, EdgeKind::SwitchCase);
                        exits.push(x);
                    }
                    let join = self.bb();
                    for x in exits {
                        self.edge(x, join, EdgeKind::Normal);
                    }
                    (head, join)
                }
                Statement::Try(t) => {
                    let try_entry_exit = self.build_stmt(&t.try_block);
                    let mut all_exits = vec![try_entry_exit.1];
                    for c in &t.catches {
                        let (e, x) = self.build_stmt(&c.block);
                        self.edge(try_entry_exit.0, e, EdgeKind::Exception);
                        all_exits.push(x);
                    }
                    let mut join = None;
                    if let Some(fin) = &t.finally_clause {
                        let (fe, fx) = self.build_stmt(&fin.block);
                        for x in all_exits.drain(..) {
                            self.edge(x, fe, EdgeKind::Finally);
                        }
                        join = Some(fx);
                    }
                    let j = join.unwrap_or_else(|| {
                        let bb = self.bb();
                        for x in all_exits {
                            self.edge(x, bb, EdgeKind::Normal);
                        }
                        bb
                    });
                    (try_entry_exit.0, j)
                }
                _ => {
                    // Single statement in its own block
                    let b = self.bb();
                    (b, b)
                }
            }
        }
    }

    let mut b = Builder::new();
    let (entry, exit) = b.build_stmt(stmt);
    b.cfg.entry = Some(entry);
    b.cfg.exits.push(exit);
    b.cfg
}

/// Collection of per-method Control Flow Graphs keyed by fully-qualified method name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ControlFlowGraphs(pub HashMap<String, ControlFlowGraph>);
