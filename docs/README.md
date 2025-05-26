
# BSharp C# Parser Documentation

BSharp is a comprehensive C# parser and analysis toolkit written in Rust. It provides a complete solution for parsing C# source code into an Abstract Syntax Tree (AST), performing various code analyses, and generating insights about code quality and structure.

## What is BSharp?

BSharp consists of several key components:

- **Parser**: A robust C# parser built using the `nom` parser combinator library
- **AST**: A complete representation of C# language constructs
- **Analysis Framework**: Tools for analyzing code structure, dependencies, and quality
- **CLI Tools**: Command-line utilities for parsing, visualization, and compilation

## Key Features

- **Complete C# Language Support**: Supports modern C# features including:
  - Classes, structs, interfaces, records, enums
  - Methods, properties, fields, events, indexers
  - All statement types (if, for, while, switch, try-catch, etc.)
  - Expression parsing with operator precedence
  - Generic types and constraints
  - Attributes and modifiers
  - Preprocessor directives

- **Robust Error Handling**: Custom error types with context information for debugging parse failures

- **AST Navigation**: Powerful navigation traits for traversing and analyzing the AST

- **Code Analysis**: Built-in analyzers for:
  - Control flow analysis
  - Dependency tracking
  - Code metrics (complexity, maintainability)
  - Type analysis
  - Code quality assessment

- **Extensible Architecture**: Modular design allowing easy extension of parsing and analysis capabilities

## Architecture Overview

The codebase is organized into several main modules:

```
src/
├── parser/           # Core parsing infrastructure
├── parsers/          # Specific parser implementations
├── analysis/         # Code analysis framework
├── cli/              # Command-line interface
├── codegen/          # Code generation utilities
└── compiler/         # Compilation infrastructure
```

## Getting Started

The easiest way to get started is using the CLI tools:

```bash
# Parse a C# file and output JSON
bsharp parse input.cs --output output.json

# Generate AST visualization
bsharp tree input.cs --output ast.svg

# Compile C# code
bsharp compile input.cs
```

## Use Cases

BSharp is designed for:

- **Static Analysis Tools**: Build custom analyzers for code quality, security, or style
- **Code Transformation**: Parse, modify, and regenerate C# code
- **Language Tooling**: Create IDE extensions, linters, or formatters
- **Educational Tools**: Understand and visualize C# code structure
- **Migration Tools**: Analyze legacy code for modernization efforts

This documentation will guide you through all aspects of using and extending BSharp.
