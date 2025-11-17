use crate::project::Project;
use crate::{AnalysisConfig, AnalysisContext, DiagnosticCollection};
use bsharp_parser::SpanTable;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use bsharp_syntax::spans::span_db::SpanDb;

/// Type-safe, thread-safe artifact store keyed by TypeId
#[derive(Default)]
pub struct ArtifactStore {
    inner: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

// Using shared SpanDb from bsharp_syntax

impl ArtifactStore {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert<T: Any + Send + Sync>(&self, value: T) {
        let mut map = self.inner.write().expect("artifact store poisoned");
        map.insert(TypeId::of::<T>(), Box::new(std::sync::Arc::new(value)));
    }

    pub fn insert_arc<T: Any + Send + Sync>(&self, value: std::sync::Arc<T>) {
        let mut map = self.inner.write().expect("artifact store poisoned");
        map.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Any + Send + Sync>(&self) -> Option<std::sync::Arc<T>> {
        let map = self.inner.read().ok()?;
        let any = map.get(&TypeId::of::<T>())?;
        // Clone via Arc if already Arc<T>, else attempt downcast_ref and clone if T: Clone is not guaranteed
        // Simpler: require stored value to be Arc<T> for reads. For now, support only Arc<T> retrievals.
        any.downcast_ref::<std::sync::Arc<T>>().cloned()
    }
}

/// Per-run analysis session
pub struct AnalysisSession {
    pub ctx: AnalysisContext,
    pub spans: SpanTable,
    pub diagnostics: DiagnosticCollection,
    pub artifacts: ArtifactStore,
    pub config: AnalysisConfig,
    // Project-wide handle
    pub project: Project,
    // Optional pointer-keyed span database (now authoritative)
    pub span_db: Option<SpanDb>,
}

impl AnalysisSession {
    pub fn new(ctx: AnalysisContext, spans: SpanTable) -> Self {
        Self {
            config: ctx.config.clone(),
            ctx,
            spans,
            diagnostics: DiagnosticCollection::default(),
            artifacts: ArtifactStore::new(),
            project: Project::new(),
            span_db: None,
        }
    }

    /// Resolve span for a NodeRef via SpanDb
    pub fn span_of(&self, node: &crate::framework::NodeRef) -> Option<(usize, usize)> {
        let db = self.span_db.as_ref()?;
        let range = db.get(node)?;
        Some((range.start, range.end.saturating_sub(range.start)))
    }


    /// Temporary placeholder: map NodeRef to SourceLocation using ctx and span_of
    pub fn at(&self, node: &crate::framework::NodeRef) -> Option<crate::diagnostics::source_location::SourceLocation> {
        self.span_of(node).map(|(start, len)| self.ctx.location_from_span(start, len))
    }

    /// Insert a typed artifact into the session store.
    pub fn insert_artifact<T: Any + Send + Sync>(&self, value: T) {
        self.artifacts.insert(value);
    }

    /// Retrieve a typed artifact from the session store.
    pub fn get_artifact<T: Any + Send + Sync>(&self) -> Option<Arc<T>> {
        self.artifacts.get::<T>()
    }
}
