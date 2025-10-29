
# Summary

[Introduction](./README.md)

# Parser Architecture

- [Overview](./parser/overview.md)
- [Core Components](./parser/core-components.md)
- [AST Structure](./parser/ast-structure.md)
- [Error Handling](./parser/error-handling.md)

# Parser Modules

- [Expression Parsing](./parser/expressions.md)
- [Statement Parsing](./parser/statements.md)
- [Declaration Parsing](./parser/declarations.md)
- [Type System](./parser/types.md)
- [Feature Completeness](./parser/feature-completeness.md)
- [Keywords and Tokens](./parser/keywords-and-tokens.md)

# Advanced Features

- [Query API](./parser/navigation.md)
- [Comment Parsing](./parser/comments.md)
- [Preprocessor Directives](./parser/preprocessor.md)

# Syntax

- [Spans](./syntax/spans.md)
- [Traits](./syntax/traits.md)
- [Derive Macros](./syntax/derive-macros.md)
- [Formatter and Emitters](./syntax/formatter.md)

# Analysis Framework

- [Analysis Overview](./analysis/overview.md)
- [Analysis Pipeline](./analysis/pipeline.md)
- [Traversal Guide](./analysis/traversal-guide.md)
- [Control Flow Analysis](./analysis/control-flow.md)
- [Dependency Analysis](./analysis/dependencies.md)
- [Metrics Collection](./analysis/metrics.md)
- [Type Analysis](./analysis/types.md)
- [Code Quality](./analysis/quality.md)
- [Passes and Rules](./analysis/passes-and-rules.md)
- [Report Schema](./analysis/report-schema.md)
- [Writing a Pass](./analysis/writing-a-pass.md)
- [Writing a Ruleset](./analysis/writing-a-ruleset.md)

# CLI Tools

- [Command Line Interface](./cli/overview.md)
- [Parse Command](./cli/parse.md)
- [Tree Visualization](./cli/tree.md)
- [Analysis Command](./cli/analyze.md)
- [Format Command](./cli/format.md)
- [Parse Errors JSON](./cli/errors-json.md)

# Workspace

- [Workspace Loading](./workspace/overview.md)

# Configuration

- [Configuration Overview](./configuration/overview.md)

# Development

- [Contributing](./development/contributing.md)
- [Testing](./development/testing.md)
- [Architecture Decisions](./development/architecture.md)
- [Cookbooks](./development/cookbooks.md)
  - [Query Cookbook](./development/query-cookbook.md)
  - [Parser Cookbook](./development/parser-cookbook.md)
- [Writing Tests](./development/writing-tests.md)
- [bsharp_tests Overview](./development/bsharp_tests.md)
- [Extending Syntax (New Nodes)](./development/extending-syntax.md)
- [Writing Parsers](./development/writing-parsers.md)
- [Spanned-first Parsers](./development/spanned-parsers.md)
- [Compliance](./development/compliance/README.md)
  - [Overview](./development/compliance/overview.md)
  - [Compliance Guide](./development/compliance/compliance_guide.md)
  - [Generator](./development/compliance/Generator.md)

# Research & Development

## CIL Runtime 

- [CIL Runtime](./cil-runtime/README.md)
  - [Overview](./cil-runtime/overview.md)
  - [Architecture](./cil-runtime/architecture.md)
  - [Phases](./cil-runtime/phases/README.md)
    - [Phase 0: Dotscope Spike](./cil-runtime/phases/00-dotscope-spike.md)
    - [Phase 1: VM MVP](./cil-runtime/phases/01-vm-mvp.md)
    - [Phase 2: Control Flow + EH](./cil-runtime/phases/02-control-flow-and-eh.md)
    - [Phase 3: Object Model and Arrays](./cil-runtime/phases/03-object-model-and-arrays.md)
    - [Phase 4: Back-Compat Polish](./cil-runtime/phases/04-back-compat-polish.md)
    - [Phase 5: AST→IL Emitter](./cil-runtime/phases/05-ast-to-il-emitter.md)
  - [dotscope Guide](./cil-runtime/dotscope-guide.md)
  - [VM Design](./cil-runtime/vm-design.md)
  - [Emitter Design](./cil-runtime/emitter-design.md)
  - [Testing & Conformance](./cil-runtime/testing.md)
  - [Roadmap](./cil-runtime/roadmap.md)
  - [Open Questions](./cil-runtime/open-questions.md)
  - [Glossary](./cil-runtime/glossary.md)

