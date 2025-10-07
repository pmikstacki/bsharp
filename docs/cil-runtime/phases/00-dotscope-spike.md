# Phase 0: Dotscope Spike (Read/Emit)

## Goals
- **Validate** dotscope API surface for PE/metadata/IL read & emit.
- **Establish** patterns for builder context and method body assembly.

## Detailed tasks
- **[load-assembly]** Load an assembly and enumerate content
  - Use `CilAssemblyView::from_file(path)` to open an input DLL.
  - Enumerate types/methods via metadata tables; for each managed method fetch body bytes if present.
  - Disassemble one method using dotscope's `assembly` module instruction processing to inspect opcodes and operands.
- **[mutable-assembly]** Create a mutable assembly for editing and writing
  - Wrap the view with `CilAssembly::new(view)` and `BuilderContext::new(assembly)`.
  - Confirm `BuilderContext::finish()` returns `CilAssembly` and supports `write_to_file()`.
- **[emit-method]** Build a simple method body
  - Use `MethodBodyBuilder::new().implementation(|asm| { asm.ldarg_0()?.ldarg_1()?.add()?.ret()?; Ok(()) })`.
  - Call `.build(&mut context)` to produce `(bytes, local_sig_token)`.
  - Create necessary metadata (assembly, type, method) using table builders; attach method body.
  - Persist to `output.dll` using `write_to_file`.
- **[roundtrip]** Reload and verify
  - Load `output.dll` with `CilAssemblyView`.
  - Disassemble the emitted method and verify expected instruction sequence.

## Deliverables
- **disasm example**: CLI or test that prints IL for a chosen method.
- **emit example**: CLI or test that writes `add(i32,i32)` method into a DLL and round-trips via disassembly.

## Acceptance criteria
- Emitted DLL round-trips: reload and decode bytes into expected instructions.
- Examples run deterministically and are added to CI.

## References
- dotscope assembly module (disassembly/assembly): docs.rs/dotscope/latest/dotscope/assembly/index.html
- `CilAssembly`, `BuilderContext`: docs.rs/dotscope/latest/dotscope/struct.CilAssembly.html, docs.rs/dotscope/latest/dotscope/struct.BuilderContext.html
- `MethodBodyBuilder`, `InstructionAssembler`: docs.rs/dotscope/latest/dotscope/struct.MethodBodyBuilder.html, docs.rs/dotscope/latest/dotscope/assembly/struct.InstructionAssembler.html
