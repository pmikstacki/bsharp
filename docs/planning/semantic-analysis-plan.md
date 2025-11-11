 # Semantic Analysis and Diagnostics Redesign Plan

 ## Objectives
 - **DRY diagnostics**: Generate `DiagnosticCode`, severity, category, message mappings, and helpers from a declarative source (proc-macro), not hand-maintained tables.
 - **Low-boilerplate analyzers**: Author semantic rules with minimal glue. Prefer `rule!`/`ruleset!` macros and typed visit hooks over manual `Rule` impls.
 - **First-class spans**: Use `Spanned<T>`/`HasSpan` consistently across analysis. Remove stringly-typed span keys like "ctor::..."/"method::...".
 - **Composable passes**: Typed artifact store and pass execution model that makes dependencies explicit and discoverable.
 - **Robustness and extensibility**: Make it easy to add new rules from docs/development/semantic-rules.md without duplicating plumbing.

 ## Current State (Findings)
 - **Diagnostics**
   - `diagnostics/diagnostic_code.rs` is a large hand-maintained enum + mappings for severity/category/default messages.
   - `framework/diagnostic_builder.rs` provides builder ergonomics but still requires manual code/message/placement.
   - Severity/category overrides are supported via `AnalysisConfig.rule_severities`.
 - **Spans**
   - Parser provides `Spanned<T>` (`span.rs` in bsharp_parser) and a sidecar `SpanTable` mapping string keys to `Range<usize>`.
   - Analysis currently relies on `SpanTable` string keys (e.g., `find_ctor_span`, `find_method_span`) to locate nodes for diagnostics.
   - `HasSpan` exists for `Spanned<T>`, but AST nodes in analysis are un-spanned; walker traverses plain AST with `NodeRef`.
 - **Analyzer framework**
   - Rules implement `Rule` with `visit(&NodeRef, &mut AnalysisSession)`. Rule sets are registered via `AnalyzerRegistry` and run via `AnalyzerPipeline` + `AstWalker`.
   - Rules are verbose (pattern matching, span lookup via ad-hoc helpers, builder ceremony).
 - **Artifacts**
   - `ArtifactStore` exists in `AnalysisSession`, but rule authors must manually fetch artifacts; dependencies between passes/rules aren’t declared at type level.

 ## Design Principles
 - **Macro-first ergonomics**: Diagnostics and rule registration generated from declarative specs.
 - **Typed spans everywhere**: Every visited node can be mapped to a `ByteRange` via a stable, non-string identifier.
 - **Return-early, explicit errors**: No hidden control flow; avoid silent failures; fail fast with clear diagnostics.
 - **Single traversal, multi-visitor**: Keep one pass per phase with a shared visitor dispatch to minimize AST walking overhead.

 ---

 ## Phase 1 — Diagnostics Macroization (proc-macro crate)
 - **Deliverables**
   - New crate `bsharp_diagnostics_macros` that exports a `diagnostics! { ... }` macro.
   - Generated items in `bsharp_analysis::diagnostics`:
     - `enum DiagnosticCode` with documented variants.
     - `impl DiagnosticCode` with `as_str()`, `severity()`, `category()`, `default_message()`.
     - `fn parse_code(&str) -> Option<DiagnosticCode>` and `fn all_codes() -> &'static [DiagnosticCode]` for tooling.
   - Replace current `diagnostic_code.rs` with generated equivalent; keep `Diagnostic`, `DiagnosticCollection`, `SourceLocation` unchanged.

 - **Macro DSL sketch**
   ```rust
   diagnostics! {
       // group "Semantic"; error
       BSE01001 => {
           category: Semantic,
           severity: Error,
           title: "Async constructor not allowed",
           message: "Constructors cannot be declared async"
       },
       BSE02009 => {
           category: Semantic,
           severity: Error,
           title: "Async returns Task or Task<T>",
           message: "Async methods must return Task or Task<T>"
       },
       BSW02005 => {
           category: Style,
           severity: Warning,
           title: "Unused variable",
           message: "Variable is declared but never used"
       }
   }
   ```
   - The macro expands to the enum, metadata tables, and helper impls.
   - Source-of-truth becomes the macro invocation; no hand-edited match arms.

 - **Builder ergonomics**
   - Introduce `diag!` helper macro to cut boilerplate:
     ```rust
     // Minimal (default message, at node)
     diag!(session, BSE02009, at node);

     // With custom message and related
     diag!(session, BSE02009, at node,
          msg: format!("Invalid return type '{}'", ty),
          related: [(other_node, "Reason here")]);
     ```
   - `diag!` resolves spans via Phase 2 span APIs (see below). Falls back to default message if `msg:` omitted.

 - **Config overrides**
   - Preserve `AnalysisConfig.rule_severities` override in `diag!` expansion (post-build severity patch stays intact).

 ---

 ## Phase 2 — Unified Span Model
 - **Goals**
   - Eliminate string-keyed `SpanTable` lookups from rules.
   - Provide a stable way to obtain `ByteRange` for any `NodeRef` or concrete AST node.

 - **Components**
   - `SpanDb` (per file): `NodeId -> ByteRange` + optional `TextRange`.
   - `NodeId`: opaque stable identifier for nodes during a single analysis run.
     - Implementation options:
       - Pointer identity hash of underlying AST node (stable within a run and traversal).
       - Or, parser-side arena allocation assigning sequential IDs during AST construction.
     - Chosen path: start with pointer-identity hash via `NodeRef` to avoid AST changes; migrate to arena IDs later if needed.
   - `SpanProvider` trait exposed by analysis:
     ```rust
     pub trait SpanProvider {
         fn span_of(&self, node: &NodeRef) -> Option<(usize, usize)>; // (start, len)
     }
     ```
   - `AnalysisSession` gains `spans_db: SpanDb` (replacing raw `SpanTable` for rules). Back-compat accessor remains temporarily.

 - **Parser integration**
   - Maintain `Spanned<T>` in `bsharp_parser` as it is.
   - During parse, build `(CompilationUnit, SpanDb)` instead of `(CompilationUnit, SpanTable)` by pairing nodes to their spans when the AST is constructed or immediately after (bridge phase).
   - For nodes created without direct parser span (synthesized nodes), map to nearest enclosing span where reasonable.

 - **`HasSpan` usage**
   - Extend analysis helper to resolve span for any `NodeRef` via `session.span_of(node)`.
   - For rules that have direct access to a concrete AST node type (e.g., `&MethodDeclaration`), provide `at(node)` convenience:
     - `at(node)` internally forms a `NodeRef::from(node)` and queries `SpanDb`.

 - **Remove string keys**
   - Rewrite helpers like `find_ctor_span`/`find_method_span` to use typed nodes:
     - Inside `on::<ClassBodyDeclaration::Constructor>` handlers, call `at(ctor)` directly.

 ---

 ## Phase 3 — Analyzer Authoring Macros and Typed Visits
 - **Goals**
   - Replace verbose `Rule` implementations with declarative macros and typed hooks.
   - Keep a single traversal per phase, dispatching to many rules.

 - **Macros**
   - `ruleset!` for grouping:
     ```rust
     ruleset!("semantic",
         CtorNoAsync,
         CtorNameMatchesClass,
         MethodNoStaticOverride,
         AsyncReturnsTask,
     );
     ```
   - `rule!` for individual rules:
     ```rust
     rule! {
       id: "semantic.ctor.no_async",
       category: Semantic,
       on bsharp_syntax::declarations::ClassBodyDeclaration::Constructor as ctor => |cx| {
           if cx.ctor.modifiers.contains(&Modifier::Async) {
               diag!(cx.session, BSE01001, at cx.ctor);
           }
       }
     }
     ```
     - `cx` provides: `session`, `node`-specific fields (e.g., `ctor`), and `span_of`/`emit` helpers.
     - Additional hooks: `on_enter<T>`, `on_exit<T>`, and `filter` predicates.

 - **Typed visitor glue**
   - Generate a `Visit` implementation that converts `NodeRef` to the target node type when possible and calls the rule body.
   - Errors in downcast should never be silent; no-ops are acceptable when node type does not match.

 - **Rule dependencies**
   - Optional macro attribute for declaring required artifacts:
     ```rust
     #[requires(artifact = Symbols, phase = Semantic)]
     rule! { /* ... */ }
     ```
   - The generated code performs runtime checks (`session.get_artifact::<Symbols>()`). If missing, it should return early; not emit diagnostics that depend on missing context.

 ---

 ## Phase 4 — Pass/Artifact API Hardening
 - **Goals**
   - Make artifacts strongly typed and easy to fetch; reduce boilerplate.

 - **Improvements**
   - Add `artifact!` derive macro for new artifacts, registering type name & debug summary.
   - `AnalysisSession` helper methods:
     - `session.require::<T>() -> Arc<T>` returns artifact or panics with clear message (internal bug) — only for rules that declare `#[requires(T)]`.
     - `session.try_get::<T>() -> Option<Arc<T>>` for optional artifacts.
   - Keep `ArtifactStore` as is; this is API sugar and safety around it.

 ---

 ## Phase 5 — Reimplement Baseline Semantic Rules
 - **Targets (from docs/development/semantic-rules.md)**
   - Migrate currently implemented constructor/method semantics to the macro style:
     - BSE01001 (async ctor), BSE01005 (ctor name mismatch), BSE01003 (ctor virtual/abstract),
       BSE02001 (abstract method with body), BSE02006 (static override), BSE02009 (async returns Task/Task<T>).
   - For each newly implemented rule from the table:
     - Add its entry to the `diagnostics!` macro.
     - Implement with a typed `on <NodeType>` handler.
     - Use `diag!(..., at node)` instead of manual span lookup.

 - **Patterns**
   - Prefer `on` hooks closest to the node of interest to avoid walking unrelated parts.
   - For cross-node checks (e.g., override resolution), fetch artifacts (`Symbols`, `Binding`, `Types`) produced by semantic passes.

 ---

 ## Phase 6 — CLI/Reporting Integration
 - Preserve `AnalyzerPipeline` phase ordering.
 - Ensure `AnalyzerPipeline::sort_diagnostics` remains stable.
 - Pretty formatting stays under `diagnostics/format.rs`; update only if span model changes affect caret length/placement.
 - JSON schema of `AnalysisReport` remains stable; category/severity derive from generated code.

 ---

 ## Phase 7 — Tests and Validation
 - **Unit tests**
   - `diagnostics!` macro expansion: enum count, message mapping, category/severity per code.
   - `diag!` macro: default message vs custom message, severity override from config.
   - Span resolution: `SpanDb::span_of(NodeRef)` returns expected byte ranges for representative nodes.
 - **Rule tests**
   - Golden tests per rule using small code samples; assert emitted diagnostics (code, message, location) in stable order.
   - Negative tests to ensure no false positives.
 - **Integration**
   - End-to-end file analysis via `AnalyzerPipeline::analyze_file_report`; snapshot of diagnostics for curated fixtures.

 ---

 ## Phase 8 — Migration and Cleanup
 - Replace `SpanTable` usage in rules with `SpanDb`.
 - Remove ad-hoc helpers `find_ctor_span`/`find_method_span` and string concatenations for span keys.
 - Delete or gate old manual `diagnostic_code.rs` behind a feature while the proc-macro version bakes; then remove.
 - Convert `rules/semantic.rs` and `rules/naming.rs` to macro-based rulesets incrementally; keep mixed mode temporarily if needed.
 - Document authoring guidelines:
   - Add a `CONTRIBUTING.md` section for writing rules using `rule!`/`ruleset!` and `diag!`.

 ---

 ## API Sketch (Consolidated)
 - **Span API**
   ```rust
   // session additions
   impl AnalysisSession {
       pub fn span_of(&self, n: &NodeRef) -> Option<(usize, usize)>;
       pub fn at(&self, n: &NodeRef) -> Option<SourceLocation>; // uses ctx.location_from_span
   }
   ```

 - **Diagnostics helpers**
   ```rust
   // builder alternative via macro
   diag!(session, BSE03011, at node, msg: "Duplicate symbol");
   ```

 - **Rule authoring**
   ```rust
   ruleset!("semantic", CtorNoAsync, MethodNoStaticOverride);

   rule! {
       id: "semantic.method.no_static_override",
       category: Semantic,
       on bsharp_syntax::declarations::ClassBodyDeclaration::Method as m => |cx| {
           if m.modifiers.contains(&Modifier::Static) && m.modifiers.contains(&Modifier::Override) {
               diag!(cx.session, BSE02006, at m);
           }
       }
   }
   ```

 ---

 ## Risks and Mitigations
 - **NodeId stability**: Pointer-identity approach is stable only within a process. This is acceptable for analysis runs; if we need persistence, switch to parser-assigned arena IDs.
 - **Macro complexity**: Keep macro DSL focused; avoid over-abstracting. Start with `on <ExactNodeType>` and add filters later.
 - **Performance**: Single traversal with many rules remains; per-node dispatch via small Vec of rules is cheap and already present.

 ---

 ## Immediate Next Steps
 - **Diagnostics**: Implement `bsharp_diagnostics_macros` and replace `diagnostic_code.rs` with macro-generated content.
 - **Spans**: Introduce `SpanDb`, add `AnalysisSession::span_of`, bridge existing `SpanTable` to `SpanDb` for initial runs.
 - **Ergonomics**: Add `diag!`, `ruleset!`, `rule!` macros; port the 6 existing semantic rules as exemplars.
 - **Tests**: Add golden tests for migrated rules and macro helpers.

