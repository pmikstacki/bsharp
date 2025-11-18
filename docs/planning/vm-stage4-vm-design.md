# Stage 4 – VM Core Design and Implementation

This document covers the design and implementation of the VM that executes BSharp bytecode derived from the IR.

## Objectives

*   Define the **VM state** (stacks, frames, instruction pointer, globals).
*   Specify the mapping from IR instructions to **concrete bytecode opcodes**.
*   Implement the **dispatch loop** and runtime error model.

## VM state

*   **Thread / VM struct**
    *   Holds all mutable execution state for a single script execution:
        *   Register stack (homogeneous array of value slots).
        *   Call frame stack.
        *   Global storage (if needed).
        *   Heap storage for `ObjectRef` and closure objects.
        *   Instruction pointer and current function/program references.
*   **Value representation**
    *   `enum Value { Int32(i32), Bool(bool), String(String), Null, ObjectRef(ObjectHandle), Function(FunctionId), ... }`.
    *   `ObjectRef` is represented as a small handle into a heap managed by the VM (e.g. `struct ObjectHandle { index: u32, generation: u16 }`).
    *   Heap objects are stored in a `Vec<Option<HeapObject>>` with a free list for reuse; this behaves like an arena per script run but remains GC-friendly for future mark/sweep or generational collectors.
*   **Call frames**
    *   Each frame stores:
        *   Function reference.
        *   Return instruction pointer.
        *   Base register index (window into the register stack).
    *   Follow the separation from "Writing Interpreters in Rust": register stack vs call frame stack.
*   **Closures**
    *   Closure objects live on the heap and contain:
        *   The target `FunctionId`.
        *   An environment of captured values or cells (upvalues).
    *   The VM must maintain a notion of the "current closure" when executing functions that were created as closures, so that upvalue access instructions can read/write the correct environment.

## Bytecode encoding

**Opcode word**

*   Fixed-width (e.g. 32-bit) word containing:
    *   Opcode ID (8 bits).
    *   Up to three 8-bit register operands or small immediates.

**Instruction stream**

*   Each function has a contiguous stream of opcode words.
*   VM maintains an instruction pointer into this stream.

**Mapping from IR**

*   Most `IrInstr` translate directly to one or a few opcodes.
*   Jumps and branches use resolved offsets instead of block labels.

**Opcode reference**

*   The full, normative opcode list and operand formats are defined in `vm-opcode-table.md`.
*   This Stage 4 document describes how those opcodes are interpreted and how they relate to VM data structures.

## Program layout and multi-module support

*   At runtime, the VM executes a **single flattened program representation** (e.g. `BytecodeProgram`) instead of separate per-module bytecode units:
    *   A global function table (`Vec<BytecodeFunction>`).
    *   Shared constant pool and global storage arrays.
    *   Debug information grouped per function.
*   Multiple IR modules produced at compile time are **linked** into this single program:
    *   Cross-module calls are resolved ahead of time to direct `FunctionId` indices into the global function table.
    *   There is no per-call name or module lookup in the VM; all calls are direct by index for performance.

## Dispatch loop

*   Classic fetch-decode-execute loop:
    *   Fetch opcode at IP.
    *   Decode opcode and operands.
    *   Execute operation, mutating registers, frames, IP.
*   Maintain a clean separation of responsibilities:
    *   Decoding layer vs operation implementation.
    *   Clear error paths via `Result` types.

## Runtime errors and panics

*   **Error types**
    *   Define `VmError` with variants for:
        *   Type mismatch.
        *   Divide by zero.
        *   Invalid instruction or opcode.
        *   Out-of-bounds access.
    *   Never panic for user-level errors; propagate `VmError` instead.
*   **Error propagation**
    *   Top-level `run` API returns `Result<Value, VmError>`.
    *   Leave host-level decisions (logging, termination) to the caller.

## Debug information

*   To support diagnostics and a future debugger, the VM consumes debug metadata generated during bytecode compilation:
    *   **Line tables** mapping bytecode offsets to source spans (or line/column via the span DB).
    *   **Local variable tables** mapping register indices and live ranges to variable names, value kinds, and declaration spans.
*   When a `VmError` occurs, the VM reports the function and instruction offset, and higher layers can use the debug tables to map these back to user-facing source locations and variable names.

## Testing strategy

*   Unit tests for individual opcodes and small instruction sequences.
*   Integration tests using compiled IR from small scripts.
*   Fuzzing or property-based tests (optional, later) to stress the VM.

## Deliverables

*   `bsharp_vm` crate with VM core implemented.
*   Mapping from IR to concrete bytecode implemented.
*   Tests validating core opcodes and control-flow constructs.