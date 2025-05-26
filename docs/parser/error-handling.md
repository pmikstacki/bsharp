
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
