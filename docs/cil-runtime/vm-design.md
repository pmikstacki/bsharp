# VM Design

## Runtime model
- **Value**: bool, i32/i64, f32/f64, ref(object), array, string handle.
- **Frame**: args, locals, eval stack, instruction pointer.
- **Heap**: opaque objects initially; field tables later.

## Interpreter loop
- Decode IL at offset; dispatch on opcode.
- Implement subset first: constants, loads/stores, arithmetic, comparisons, branches, returns, static calls.

## Loader and binder
- Load with `CilAssemblyView`.
- Resolve tokens to runtime handles (types, methods, fields).

## Exceptions (phase 2)
- Parse EH clauses.
- Unwinding: scan frames, match try/catch, run finally.

## Host interop
- Registry mapping specific MemberRefs/MethodDefs to Rust functions.
- Gradually add P/Invoke support (ImplMap) for native calls.
