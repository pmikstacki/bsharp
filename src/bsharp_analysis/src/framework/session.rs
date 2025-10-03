use crate::{AnalysisConfig, AnalysisContext, DiagnosticCollection};
use bsharp_parser::SpanTable;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;

/// Type-safe, thread-safe artifact store keyed by TypeId
#[derive(Default)]
pub struct ArtifactStore {
    inner: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

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
    // Project-wide handle; minimal placeholder for now
    pub project: Option<crate::project::Project>,
}

impl AnalysisSession {
    pub fn new(ctx: AnalysisContext, spans: SpanTable) -> Self {
        Self {
            config: ctx.config.clone(),
            ctx,
            spans,
            diagnostics: DiagnosticCollection::default(),
            artifacts: ArtifactStore::new(),
            project: None,
        }
    }
}
