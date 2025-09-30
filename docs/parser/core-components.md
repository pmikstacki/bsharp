
# Core Parser Components

This document details the fundamental components that make up the BSharp parser infrastructure.

## Public Parser API

### Parser Struct

The main entry point for all parsing operations:

```rust
#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self
    pub fn parse(&self, input: &str) -> Result<ast::CompilationUnit, String>
}
```

The `Parser` provides a clean, simple interface that abstracts away the complexity of the underlying parsing implementation.

## Error System

### ErrorTree (nom-supreme)

BSharp uses nom-supreme's ErrorTree for rich error diagnostics:

```rust
pub type BResult<I, O> = IResult<I, O, ErrorTree<I>>;
```

**Location:** Type alias defined in parser infrastructure

Key features:
- **Context Stack**: Maintains parsing contexts via `.context()` calls
- **Position Tracking**: Built-in span tracking for error locations
- **Rich Diagnostics**: Tree structure shows complete parse failure path
- **Integration**: Seamless with nom combinators

### Error Helpers

Utility functions for enhanced error handling:

**Location:** `src/syntax/parser_helpers.rs`

- `context()`: Adds contextual information to parser errors
- `bws()`: Whitespace-aware wrapper with error context
- `bdelimited()`: Delimited parsing with cut on closing delimiter
- `cut()`: Commits to parse branch, preventing misleading backtracking
- Error recovery mechanisms for common parsing scenarios

### Pretty Error Formatting

**Location:** `src/syntax/errors.rs`

```rust
pub fn format_error_tree(input: &str, error: &ErrorTree<&str>) -> String;
```

Produces rustc-like error messages with:
- Line and column numbers
- Source code context
- Caret pointing to error location
- Context stack showing parse path

## AST Foundation

### CompilationUnit

The root node of every parsed C# file:

```rust
pub struct CompilationUnit {
    pub global_attributes: Vec<GlobalAttribute>,
    pub using_directives: Vec<UsingDirective>,
    pub declarations: Vec<TopLevelDeclaration>,
    pub file_scoped_namespace: Option<FileScopedNamespaceDeclaration>,
    pub top_level_statements: Vec<Statement>,
}
```

Represents the complete structure of a C# source file, supporting both traditional and modern C# features.

### TopLevelDeclaration

Enum representing all possible top-level declarations:

```rust
pub enum TopLevelDeclaration {
    Namespace(NamespaceDeclaration),
    FileScopedNamespace(FileScopedNamespaceDeclaration),
    Class(ClassDeclaration),
    Struct(StructDeclaration),
    Record(RecordDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Delegate(DelegateDeclaration),
    GlobalAttribute(GlobalAttribute),
}
```

## Keyword Parsing

### Keyword Module Organization

**Location:** `src/parser/keywords/`

Keywords are organized by category in dedicated modules for maintainability and consistency:

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

### Keyword Parsing Strategy

**Word Boundary Enforcement:**

```rust
pub fn keyword(kw: &'static str) -> impl Fn(&str) -> BResult<&str, &str>;
```

The `keyword()` helper enforces `[A-Za-z0-9_]` word boundaries to prevent partial matches:
- Correctly rejects "int" when parsing "int32"
- Ensures "class" doesn't match "classname"
- Consistent across all keyword parsers

**Benefits:**
- **Maintainability**: Easy to find and update keyword parsers
- **Consistency**: Uniform keyword parsing strategy
- **Bug Prevention**: Avoids partial match issues
- **Centralization**: Single source of truth for keywords

## Parser Helpers

### Context Management

Functions for maintaining parsing context:

```rust
pub fn context<I, O, F>(
    ctx: &'static str,
    parser: F
) -> impl FnMut(I) -> BResult<I, O>
```

Wraps parsers with contextual information that appears in error messages, making debugging much easier.

### Parser Composition

Utilities for combining smaller parsers into larger ones:

- Sequencing parsers with error propagation
- Optional parsing with fallbacks
- Alternative parsing with preference ordering
- Repetition parsing with separators

### Whitespace and Comment Handling

Consistent handling of whitespace and comments throughout the parser:

- Automatic whitespace skipping between tokens
- Comment preservation for documentation tools
- Preprocessor directive handling

## Node Structure Standards

### Common Traits

All AST nodes implement standard traits:

- `Debug`: For debugging and logging
- `PartialEq`: For testing and comparison
- `Clone`: For AST manipulation
- `Serialize/Deserialize`: For JSON export/import

### Node Organization

AST nodes are organized hierarchically:

```
nodes/
├── declarations/     # Type and member declarations
├── expressions/      # All expression types
├── statements/       # All statement types
├── types/           # Type system representations
└── ...              # Other language constructs
```

### Identifier Handling

Consistent identifier representation throughout the AST:

```rust
pub struct Identifier {
    pub name: String,
    // Additional metadata like source location
}
```

## Type System Integration

### Type Representation

The parser builds a complete representation of C# types:

- Primitive types (int, string, bool, etc.)
- Reference types (classes, interfaces)
- Value types (structs, enums)
- Generic types with constraints
- Array and pointer types
- Nullable types

### Generic Support

Full support for C# generics:

- Type parameters with constraints
- Variance annotations (in, out)
- Generic method declarations
- Complex constraint combinations

## Memory Management

### Zero-Copy Parsing

Where possible, the parser avoids unnecessary string allocations:

- String slices reference original input
- Minimal cloning during parsing
- Efficient error reporting without excessive allocation

### AST Ownership

Clear ownership semantics for AST nodes:

- Parent nodes own their children
- Shared references through navigation traits
- No circular references in the AST structure

This foundation provides a robust base for parsing complex C# code while maintaining performance and usability.
