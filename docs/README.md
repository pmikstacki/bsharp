
# BSharp C# Parser Documentation

BSharp is a comprehensive C# parser and analysis toolkit written in Rust. It provides a complete solution for parsing C# source code into an Abstract Syntax Tree (AST), performing various code analyses, and generating insights about code quality and structure.

## What is BSharp?

BSharp consists of several key components:

- **Parser**: A robust C# parser built using the `nom` parser combinator library
- **AST**: A complete representation of C# language constructs
- **Analysis Framework**: Tools for analyzing code structure, dependencies, and quality
- **CLI Tools**: Command-line utilities for parsing, visualization, and analysis

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
├── bsharp_parser/    # Parser crate (expressions, statements, declarations, helpers)
├── bsharp_syntax/    # AST nodes and shared syntax types (re-exported by parser)
├── bsharp_analysis/  # Analysis framework and workspace loader
├── bsharp_cli/       # Command-line interface
└── bsharp_tests/     # External tests for parser/analysis/CLI
```

### Key Components

**Parser (`src/bsharp_parser/`, `src/bsharp_syntax/`)**
- Modular parser using nom combinators
- Complete C# language support
- Rich error diagnostics with ErrorTree
- Keyword parsing organized by category
- AST nodes follow PascalCase naming without 'Syntax' suffix

**Workspace Loading (`src/bsharp_analysis/src/workspace/`)**
- Solution file (.sln) parsing
- Project file (.csproj) parsing with XML
- Transitive ProjectReference resolution
- Source file discovery with glob patterns
- Deterministic project ordering

**Analysis Framework (`src/bsharp_analysis/src/`)**
- Pipeline-based architecture with phases
- Extensible passes and rules system
- Metrics collection (complexity, maintainability)
- Control flow analysis
- Dependency tracking
- Code quality assessment

<!-- Code generation/compilation is currently out of scope and intentionally omitted. -->

**CLI Tools (`src/bsharp_cli/`)**
- `parse` - Parse C# to JSON
- `tree` - Generate AST visualization (Mermaid/DOT)
- `analyze` - Comprehensive code analysis

## Getting Started

The easiest way to get started is using the CLI tools:

```bash
# Parse a C# file and output JSON
bsharp parse input.cs --output output.json

# Generate AST visualization
bsharp tree input.cs --output ast.svg

# Analyze a project or solution
bsharp analyze MyProject.csproj --out report.json
```

## Use Cases

BSharp is designed for:

- **Static Analysis Tools**: Build custom analyzers for code quality, security, or style
- **Code Transformation**: Parse, modify, and regenerate C# code
- **Language Tooling**: Create IDE extensions, linters, or formatters
- **Educational Tools**: Understand and visualize C# code structure
- **Migration Tools**: Analyze legacy code for modernization efforts

This documentation will guide you through all aspects of using and extending BSharp.
