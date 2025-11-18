# BSharp VM / IR Runtime – Overview

This document outlines the plan for building a new BSharp scripting runtime based on a custom IR and virtual machine, **fully independent of CIL/ECMA-335**. It builds on the existing `bsharp_parser`, `bsharp_syntax`, and `bsharp_analysis` crates.

## Current state (codebase)

*   **Parsing and AST**
    *   `bsharp_parser` exposes `parse_csharp_source_spanned` producing `Spanned<syntax::ast::CompilationUnit>`.
    *   `bsharp_syntax` defines the full C# AST, including expressions, statements, declarations, and root `CompilationUnit`.
*   **Analysis**
    *   `bsharp_analysis` provides an analysis pipeline, span DB, and semantic infrastructure that can supply name and type information.
*   **IL / CIL work**
    *   `bsharp_il` and `docs/cil-runtime/*` focus on a CIL-oriented runtime and are **not** a dependency for this VM.

## Goals and constraints

**Custom IR and VM only**

*   No dependency on CIL, ECMA-335 metadata tables, or .NET runtime.
*   Define a dedicated BSharp IR and bytecode tailored for a C#-like scripting runtime.

**Tight integration with existing crates**

*   Reuse `bsharp_parser` and `bsharp_syntax` as the front-end.
*   Leverage `bsharp_analysis` for name/type information where it simplifies lowering.

**Host-embeddable scripting runtime**

*   Provide a `ScriptEngine` API for embedding into host processes.
*   Support interpreting C#-like scripts with a well-defined subset of the language.

**Clear, incremental stages**

*   Each stage should be independently testable and keep the workspace green.
*   Prefer return-early style, explicit errors, and no hidden control flow.

**Performance-oriented runtime design**

*   Use a handle-based heap for `ObjectRef` values backed by a `Vec` of heap objects, with a free list for reuse.
*   Treat this heap as an arena for early iterations (objects can live for the duration of a script run), while keeping the representation GC-friendly for future mark/sweep or generational collectors.
*   Represent debug information using per-function line tables and local variable tables that map bytecode offsets back to source spans and variable names, following common JVM/Python-style designs.
*   Compile multiple IR modules into a single flattened bytecode program with a global function table and constant/globals arrays so that runtime calls are always direct by index (no per-call name or module lookup).
*   Support closures and first-class functions explicitly in IR and bytecode via heap-allocated closure objects, upvalue access instructions, and both direct and dynamic call opcodes.
*   Provide an optional IR optimization pipeline (e.g., constant folding, simple copy/dead-code elimination) that can be enabled to reduce bytecode size and branch count without changing language semantics.

## High-level architecture

```
C# source
   ↓
parse_csharp_source_spanned (bsharp_parser)
   ↓
syntax::ast::CompilationUnit (bsharp_syntax)
   ↓
(optional) semantic analysis (bsharp_analysis)
   ↓
[NEW] bsharp_vm_ir: IR module, functions, blocks, instructions
   ↓
[NEW] bsharp_vm: bytecode format + VM + runtime library
   ↓
ScriptEngine: compile + run in-process
```

## Stages of development

*   **Stage 1 – Architecture and constraints**
    *   Finalize requirements, semantics, and IR/VM design choices (stack vs register, function model, value representation).
    *   Decide supported C# subset for the initial scripting runtime.
*   **Stage 2 – IR model design**
    *   Define `IrModule`, `IrFunction`, `IrBlock`, `IrInstr`, and `ValueKind` types.
    *   Specify control-flow and data-flow conventions.
*   **Stage 3 – AST and analysis lowering**
    *   Implement lowering from `CompilationUnit` + semantic info to IR.
    *   Map key expressions/statements to IR instructions.
*   **Stage 4 – VM core**
    *   Implement the VM, stacks/frames, instruction dispatch, and error model.
    *   Define the bytecode encoding for the instruction set.
*   **Stage 5 – Scripting runtime and embedding**
    *   Build a `ScriptEngine` API, runtime library, and CLI integration.
    *   Establish testing strategy and roadmap for future features.

Each stage has its own planning document under `docs/planning/` with detailed scope, deliverables, and open questions.

## References

The design draws on prior art from:

*   **Writing Interpreters in Rust: a Guide**
    *   [Bytecode](https://rust-hosted-langs.github.io/book/chapter-interp-bytecode.html)
    *   [Virtual Machine: Architecture and Design](https://rust-hosted-langs.github.io/book/chapter-interp-vm-design.html)
    *   [VM implementation](https://rust-hosted-langs.github.io/book/chapter-interp-vm-impl.html)
*   **Lua 5 / register-based VMs** and general literature on stack vs register VM trade-offs.