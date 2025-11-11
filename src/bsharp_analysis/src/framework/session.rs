use crate::project::Project;
use crate::{AnalysisConfig, AnalysisContext, DiagnosticCollection};
use bsharp_parser::SpanTable;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use crate::framework::NodeRef;
use crate::syntax::declarations::{ClassDeclaration, ConstructorDeclaration, MethodDeclaration, NamespaceDeclaration, PropertyDeclaration};
use crate::framework::{class_fqn, method_fqn, namespace_fqn};
use crate::framework::fqn::{constructor_owner_fqn, property_fqn};
use crate::syntax::ast::CompilationUnit;
use std::ops::Range;
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
    // Pointer to current CompilationUnit for FQN computation (lives outside session)
    cu_ptr: Option<*const CompilationUnit>,
    // Optional pointer-keyed span database (Phase 2 interim)
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
            cu_ptr: None,
            span_db: None,
        }
    }

    /// Set the current CompilationUnit pointer for FQN-based span lookup during analysis
    pub fn set_current_cu(&mut self, cu: &CompilationUnit) {
        self.cu_ptr = Some(cu as *const CompilationUnit);
    }

    /// Bridge: resolve span for a NodeRef via current SpanTable using FQN-based keys
    pub fn span_of(&self, node: &crate::framework::NodeRef) -> Option<(usize, usize)> {
        // Prefer SpanDb if present
        if let Some(db) = &self.span_db {
            if let Some(range) = db.get(node) {
                return Some((range.start, range.end.saturating_sub(range.start)));
            }
        }
        let cu = self.cu_ptr?;
        // SAFETY: cu_ptr is set to a live &CompilationUnit during analysis run scope
        let cu_ref: &CompilationUnit = unsafe { &*cu };
        // Try downcasts in most common order
        if let Some(m) = node.of::<MethodDeclaration>() {
            let full = method_fqn(cu_ref, m); // "ClassPath::Method"
            let key = format!("method::{}", full);
            if let Some(range) = self.spans.get(&key) {
                return Some((range.start, range.end.saturating_sub(range.start)));
            }
        }
        if let Some(c) = node.of::<ConstructorDeclaration>() {
            let owner = constructor_owner_fqn(cu_ref, c);
            if !owner.is_empty() {
                let key = format!("ctor::{}", owner);
                if let Some(range) = self.spans.get(&key) {
                    return Some((range.start, range.end.saturating_sub(range.start)));
                }
            }
        }
        if let Some(p) = node.of::<PropertyDeclaration>() {
            let fq = property_fqn(cu_ref, p); // cfqn::name
            if !fq.is_empty() {
                let key = format!("property::{}", fq);
                if let Some(range) = self.spans.get(&key) {
                    return Some((range.start, range.end.saturating_sub(range.start)));
                }
            }
        }
        if let Some(cls) = node.of::<ClassDeclaration>() {
            let cfqn = class_fqn(cu_ref, cls);
            let key = format!("class::{}", cfqn);
            if let Some(range) = self.spans.get(&key) {
                return Some((range.start, range.end.saturating_sub(range.start)));
            }
        }
        if let Some(ns) = node.of::<NamespaceDeclaration>() {
            let nfqn = namespace_fqn(cu_ref, ns);
            let key = format!("namespace::{}", nfqn);
            if let Some(range) = self.spans.get(&key) {
                return Some((range.start, range.end.saturating_sub(range.start)));
            }
        }
        None
    }

    /// Populate a pointer-keyed SpanDb using the existing string-keyed SpanTable.
    /// This is an interim step before the parser emits a true SpanDb.
    pub fn populate_span_db_from_table(&mut self, cu: &CompilationUnit) {
        let mut db = SpanDb::new();
        // File-scoped namespace is tracked in SpanTable by key, but differs in type; skip here.
        // Top-level declarations
        for decl in &cu.declarations {
            match decl {
                crate::syntax::ast::TopLevelDeclaration::Namespace(ns) => {
                    let nfqn = namespace_fqn(cu, ns);
                    let key = format!("namespace::{}", nfqn);
                    if let Some(range) = self.spans.get(&key) {
                        db.insert((&*ns).into(), range.clone());
                    }
                    // Recurse nested namespaces and classes via helper to cover methods too
                    self.populate_for_namespace(cu, ns, &mut db);
                }
                crate::syntax::ast::TopLevelDeclaration::Class(cls) => {
                    let cfqn = class_fqn(cu, cls);
                    let key = format!("class::{}", cfqn);
                    if let Some(range) = self.spans.get(&key) {
                        db.insert((&*cls).into(), range.clone());
                    }
                    self.populate_for_class(cu, None, cls, &mut db);
                }
                _ => {}
            }
        }
        self.span_db = Some(db);
    }

    fn populate_for_namespace(
        &self,
        cu: &CompilationUnit,
        ns: &crate::syntax::declarations::NamespaceDeclaration,
        db: &mut SpanDb,
    ) {
        use crate::syntax::declarations::namespace_declaration::NamespaceBodyDeclaration as Nbd;
        for m in &ns.declarations {
            match m {
                Nbd::Namespace(inner) => {
                    let nfqn = namespace_fqn(cu, inner);
                    let key = format!("namespace::{}", nfqn);
                    if let Some(range) = self.spans.get(&key) {
                        db.insert((&*inner).into(), range.clone());
                    }
                    self.populate_for_namespace(cu, inner, db);
                }
                Nbd::Class(class) => {
                    let cfqn = class_fqn(cu, class);
                    let key = format!("class::{}", cfqn);
                    if let Some(range) = self.spans.get(&key) {
                        db.insert((&*class).into(), range.clone());
                    }
                    self.populate_for_class(cu, Some(ns), class, db);
                }
                _ => {}
            }
        }
    }

    fn populate_for_class(
        &self,
        cu: &CompilationUnit,
        ns: Option<&crate::syntax::declarations::NamespaceDeclaration>,
        class: &ClassDeclaration,
        db: &mut SpanDb,
    ) {
        use crate::syntax::declarations::ClassBodyDeclaration as Cbd;
        for member in &class.body_declarations {
            match member {
                Cbd::Method(m) => {
                    let fqn = method_fqn(cu, m);
                    let key = format!("method::{}", fqn);
                    if let Some(range) = self.spans.get(&key) {
                        db.insert((&*m).into(), range.clone());
                    }
                }
                Cbd::Constructor(c) => {
                    let owner = constructor_owner_fqn(cu, c);
                    if !owner.is_empty() {
                        let key = format!("ctor::{}", owner);
                        if let Some(range) = self.spans.get(&key) {
                            db.insert((&*c).into(), range.clone());
                        }
                    }
                }
                Cbd::Property(p) => {
                    let fq = property_fqn(cu, p);
                    if !fq.is_empty() {
                        let key = format!("property::{}", fq);
                        if let Some(range) = self.spans.get(&key) {
                            db.insert((&*p).into(), range.clone());
                        }
                    }
                }
                Cbd::NestedClass(nested) => {
                    let cfqn = class_fqn(cu, nested);
                    let key = format!("class::{}", cfqn);
                    if let Some(range) = self.spans.get(&key) {
                        db.insert((&*nested).into(), range.clone());
                    }
                    self.populate_for_class(cu, ns, nested, db);
                }
                _ => {}
            }
        }
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
