# Stage 1 – VM Architecture and Constraints

This document defines the architectural constraints and high-level design decisions for the BSharp VM and IR.

## Objectives

- Clarify the **execution model** (stack vs register) and IR level.
- Define the **scope of the initial C# subset** for scripting.
- Identify **host embedding** and runtime requirements.
- Enumerate key constraints to guide later stages.

## Execution model and IR level

- **VM style**
  - Target a **register-based VM** inspired by Lua 5 and the "Writing Interpreters in Rust" guide.
  - Use fixed-width opcodes (e.g. 32-bit) with fields for operator and register operands.
- **IR level**
  - Introduce a low-level IR close to bytecode, with:
    - `IrModule` containing functions and global values.
    - `IrFunction` with parameters, locals, and basic blocks.
    - `IrBlock` with a linear list of `IrInstr` terminates in a branch/return.
  - IR should be **typed enough** to support runtime checks (e.g. `Int32`, `Bool`, `String`, `ObjectRef`).
- **Call frames and stacks**
  - Maintain separate data structures for:
    - A **register stack** (homogeneous array of value slots or tagged values).
    - A **call frame stack** containing function, return IP, and base index, following the Rust-hosted-langs design.

## C# subset (initial scripting profile)

The first implementation will target a restricted, well-defined subset of C#:

- **Expressions**
  - Literals: integer, boolean, string, null.
  - Binary operators: `+`, `-`, `*`, `/`, comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`).
  - Logical operators: `&&`, `||`, `!` (with short-circuit semantics).
  - Simple invocation of functions/intrinsics.
- **Statements**
  - Local variable declarations and assignments.
  - `if` / `else`.
  - `while` loops.
  - `return` statements.
  - Expression statements.
- **Top-level**
  - Support `CompilationUnit.top_level_statements` as a synthetic script `main`.
  - Ignore complex OO (classes, interfaces, generics) in the initial stage; treat functions as top-level or free functions.

Later stages can extend this subset with more statements, methods, records, exceptions, and async.

## Host embedding and runtime

- **Embedding model**
  - Provide a `ScriptEngine` API that allows:
    - Compiling a script string to an internal module handle.
    - Executing a script, returning a `Value` and/or raising a VM error.
- **Runtime library**
  - Define a minimal set of intrinsics for early experimentation:
    - `print` / `println`.
    - Basic math helpers where needed.
  - Map certain `InvocationExpression` nodes to built-in intrinsics, **not** to .NET or CIL.

## Constraints and non-goals

- **No CIL compatibility**
  - Do not reuse CIL opcode names or ECMA-335 metadata.
  - Avoid leaking .NET-specific concepts into IR or VM types.
- **No JIT compilation (initially)**
  - Focus on a pure interpreter first; leave JIT considerations as future work.
- **Security and safety**
  - Explicit error handling: VM operations and compilation must return structured errors, never silently fail.
  - No arbitrary host process access by default; host embedding must intentionally expose APIs.

## Deliverables

- A concrete description of the VM execution model and IR level.
- A documented C# subset for the initial scripting runtime.
- A set of constraints guiding subsequent stages (IR design, lowering, VM implementation).

## References

- "Writing Interpreters in Rust: a Guide" – chapters on bytecode and VM design.
- Existing `docs/cil-runtime/*` as an example of how to structure runtime planning, while keeping this VM independent.
