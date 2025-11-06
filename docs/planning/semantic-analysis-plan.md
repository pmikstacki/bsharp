# Semantic Analysis Plan (BSharp)

This document lays out a phased implementation plan for semantic analysis across all C# language structures in the `bsharp_analysis` crate. It is organized to integrate cleanly with the current `AnalyzerPipeline` phases and reporting.

- Design goals
- Phased rollout overview
- Detailed phase specs (inputs, outputs, diagnostics, data structures, tests)
- Language structure coverage matrix
- Integration with AnalyzerPipeline and diagnostics
- Performance, determinism, and testing strategy

## Design Goals
- Deterministic, incremental-friendly passes with explicit inputs/outputs.
- Clear ownership boundaries: parser (syntax), analysis (semantics), reporting (diagnostics/artifacts).
- One responsibility per pass; explicit ordering via labels.
- Reuse and extend existing artifact stores and diagnostic pipeline.
- Support both strict and lenient parsing modes (recovery-aware diagnostics where possible).

## Phased Rollout Overview
We’ll map to the existing pipeline stages while keeping passes fine-grained and composable.

- Phase 0: Syntax artifacts (already present)
- Phase 1: Symbol indexing (types, members, namespaces)
- Phase 2: Name binding and scope resolution
- Phase 3: Type checking, conversions, and overload resolution
- Phase 4: Generics and constraints validation
- Phase 5: Flow analysis: definite assignment, reachability, variable capture
- Phase 6: Nullability and flow state (NRT)
- Phase 7: Attributes, constants, and compile-time evaluation
- Phase 8: Accessibility and modifiers validation
- Phase 9: Extension constructs (extension methods today, extension members in `extension {}` blocks)
- Phase 10: Diagnostics aggregation, cross-file checks, and reporting

Each phase defines its resources, diagnostics, and test strategy.

---

## Phase 1: Symbol Indexing
- Goal: Build symbol tables (projects/files) for namespaces, types (class/struct/interface/record/enum/delegate), and members (fields, properties, events, methods, constructors, indexers, operators).
- Inputs: AST `CompilationUnit`; configuration.
- Outputs: `SymbolTable` artifact per file, merged at workspace-level; symbol IDs for later passes.
- Data structures:
  - `SymbolId`, `SymbolKind`, `SymbolTable { scopes, symbols_by_name, parents }`.
  - `TypeKey`, `MemberKey` for stable references.
- Diagnostics: Duplicates (type/member), illegal partial declarations, illegal nested constructs.
- Tests: Unit tests per construct; integration for nested/partial types; cross-file duplicate tests.
- Pipeline: `Phase::Index` (already available). Store artifacts in `ArtifactStore`.

## Phase 2: Name Binding and Scope Resolution
- Goal: Resolve identifier references: types, namespaces, members, using-aliases; build bound trees for members.
- Inputs: AST + `SymbolTable` + using directives.
- Outputs: `BindingTable` mapping AST nodes to symbol/namespace/type refs; per-file bound member trees.
- Data structures: `BoundNode`, `BoundExpr`, `BoundStmt`, `BoundPattern` with symbol links.
- Diagnostics: Unresolved identifiers, ambiguous names, circular usings, invalid alias targets.
- Tests: Positive/negative binding scenarios, namespace aliasing, global using.
- Pipeline: `Phase::Semantic` (binding pass runs before type checking).

## Phase 3: Type Checking, Conversions, and Overload Resolution
- Goal: Compute expression types; implement standard/implicit/explicit conversions; resolve method group invocations and overloads; handle user-defined operators.
- Inputs: Bound trees + `SymbolTable` + type system primitives.
- Outputs: `TypeInfoTable` (per-node types), `ConversionTable`, `CallResolutionTable`.
- Diagnostics: No viable overload, ambiguous call, invalid conversions, boxing/unboxing errors.
- Tests: Overload resolution matrices, extension methods applicability, operator resolution.
- Pipeline: `Phase::Semantic` (after binding).

## Phase 4: Generics and Constraints Validation
- Goal: Validate type/ method type parameters and constraints (`where`), variance, default type args.
- Inputs: Bound trees + `SymbolTable`.
- Outputs: `GenericsCheckReport` per file.
- Diagnostics: Constraint violations, invalid variance, recursive constraints, `new()` misuse.
- Tests: Constraint satisfaction across type arguments, variance edge cases.
- Pipeline: `Phase::Semantic` (after type checking to leverage inferred types where needed).

## Phase 5: Flow Analysis (DA/Reachability/Captures)
- Goal: Definite assignment, definite unassignment, unreachable code, variable capture (closures), async/iterator flow checks.
- Inputs: Bound statements + TypeInfo.
- Outputs: `FlowState` per block, `CaptureInfo` per lambda/local function.
- Diagnostics: Use-of-unassigned, unreachable, missing return, invalid capture (ref/stack), iterator/async constraints.
- Tests: Classic DA suites; branching, loops, try/catch/finally interaction; lambdas and locals.
- Pipeline: `Phase::Semantic` (after type checking).

## Phase 6: Nullability (NRT) Flow State
- Goal: Track nullability annotations and flow; enforce dereference safety and assignment rules.
- Inputs: TypeInfo + FlowState + annotations context (`#nullable`).
- Outputs: `NullabilityState` per expression and variable.
- Diagnostics: Possible null dereference, redundant checks, incorrect suppression.
- Tests: NRT on/off, flow-sensitive examples, attributes that affect nullability.
- Pipeline: `Phase::Semantic`.

## Phase 7: Attributes, Constants, and Compile-time Evaluation
- Goal: Bind attributes and arguments, evaluate constants (const expressions), fold where allowed.
- Inputs: Bound trees + TypeInfo + Binding.
- Outputs: `AttributeTable`, `ConstEvalTable`.
- Diagnostics: Misapplied attributes, invalid const expressions, attribute ctor resolution errors.
- Tests: Attribute targets, global attributes, constant folding edge cases.
- Pipeline: `Phase::Semantic` (after type/binding).

## Phase 8: Accessibility and Modifiers Validation
- Goal: Validate accessibility rules and modifier combinations for types and members.
- Inputs: Symbols + Binding.
- Outputs: `AccessCheckReport`.
- Diagnostics: Inaccessible type/member, illegal modifiers combinations, override/virtual/sealed rules.
- Tests: Cross-assembly (mock), nested types, tricky override chains.
- Pipeline: `Phase::Semantic`.

## Phase 9: Extension Constructs
- Goal: Validate extension methods and upcoming extension members blocks (`extension T { ... }`).
- Inputs: Symbols + Binding + TypeInfo.
- Outputs: `ExtensionAnalysisReport`.
- Rules:
  - Extension methods: static, first parameter `this T`, accessibility rules, shadowing/ambiguity.
  - Extension members (experimental):
    - Members must be static.
    - No instance state (fields) permitted.
    - Receiver type must be non-dynamic; closed generic arguments resolved.
    - Disallow constructors and destructors.
- Diagnostics: Invalid extension receiver, non-static member in extension, conflicts with existing instance members, ambiguity vs. extension methods.
- Tests: Positive and negative cases; ambiguous resolution; recovery when inside extension body (lenient mode).
- Pipeline: `Phase::Semantic` (after type/binding).

## Phase 10: Diagnostics Aggregation and Workspace Reporting
- Goal: Merge per-file reports, stable sort, cross-file diagnostics (e.g., duplicate symbols across files, partial types completeness).
- Inputs: Per-file reports from earlier phases.
- Outputs: Final `AnalysisReport` fields (`diagnostics`, `cfg`, `deps`, metrics).
- Diagnostics: Aggregated and de-duplicated with stable sorting.
- Pipeline: Already implemented in `AnalyzerPipeline::run_workspace*`.

---

## Language Structure Coverage Matrix (Semantics)
- Namespaces & Usings: binding, alias targets, cycles.
- Types: class/struct/interface/record/enum/delegate (symbols, inheritance/implements, constraints).
- Members: fields/properties/events/indexers/operators/methods/constructors/destructors.
- Generics: type/method params, variance, constraints.
- Expressions: types, conversions, overloads, constant folding.
- Statements: DA/reachability, pattern semantics, switch rules (fall-through legality), try/catch/filter.
- Lambdas & Local functions: capture analysis, async/iterator rules.
- Attributes: binding and validation.
- Nullability: flow-sensitive checks.
- Extension constructs: methods and extension blocks (experimental semantics).

---

## Integration with AnalyzerPipeline
- Add a new `semantic` module with subpasses:
  - `symbols`, `binding`, `types`, `overload`, `generics`, `flow`, `nullability`, `attributes`, `access`, `extensions`.
- Register passes under `Phase::Semantic` in explicit order; keep pass functions small and testable.
- Use `ArtifactStore` to persist tables between passes.
- Emit diagnostics via existing `Diagnostic` API; keep messages stable and include precise locations.

---

## Performance & Determinism
- Use stable, deterministic traversal orders and sorting for merged diagnostics.
- Employ interning for type/symbol keys to reduce memory.
- Provide configuration toggles (e.g., enable/disable NRT) via `AnalysisConfig`.

## Testing Strategy
- Unit tests per pass and construct.
- Integration tests per phase ordering.
- Workspace-level tests for cross-file issues.
- Lenient vs. strict mode parity tests (where appropriate), including recovery in extension bodies.

---

## Initial Execution Plan (Iteration Order)
1. Phase 1: Symbol indexing (types/members), partial types merge basics.
2. Phase 2: Binding of identifiers, namespaces, and usings.
3. Phase 3: Type checking + conversions + overload resolution.
4. Phase 4: Generics/constraints validation.
5. Phase 5: Flow analysis for DA/reachability/captures.
6. Phase 6: Nullability.
7. Phase 7: Attributes/const eval.
8. Phase 8: Accessibility/modifiers.
9. Phase 9: Extension constructs (methods + extension blocks semantics).
10. Phase 10: Aggregation checks.

Each step lands with tests and documentation updates.

---

## Ready-to-run Prompt for Next Session
Copy-paste the following prompt to kick off execution of this plan:

"""
Continue with Semantic Analysis implementation in bsharp_analysis:

- Create `src/bsharp_analysis/src/semantic/` with submodules:
  - `symbols.rs`, `binding.rs`, `types.rs`, `overload.rs`, `generics.rs`, `flow.rs`, `nullability.rs`, `attributes.rs`, `access.rs`, `extensions.rs`.
- Wire passes into `AnalyzerPipeline` under `Phase::Semantic` in that order, storing artifacts in `ArtifactStore`.
- Implement Phase 1 (symbols) and Phase 2 (binding) end-to-end with:
  - Artifacts (`SymbolTable`, `BindingTable`).
  - Diagnostics for duplicates and unresolved names.
  - Unit tests in `bsharp_tests/src/analysis/` and incremental workspace tests.
- Add recovery-aware diagnostics for malformed members inside `extension { }` when in lenient mode.
- Update docs: feature matrix and planning per pass as we land work.

Stop after symbols + binding passes are green and documented.
"""
