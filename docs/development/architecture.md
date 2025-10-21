# Architecture Decisions

This document explains the key architectural decisions made in the BSharp project, their rationale, and their implications for contributors.

---

## Core Design Philosophy

BSharp is designed as a **modular, extensible C# parser and analysis toolkit** written in Rust. The architecture prioritizes:

1. **Correctness** - Accurate parsing of C# syntax
2. **Performance** - Efficient parsing and analysis of large codebases
3. **Maintainability** - Clear module boundaries and minimal coupling
4. **Extensibility** - Easy addition of new language features and analyzers

---

## Parser Architecture

### Why nom Parser Combinators?

**Decision:** Use the `nom` parser combinator library as the foundation for parsing.

**Rationale:**
- **Composability**: Small, focused parsers combine to handle complex syntax
- **Type Safety**: Rust's type system catches parser errors at compile time
- **Performance**: Zero-copy parsing with minimal allocations
- **Testability**: Individual parser functions are easily unit tested
- **Maintainability**: Declarative style is easier to understand than hand-written parsers

**Trade-offs:**
- Learning curve for contributors unfamiliar with parser combinators
- Error messages require additional work (addressed with nom-supreme)

**Implementation:**
- Core parsing infrastructure: `src/bsharp_parser/src/helpers/`
- Parser implementations: `src/bsharp_parser/src/`
- All parsers return `BResult<I, O>` type alias

### Error Handling Strategy

**Decision:** Use `nom-supreme::ErrorTree` for all parser errors.

**Rationale:**
- **Rich Context**: Tree structure preserves full parse failure path
- **Better Diagnostics**: Context annotations via `.context()` method
- **Integration**: Seamless integration with nom combinators
- **Debugging**: Pretty-printing via `format_error_tree()`

**Evolution:**
- Initially used custom `BSharpParseError` type
- Migrated to `ErrorTree` for better diagnostics
- Custom error type deprecated and removed

**Implementation:**
```rust
pub type BResult<I, O> = IResult<I, O, ErrorTree<I>>;
```

**Helper Functions (in `src/bsharp_parser/src/helpers/`)**
- `context()` - Adds contextual information
- `cut()` - Commits to parse branch (prevents misleading backtracking)
- `bws()` - Whitespace-aware wrapper with error context
- `bdelimited()` - Delimited parsing with cut on closing delimiter

### Module Organization

**Decision:** Separate the parser crate from the syntax (AST) crate, and keep analysis in its own crate.

**Structure:**
```
src/
├── bsharp_parser/          # Parser implementations and public facade
│   ├── src/
│   │   ├── expressions/    # Expression parsers
│   │   ├── keywords/       # Keyword parsing (modularized)
│   │   ├── helpers/        # Parsing utilities (bws, cut, context, directives, ...)
│   │   ├── facade.rs       # Public Parser facade
│   │   └── ...
├── bsharp_syntax/          # AST node definitions and shared syntax types
│   └── src/                # (re-exported by bsharp_parser as `syntax`)
├── bsharp_analysis/        # Analysis framework and workspace
│   └── src/
└── bsharp_cli/             # CLI entry and subcommands
```

**Rationale:**
- **Separation of Concerns**: Infrastructure vs implementation
- **Reusability**: Helpers used across all parsers
- **API Clarity**: `syntax` module is the public API
- **Testing**: Infrastructure can be tested independently

### Keyword Modularization

**Decision:** Organize keywords by category in dedicated modules.

**Structure:**
```
src/parser/keywords/
├── mod.rs                      # Keyword infrastructure
├── access_keywords.rs          # public, private, protected, internal
├── accessor_keywords.rs        # get, set, init, add, remove
├── type_keywords.rs            # class, struct, interface, enum, record
├── modifier_keywords.rs        # static, abstract, virtual, sealed
├── flow_control_keywords.rs    # if, else, switch, case, default
├── iteration_keywords.rs       # for, foreach, while, do
├── expression_keywords.rs      # new, this, base, typeof, sizeof
├── linq_query_keywords.rs      # from, where, select, orderby
└── ...
```

**Rationale:**
- **Maintainability**: Easy to find and update keyword parsers
- **Consistency**: Uniform keyword parsing strategy
- **Word Boundaries**: All keywords use `keyword()` helper for boundary checking
- **Prevents Bugs**: Avoids partial matches (e.g., "int" vs "int32")

**Implementation:**
- `keyword()` function enforces `[A-Za-z0-9_]` word boundaries
- Parsers grouped under `src/bsharp_parser/src/keywords/`

---

## AST Design

### Naming Convention

**Decision:** Use PascalCase names without 'Syntax' suffix for all AST nodes.

**Examples:**
- `ClassDeclaration` (not `ClassDeclarationSyntax`)
- `MethodDeclaration` (not `MethodDeclarationSyntax`)
- `ExpressionStatement` (not `ExpressionStatementSyntax`)
- `IfStatement` (not `IfStatementSyntax`)

**Rationale:**
- **Clarity**: Shorter, clearer names
- **Roslyn Inspiration**: Mirrors Roslyn's structure where appropriate
- **Consistency**: Uniform naming across entire codebase
- **User Preference**: Explicit design decision (documented in memories)

**Implications:**
- All AST node types follow this convention
- Test code uses these names
- Documentation uses these names
- Breaking change from earlier versions with 'Syntax' suffix

### AST Ownership Model

**Decision:** Parent nodes own their children; no circular references.

**Structure:**
```rust
pub struct ClassDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub primary_constructor_parameters: Option<Vec<Parameter>>,
    pub base_types: Vec<Type>,
    pub body_declarations: Vec<ClassBodyDeclaration>,  // Owned
    pub documentation: Option<XmlDocumentationComment>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}
```

**Rationale:**
- **Rust Ownership**: Leverages Rust's ownership system
- **Memory Safety**: No reference cycles or lifetime complexity
- **Simplicity**: Clear ownership semantics
- **Traversal**: Navigation traits provide search without ownership issues

**Trade-offs:**
- Cannot directly reference parent from child
- Navigation requires traversal from root
- Mitigated by `AstNavigate` and `FindDeclarations` traits

### Zero-Copy Parsing

**Decision:** Minimize string allocations during parsing where possible.

**Implementation:**
- String slices reference original input
- Identifiers store `String` (owned) for convenience
- Literals preserve original format as `String`

**Rationale:**
- **Performance**: Reduces allocation overhead
- **Memory Efficiency**: Lower memory footprint
- **Trade-off**: Some allocations necessary for AST lifetime

### Spans and Location Tracking

**Decision:** Track source locations via spans for precise diagnostics and tooling.

**Implementation:**
- `Span` type based on `nom_locate::LocatedSpan` lives in `src/bsharp_parser/src/syntax/span.rs` and is re-exported through the public parser API.
- The parser facade supports `parse_with_spans()` which returns both the AST and span table for mapping nodes back to source locations.
- Error reporting uses spans to include line/column, highlighting ranges via `format_error_tree()`.

**Rationale:**
- **Diagnostics:** Accurate error locations and ranges.
- **Tooling:** Enables IDE features, navigation, and source mapping.
- **Testing:** Stable, comparable locations for snapshot tests.

**See also:** `docs/syntax/spans.md`.

---

## Analysis Framework

### Framework-Driven Architecture

**Decision:** Implement a pipeline-based analysis framework with passes, rules, and visitors.

**Structure:**
```
src/analysis/
├── framework/        # Core analysis infrastructure
│   ├── pipeline.rs   # Analysis pipeline orchestration
│   ├── passes.rs     # Analysis pass trait and phases
│   ├── rules.rs      # Rule trait and rulesets
│   ├── walker.rs     # AST walker and visitor pattern
│   ├── registry.rs   # Analyzer registry
│   └── session.rs    # Analysis session and state
├── passes/           # Concrete analysis passes
├── rules/            # Concrete analysis rules
├── artifacts/        # Analysis artifacts (symbols, metrics, CFG)
└── ...
```

**Rationale:**
- **Extensibility**: Easy to add new analyzers
- **Composability**: Passes and rules compose via registry
- **Performance**: Single-pass traversal for local rules
- **Configurability**: Enable/disable passes and rules via config

**Phases:**
1. **Index** - Symbol indexing and scope building
2. **Local** - Single-pass local rules and metrics collection
3. **Global** - Cross-file analysis (dependencies, etc.)
4. **Semantic** - Type checking and semantic rules
5. **Reporting** - Report generation and formatting

### Visitor Pattern

**Decision:** Use visitor pattern for AST traversal.

**Implementation:**
```rust
pub trait Visit {
    fn enter(&mut self, node: &NodeRef, session: &mut AnalysisSession);
    fn exit(&mut self, node: &NodeRef, session: &mut AnalysisSession) {}
}

pub struct AstWalker {
    visitors: Vec<Box<dyn Visit>>,
}
```

**Rationale:**
- **Separation of Concerns**: Traversal logic separate from analysis logic
- **Composability**: Multiple visitors in single traversal
- **Performance**: Single pass for multiple analyses
- **Extensibility**: Easy to add new visitors

### Query API

**Decision:** Use a typed Query API over a minimal `NodeRef` to traverse the AST. This is the current traversal API; the term “legacy” only refers to older navigation traits that the Query API replaced.

**Implementation:**
- `NodeRef` enumerates coarse node categories (compilation unit, namespaces, declarations, methods, statements, expressions), and now includes top-level items like file-scoped namespaces, using directives, global using directives, and global attributes.
- `Children` provides child enumeration for `NodeRef`.
- `Extract<T>` enables `Query::of<T>()` to yield typed nodes without extending `NodeRef` for every concrete type.
- Macro helpers `impl_extract_expr!` and `impl_extract_stmt!` simplify adding `Extract` impls for expression/statement variants.
 - Location: `src/bsharp_syntax/src/query/` (re-exported as `bsharp_analysis::framework::Query`)

**Rationale:**
- **Composability**: Typed filters via `Query::filter_typed`.
- **Maintainability**: Avoids wide trait surfaces and duplicated traversal.
- **Performance**: Focused walkers remain available for hot paths.
- **Determinism**: Traversal order and artifact hashing remain stable.

**See also:**
- `docs/parser/navigation.md` (Query API overview)
- `docs/analysis/traversal-guide.md` (using Query in passes)
- `docs/development/query-cookbook.md` (recipes)

---

## Formatting and Emitters

**Decision:** Implement formatting via an `Emit` trait with per-node emitters in `bsharp_syntax`.

**Implementation:**
- `Emit` trait and emitters live under `src/bsharp_syntax/src/emitters/` (e.g., `emitters/declarations/*`, `emitters/expressions/*`, `emitters/statements/*`).
- Formatting is separated from parsing; emitters reconstruct code from AST with consistent whitespace and trivia handling.
- Trivia and XML doc emitters are under `emitters/trivia/`.

**Rationale:**
- **Separation of Concerns:** Parsing and formatting evolve independently.
- **Consistency:** Centralized formatting rules for all nodes.
- **Extensibility:** Adding a new node implies an `Emit` impl in a known location.

**See also:** `docs/syntax/formatter.md`.

---

## Workspace Loading

### Multi-Format Support

**Decision:** Support loading from .sln, .csproj, or directory.

**Implementation:**
```rust
pub struct WorkspaceLoader;

impl WorkspaceLoader {
    pub fn from_path(path: &Path) -> Result<Workspace>;
    pub fn from_path_with_options(path: &Path, opts: WorkspaceLoadOptions) -> Result<Workspace>;
}
```

**Rationale:**
- **Flexibility**: Support different entry points
- **IDE Integration**: Match IDE project loading behavior
- **Incremental Analysis**: Load only what's needed

**Features:**
- Solution file (.sln) parsing
- Project file (.csproj) parsing with XML
- Transitive ProjectReference following
- Source file discovery with glob patterns
- Deterministic project ordering

### Error Resilience

**Decision:** Continue loading workspace even if individual projects fail.

**Implementation:**
- Failed projects recorded as stubs with error messages
- Workspace loading succeeds with partial results
- Errors accessible via `Project::errors` field

**Rationale:**
- **Robustness**: Don't fail entire workspace for one bad project
- **User Experience**: Show what can be analyzed
- **Debugging**: Error messages preserved for investigation

---

## Testing Strategy

### External Test Organization

**Decision:** Externalize tests; in the current workspace they live under `src/bsharp_tests/` rather than inline `#[cfg(test)]` modules.

**Structure:**
```
src/bsharp_tests/src/
├── parser/
│   ├── expressions/
│   ├── statements/
│   ├── declarations/
│   └── types/
├── cli/
└── integration/
```

**Rationale:**
- **Separation**: Test code separate from implementation
- **Organization**: Clear structure mirrors crates
- **Compilation**: Tests don't bloat production binaries

**Note:** A future migration to top-level `tests/` may be considered.

### Test Helpers

**Decision:** Provide `expect_ok()` helper for readable test failures.

**Implementation:**
```rust
pub fn expect_ok<T>(input: &str, result: BResult<&str, T>) -> T {
    match result {
        Ok((_, value)) => value,
        Err(e) => {
            eprintln!("{}", format_error_tree(&input, &e));
            panic!("Parse failed");
        }
    }
}
```

**Rationale:**
- **Diagnostics**: Pretty-printed errors on failure
- **Debugging**: Shows parse failure context
- **Consistency**: Uniform test error reporting

### Snapshot Testing

**Decision:** Use `insta` crate for snapshot testing.

**Implementation:**
- `Cargo.toml` includes `insta` in dev-dependencies
- Snapshot tests for complex AST structures
- JSON serialization for comparison

**Rationale:**
- **Regression Prevention**: Catch unintended AST changes
- **Review**: Visual diff of AST changes
- **Maintenance**: Update snapshots when intentional

---

## Performance Considerations

### Parallel Analysis

**Decision:** Optional parallel analysis via `rayon` feature.

**Implementation:**
```toml
[features]
parallel_analysis = ["rayon"]
```

**Rationale:**
- **Scalability**: Faster analysis for large workspaces
- **Optional**: Not required for single-file use cases
- **Trade-off**: Adds dependency and complexity

### Incremental Parsing

**Decision:** Not implemented yet; designed for future addition.

**Future Design:**
- Cache parsed ASTs by file hash
- Reparse only changed files
- Incremental analysis based on change scope

**Rationale:**
- **Performance**: Critical for IDE integration
- **Complexity**: Requires careful cache invalidation
- **Priority**: Deferred until core features stable

---

<!-- Compiler backend and code generation are intentionally out of scope for now. -->

---

## CLI Design

### Subcommand Structure

**Decision:** Use `clap` with subcommands for different operations.

**Commands:**
- `parse` - Parse C# file to JSON
- `tree` - Generate AST visualization (Mermaid/DOT)
- `analyze` - Run analysis and generate report

**Rationale:**
- **Clarity**: Each command has clear purpose
- **Extensibility**: Easy to add new commands
- **Discoverability**: `--help` shows all options
- **Consistency**: Follows common CLI patterns

### Output Formats

**Decision:** Support multiple output formats (JSON, pretty-JSON, SVG).

**Implementation:**
- JSON for machine consumption
- Pretty-JSON for human readability
- SVG for visualization

**Rationale:**
- **Integration**: JSON for tool integration
- **Debugging**: Pretty-JSON for manual inspection
- **Visualization**: SVG for understanding AST structure

---

## Future Extensibility

### Planned Enhancements

1. **Incremental Parsing**
   - Cache parsed ASTs
   - Reparse only changed regions
   - Critical for IDE integration

2. **Language Server Protocol (LSP)**
   - IDE integration
   - Real-time diagnostics
   - Code completion

3. **More Analysis Passes**
   - Nullability analysis
   - Lifetime analysis
   - Security analysis

4. **Code Transformation**
   - AST modification API
   - Code generation from AST
   - Refactoring support

### Design for Extension

**Principles:**
- **Trait-Based**: Use traits for extensibility points
- **Registry Pattern**: Dynamic registration of analyzers
- **Configuration**: Enable/disable features via config
- **Versioning**: Stable API with clear versioning

---

## Lessons Learned

### What Worked Well

1. **Parser Combinators**: Excellent for composability and testing
2. **Module Organization**: Clear boundaries reduce coupling
3. **Error Context**: `ErrorTree` provides excellent diagnostics
4. **External Tests**: Clean separation improves maintainability

### What We'd Do Differently

1. **Earlier Keyword Modularization**: Should have organized keywords from start
2. **Error Type Migration**: Earlier adoption of `ErrorTree` would have saved refactoring
3. **Documentation**: More inline documentation from the beginning

### Recent Refactoring

Major refactoring improvements completed:
- Expression precedence chain builder implemented
- Statement group deduplication completed
- Consistent error recovery with `skip_to_member_boundary_top_level()`
- Whitespace handling standardization via `bws()` combinator
- Keyword modularization by category

---

## Contributing Guidelines

When adding new features, follow these architectural principles:

1. **Use Existing Patterns**: Follow established parser patterns
2. **Add Tests**: External tests in `tests/` directory
3. **Document Decisions**: Update this file for significant changes
4. **Error Context**: Add `.context()` calls for debugging
5. **Naming Convention**: PascalCase without 'Syntax' suffix
6. **Keyword Boundaries**: Use `keyword()` helper for all keywords

See `docs/development/contributing.md` for detailed contribution guidelines.
