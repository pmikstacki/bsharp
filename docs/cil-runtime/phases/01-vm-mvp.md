# Phase 1: VM MVP

## Goals
- Execute simple static methods with primitives and basic control flow.
- Deterministic interpreter with clear errors for unsupported opcodes.

## Detailed tasks
- **[runtime-values]** Define runtime types
  - `Value`: bool, i32/i64, f32/f64, (opaque) object ref, (opaque) string handle (planned), Null.
  - Numeric conversions (implicit in MVP avoided; explicit `conv.*` deferred to Phase 4).
- **[frames]** Implement frames and call stack
  - Frame: args, locals, evaluation stack (Vec<Value>), instruction pointer (u32 offset), method handle.
  - Stack discipline: push/pop per opcode; underflow/overflow checks with good diagnostics (offset, opcode).
- **[loader]** Load IL and metadata with dotscope
  - Use `CilAssemblyView::from_file(path)` to load DLLs.
  - For a `MethodDef`, obtain method body bytes + header; optionally disassemble via `assembly` module for debugging.
- **[decoder]** Instruction decoding
  - Implement a minimal decoder (or iterate via dotscope disassembly) for:
    - `ldc.i4`, `ldc.i8`, `ldc.r4`, `ldc.r8`, short forms where convenient.
    - `ldloc.*`/`stloc.*`, `ldarg.*`/`starg.*` (including short forms `_0`..`_3`).
    - `add`, `sub`, `mul`, `div`, `rem` for ints/floats (MVP: matching operand types only).
    - `ceq`, `clt`, `cgt` → bool.
    - Branching: `br`/`br.s`, `brtrue`/`brfalse` with signed relative offsets.
    - `ret`.
    - `call` (static) within same module.
- **[calls]** Static calls
  - Resolve `MethodDef`/`MemberRef` to a method body within the same `CilAssemblyView`.
  - Create new frame with provided args; return pushes result if any.
- **[host-intrinsics]** Minimal IO
  - Register a small set of host intrinsics (e.g., `Console.WriteLine(int)` equivalent) mapped by token to Rust closures.
  - MVP: avoid strings; provide `print_i32` style helper.

## Deliverables
- `bsharp_vm` crate with:
  - Value model, frame/stack, interpreter loop, loader.
  - Unit tests covering arithmetic, comparisons, branches, locals/args, static calls, `ret`.

## Acceptance criteria
- Deterministic execution of hand-authored IL methods created via dotscope’s `MethodBodyBuilder`.
- Clear error on unknown opcode with IL offset and mnemonic (if available).

## References
- dotscope `CilAssemblyView`, `assembly` module, method body access.
- ECMA-335 Partition III (CIL Instruction Set).
- Microsoft Learn: `System.Reflection.Emit.OpCodes` for per-opcode semantics and short forms.
