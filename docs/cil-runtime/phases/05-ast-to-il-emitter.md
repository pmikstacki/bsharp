# Phase 5: AST→IL Emitter (dotscope)

## Goals
- Emit IL/DLLs from our AST using dotscope, structured by the analysis CFG, and validate by dual execution (our VM and .NET).
- Provide a deterministic, well-instrumented lowering pipeline with actionable diagnostics when constructs are unsupported.

## Detailed tasks
- **inputs** Inputs and prerequisites
  - AST from `bsharp_parser` nodes and expressions.
  - `control_flow_graph` from `bsharp_analysis/src/artifacts/control_flow_graph/` to create labeled blocks and branches.
  - Symbol/binding information for locals/params/method references (extend current indexing pass to a binder map).
- **setup** Emitter scaffolding
  - Create crate `bsharp_il` with modules: `lowering`, `metadata`, `types`, `errors`.
  - Initialize `CilAssembly` and `BuilderContext` for the target assembly; map namespaces/types from AST to metadata table entries.
- **types** Type mapping
  - Map AST types to ECMA-335 signatures: bool → `TypeSignature::Boolean`, int → `I4`, long → `I8`, float/double → `R4/R8`, void → `Void`.
  - For method signatures, build param and return type signatures; store calling convention (default).
- **locals** Locals and arguments
  - Pre-scan method bodies to declare locals: use `MethodBodyBuilder.local(name, TypeSignature)`; preserve stable local index order.
  - Provide a mapping LocalId → index for use during lowering.
- **expr** Expression lowering to stack IL
  - Literals: `ldc.i4/i8/r4/r8`, `ldnull`.
  - Identifiers: `ldarg.*`, `ldloc.*`/`stloc.*` based on binding.
  - Unary/binary: map to arithmetic/comparison opcodes; ensure operand type cohesion (insert `conv.*` only if explicit cast in AST; implicit conversions deferred).
  - Calls: static method calls use token resolution (MethodDef/MemberRef) with `call`.
  - Object/array constructs (post Phase 3): `newobj`, `newarr`, element access via `ldelem.*`/`stelem.*`.
- **stmt** Statement/control-flow lowering with CFG
  - Build `MethodBodyBuilder::implementation(|asm| { ... })` using labels:
    - Create labels for CFG basic blocks, emit code for each block’s statements, end with a branch/ret per terminator.
    - `if/else`: evaluate condition, `brtrue`/`brfalse` to labeled blocks, join at merge label.
    - Loops (while/for): loop head label, body label, back-edge `br` to head; `break`/`continue` map to appropriate labels.
    - `switch`: emit evaluation and `switch` with case label table; fall-through blocks join at a common label.
  - Returns: `ret` with value if any.
- **eh** Exception handling lowering
  - Use `MethodBodyBuilder` EH helpers (ranges via labels) to encode try/catch/finally; ensure try/handler label ranges match emitted code.
  - Typed catch: resolve exception type token; finally regions always run on exit paths.
- **metadata** Metadata emission
  - Use `AssemblyBuilder` to define assembly identity (name, version, culture).
  - Define `TypeDef` entries for emitted types; `MethodDef` entries with body RVA and local sig token returned by `build()`.
  - For external references, emit `TypeRef`/`MemberRef` entries; record `AssemblyRef` for external assemblies.
- **validation** Validation and diagnostics
  - Validate stack discipline during lowering (track expected stack depth per CFG block) and report mismatches early with AST locations.
  - Emit clear errors for unsupported AST constructs at this phase, suggesting later phase coverage.
- **persist** Persistence and round-trip
  - Finalize via `context.finish()` and `write_to_file("out.dll")`.
  - Reload with `CilAssemblyView` and disassemble selected methods for snapshot testing.
- **dual-run** Dual execution
  - Execute emitted methods on `bsharp_vm` and, optionally, on .NET via hosting or a test harness; compare outputs for conformance.

## Deliverables
- `bsharp_il` crate capable of emitting a subset of C# methods to IL/PE using dotscope.
- E2E tests: parse → emit → reload/disassemble → run in VM → assert results; optional .NET run for parity.

## Acceptance criteria
- Emitted DLLs are valid and loadable by dotscope; method bodies decode to expected instruction sequences.
- For the supported subset, VM and .NET produce identical observable results.

## References
- dotscope: `MethodBodyBuilder`, `InstructionAssembler`, `CilAssembly`, `BuilderContext`, metadata table builders.
- ECMA-335 Partitions II (metadata) and III (CIL instruction set) for signatures/opcodes and EH encoding.
- Microsoft Learn: `System.Reflection.Emit.OpCodes` pages for per-opcode semantics and short forms.
