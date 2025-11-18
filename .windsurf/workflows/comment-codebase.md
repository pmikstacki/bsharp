---

## description: Systematically add comprehensive comments to B# syntax codebase

# Workflow: Add Comprehensive Comments to B# Syntax Codebase

This workflow guides the process of adding detailed documentation and comments to the B# syntax tree implementation across both `bsharp_syntax_derive` and `bsharp_syntax` directories.

## Prerequisites

*   Ensure you have write access to the codebase
*   Familiarity with Rust documentation conventions (`//!` for module docs, `///` for item docs)
*   Understanding of the B# language syntax and AST structure

## Step-by-Step Process

### Phase 1: Core Infrastructure (COMPLETED)

**bsharp\_syntax\_derive/src/lib.rs**

*   Add module-level documentation explaining the derive macro purpose
*   Document the `derive_ast_node` function with examples
*   Add detailed comments to `gen_push_for_type` explaining type handling logic
*   Document `is_primitive_like` function with rationale

**bsharp\_syntax/src/lib.rs**

*   Add comprehensive module-level documentation
*   Document the AST architecture and design principles
*   Add usage examples and API overview
*   Improve inline comments for module organization

### Phase 2: Core Syntax Types (COMPLETED)

**bsharp\_syntax/src/identifier.rs**

*   Add module-level documentation for identifier types
*   Document `Identifier` enum variants with examples
*   Document `OverrideOperatorType` enum
*   Add comprehensive `Display` implementation docs
*   Document constructor methods with usage patterns

**bsharp\_syntax/src/expressions/expression.rs**

*   Add module-level documentation explaining expression categories
*   Document all 30+ expression variants organized by logical groups:
    *   Object creation (AnonymousObject, Tuple, New)
    *   Collection operations (Range, Index, Collection)
    *   Member access (MemberAccess, NullConditional, Indexing)
    *   Method invocation (Invocation, Lambda, AnonymousMethod)
    *   Operators (Binary, Unary, PostfixUnary, Assignment)
    *   Literals and variables (Literal, Variable, This, Base)
    *   Control flow (Conditional, SwitchExpression, IsPattern, As, Cast)
    *   Exception handling (Throw)
    *   Language features (Await, Query, With, Pattern matching)
    *   Reflection and metadata (Nameof, Typeof, Sizeof, Default)
    *   Memory management (Ref, StackAlloc)
    *   Overflow checking (Checked, Unchecked)
*   Document supporting structs (CollectionElement, WithInitializerEntry, SwitchExpression, SwitchExpressionArm)

**bsharp\_syntax/src/expressions/binary\_operator.rs** (COMPLETED)

*   Add module-level documentation for binary operators
*   Document all operator variants organized by categories:
    *   Arithmetic operators (+, -, \*, /, %)
    *   Assignment operators (=, +=, -=, \*=, /=, %=, &=, |=, ^=, \<\<=, >>=, >>>=, ??=)
    *   Comparison operators (==, !=, \<, >, \<=, >=)
    *   Type testing operators (is, as)
    *   Logical operators (&&, ||)
    *   Bitwise operators (&, |, ^, \<\<, >>, >>>)
    *   Special operators (??, ..)
*   Document `precedence()` method with detailed hierarchy
*   Document `rhs_expectation()` method for error messages

### Phase 3: Type System (IN PROGRESS)

**bsharp\_syntax/src/types/type\_.rs** (PARTIALLY COMPLETED)

*   Add module-level documentation for type system
*   Document all `Type` enum variants:
    *   Primitive types (int, string, bool, etc.)
    *   Reference types (named types)
    *   Generic types (with type arguments)
    *   Array types (single and multi-dimensional)
    *   Pointer types (unsafe)
    *   Nullable types (value and reference)
    *   Special types (dynamic, void, var, implicit arrays)
    *   Function types (function pointers, ref returns)
*   Document `Display` implementation

**bsharp\_syntax/src/types/primitive\_type.rs**

*   Document all primitive type variants
*   Add examples for each primitive type
*   Document size and behavior characteristics

**bsharp\_syntax/src/types/parameter.rs**

*   Document `Parameter` struct and `ParameterModifier` enum
*   Add examples of different parameter types
*   Document parameter passing semantics

### Phase 4: Statements (NEXT)

**bsharp\_syntax/src/statements/statement.rs**

*   Add module-level documentation for statement types
*   Document all `Statement` enum variants organized by categories:
    *   Control flow (If, Switch, For, While, ForEach, DoWhile)
    *   Jump statements (Break, Continue, Return, Goto, GotoCase)
    *   Exception handling (Try, Throw)
    *   Blocks and scoping (Block, Empty, Using, Lock, Fixed)
    *   Unsafe code (Unsafe)
    *   Special statements (Yield, LocalFunction, Label, Checked, Unchecked)
    *   Declarations (Declaration, Deconstruction)
    *   Expressions (Expression)
*   Add examples for each statement category

**Key statement implementation files**

*   `if_statement.rs` - Document conditional branching
*   `for_statement.rs` - Document traditional for loops
*   `for_each_statement.rs` - Document iteration over collections
*   `try_statement.rs` - Document exception handling
*   `switch_statement.rs` - Document pattern-based switching

### Phase 5: Declarations

**bsharp\_syntax/src/declarations/type\_declaration.rs**

*   Document base type declaration traits
*   Add examples of different declaration patterns

**bsharp\_syntax/src/declarations/class\_declaration.rs**

*   Document class structure and inheritance
*   Add examples of class declarations

**bsharp\_syntax/src/declarations/method\_declaration.rs**

*   Document method signatures and bodies
*   Add examples of different method types

**bsharp\_syntax/src/declarations/field\_declaration.rs**

*   Document field declarations and initializers
*   Add examples of field patterns

### Phase 6: Supporting Infrastructure

**Node infrastructure**

*   `node/ast_node.rs` - Document core AST node trait
*   `node/dyn_node_ref.rs` - Document dynamic node references

**Query and navigation**

*   `query/` - Document AST querying utilities
*   `formatter/` - Document code formatting capabilities

## Documentation Standards

### Module Documentation

```
//! Brief description of the module purpose.
//!
//! More detailed explanation including:
//! - What the module contains
//! - How components relate to each other
//! - Key types and their roles
//! - Usage examples when appropriate
//!
//! # Categories
//!
//! - **Category 1**: Description
//! - **Category 2**: Description
//!
//! # Examples
//!
//! ```rust
//! use bsharp_syntax::*;
//!
//! // Example code
//! ```
```

### Enum/Struct Documentation

```
/// Brief description of the type.
///
/// Detailed explanation including:
/// - Purpose and role in the AST
/// - When and how it's used
/// - Relationship to other types
///
/// # Fields/Variant Details
///
/// - [`field1`]: Description of field1
/// - [`field2`]: Description of field2
///
/// # Examples
///
/// ```rust
/// // Usage example
/// ```
```

### Function Documentation

```
/// Brief description of what the function does.
///
/// Detailed explanation including:
/// - Algorithm or approach used
/// - Performance considerations
/// - Error handling behavior
///
/// # Arguments
///
/// * `arg1` - Description of argument1
/// * `arg2` - Description of argument2
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```rust
/// // Function usage example
/// ```
```

### Inline Comments

*   Explain complex logic or algorithms
*   Provide context for non-obvious code
*   Reference external specifications when relevant
*   Include TODO comments for future improvements

## Quality Checklist

For each file completed:

*   Module-level documentation added
*   All public types documented with examples
*   All public methods documented with parameter/return descriptions
*   Complex logic explained with inline comments
*   Cross-references added between related types
*   Examples compile and are accurate
*   Documentation follows Rust conventions
*   Spelling and grammar are correct

## Tools and Commands

### Documentation Generation

```
# Generate and view documentation
cargo doc --open

# Check for missing documentation
cargo doc --document-private-items
```

### Validation

```
# Run tests to ensure examples work
cargo test

# Check for warnings
cargo clippy
```

## Notes

*   Focus on public APIs first, private documentation can be added later
*   Examples should be minimal but complete
*   Reference C# language specification when appropriate
*   Consider both compiler users and AST consumers as documentation audience
*   Maintain consistency in documentation style across the codebase