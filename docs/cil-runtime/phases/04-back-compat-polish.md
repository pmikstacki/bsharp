# Phase 4: Back-Compat Polish

## Goals
- Improve compatibility with existing .NET assemblies and platform interop.
- Add conversions, cross-module resolution, P/Invoke, and basic verification/guards.

## Detailed tasks
- **multimodule** Cross-module resolution
  - Load referenced modules/assemblies using `AssemblyRef`/`ModuleRef` from metadata (dotscope view per file).
  - Build a resolver that maps tokens across modules to runtime handles (types, fields, methods).
  - Cache resolution results to avoid repeated metadata traversals.
- **pinvoke** P/Invoke via ImplMap
  - Parse `ImplMap` table entries; bind to host-native call surface under a feature gate.
  - Respect `PInvokeAttributes` (calling convention, char set, SetLastError) as possible in MVP; document limitations.
  - Provide safe wrappers and a config switch to disable native interop by default.
- **conversions** Numeric conversions
  - Implement `conv.*` opcodes for supported primitives (`conv.i1/i2/i4/i8`, `conv.u1/u2/u4/u8`, `conv.r4/r8`, `conv.ovf.*` deferred or limited).
  - Define truncation/overflow semantics; for `ovf` forms, return error or panic per configuration if overflow occurs.
- **verification** Minimal verification/guards
  - Runtime checks: stack underflow/overflow, type compatibility on arithmetic and stores, null checks on `callvirt`.
  - Optional pre-execution pass: quick stack-effect scan for obvious inconsistencies; configurable strict mode.
- **strings** String interop improvements
  - Expand `ldstr` handling: intern pool + provide host shims for basic concatenation/compare/print if needed.
  - Consider mapping a minimal subset of `System.String` methods to host intrinsics.
- **profiling** Performance scaffolding
  - Add simple counters/timers around interpreter dispatch, calls, allocations.
  - Prepare seam for alternate backend (Cranelift) behind a trait.

## Deliverables
- Multi-assembly loader + resolver.
- P/Invoke demo guarded by a flag.
- Conversion opcodes implemented with tests.
- Basic verification errors with actionable diagnostics.

## Acceptance criteria
- Successfully load and execute sample assemblies with cross-module calls.
- P/Invoke sample works when enabled and is safely disabled by default.
- Conversions tested across boundary values; verification catches common issues in tests.

## References
- dotscope metadata: `AssemblyRef`, `ModuleRef`, `ImplMapBuilder`/`NativeImportsBuilder` (for emission tests).
- ECMA-335 Partition II (metadata resolution) and III (`conv.*`, `calli` deferred).
- Microsoft Learn: `PInvokeAttributes`, conversion opcode semantics.
