# BSharp LSP Module: Detailed Implementation Plan

This document outlines the full plan for implementing a Language Server Protocol (LSP) module for BSharp, reusing existing components to avoid duplication.

## Goals

- Provide a production-grade LSP server for C# using BSharp’s parser and analysis.
- Zero duplication: reuse existing parser, analysis, workspace, and symbol indexing.
- Support essential IDE features: diagnostics, navigation, hover, completion, document/workspace symbols.
- Be extensible: enable advanced features (semantic tokens, code actions, inlay hints) incrementally.

## Crate Choice

- Use `tower-lsp` as the LSP framework.
  - Async-friendly, widely adopted, supports proposed features.
  - Strong typing via `lsp-types`.

Dependencies to add in `Cargo.toml`:

```toml
[dependencies]
tower-lsp = "0.20"
lsp-types = "0.95"
tokio = { version = "1.35", features = ["full"] }
dashmap = "5.5"
```

## Architecture

```
src/
└── lsp/
    ├── mod.rs
    ├── server.rs                 # Backend implementing LanguageServer
    ├── capabilities.rs           # Server capabilities
    ├── handlers/
    │   ├── mod.rs
    │   ├── text_document.rs      # open/change/close/save, documentSymbol
    │   ├── workspace.rs          # workspaceSymbol, didChangeConfiguration
    │   ├── goto_definition.rs    # definition provider
    │   ├── hover.rs              # hover provider
    │   ├── completion.rs         # completion provider
    │   └── diagnostics.rs        # publishing diagnostics
    ├── state/
    │   ├── mod.rs
    │   ├── document_store.rs     # in-memory text + AST + spans + diags
    │   ├── workspace_state.rs    # loaded .sln/.csproj, SourceMap
    │   └── symbol_cache.rs       # per-file + workspace symbol indices
    └── conversion/
        ├── mod.rs
        ├── position.rs           # Position↔offset, Range↔span
        ├── diagnostics.rs        # BSharp→LSP diagnostics
        └── symbols.rs            # BSharp→LSP symbols
```

## Reuse (No Duplication)

- Parser: `Parser::parse_with_spans()` from `src/parser/facade.rs`
- AST: `CompilationUnit` and nodes from `src/syntax/ast.rs` and `src/syntax/nodes/`
- Analysis: `AnalysisSession`, passes in `src/analysis/framework/` and `src/analysis/passes/`
- Symbols: `SymbolIndex` in `src/analysis/artifacts/symbols.rs` + `IndexingPass`
- Workspace: `WorkspaceLoader` and `SourceMap` in `src/workspace/`
- Navigation: `AstNavigate`, `FindDeclarations` in `src/analysis/navigation/`
- Locations: `AnalysisContext::location_from_span()` in `src/analysis/context.rs`

## Capabilities

- textDocumentSync: full
- diagnostics: push on open/change/save
- definitionProvider: true
- documentSymbolProvider: true (hierarchical)
- workspaceSymbolProvider: true
- hoverProvider: true
- completionProvider: keywords, types, members
- semanticTokensProvider: planned (optional)
- codeActionProvider: planned (optional)
- inlayHintProvider: planned (optional)

## Document Lifecycle

1. didOpen: cache text, parse + index, publish diagnostics
2. didChange: apply edits, re-parse + re-index incrementally, publish diagnostics
3. didClose: drop per-document caches, keep workspace index if needed
4. didSave: optional re-analysis

`DocumentStore` (DashMap):
- version, text, ast (CompilationUnit), span_table, diagnostics

## Diagnostics

- Convert parse and analysis diagnostics to LSP diagnostics.
- Map severity using `DiagnosticSeverity`.
- Use `location_from_span()` + LSP range conversion.
- Include related information where available.

## Navigation (Go to Definition)

- Use span table and AST to detect symbol at cursor.
- Resolve to definition via `SymbolIndex` (per-file) + workspace cache.
- Return `Location` with exact range.

## Hover

- Show signatures and summary documentation when available.
- For classes/structs/interfaces: show declaration header.
- For methods: return method signature and XML doc summary if present (`xml_documentation`).

## Completion

- Context analysis from AST + offset:
  - Member access: list members (future: requires type info; start with symbol-name completion and keywords).
  - Type positions: symbols of kinds Class/Interface/Struct/Enum/Delegate.
  - Keywords/snippets: provide keyword list and common snippets (if allowed).

## Document & Workspace Symbols

- Document symbols: hierarchical via `FindDeclarations` with node ranges.
- Workspace symbols: aggregate symbol names and kinds from workspace index.

## Workspace Initialization

- On initialize: detect rootUri, use `WorkspaceLoader` to load .sln/.csproj.
- `SourceMap::from_paths()` for canonical file list.
- Parse + index in background (throttled) and stream diagnostics per file.

## CLI Entrypoint

- Add `bsharp lsp` subcommand to start stdio server (optional TCP for debugging).

## Testing

- Unit tests per handler under `tests/lsp/`.
- Integration tests for:
  - edit→diagnostics, cross-file definition, workspace search, concurrent requests.
- Fixtures: reuse `tests/fixtures` and `tests/cs_test_cases`.

## Future Extensions

- Semantic Tokens: identifiers by kind using symbol index.
- Code Actions: quick fixes for common diagnostics.
- Inlay Hints: parameter names, `var` types.
- Rename: full-project rename (requires references index).

## Acceptance Criteria

- Real-time diagnostics on edit.
- Accurate definitions across files.
- Useful hover info for types/methods.
- Relevant completion items in common contexts.
- Workspace symbols query returns expected results.
- Thread-safe and responsive under typical workloads.
