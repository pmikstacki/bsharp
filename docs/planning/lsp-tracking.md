# LSP Module: Tracking and Milestones

This document tracks progress for the BSharp LSP implementation. It mirrors the repository structure and links to relevant modules and tests.

## Scope

- Core server and state management
- Text synchronization and diagnostics
- Navigation (Go to Definition)
- Hover
- Completion
- Document symbols and workspace symbols
- Workspace loading and indexing

## Milestones

- Core Infrastructure
  - [ ] Add dependencies (tower-lsp, lsp-types, tokio, dashmap)
  - [ ] Create `src/lsp/` skeleton (server, capabilities, handlers, state, conversion)
  - [ ] Add `bsharp lsp` CLI subcommand

- Text Synchronization & Diagnostics
  - [ ] didOpen: parse + index + diagnostics
  - [ ] didChange: incremental updates + diagnostics
  - [ ] didClose: cleanup
  - [ ] Diagnostic conversion (BSharp → LSP)
  - [ ] Tests: unit + integration

- Symbols & Navigation
  - [ ] Per-file symbol indexing (IndexingPass)
  - [ ] Workspace symbol cache
  - [ ] Go to Definition
  - [ ] Document symbols (hierarchical)
  - [ ] Workspace symbols (query)
  - [ ] Tests: navigation, cross-file, performance

- Code Intelligence
  - [ ] Hover (signatures, docs)
  - [ ] Completion (keywords, types, members)
  - [ ] Tests: relevance and correctness

- Workspace Support
  - [ ] Initialize workspace from rootUri (.sln/.csproj)
  - [ ] SourceMap integration
  - [ ] Background indexing
  - [ ] Tests: multi-project workspaces

## References

- Parser: `src/parser/facade.rs`, `src/syntax/ast.rs`
- Analysis: `src/analysis/framework/`, `src/analysis/passes/`
- Symbols: `src/analysis/artifacts/symbols.rs`
- Workspace: `src/workspace/`
- Tests: `tests/` (parser, analysis, integration)

## Test Matrix

- Document lifecycle: open/change/save/close
- Diagnostics: parse errors, analysis warnings, severities, related info
- Navigation: class/method/field, cross-file, unresolved symbols
- Hover: classes, methods, variables, keywords
- Completion: keywords, type names, member access
- Symbols: document hierarchy, workspace fuzzy search

## Quality Gates

- >80% coverage for `src/lsp/`
- No panics on malformed input
- Concurrent requests are thread-safe
- Memory usage bounded for large workspaces

## Status

- Current: Planning and architecture
- Next: Implement core server skeleton and document store
