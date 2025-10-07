# Phases

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
- **Tasks**
  - Define `Value` (bool, i32/i64, f32/f64) and `Frame` (args, locals, eval stack, IP).
  - Loader: use `CilAssemblyView` to fetch method body bytes and decode to an instruction sequence (or iterate bytes with a small decoder; disassembler can guide operand formats).
  - Implement interpreter loop with dispatch for:
    - Stack/data: `ldc.i4`, `ldloc.*`/`stloc.*`, `ldarg.*`/`starg.*`.
    - Arithmetic/comparison: `add/sub/mul/div`, `rem`, `ceq/clt/cgt`.
    - Branching/return: `br`/`br.s`, `brtrue`/`brfalse`, `ret`.
    - Calls: `call` (static) to other methods in the same module.
  - Host intrinsics registry for a minimal IO function (e.g., `print(int)`), mapped by MemberRef/MethodDef.
- **Deliverables**
  - `bsharp_vm` crate with interpreter core and a test harness that executes a few hand-authored methods.
- **Acceptance**
  - Unit tests cover arithmetic, branching, locals/args, and static calls; all pass deterministically.

## Phase 2: Control flow + EH
- **Goals**
  - Support richer branching and basic exception handling semantics.
- **Tasks**
  - Implement `switch` (computed jump via table) and `leave`/`leave.s` semantics.
  - Parse EH clauses from method bodies (try/catch/finally ranges) via dotscope; attach to method runtime metadata.
  - Interpreter unwinding: on `throw`, walk frames to find matching catch; ensure `finally` executes on both normal and exceptional paths.
  - In emitter examples, use `MethodBodyBuilder` EH helpers (catch/finally with labels) to validate emission paths.
- **Deliverables**
  - VM supports try/catch/finally for a subset of exceptions (e.g., any-catch and typed catch); sample methods emitted and executed.
- **Acceptance**
  - Tests for: multi-branch `switch`, nested try/catch, try/finally, and `leave` to outer handlers.

## Phase 3: Object model (subset)
- **Goals**
  - Introduce instances, fields, instance calls, and arrays.
- **Tasks**
  - Runtime type info: ingest TypeDef/Field/MethodDef metadata; create minimal runtime type descriptors.
  - `newobj` allocation path (opaque object struct) and constructor invocation.
  - Field access: `ldfld`/`stfld` (instance), `ldsfld`/`stsfld` (static) with a simple memory model.
  - Instance calls: `call` (non-virtual) and a first pass at `callvirt` (null-check + dispatch via simple method table for non-interface types).
  - Arrays: `newarr`, `ldelem.*`, `stelem.*` for primitive arrays; bounds checks.
  - Strings: map `ldstr` to a host-managed intern pool for MVP.
- **Deliverables**
  - Object/array support sufficient to run small OOP-style samples.
- **Acceptance**
  - Tests: field get/set, constructors, instance methods, array creation/indexing, `ldstr` handling.

## Phase 4: Back-compat polish
- **Goals**
  - Improve compatibility with existing assemblies and interop.
- **Tasks**
  - Cross-module resolution: AssemblyRef/ModuleRef loading and method/field token binding across modules.
  - P/Invoke support guided by `ImplMap` and native import tables (`NativeImportsBuilder`), with a configurable host boundary.
  - Conversions: numeric widening/narrowing opcodes, `conv.*` semantics for supported primitives.
  - Minimal verification/guards: stack underflow/type checks, null checks for `callvirt`.
  - Performance scaffolding: profiling hooks and hotspots identification (future JIT hook).
- **Deliverables**
  - Ability to load small multi-assembly programs; basic P/Invoke demos.
- **Acceptance**
  - Tests covering AssemblyRef resolution, simple P/Invoke call, and conversion correctness.

## Phase 5: AST→IL emitter
- **Goals**
  - Emit IL/DLLs from our AST using dotscope; ensure parity with the VM and .NET runtime.
- **Tasks**
  - Map AST expressions to evaluation stack IL using `InstructionAssembler` helpers (e.g., `ldarg_0`, `add`, `ret`).
  - Use analysis CFG (`control_flow_graph`) to create labeled blocks and branch edges for `if/else`, loops, and `switch`.
  - Emit EH using label helpers for try/catch/finally when present in AST.
  - Create metadata entries with `AssemblyBuilder` and related table builders; attach method body bytes and local sig tokens from `MethodBodyBuilder::build`.
  - Write DLL and re-load via dotscope to validate metadata integrity.
  - Dual-run: execute on our VM and (optionally) via .NET host to compare outputs.
- **Deliverables**
  - `bsharp_il` crate capable of emitting a subset of C# methods to IL/PE.
- **Acceptance**
  - E2E tests: parse → emit → run on VM; conformance checks pass against .NET for agreed subset.
