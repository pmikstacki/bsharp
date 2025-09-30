
# Error Handling

BSharp implements a comprehensive error handling system that provides detailed context information for debugging parse failures.

## Error Types

The parser uses a custom error type `BResult` which provides structured error information:

```rust
pub type BResult<I, O> = nom::IResult<I, O, BError>;
```

### BError Structure

The `BError` type contains:
- **Context**: Descriptive information about what was being parsed
- **Location**: Position in the source code where the error occurred
- **Expected**: What the parser expected to find
- **Found**: What was actually encountered

## Error Recovery

The parser implements several error recovery strategies:

### 1. Malformed Syntax Recovery

When encountering malformed syntax, the parser attempts to skip to recovery points:
- Semicolons (`;`)
- Closing braces (`}`)
- End of input

### 1.a Declaration Error Recovery (Type Member Top-Level)

For type declarations (classes, structs, records, interfaces), malformed members are recovered using a lightweight, scope-aware helper:

- Helper: `skip_to_member_boundary_top_level()`
- Location: `src/parser/expressions/declarations/type_declaration_helpers.rs`

Contract:
- Only use from within a type body when a member parser fails.
- Stops at the next safe boundary at top level of the current type:
  - Consumes a top-level `;` and returns the slice after it.
  - Or stops at a top-level `}` without consuming it (so the caller can close the current body cleanly).
  - Returns an empty slice at EOF.
- Depth-tracks `()`, `[]`, `{}`, and a heuristic `<>` to avoid stopping inside expressions, attribute arguments, or generic argument lists.
- Ignores control characters inside strings, chars, and comments.

Limitations:
- Angle-bracket tracking is heuristic and does not fully disambiguate generics from shift operators.
- Verbatim/interpolated strings are not fully lexed here; this helper is intended for robust, not perfect, recovery.

Usage example (simplified):

```rust
match member_parser(cur) {
    Ok((rest, member)) => { members.push(member); cur = rest; }
    Err(_) => {
        let next = skip_to_member_boundary_top_level(cur);
        if next.is_empty() || next == cur { break; }
        cur = next;
    }
}
```

### 1.b Namespace Body: Using-Directives Before Members

Inside a block-scoped namespace body, `using` directives are accepted before type and nested-namespace members.

- Implementation: `parse_namespace_declaration()` scans for `using` immediately after the opening `{` and collects all consecutive directives before parsing members.
- This ensures inputs like the following are parsed deterministically without interleaving usings with members:

```csharp
namespace Outer {
    using System;
    namespace Inner {
        using System.Collections;
        class MyClass {}
    }
}
```

Contract and limitations:
- Only leading `using` directives at the current namespace body level are collected.
- Interleaving `using` directives among members is not supported yet (matches common style and avoids ambiguous recovery).

### 1.c File-Scoped Namespace

When parsing a file-scoped namespace, the parser also skips preprocessor directives following the `namespace` line before parsing members, mirroring the block-scoped behavior.

## Preprocessor Directives and Trivia

Preprocessor directives (e.g., `#pragma`, `#line`) are treated as structured trivia, not AST declarations:

- Parser entrypoints (e.g., `parse_csharp_source()`) skip directive lines anywhere they can appear at the compilation-unit level.
- `parse_preprocessor_directive()` consumes the entire directive line including an optional trailing newline.
- Current status: directives inside type and namespace bodies are planned to be skipped similarly; tests are tracked and temporarily ignored until this is integrated.

Example:

```csharp
#pragma warning disable CS0168
namespace N {
    // class and members...
}
```

The directive is skipped and not present as a namespace member.

### 2. Context-Aware Errors

Errors include contextual information about the parsing context:

```rust
bs_context("method declaration", parse_method_body)(input)
```

This provides clear error messages like "expected method body in method declaration context".

### 3. Graceful Degradation

The parser continues parsing even after encountering errors, collecting multiple errors to provide comprehensive feedback.

## Error Reporting

Errors are reported with:
- **Line and column numbers**
- **Surrounding context**
- **Suggestions for fixes**
- **Parser state information**

## Common Error Scenarios

### Syntax Errors
- Missing semicolons
- Unmatched braces
- Invalid identifiers

### Type Errors
- Unknown type references
- Generic constraint violations
- Invalid type parameter usage

### Declaration Errors
- Conflicting modifiers
- Missing required elements
- Invalid access levels

## Debugging Tips

1. **Use verbose error output** to get detailed parser state
2. **Check recovery points** when errors cascade
3. **Validate input syntax** with simpler test cases first
4. **Use parser context** to understand where parsing failed
