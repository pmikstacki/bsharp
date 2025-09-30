# Dotscope + CIL Integration Tracking

This document tracks milestones, tasks, status, and validation for integrating `dotscope` into BSharp to enable CIL read and output features.

- Plan: `docs/planning/dotscope-cil-plan.md`
- Scope: Read, disassemble, metadata inspect, encode IL method bodies, and eventually emit PE assemblies.

## Milestones

- M1 — CIL Read + Disassemble (CLI)
- M2 — AST→IL Encoding (Backend Artifact)
- M3 — PE Packaging and Round-Trip Validation

## Status Summary

- Overall: Planned
- Current Milestone: M1

## Work Breakdown Structure (WBS)

### M1 — CIL Read + Disassemble (CLI)

- [ ] Add `dotscope = "0.4"` dependency and `cil_backend` feature in `Cargo.toml`.
- [ ] Implement `src/cil/read.rs` helper API (read+validate wrappers).
- [ ] CLI: `bsharp cil disasm <path>` command in `src/cli/commands/cil_disasm.rs`.
- [ ] CLI: `bsharp cil meta <path>` command in `src/cli/commands/cil_meta.rs`.
- [ ] Tests: `tests/cli/cil_disasm_tests.rs` with snapshot of decoded IL.
- [ ] Tests: `tests/cli/cil_meta_tests.rs` with snapshot of headers/streams.
- [ ] Docs: usage examples in `docs/planning/dotscope-cil-plan.md` and CLI docs stub in `docs/cli/`.

### M2 — AST→IL Encoding (Backend Artifact)

- [ ] Introduce backend trait and `Artifact` enum in `src/codegen/backend.rs`.
- [ ] Refactor current Cranelift code (`src/codegen/mod.rs`) to implement `Backend`.
- [ ] Add `src/codegen/cil/` with:
  - [ ] `mod.rs` — backend glue.
  - [ ] `lower.rs` — AST→IL lowering.
  - [ ] `types.rs` — type/signature helpers.
  - [ ] `artifact.rs` — `CilArtifact` and `CilMethodArtifact`.
- [ ] Implement minimal IL lowering:
  - [ ] Return statements for void/non-void.
  - [ ] Literals and arithmetic (add/sub/mul/div).
  - [ ] Locals load/store.
  - [ ] Simple `if`/`while` using labels and branches.
  - [ ] Direct `call` invocation for simple methods.
- [ ] Instruction encoding with `InstructionEncoder` and label fixups.
- [ ] CLI: `bsharp compile --target cil` to emit `.il.txt` and `.cilbundle.json`.
- [ ] Tests: `tests/codegen/cil_basic_methods.rs` snapshot IL for small programs in `tests/cs_test_cases/`.

### M3 — PE Packaging and Round-Trip

- [ ] Investigate dotscope writer/packaging support (if exists) or alternative PE+ECMA-335 writers.
- [ ] Implement minimal metadata and PE emitter to package `CilArtifact` into `.dll`.
- [ ] Round-trip tests: compile→emit→read with `CilObject`→compare IL/metadata basics.
- [ ] Add validation step using `ValidationConfig`/`ValidationEngine` where applicable.

## Decision Log

- Backend Strategy: Parallel targets (Cranelift native object vs. dotscope CIL) via trait.
- Emission Strategy: Phase IL-first; package into PE once a writer is viable.

## Risks

- **PE writer availability**: May require custom or third-party emitter.
  - Mitigation: Two-stage pipeline; keep IL artifacts stable.
- **Stack correctness for IL**: Incorrect `maxstack`/flow can break runtime.
  - Mitigation: Use `StackBehavior` and conservative estimates; validation tests.
- **Metadata complexity**: Tokens/signatures for advanced scenarios.
  - Mitigation: Start with primitives; expand incrementally with tests.

## Test Matrix (initial)

- Read/Disasm (M1):
  - [ ] Small library with 1–3 methods (arithmetics).
  - [ ] System library sample (subset) for header/streams read.
- Encode (M2):
  - [ ] Method: `int Add(int a,int b)`.
  - [ ] Method: `void PrintConst()`.
  - [ ] If/else and simple while loop.
  - [ ] Direct static call.
- Packaging (M3):
  - [ ] DLL with single type/method round-trips.

## Acceptance Criteria

- M1:
  - [ ] `bsharp cil disasm foo.dll` prints valid IL.
  - [ ] `bsharp cil meta foo.dll` prints COR20 and stream/table info.
- M2:
  - [ ] `bsharp compile --target cil file.cs` produces `.il.txt` and `.cilbundle.json`.
  - [ ] Snapshot tests stable for simple methods.
- M3:
  - [ ] Emit `.dll` and read back with `CilObject` producing matching IL for sample methods.

## Cross-References

- Existing codegen: `src/codegen/mod.rs`
- Parser/AST: `src/syntax/` and `src/parser/`
- CLI structure: `src/cli/`

## Progress Log

- 2025-09-30: Created planning docs and initial plan/tracking in `docs/planning/`.
