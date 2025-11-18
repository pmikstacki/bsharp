# Stage 5 – Scripting Runtime and Embedding

This document describes the scripting runtime API exposed to hosts and the integration with the CLI.

## Objectives

- Design a **ScriptEngine** or equivalent API for embedding.
- Define a minimal **runtime library** (intrinsics) for scripts.
- Integrate scripting into `bsharp_cli` for easy experimentation.

## ScriptEngine API

- Responsibilities:
  - Accept C#-like script source.
  - Compile it using the existing front-end and new IR/VM pipeline.
  - Execute scripts and return results or errors.
- Candidate API surface:
  - `compile(source: &str) -> Result<ModuleHandle, CompileError>`.
  - `run(module: ModuleHandle) -> Result<Value, VmError>`.
  - Optional: `register_host_function(name: &str, fn: HostFn)` for controlled host interop.

## Runtime library (intrinsics)

- Start with a small, well-defined set of built-ins:
  - `print` / `println` for text output.
  - Basic math or utility functions if needed.
- Implementation notes:
  - Represent intrinsics in IR as `CallIntrinsic` instructions.
  - Map intrinsic IDs to Rust functions inside the VM runtime.
  - Do not expose arbitrary host APIs by default; keep the surface secure and explicit.

## CLI integration

- Extend `bsharp_cli` with a `run` or `script` subcommand:
  - Example: `bsharp run script.cs`.
- Behavior:
  - Parse the input file using `bsharp_parser`.
  - Lower to IR and compile to bytecode.
  - Execute via the VM and report the final result or error.

## Testing and examples

- Provide example scripts under a dedicated directory (e.g. `examples/vm-scripts/`).
- Add documentation examples in the docs:
  - Show how to run a script via CLI.
  - Show how to embed the ScriptEngine into another Rust binary.

## Future extensions

- Add support for:
  - Functions with parameters and return values.
  - Simple data structures (arrays, dictionaries).
  - Error handling constructs and possibly exceptions.
  - Async or cooperative scheduling (separate design document if pursued).

## Deliverables

- Public ScriptEngine API in `bsharp_vm`.
- CLI subcommand wired to the VM runtime.
- Documented examples and tests validating end-to-end script execution.
