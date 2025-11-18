# Stage 3 – AST and Analysis Lowering

This document describes the lowering pipeline from the C# AST (and optional semantic analysis) to the BSharp IR.

## Objectives

- Define the **entrypoints** for lowering from `CompilationUnit` to `IrModule`.
- Specify how key **expressions** and **statements** are mapped to IR instructions.
- Decide how to consume **semantic information** from `bsharp_analysis`.

## Lowering entrypoints

- Primary function:
  - `lower_compilation_unit(cu: &CompilationUnit, ctx: &SemanticContext?) -> Result<IrModule, CompileError>`.
- Responsibilities:
  - Create a script entry `IrFunction` from `CompilationUnit.top_level_statements`.
  - Discover and lower additional functions / methods (as needed for scripting profile).
  - Populate the IR function table and entry point metadata.

## Statement lowering

Initial mapping for the scripting subset:

- **Local variable declarations**
  - Allocate a local slot and assign it a register index.
  - If an initializer is present, lower the expression and emit `StoreLocal`.
- **Assignments**
  - Lower RHS expression to a register, then generate `StoreLocal`.
- **If / else**
  - Lower condition into a register `r_cond`.
  - Create blocks for `then`, `else` (if present), and `join`.
  - Emit `JumpIfFalse { cond: r_cond, target: else_or_join }`.
- **While loops**
  - Create blocks for `loop_header`, `loop_body`, and `after_loop`.
  - Emit condition check and conditional jump back to `loop_header` or to `after_loop`.
- **Return**
  - Lower return expression (if present) to a register, then emit `Return { value }`.
- **Expression statements**
  - Lower expression to a register and discard result unless required.

## Expression lowering

- **Literals**
  - Emit `LoadConst { dst, value }` using a fresh register.
- **Binary operators**
  - Lower operands into registers `a` and `b`.
  - Emit appropriate arithmetic or comparison instruction writing into `dst`.
- **Logical operators** (`&&`, `||`)
  - Use control flow to preserve short-circuit semantics.
- **Invocations**
  - Resolve the callee (via `SemanticContext` when available).
  - Lower arguments into registers and emit `Call` or `CallIntrinsic`.

## Use of semantic analysis

- Integrate with `bsharp_analysis` where beneficial:
  - **Name resolution**: map identifiers and member access expressions to function IDs or intrinsic IDs.
  - **Type information**: choose correct instruction variants (e.g. integer vs floating-point arithmetic) or insert runtime checks.
- Keep lowering robust when semantic info is partial:
  - Prefer explicit, early errors over guessed behavior.

## Error model

- Lowering failures return `CompileError` with:
  - A stable code (e.g. `BSC0101` – unresolved symbol in script lowering).
  - A clear message and source location.
- Do not swallow or downgrade semantic errors; propagate them into the scripting layer.

## Deliverables

- Lowering implementation in `bsharp_vm_ir` (or a dedicated lowering module).
- Tests covering:
  - Simple scripts (arithmetic, conditionals, loops).
  - Error scenarios (unresolved names, type mismatches in unsupported patterns).
