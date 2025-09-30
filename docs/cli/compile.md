# Compilation Command

The `compile` command compiles C# source code to native binary using the Cranelift backend.

---

## Usage

```bash
bsharp compile <INPUT>
```

### Arguments

**`<INPUT>`** (required)
- Path to C# source file
- Must have `.cs` extension

---

## Examples

### Basic Compilation

```bash
# Compile C# file
bsharp compile Program.cs

# Output: Program (executable)
```

### Compilation Process

```bash
$ bsharp compile HelloWorld.cs
Parsing...
Generating IR...
Compiling...
Linking...
Successfully compiled: HelloWorld
```

---

## Compilation Pipeline

### 1. Parsing

Parse C# source code to AST:
```
HelloWorld.cs → AST (CompilationUnit)
```

### 2. IR Generation

Generate intermediate representation:
```
AST → Cranelift IR
```

### 3. Code Generation

Compile IR to machine code:
```
Cranelift IR → Native Code
```

### 4. Linking

Link with runtime and produce executable:
```
Native Code + Runtime → Executable
```

---

## Supported Features

### Currently Supported

- Basic types (int, bool, string)
- Simple expressions
- Method declarations
- Control flow (if, while, for)
- Function calls

### Planned Support

- Classes and objects
- Inheritance
- Generics
- LINQ
- Async/await
- Full .NET BCL integration

---

## Output

### Executable Format

- **Linux:** ELF binary
- **macOS:** Mach-O binary
- **Windows:** PE executable

### Output Location

Default: Same directory as input, without extension
```
Program.cs → Program
```

---

## Implementation Status

### Current State

**Status:** ⚠️ **Experimental** - Basic compilation infrastructure in place

**What Works:**
- Parsing C# to AST
- Basic IR generation
- Simple code generation with Cranelift

**What Doesn't Work Yet:**
- Full C# language support
- .NET runtime integration
- Standard library
- Complex types and generics

### Architecture

**Location:** `src/compiler.rs`, `src/codegen/`

```rust
pub struct Compiler {
    // Cranelift components
    module: ObjectModule,
    context: Context,
}

impl Compiler {
    pub fn compile(&mut self, cu: &CompilationUnit) -> Result<Vec<u8>> {
        // 1. Generate Cranelift IR from AST
        let ir = self.generate_ir(cu)?;
        
        // 2. Compile IR to machine code
        let code = self.compile_ir(ir)?;
        
        // 3. Link and produce executable
        let binary = self.link(code)?;
        
        Ok(binary)
    }
}
```

---

## Cranelift Backend

### Why Cranelift?

- **Fast Compilation:** Optimized for quick compilation
- **Portable:** Cross-platform code generation
- **Rust Integration:** Well-integrated with Rust ecosystem
- **Simpler than LLVM:** Easier to integrate initially

### Cranelift IR Example

```rust
// C# code:
// int Add(int a, int b) { return a + b; }

// Cranelift IR:
function u0:0(i32, i32) -> i32 {
block0(v0: i32, v1: i32):
    v2 = iadd v0, v1
    return v2
}
```

---

## Limitations

### Language Features

**Not Yet Supported:**
- Classes and objects
- Inheritance and polymorphism
- Generics
- LINQ
- Async/await
- Exceptions
- Garbage collection
- Most of .NET BCL

### Performance

- No advanced optimizations yet
- Basic code generation only
- Runtime performance not optimized

### Platform Support

- Tested primarily on Linux x86_64
- macOS and Windows support experimental

---

## Use Cases

### Current Use Cases

1. **Experimentation**
   - Test compilation pipeline
   - Explore Cranelift integration

2. **Simple Programs**
   - Basic algorithms
   - Mathematical computations
   - Simple CLI tools

### Future Use Cases

1. **Full C# Compilation**
   - Complete C# language support
   - .NET compatibility

2. **AOT Compilation**
   - Ahead-of-time compilation for .NET apps
   - Native binaries without runtime

3. **Embedded Systems**
   - Compile C# for embedded targets
   - No runtime overhead

---

## Comparison with Other Compilers

### vs. Roslyn (csc)

- **Roslyn:** Mature, complete, targets IL
- **BSharp:** Experimental, targets native code

### vs. CoreRT/NativeAOT

- **CoreRT:** Full .NET, production-ready
- **BSharp:** Simpler, learning project

### vs. IL2CPP

- **IL2CPP:** Unity's solution, IL → C++ → native
- **BSharp:** Direct C# → native

---

## Development Roadmap

### Phase 1: Foundation (Current)

- [x] Basic parser
- [x] Cranelift integration
- [x] Simple IR generation
- [ ] Basic type system

### Phase 2: Core Language

- [ ] Classes and objects
- [ ] Methods and properties
- [ ] Inheritance
- [ ] Basic generics

### Phase 3: Advanced Features

- [ ] Full generics support
- [ ] LINQ
- [ ] Async/await
- [ ] Exception handling

### Phase 4: Runtime Integration

- [ ] Garbage collector
- [ ] .NET BCL bindings
- [ ] Interop with C#/IL

---

## Contributing

### How to Help

1. **Test Compilation**
   - Try compiling simple C# programs
   - Report issues

2. **Implement Features**
   - Pick a language feature
   - Implement IR generation
   - Add tests

3. **Optimize**
   - Profile generated code
   - Implement optimizations
   - Benchmark improvements

### Getting Started

```bash
# Clone repository
git clone https://github.com/your-repo/bsharp.git
cd bsharp

# Build with Cranelift
cargo build --release

# Test compilation
cargo run -- compile examples/simple.cs

# Run tests
cargo test
```

---

## Troubleshooting

### Compilation Fails

```bash
$ bsharp compile MyProgram.cs
Error: Unsupported feature: classes
```

**Solution:** Check supported features list above

### Generated Binary Crashes

```bash
$ ./MyProgram
Segmentation fault
```

**Solution:**
- Compilation is experimental
- Report issue with minimal reproduction
- Check for unsupported features

### Linking Errors

```bash
Error: Failed to link: undefined symbol
```

**Solution:**
- Runtime integration incomplete
- Some features require runtime support

---

## Related Documentation

- [CLI Overview](./overview.md) - General CLI usage
- [Parse Command](./parse.md) - Parse C# to AST
- [Architecture](../development/architecture.md) - Compiler design

---

## References

- **Implementation:** `src/compiler.rs`, `src/codegen/`
- **Cranelift:** https://cranelift.dev/
- **Dependencies:** See `Cargo.toml` for Cranelift crates
