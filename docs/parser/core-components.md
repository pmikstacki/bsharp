
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

### BSharpParseError

The custom error type that provides rich context for parse failures:

```rust
pub enum BSharpParseError<I> {
    Nom(nom::error::Error<I>),
    Custom { input: I, message: String, context: Vec<String> },
    // ... other variants
}
```

Key features:
- **Context Stack**: Maintains a stack of parsing contexts for debugging
- **Position Tracking**: Tracks line and column information
- **Custom Messages**: Allows for domain-specific error messages
- **Nom Integration**: Seamlessly integrates with nom parser errors

### Error Helpers

Utility functions for enhanced error handling:

- `bs_context()`: Adds contextual information to parser errors
- `wrap_std_parser()`: Converts standard nom parsers to use BSharpParseError
- Error recovery mechanisms for common parsing scenarios

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

## Parser Helpers

### Context Management

Functions for maintaining parsing context:

```rust
pub fn bs_context<I, O, F>(
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
