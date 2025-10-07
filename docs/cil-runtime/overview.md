# CIL Runtime & Code Execution: Overview

This section documents the design and plan for adding whole-code execution to the project using a custom CIL (ECMA-335) virtual machine and the dotscope crate for IL/PE read/write.

## Goals
- **Back-compat with .NET assemblies**: Load and execute existing IL where feasible.
- **AST-driven emission**: Emit IL from our parsed AST for a supported C# subset.
- **Deterministic, testable**: Deterministic interpreter, rich golden tests, reproducible outputs.
- **Modular**: Clear crates for IL emission, VM, and host runtime.

## Scope (MVP)
- Primitives: bool, i32/i64, f32/f64.
- Control flow: if/else, while/for, switch, return.
- Locals/args, arithmetic, comparisons, simple static calls.
- Load existing DLLs (limited surface) via dotscope.

## Non-goals (initially)
- Full BCL coverage, GC, reflection, async/await, generics, full verification.
- Optimizing JIT (planned later via Cranelift).

## Building blocks
- **dotscope**: Read/emit IL and metadata (MethodBodyBuilder, *Builder* APIs, write DLLs).
- **bsharp_vm**: CIL interpreter (eval stack, frames, basic heap abstractions, host shims).
- **bsharp_il**: AST→IL emitter leveraging analysis CFG to structure branches.
- **bsharp_runtime**: Host intrinsics and minimal library surface.

## Integration with existing crates
- Reuse parser/AST and analysis pipeline. Use analysis control flow graph (CFG) to drive structured emission (labels/branches) instead of serial AST-only lowering.

## Long-term direction
- Expand IL/VM feature coverage (objects, exceptions, arrays, virtual dispatch).
- Optional native backend via Cranelift using the same loader/type system.
