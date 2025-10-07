# dotscope Guide (Read/Emit)

## Reading assemblies
```rust
use dotscope::CilAssemblyView;
let view = CilAssemblyView::from_file("input.dll")?;
// enumerate types/methods, read method bodies
```

## Emitting assemblies
```rust
use dotscope::{CilAssembly, MethodBodyBuilder, BuilderContext};
let view = dotscope::CilAssemblyView::from_file("base.dll")?;
let mut assembly = CilAssembly::new(view);
let mut ctx = BuilderContext::new(assembly);

let (body, locals_sig) = MethodBodyBuilder::new()
    .max_stack(2)
    .implementation(|asm| {
        asm.ldarg_0()?.ldarg_1()?.add()?.ret()?; Ok(())
    })
    .build(&mut ctx)?;

// Create/modify metadata tables (Assembly, TypeDef, MethodDef, etc.)
// Then write file:
ctx.assembly.write_to_file("output.dll")?;
```

## Metadata builders
- `AssemblyBuilder`: create assembly identity.
- `NativeImportsBuilder`/`NativeExportsBuilder`: import/export tables.
- `ImplMapBuilder`: P/Invoke mapping.

## Method bodies and EH
- Build tiny or fat bodies automatically.
- Use label-based helpers for EH (catch/finally) when lowering structured constructs.
