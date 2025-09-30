# BSharp vNext Vision

This document outlines the strategic vision for the next major iteration (vNext) of BSharp. It connects the current state of the project with upcoming feature plans and sets clear goals for functionality, quality, performance, and developer experience.

## Objectives

- **First-class IDE Experience**: Ship a robust LSP for diagnostics, navigation, and code intelligence across popular editors.
- **Deeper Analysis**: Expand the analysis framework with richer semantic understanding and actionable insights.
- **Ecosystem Integration**: Integrate with .NET workflows via CIL read/write support and project system alignment.
- **Performance & Scale**: Handle large solutions efficiently with incremental parsing and indexing.
- **Extensibility**: Make it easy to add analyzers, passes, and code actions.

## Pillars

- **Reliable Parsing**: Maintain near-complete C# coverage with strong error handling and recovery.
- **Actionable Diagnostics**: Provide clear diagnostics with related information and quick fixes (future).
- **Smart Navigation**: Accurate symbol discovery, cross-file navigation, and rich symbol views.
- **Workspace-Aware**: Understand .sln/.csproj, project references, and conditional compilation.
- **Open & Documented**: High-quality docs and test coverage; easy to adopt and contribute to.

## Feature Tracks

### 1. Language Server Protocol (LSP)
- Core server with `tower-lsp`.
- Text synchronization, diagnostics, hover, completion, go to definition.
- Document and workspace symbols.
- Future: semantic tokens, code actions, inlay hints, rename.

See: `docs/planning/lsp-module-plan.md` and `docs/planning/lsp-tracking.md`.

### 2. Dotscope CIL Integration
- Read/write CIL to bridge with .NET runtime artifacts.
- Enable deeper semantic and IL-level analysis.
- Provide pathways for transformation and code generation round-trips.

See: `docs/planning/dotscope-cil-plan.md` and `docs/planning/dotscope-cil-tracking.md`.

### 3. Analysis Framework Enhancements
- Expand `AnalyzerPass` coverage for semantic checks and code quality rules.
- Improve metrics and control-flow analysis outputs.
- Provide stable diagnostic codes and configuration surface.

### 4. Workspace & Build System Alignment
- Robust `.sln`/`.csproj` handling including conditional compilation symbols.
- Smarter file discovery and change tracking.
- Incremental re-indexing and cache invalidation.

### 5. Developer Experience
- CLI improvements (LSP entrypoint, profiling aids).
- Better error messages and troubleshooting guides.
- Consistent naming conventions (PascalCase AST node names without `Syntax`).

## Milestones Overview

- **LSP GA**: Core features stable, cross-editor setup documented, high test coverage.
- **CIL Bridge Alpha**: Dotscope integration behind feature flag with round-trip demos.
- **Analyzer Packs**: A set of useful analyzers enabled by default with clear docs.
- **Workspace Scale**: Proven performance on large solutions with incremental updates.

## Quality & Performance Targets

- Robust under malformed input; no panics.
- Responsive interactions (<50ms for common requests where feasible).
- Incremental parsing/indexing to avoid full recomputation.
- >80% coverage in new modules; critical paths benchmarked.

## Documentation & Book Integration

- All planning documents live in `docs/planning/` and are included in the book.
- Each feature track has a plan and a tracking page with milestones.
- Cross-links to relevant modules and tests.

## Call for Contributions

- Proposals for analyzers, diagnostics, and code actions.
- Performance profiling and benchmarks on real-world solutions.
- Editor configuration examples and community templates.

---

This vision provides the guiding structure for vNext. Detailed implementation plans and progress tracking are linked above.
