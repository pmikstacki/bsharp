# Phases

<!-- This file mirrors docs/cil-runtime/phases.md to make "Phases" a directory chapter for mdBook. -->

## Phase 0: Dotscope spike
- **Goals**
  - Validate dotscope API surface for PE/metadata/IL read & emit.
  - Establish patterns for builder context and method body assembly.
- **Tasks**
  - Load an assembly with `CilAssemblyView::from_file()`; enumerate types/methods and obtain method bodies.
  - Disassemble a method using `assembly` module facilities (instruction stream + operands) for inspection.
  - Create a mutable assembly: `let mut asm = CilAssembly::new(view); let mut ctx = BuilderContext::new(asm);`.
  - Build a method body with `MethodBodyBuilder::implementation(|asm: &mut InstructionAssembler| { ... })` and `build(&mut ctx)`.
  - Persist using `ctx.finish().write_to_file("out.dll")` or `CilAssembly::write_to_file`.
- **Deliverables**
  - Example program that prints disassembly for a sample method.
  - Example that emits a simple `add(i32,i32)` method and writes a valid DLL.
- **Acceptance**
  - Emitted DLL round-trips: reload with dotscope and method body bytes decode with expected instructions.

## Phase 1: VM MVP
- **Goals**
  - Execute simple static methods with primitives and basic control flow.
- **Tasks** (see per-phase doc for details)

## Phase 2: Control flow + EH
- **Goals**
  - Support richer branching and basic exception handling semantics.

## Phase 3: Object model (subset)
- **Goals**
  - Introduce instances, fields, instance calls, and arrays.

## Phase 4: Back-compat polish
- **Goals**
  - Improve compatibility with existing assemblies and interop.

## Phase 5: AST→IL emitter
- **Goals**
  - Emit IL/DLLs from our AST using dotscope; ensure parity with the VM and .NET runtime.
