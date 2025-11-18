# Semantic Analysis and Diagnostics Implementation Plan

This plan outlines how we will implement semantic errors and warnings using our macro-driven diagnostics and migrate rules incrementally while keeping the workspace green.

## Current state (codebase)

Macro platform

*   `bsharp_diagnostics_macros` provides:
    *   `diagnostics!` that generates `DiagnosticCode` enum plus `as_str()`, `default_message()`, `severity()`, `category()`.
    *   `diagnostic_enum!` to emit enums with `as_str()`.
    *   `enum_as_str!` for simple enum-to-string mappings (used in `bsharp_syntax::types::CallingConvention`).
*   `bsharp_analysis::diagnostics::diagnostic_code.rs` now uses a single `diagnostics!` invocation (no hand-written tables).

Analyzer framework

*   Phases and passes remain (`Index`, `LocalRules`, `Global`, `Semantic`, `Reporting`, etc.) with `AnalyzerPipeline` orchestrating them.
*   `AnalyzerRegistry` registers core passes and rulesets; rules are still a mix of legacy and newer style.

Spans

*   `AnalysisSession` initializes `span_db` (bridge built from parser spans). Typed span access is available via the session; string-keyed lookups are being phased out.

Implemented rules (documented)

*   From `docs/development/semantic-rules.md` (Status: Implemented):
    *   CS0101 → BSE03011 Duplicate symbol
    *   CS0102 → BSE03011 Duplicate symbol
    *   CS0103 → BSE03012 Unresolved or ambiguous name
    *   CS0104 → BSE03012 Unresolved or ambiguous name

Build status

*   Workspace builds. Some crates emit warnings unrelated to semantic diagnostics (to be cleaned separately per cleanup guide).

---

## Guiding principles

*   Macro-first: diagnostics defined once via `diagnostics!`; rules emit diagnostics via helpers, not manual tables.
*   Small steps: keep each phase compilable and testable.
*   Typed spans everywhere: avoid string-keyed span lookups in rules.
*   Prefer return-early logic and explicit errors; avoid hidden control flow.

---

## Phase 0 — Inventory and scoping (planning)

*   Extract/prioritize groups from `docs/development/CSharpErrorsAndWarnings.md` into `docs/development/semantic-rules.md` with B# code assignments and statuses.
*   Ensure every row in `semantic-rules.md` has a B# code family consistent with our scheme:
    *   Errors: `BSE01***` constructors, `BSE02***` methods/overrides, `BSE03***` types/symbols/generics, `BSE04***` accessibility/modifiers.
    *   Warnings: `BSW01***` maintainability, `BSW02***` style, `BSW03***` performance, `BSW04***` security.
*   Add or amend entries in the central `diagnostics!` invocation accordingly (messages are the source of truth).

Deliverables

*   Updated `semantic-rules.md` rows with B# codes and Status.
*   `diagnostics!` contains all referenced B# codes with messages.

---

## Phase 1 — Diagnostics platform hardening (macro utilities)

*   Add helpers to `DiagnosticCode` (if/when needed by tooling):
    *   `parse_code(&str) -> Option<DiagnosticCode>`
    *   `all_codes() -> &'static [DiagnosticCode]`
*   Introduce `diag!` helper macro for concise emission:
    *   `diag!(session, BSE02009, at node);`
    *   Optional overrides: `msg:`, `related:`.
*   Keep severity/category derived from code prefix (already implemented).

Deliverables

*   `diag!` macro available in analysis crates.
*   Unit tests for macro expansions (basic sanity: mapping count, messages, category/severity).

---

## Phase 2 — Typed spans and rule ergonomics

*   Stabilize `SpanDb` accessors in `AnalysisSession`:
    *   `span_of(&NodeRef)`, `at(&NodeRef)` helpers for location.
*   Remove remaining string-keyed span helpers from rules; switch to typed lookups.
*   Prepare typed visit ergonomics for rules (macro-based `rule!`/`ruleset!` in a later phase).

Deliverables

*   Rules no longer rely on string-keyed span tables.
*   Tests for span resolution against representative nodes.

---

## Phase 3 — Core semantic correctness rules (high value)

Implement or finalize these first, using typed hooks and `diag!`:

*   Symbols/names (BSE0301x–BSE03012)
    *   Duplicates, ambiguous/unresolved names, type not found.
*   Methods/overrides (BSE0200x)
    *   Abstract method with body; static cannot override; override target not found; signature mismatches.
*   Constructors (BSE0100x)
    *   Async ctor not allowed; name must match type; invalid base ctor call; interface cannot have constructors.
*   Accessibility (BSE0400x)
    *   Inaccessible member; visibility mismatches; abstract/virtual/sealed conflicts.

Deliverables

*   Each rule emits the appropriate B# code from `diagnostics!`.
*   Golden tests per rule with small fixtures; verify code, message, and location.

---

## Phase 4 — Type system and generics constraints

*   Enforce generic arity/constraints, circular dependencies, invalid inheritance relations, type argument mismatch.
*   Representative CS codes: CS0304–CS0315, CS0450–CS0457 mapped to `BSE0300x/BSE0304x` as per table.

Deliverables

*   Rules implemented with artifact access to Symbols/Types where necessary.
*   Tests covering positive/negative cases.

---

## Phase 5 — Modifiers and accessibility consistency

*   Inconsistent accessibility across members and accessors.
*   Invalid combinations: abstract/private, sealed/non-override, static/virtual, etc.
*   Map to `BSE0400x` family.

Deliverables

*   Rules with clear, actionable messages grounded in our `diagnostics!` defaults.

---

## Phase 6 — Flow analysis basics

*   Not all code paths return a value (CS0161), fall-through between case labels (CS0163), definite assignment (CS0165/CS0170/CS0171).
*   Consider minimal flow engine or reuse existing passes; prefer precise checks over noisy heuristics.

Deliverables

*   Deterministic diagnostics with minimal false positives; unit tests for edge cases.

---

## Phase 7 — Async/await and interop-specific semantics

*   Async returns Task/Task (BSE02009), invalid throw usage, invalid base usage in ctors, DllImport constraints, unmanaged calling convention validation when relevant.

Deliverables

*   Tests with async constructs and interop samples.

---

## Phase 8 — Warnings: maintainability, style, performance, security

*   Maintainability (BSW01\*\*\*): large/complex methods, duplication basics where available.
*   Style (BSW02\*\*\*): naming conventions, unused variables/parameters (where we can do reliably).
*   Performance (BSW03\*\*\*): obvious anti-patterns (boxing in hot loops, string concatenation in loops).
*   Security (BSW04\*\*\*): basic patterns (hardcoded credentials, unsafe deserialization) gated behind clear heuristics.

Deliverables

*   Rules added gradually; each has targeted tests. Warnings are opt-down via config where applicable.

---

## Phase 9 — CLI/reporting and coverage tracking

*   Keep `AnalyzerPipeline` ordering stable; reporting unaffected.
*   Track coverage directly from `semantic-rules.md` (Status column) and `all_codes()`.
*   Add a CI check that all B# codes defined in `diagnostics!` are exercised by at least one unit test (incremental goal).

---

## Working agreements

*   Update `semantic-rules.md` when adding a new rule: set B# code and Status.
*   Add diagnostics only via `diagnostics!`; do not hand-write mapping code.
*   Prefer small PR-sized changes; after each batch, run workspace build and tests.