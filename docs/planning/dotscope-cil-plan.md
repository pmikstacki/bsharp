# Dotscope + CIL Integration Plan

This document specifies how to integrate the `dotscope` crate to add CIL (Common Intermediate Language) reading and output capabilities to BSharp. It is grounded in the current codebase and the official dotscope APIs.

- Codebase paths cited below refer to:
  - Parser and AST: `src/parser/`, `src/syntax/`
  - Existing backend: `src/codegen/mod.rs` (Cranelift)
  - CLI: `src/cli/`

## Goals

- **CIL Read**: Load .NET assemblies from disk/memory, inspect metadata, and disassemble method bodies.
- **CIL Output**: Generate IL method bodies from BSharp AST and provide an artifact suitable for packaging into a .NET PE assembly.
- **CLI Tools**: Add user-facing commands to read/disassemble assemblies and to emit IL from source.
- **Phaseable Delivery**: Deliver value early with read/disasm; follow with IL encoding; finalize with PE emission.

## Dotscope APIs (grounded references)

- Crate docs: `https://docs.rs/dotscope/latest/dotscope/`
- Re-exports (from `dotscope` index):
  - `dotscope::CilAssemblyView`
  - `dotscope::CilObject`
  - `dotscope::ValidationConfig`, `dotscope::ValidationEngine`
  - Streams and headers: `Blob`, `Strings`, `Guid`, `TablesHeader`, `UserStrings`, etc.
- Reading assemblies:
  - `CilObject::from_file(&Path) -> Result<CilObject>`
  - `CilObject::from_mem(Vec<u8>) -> Result<CilObject>`
  - Validation variants: `from_file_with_validation`, `from_mem_with_validation`
  - Accessors (example): `CilObject::cor20header()`
  - Lightweight view: `CilAssemblyView::from_file(&Path)`
- IL disassembly/assembly (`dotscope::assembly`):
  - Types: `CilInstruction`, `Instruction`, `BasicBlock`, `InstructionAssembler`, `InstructionEncoder`, `LabelFixup`, `StackBehavior`
  - Functions: `decode_stream`, `decode_instruction`, `decode_blocks`

## Scope and Non-goals

- In-scope:
  - Read and validate assemblies; print metadata fundamentals and IL.
  - Lower a subset of BSharp AST to IL, initially for primitives, arithmetic, calls, returns, branches.
  - Produce an IL artifact that can later be packaged into a PE.
- Out-of-scope (initial phases):
  - Full metadata/PE writer if not provided by `dotscope`.
  - Advanced C# features (generics, exceptions, async, etc.) beyond minimal IL necessary for samples.

## Architecture Changes

### 1. Dependency and Features

- `Cargo.toml`:
  - Add: `dotscope = "0.4"`
  - Add feature: `cil_backend = []`

### 2. Backend Abstraction

- New trait: `src/codegen/backend.rs`
  ```rust
  pub enum Artifact {
      Object(Vec<u8>),      // Existing Cranelift object bytes
      Cil(CilArtifact),     // New IL artifact (see below)
  }

  pub trait Backend {
      fn compile(&mut self, cu: &syntax::ast::CompilationUnit) -> Result<Artifact, CodegenError>;
  }
  ```
- Refactor Cranelift implementation in `src/codegen/mod.rs` to implement `Backend`.

### 3. CIL Backend

- New module: `src/codegen/cil/`
  - `mod.rs`: glue implementing `Backend`
  - `lower.rs`: AST→IL lowering
  - `types.rs`: type/signature helpers
  - `artifact.rs`: `CilArtifact` definition

- `CilArtifact` (Phase 1):
  ```rust
  pub struct CilArtifact {
      pub assembly_name: String,
      pub methods: Vec<CilMethodArtifact>,
  }

  pub struct CilMethodArtifact {
      pub full_name: String,        // Namespace.Type::Method(args)
      pub il_bytes: Vec<u8>,        // encoded with InstructionEncoder
      pub max_stack: u16,
      pub locals_sig: Option<Vec<MethodLocalType>>, // future use
  }
  ```

### 4. CIL Read Helpers

- New helper module `src/cil/read.rs`:
  ```rust
  pub fn read_cil(path: &std::path::Path, cfg: dotscope::ValidationConfig) -> anyhow::Result<dotscope::CilObject>;
  ```

### 5. CLI Additions

- New top-level `cil` group in `src/cli/commands/`:
  - `cil_disasm.rs`
    - `bsharp cil disasm <path> [--method <Type.Method>] [--json] [--validate minimal|production|comprehensive]`
    - Uses `CilObject::from_file[_with_validation]` and `assembly::decode_stream`.
  - `cil_meta.rs`
    - `bsharp cil meta <path> [--tables] [--heaps] [--cor20] [--stats]`
    - Reads COR20 header, tables header, and stream sizes (`Strings`, `Blob`, `Guid`).
- Extend existing `compile` command (`src/cli/commands/compile.rs`) to accept `--target cil`:
  - Select backend based on flag (`cil_backend` feature must be enabled).
  - Output `.il.txt` and/or `.cilbundle.json` (intermediate) in Phase 1.

## AST→IL Lowering (initial coverage)

- Types: map `syntax::nodes::types::PrimitiveType` to IL value kinds.
- Methods: build minimal headers (locals, maxstack). Compute or conservatively set `maxstack`.
- Expressions:
  - Literals → `ldc.i4`, `ldc.i8`, `ldc.r4`, `ldc.r8`, `ldstr`
  - Variables/locals → `ldloc`, `stloc`
  - Binary arithmetic → `add`, `sub`, `mul`, `div`, `rem`
  - Comparisons/branching → `ceq`, `cgt`, `clt` + `brtrue`, `brfalse`, `br`
  - Member access/invocation → `call` (initially non-virtual)
  - Assignment → `stloc`/`stfld`
- Statements:
  - `ReturnStatement` → `ret` (optionally preceded by value)
  - `IfStatement` → label + branch form (`brfalse` / `brtrue`)
  - Loops (`while`, `for`) → labels + conditional branches

- Use `InstructionAssembler` and `LabelFixup` for branch targets; finalize with `InstructionEncoder` to bytes.

## Phases and Deliverables

- **Phase A: Read + Disassemble (CLI)**
  - Implement `cil disasm` and `cil meta`.
  - Test on sample assemblies.

- **Phase B: AST→IL Encoding (Artifact)**
  - Implement minimal `CilBackend` encoding void and primitive-return methods.
  - Support literals, arithmetic, calls, returns, simple branches.
  - Emit `.il.txt` and `.cilbundle.json` artifacts.

- **Phase C: PE Emission**
  - Investigate dotscope writer support; if available, integrate to emit `.dll/.exe`.
  - If not, integrate a PE + ECMA-335 metadata writer (or build minimal one) to package `CilArtifact`.
  - Add round-trip tests: emit → read back with dotscope → validate IL.

## Risks and Mitigations

- **PE writer availability**: Proceed with two-step pipeline; provide stable IL artifacts first.
- **Stack correctness**: Use `StackBehavior` and defensive maxstack; add validation in CI.
- **Type/metadata complexity**: Start with primitives and expand incrementally.

## Testing Strategy

- All tests under `tests/` (project convention):
  - `tests/cli/cil_disasm_tests.rs`: snapshot disassembly of known assemblies in `tests/fixtures/assemblies/`.
  - `tests/cli/cil_meta_tests.rs`: snapshot headers/streams stats.
  - `tests/codegen/cil_basic_methods.rs`: compile tiny C# cases in `tests/cs_test_cases/` to IL artifacts; snapshot instructions.
  - Later: round-trip emission tests.

## Acceptance Criteria

- `bsharp cil disasm foo.dll` prints valid IL for multiple methods.
- `bsharp compile --target cil file.cs` produces IL artifact and readable `.il.txt` for simple methods.
- Documentation added under `docs/planning/` and `docs/cli/`.
- CI snapshots stable for basic samples.
