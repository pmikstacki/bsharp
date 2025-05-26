
# Preprocessor Directives

BSharp implements comprehensive parsing for C# preprocessor directives, which are processed before the main parsing phase and can affect code compilation and parsing behavior.

## Directive Types

### 1. Conditional Compilation

#### #if, #elif, #else, #endif

```csharp
#if DEBUG
    Console.WriteLine("Debug mode");
#elif RELEASE
    Console.WriteLine("Release mode");
#else
    Console.WriteLine("Unknown mode");
#endif
```

#### Complex Conditions

```csharp
#if DEBUG && (WINDOWS || LINUX)
    // Platform-specific debug code
#endif

#if !(NET5_0_OR_GREATER)
    // Legacy framework code
#endif
```

### 2. Symbol Definition

#### #define and #undef

```csharp
#define FEATURE_ENABLED
#define VERSION_2_0

#undef OLD_FEATURE
```

### 3. Diagnostic Directives

#### #warning

```csharp
#warning This code is deprecated and will be removed in the next version
```

#### #error

```csharp
#if UNSUPPORTED_PLATFORM
#error This platform is not supported
#endif
```

### 4. Line Directives

#### #line

```csharp
#line 100 "OriginalFile.cs"
// Following code appears to come from line 100 of OriginalFile.cs

#line default
// Reset to actual file and line numbers

#line hidden
// Hide following lines from debugger
```

### 5. Region Directives

#### #region and #endregion

```csharp
#region Private Methods
private void HelperMethod()
{
    // Implementation
}

private void AnotherHelper()
{
    // Implementation
}
#endregion
```

### 6. Pragma Directives

#### #pragma warning

```csharp
#pragma warning disable CS0618
// Use of obsolete members
ObsoleteMethod();
#pragma warning restore CS0618

#pragma warning disable CS0162, CS0168
// Disable multiple warnings
#pragma warning restore CS0162, CS0168
```

#### #pragma checksum

```csharp
#pragma checksum "file.cs" "{406EA660-64CF-4C82-B6F0-42D48172A799}" "checksum_bytes"
```

### 7. Nullable Context Directives

#### #nullable

```csharp
#nullable enable
string? nullable = null;  // Nullable reference types enabled

#nullable disable
string notNullable = null;  // Warning disabled

#nullable restore
// Restore previous nullable context
```

## Preprocessor Expression Evaluation

### Symbols and Operators

#### Boolean Operators
```csharp
#if DEBUG && !RELEASE           // AND and NOT
#if WINDOWS || LINUX || MACOS   // OR
#if (A && B) || (C && D)        // Grouping with parentheses
```

#### Equality Operators
```csharp
#if VERSION == "2.0"            // String equality
#if BUILD_NUMBER >= 1000        // Numeric comparison (limited support)
```

### Symbol Resolution

Symbols can be defined:
1. **Source code**: `#define SYMBOL`
2. **Compiler flags**: `/define:SYMBOL`
3. **Project settings**: `<DefineConstants>`
4. **Environment**: Predefined symbols

### Predefined Symbols

Common predefined symbols:
```csharp
#if NET5_0_OR_GREATER          // Framework version
#if WINDOWS                    // Platform
#if DEBUG                      // Configuration
#if X64                        // Architecture
```

## Preprocessor AST Representation

### Preprocessor Directive Node

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreprocessorDirective {
    If {
        condition: PreprocessorExpression,
        then_block: Vec<PreprocessorDirective>,
        elif_blocks: Vec<(PreprocessorExpression, Vec<PreprocessorDirective>)>,
        else_block: Option<Vec<PreprocessorDirective>>,
    },
    Define(String),
    Undef(String),
    Warning(String),
    Error(String),
    Line {
        line_number: Option<u32>,
        file_name: Option<String>,
        hidden: bool,
    },
    Region {
        name: String,
        content: Vec<PreprocessorDirective>,
    },
    Pragma {
        directive: String,
        arguments: Vec<String>,
    },
    Nullable(NullableDirective),
}
```

### Preprocessor Expression

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreprocessorExpression {
    Symbol(String),
    Not(Box<PreprocessorExpression>),
    And(Box<PreprocessorExpression>, Box<PreprocessorExpression>),
    Or(Box<PreprocessorExpression>, Box<PreprocessorExpression>),
    Equal(Box<PreprocessorExpression>, Box<PreprocessorExpression>),
    NotEqual(Box<PreprocessorExpression>, Box<PreprocessorExpression>),
    Parenthesized(Box<PreprocessorExpression>),
    Literal(String),
}
```

## Conditional Compilation Processing

### Block Structure

Conditional blocks create a tree structure:

```csharp
#if CONDITION_A
    // Block A
    #if NESTED_CONDITION
        // Nested block
    #endif
#elif CONDITION_B
    // Block B
#else
    // Default block
#endif
```

### Active Code Determination

The preprocessor determines which code blocks are active:

1. **Evaluate conditions**: Process #if expressions
2. **Symbol lookup**: Resolve defined symbols
3. **Block selection**: Choose active code paths
4. **Nested processing**: Handle nested conditionals

## Integration with Main Parser

### Two-Phase Parsing

1. **Preprocessor Phase**: Process directives and determine active code
2. **Main Parse Phase**: Parse the active code sections

### Conditional Code Exclusion

Inactive code blocks are:
- **Excluded from parsing**: Not processed by main parser
- **Preserved in AST**: Available for analysis tools
- **Marked as inactive**: Flagged for tooling

### Directive Preservation

All directives are preserved for:
- **Code formatting tools**
- **Refactoring utilities**
- **Documentation generation**
- **Build system integration**

## Error Handling

### Directive Validation

The parser validates:
- **Balanced conditionals**: Every #if has matching #endif
- **Valid expressions**: Preprocessor expressions are syntactically correct
- **Symbol definitions**: #define follows naming rules
- **Pragma syntax**: Pragma directives have valid format

### Error Recovery

When encountering malformed directives:
- **Skip invalid directives**: Continue parsing
- **Report detailed errors**: Show directive location and issue
- **Maintain structure**: Keep conditional block structure intact

## Advanced Features

### Nested Regions

```csharp
#region Outer Region
    #region Inner Region
        // Nested region content
    #endregion
#endregion
```

### Complex Pragma Directives

```csharp
#pragma warning disable IDE0051 // Remove unused private members
#pragma warning restore IDE0051

#pragma nullable enable warnings
#pragma nullable disable annotations
```

### Source Mapping

Line directives affect source mapping:

```csharp
#line 1 "Generated.cs"
// This appears to come from Generated.cs line 1
var generated = true;
#line default
// Back to actual file location
```

## Usage in Analysis

### Conditional Code Analysis

Analysis tools can:
- **Detect dead code**: Find code that's never compiled
- **Track feature flags**: Analyze conditional compilation usage
- **Generate reports**: Show compilation configurations

### Symbol Tracking

Track symbol definitions and usage:
- **Definition locations**: Where symbols are defined
- **Usage contexts**: Where symbols are referenced
- **Scope analysis**: Symbol visibility across files

## Performance Considerations

### Preprocessing Optimization

- **Symbol caching**: Cache symbol resolution results
- **Lazy evaluation**: Process conditionals only when needed
- **Memory efficiency**: Minimize directive storage overhead

### Integration Efficiency

- **Single-pass processing**: Process directives during parsing
- **Minimal backtracking**: Avoid reparsing conditional blocks
- **Incremental updates**: Support for incremental parsing with directive changes

The preprocessor directive system ensures that all C# preprocessing features are supported while maintaining the ability to analyze and manipulate code across different compilation configurations.
