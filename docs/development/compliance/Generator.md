# Generator

```admonish warning
This Compliance section is a work in progress. Content, mappings, and assertions may evolve and change between versions.
```

This document describes how the Roslyn structure test generator works in `bsharp_compliance` and how it produces executable tests for `bsharp_compliance_testing`.

## Inputs

- Roslyn source files under `roslyn_testing/roslyn_repo/src/Compilers/CSharp/Test/Syntax/Parsing/`.
- The generator scans for `UsingTree(...)` calls and parses the immediately following Roslyn structure DSL composed of `N(SyntaxKind.X)` and `EOF()` entries (with `M(...)` ignored as "missing").

## Pipeline

1. Scan and collect test methods
   - Locates Roslyn `[Fact]` methods and all `UsingTree(...)` call sites.
   - Captures the closest preceding `var text = "...";` snippet as input source, when present.

2. Parse structure DSL
   - Reads the DSL block following `UsingTree(...)` and constructs a nested `ExpectedTree` (`ExpectedNode` graph) mirroring the Roslyn node hierarchy.
   - Tolerates whitespace, comments, and missing markers (`M(...)`).

3. Kind translation and normalization
   - `generator/kind_map.rs` maps Roslyn kinds to our canonical naming (PascalCase, no `Syntax` suffix).
   - Filters token/keyword nodes, lifting identifier text (`IdentifierToken` → parent `IdentifierName.token_value`).
   - Applies targeted renames (e.g., `RecordStructDeclaration` → `RecordDeclaration`).

4. Emit tests
   - Writes Rust tests into `bsharp_compliance_testing/src/generated/<module>.rs`.
   - Each test parses the captured `src` with `bsharp_parser` and asserts structure via `custom_asserts/structure_assert.rs`.

## Assertions

- Structure assertions build a comparable expected tree from our actual AST (`bsharp_syntax`) and compare:
  - Node kinds and order
  - Selected token payloads (e.g., `IdentifierName.token_value`)
- Normalization in the assert layer adapts Roslyn’s harness (class + method) to our top-level statements when applicable.

## Extending the Generator

- Update `generator/kind_map.rs` to add or refine kind mappings.
- Expand `custom_asserts/structure_assert.rs` to walk deeper AST areas (e.g., records, types, constraints).
- Improve the DSL parser (`generator/structure_dsl.rs`) as new Roslyn DSL shapes appear.

## Output Location

- Generated files live under `roslyn_testing/bsharp_compliance_testing/src/generated/`.
- Modules track Roslyn file groups, e.g. `record_parsing.rs`, `using_directive_parsing_tests.rs`.
