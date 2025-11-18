# Stage 2 – IR Model Design

This document specifies the IR data structures used as the compilation target for BSharp scripts.

## Objectives

- Define the **IR module, function, block, and instruction** types.
- Describe **value kinds** and minimal typing needed for runtime checks.
- Clarify basic control-flow conventions and relationships to bytecode.

## IR containers

- **IrModule**
  - Represents a single compilation unit for the VM.
  - Contains:
    - Function table (user-defined and intrinsic stubs).
    - Optional global values (e.g. constants, string table).
    - Metadata such as entry point (script `main`).
- **IrFunction**
  - Parameters: ordered list with value kinds and names.
  - Local variables: local descriptors, each mapped to a register index.
  - Basic blocks: sequence of `IrBlock` IDs with explicit control-flow edges.
  - Flags: e.g. `is_script_entry`, `is_intrinsic_stub`.
- **IrBlock**
  - `id` (unique within function).
  - `instructions: Vec<IrInstr>`.
  - Last instruction must be a **terminator** (branch, conditional branch, `Return`, `Unreachable`).

## Values and value kinds

- **ValueKind** (logical type, not host runtime type):
  - `Int32`, `Bool`, `String`, `Null`, `ObjectRef`.
  - Extensions possible for `Float64`, `Array`, etc.
- **ValueId / RegisterId**
  - Within a function, values live in **registers**.
  - Each instruction reads/writes registers; register indices will map directly to VM register indices or via a simple translation.

## Instruction set (IR level)

At the IR level, instructions are a typed enum. The initial set should cover:

- **Data movement and constants**
  - `LoadConst { dst, value }`
  - `Move { dst, src }`
  - `LoadGlobal { dst, global_id }`
- **Local variables**
  - `LoadLocal { dst, local }`
  - `StoreLocal { local, src }`
- **Arithmetic and comparison**
  - `Add`, `Sub`, `Mul`, `Div` with variants or operands describing the value kind.
  - `CmpEq`, `CmpNe`, `CmpLt`, `CmpLe`, `CmpGt`, `CmpGe`.
- **Logical operations**
  - `LogicalAnd`, `LogicalOr`, `LogicalNot` (may be compiled using control flow).
- **Control flow**
  - `Jump { target }`
  - `JumpIfTrue { cond, target }`
  - `JumpIfFalse { cond, target }`
  - `Return { value: Option<RegisterId> }`
- **Calls**
  - `Call { dst: Option<RegisterId>, function, args: Vec<RegisterId> }`
  - `CallIntrinsic { dst: Option<RegisterId>, intrinsic_id, args }`
  - `CallDynamic { dst: Option<RegisterId>, callee: RegisterId, args: Vec<RegisterId> }` – call through a value that may be a plain function or a closure.
- **Closures and upvalues**
  - `MakeClosure { dst, function, captured: Vec<CapturedVar> }` – create a heap-allocated closure object capturing locals and/or upvalues.
  - `LoadUpvalue { dst, upvalue_index }` – read a captured value from the current closure environment.
  - `StoreUpvalue { upvalue_index, src }` – write a value into the current closure environment.

## Control-flow conventions

- Each `IrFunction` forms a directed graph of blocks.
- Entry block is well-defined and must be reachable.
- Terminator instructions must be the last in each block.
- No implicit fallthrough: all control flow is explicit.

## Relationship to bytecode

- The IR is **close to the eventual bytecode**:
  - Most `IrInstr` map 1:1 or 1:small N to VM opcodes.
  - Register indices and function IDs can be preserved.
- A separate **lowering pass** from IR to concrete bytecode encoding will:
  - Pack operator and register operands into fixed-width opcode words.
  - Resolve block labels to instruction offsets.

## IR optimization pipeline

- Before lowering to bytecode, IR can be fed through an **optional optimization pipeline** that preserves semantics while simplifying the instruction stream:
  - Constant folding and simple constant propagation.
  - Copy propagation to eliminate redundant `Move` chains.
  - Dead code and dead block elimination for obviously unreachable instructions/blocks.
- These passes operate at the `IrFunction`/CFG level and are controlled via configuration on the ScriptEngine or compiler entrypoints.
- The default configuration should favor **clarity and safety**, enabling only well-understood, local optimizations initially.

## Error handling and diagnostics

- IR construction must return `Result<IrModule, CompileError>`.
- Compile-time errors should carry:
  - A code (e.g. `BSCxxxx` for scripting-compiler errors).
  - A message.
  - Source locations via `Spanned` nodes from the parser/analysis layer.

## Deliverables

- IR type definitions in the `bsharp_vm_ir` crate.
- Basic display/debug tooling for IR (e.g. human-readable dump for tests).
- Unit tests covering simple IR construction scenarios.
