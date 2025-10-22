# Compliance Overview

```admonish warning
This Compliance section is a work in progress. Content, mappings, and assertions may evolve and change between versions.
```

This section describes the Roslyn compliance effort for the C# parser, using our Rust-based `bsharp_parser` and the `bsharp_syntax` AST. The goal is to automatically extract structural assertions from Roslyn tests and validate that our AST shape and key payloads match Roslyn’s expectations (normalized to our naming conventions: PascalCase, no "Syntax" suffix).

## High-Level Flow

- Source: Roslyn test files in `roslyn_testing/roslyn_repo/src/Compilers/CSharp/Test/Syntax/Parsing/`.
- Extraction: A generator scans for `UsingTree(...)` blocks and parses the following DSL of `N(SyntaxKind.X)` nodes.
- Translation: The extracted Roslyn tree is translated and normalized to our canonical kinds and structure.
- Running: Tests are emitted into `bsharp_compliance_testing`, parsing provided C# snippets with `bsharp_parser` and comparing the actual AST with the expected structure.

## Core Components

- `bsharp_compliance` (generator)
  - Reads Roslyn files and extracts structural expected trees.
  - Parses the Roslyn DSL (`N(SyntaxKind.X)`, `M(...)`, `EOF()`).
  - Normalizes kinds via `kind_map.rs` (e.g., `RecordStructDeclaration` → `RecordDeclaration`).
  - Emits Rust tests into `bsharp_compliance_testing/src/generated/`.

- `bsharp_compliance_testing` (tests & asserts)
  - Contains custom structural assertions in `custom_asserts/structure_assert.rs`.
  - Walks real `bsharp_syntax` nodes to build a comparable `ExpectedTree`.
  - Compares node kind shapes and selected token payloads (e.g., identifier text).

## Normalization Principles

- Node names are PascalCase and omit Roslyn’s `...Syntax` suffix.
- Tokens/keywords are filtered from structure; identifier text is lifted where relevant.
- Harness differences (Roslyn’s class-with-method wrappers vs. our top-level statements) are normalized at assert time when needed.

## What This Validates

- Structural presence and order of major nodes (CompilationUnit, declarations, using directives, type parameters, constraint clauses, etc.).
- Selected payloads (e.g., `IdentifierName.token_value`).
- Deeper constructs incrementally (e.g., `TypeParameterConstraintClause`, “allows ref struct” constraints, record primary parameter lists).

## Roadmap

- Expand kind mapping and walker coverage across more Roslyn suites.
- Tighten token payload checks where meaningful.
- Add targeted hand-authored structure tests for corner cases.
