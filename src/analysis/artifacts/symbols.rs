use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Basic symbol kinds for indexing (expand later)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    Namespace,
    Class,
    Interface,
    Struct,
    Record,
    Enum,
    Delegate,
    Method,
    Field,
    Property,
}

/// Opaque, stable symbol identifier within a single analysis session.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SymbolId(pub u64);

/// Concrete symbol information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Symbol {
    pub id: SymbolId,
    pub name: String,
    pub kind: SymbolKind,
    pub fqn: Option<String>,
    pub file: Option<String>,
    pub span_start: Option<usize>,
    pub span_end: Option<usize>,
}

/// A symbol index tracking symbols by id and by name (allowing duplicates across namespaces/files).
#[derive(Default)]
pub struct SymbolIndex {
    next_id: u64,
    pub by_id: HashMap<SymbolId, Symbol>,
    pub by_name: HashMap<String, Vec<SymbolId>>, // e.g., "MyClass" -> [SymbolId]
}

impl SymbolIndex {
    pub fn new() -> Self { Self::default() }

    pub fn insert(
        &mut self,
        name: impl Into<String>,
        kind: SymbolKind,
        fqn: Option<String>,
        file: Option<String>,
        span_start: Option<usize>,
        span_end: Option<usize>,
    ) -> SymbolId {
        let name = name.into();
        let id = SymbolId(self.next_id);
        self.next_id += 1;
        let sym = Symbol { id, name: name.clone(), kind, fqn, file, span_start, span_end };
        self.by_id.insert(id, sym);
        self.by_name.entry(name).or_default().push(id);
        id
    }

    pub fn get_ids_by_name(&self, name: &str) -> Option<&Vec<SymbolId>> { self.by_name.get(name) }
    pub fn get(&self, id: SymbolId) -> Option<&Symbol> { self.by_id.get(&id) }
}

/// A flat name index for quick lookups; may duplicate SymbolIndex but optimized for names only.
#[derive(Default)]
pub struct NameIndex(pub HashMap<String, usize>);

/// Fully qualified name map; maps a local name to FQN(s).
#[derive(Default)]
pub struct FqnMap(pub HashMap<String, Vec<String>>);

