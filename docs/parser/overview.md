
# Parser Overview

The BSharp parser transforms C# source code into a structured Abstract Syntax Tree (AST). Built using the `nom` parser combinator library, it provides a robust and extensible foundation for parsing modern C# syntax as part of the BSharp toolkit.

## Architecture

The parser follows a modular architecture with clear separation of concerns. It serves as the frontend for tools that consume the AST (analysis, visualization, etc.):

### Parser Infrastructure (`src/bsharp_syntax/src/`)

- **`mod.rs`**: Public API and re-exports
- **`ast.rs`**: Root AST node definitions (`CompilationUnit`, `TopLevelDeclaration`)
- **`errors.rs`**: Error formatting utilities (`format_error_tree`)
- **`parser_helpers.rs`**: Core parsing utilities (`context`, `bws`, `keyword`, etc.)
- **`test_helpers.rs`**: Testing utilities (`expect_ok`, etc.)
- **`nodes/`**: AST node definitions organized by category

### Parser Implementations (`src/bsharp_parser/src/`)

The parsers are organized by language construct type:

- **`expressions/`**: All expression parsing (literals, operators, method calls, etc.)
- **`keywords/`**: Keyword parsing organized by category
- **`types/`**: Type system parsing (primitives, generics, arrays, etc.)
- **`helpers/`**: Declaration helpers and utilities
- **`preprocessor/`**: Preprocessor directive parsing

### AST Node Definitions (`src/bsharp_syntax/src/`)

Structured node definitions that mirror C# language constructs:

- **`declarations/`**: All declaration node types
- **`expressions/`**: All expression node types  
- **`statements/`**: All statement node types
- **`types/`**: Type system node definitions

## Parser Design Principles

### 1. Compositional Design

The parser is built from small, focused parser functions that combine to handle complex language constructs:

```rust
// Example: Method declaration combines multiple sub-parsers
fn parse_method_declaration(input: &str) -> BResult<&str, MethodDeclaration> {
    let (input, attributes) = parse_attributes(input)?;
    let (input, modifiers) = parse_modifiers(input)?;
    let (input, return_type) = parse_type(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, parameters) = parse_parameter_list(input)?;
    let (input, body) = opt(parse_block_statement)(input)?;
    // ... construct and return MethodDeclaration
}
```

### 2. Error Recovery and Context

Custom error types provide detailed context about parse failures:

- Location information (line, column)
- Expected vs. actual input
- Contextual error messages
- Error recovery strategies

### 3. Extensibility

The modular design allows easy addition of new language features:

- Add new expression types by extending the `Expression` enum
- Implement new statement parsers following established patterns
- Extend AST navigation traits for new analysis capabilities

## Parsing Flow

### 1. Entry Point

Parsing begins via the public facade in `src/bsharp_parser/src/facade.rs`:

```rust
let parser = bsharp_parser::facade::Parser::new();
let cu = parser.parse(source)?;
```

### 2. Compilation Unit Parsing

The parser starts by parsing a `CompilationUnit`, which represents a complete C# source file:

- Global attributes (assembly/module level)
- Using directives
- Top-level declarations (namespaces, classes, etc.)
- File-scoped namespaces (C# 10+)
- Top-level statements (C# 9+)

### 3. Recursive Descent

The parser uses recursive descent to handle nested structures:

- Namespaces contain type declarations
- Types contain member declarations
- Methods contain statements
- Statements contain expressions

## Key Parser Features

### Expression Parsing with Precedence

The expression parser handles operator precedence correctly:

- Primary expressions (literals, identifiers, parentheses)
- Unary operators (!, -, +, ++, --, etc.)
- Binary operators with correct precedence and associativity
- Conditional expressions (ternary operator)
- Assignment expressions

### Statement Parsing

Comprehensive support for all C# statement types:

- Control flow: `if`, `switch`, `for`, `foreach`, `while`, `do-while`
- Jump statements: `break`, `continue`, `return`, `throw`, `goto`
- Exception handling: `try-catch-finally`
- Resource management: `using`, `lock`
- Local declarations and assignments

### Declaration Parsing

Full support for C# type and member declarations:

- Types: classes, structs, interfaces, records, enums, delegates
- Members: methods, properties, fields, events, indexers, operators
- Modifiers: access modifiers, static, abstract, virtual, override, etc.
- Generics: type parameters, constraints, variance

### Modern C# Features

Support for recent C# language additions:

- Records (C# 9)
- File-scoped namespaces (C# 10)
- Top-level statements (C# 9)
- Pattern matching enhancements
- Nullable reference types

## Error Handling Strategy

The parser uses a multi-layered error handling approach:

1. **Parse Errors**: Detailed information about what went wrong during parsing
2. **Context Propagation**: Errors include context about where in the parsing process they occurred
3. **Recovery Mechanisms**: Ability to continue parsing after certain types of errors
4. **User-Friendly Messages**: Clear, actionable error messages for developers

This design makes the parser robust and helpful for development and debugging. Code generation/compilation is out of scope for now; the parser and analysis crates form the current focus of the toolkit.
