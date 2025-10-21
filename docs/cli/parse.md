# Parse Command

The `parse` command parses C# source code and prints a textual AST tree representation to stdout.

---

## Usage

```bash
bsharp parse <INPUT> [--errors-json] [--no-color] [--lenient]
```

### Arguments

**`<INPUT>`** (required)
- Path to C# source file
- Must have `.cs` extension
- File must exist and be readable

### Options

**`--errors-json`**
- Print a machine-readable JSON error object to stdout on parse failure and exit non-zero
- Disables pretty error output

See: [Parse Errors JSON Output](./errors-json.md)

**`--no-color`**
- Disable ANSI colors in pretty error output

**`--lenient`**
- Enable best-effort recovery mode (default is strict)

---

Note: The `--output` option is currently not used; the command writes the textual tree to stdout.

## Examples

### Basic Parsing

```bash
# Parse and print textual AST tree to stdout
bsharp parse Program.cs
```

### Batch Parsing

```bash
# Parse all C# files in a directory (prints textual trees)
for file in src/**/*.cs; do
    bsharp parse "$file"
done
```

---

## Output

The command prints a human-readable textual tree describing the AST. For visualization outputs (Mermaid/DOT), use the `tree` command.

---

## Error Handling

### Parse Errors

```bash
$ bsharp parse InvalidSyntax.cs
Error: Parse failed

0: at line 5, in keyword "class":
public clas MyClass { }
       ^--- expected keyword "class"

1: in context "class declaration"
```

**Error Information:**
- Line and column numbers
- Context stack showing where parsing failed
- Expected vs. actual input
- Helpful error messages

### File Errors

```bash
$ bsharp parse NonExistent.cs
Error: Failed to read file: NonExistent.cs
Caused by: No such file or directory (os error 2)
```

---

## Use Cases

### 1. Syntax Validation

```bash
# Check if file has valid syntax
if bsharp parse MyFile.cs > /dev/null 2>&1; then
    echo "Syntax OK"
else
    echo "Syntax Error"
    exit 1
fi
```

### 2. AST Inspection

```bash
# Parse and inspect AST structure
bsharp parse MyClass.cs --output ast.json
jq '.declarations[0].Class.name.name' ast.json
```

### 3. Documentation Input

```bash
# Parse C# and generate documentation using your own script
bsharp parse MyFile.cs --output ast.json
python generate_docs.py ast.json > docs.md
```

### 4. Static Analysis

```bash
# Parse and analyze with custom tool
bsharp parse MyFile.cs --output ast.json
./my-analyzer ast.json
```

---

## Performance

### Parsing Speed

- **Small files** (< 100 lines): < 10ms
- **Medium files** (100-1000 lines): 10-100ms
- **Large files** (1000-10000 lines): 100ms-1s
- **Very large files** (> 10000 lines): 1-10s

### Memory Usage

- Memory usage scales linearly with file size
- Typical: 1-5 MB per 1000 lines of code
- Peak memory during AST construction

---

## Integration

### CI/CD Pipeline

```yaml
# GitHub Actions
- name: Validate C# Syntax
  run: |
    find . -name "*.cs" | while read file; do
      bsharp parse "$file" || exit 1
    done
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

git diff --cached --name-only --diff-filter=ACM | grep '\.cs$' | while read file; do
    if ! bsharp parse "$file" > /dev/null 2>&1; then
        echo "Parse error in $file"
        exit 1
    fi
done
```

### Build Script

```bash
#!/bin/bash
# validate-syntax.sh

errors=0
for file in src/**/*.cs; do
    if ! bsharp parse "$file" > /dev/null 2>&1; then
        echo "ERROR: $file"
        ((errors++))
    fi
done

if [ $errors -gt 0 ]; then
    echo "Found $errors files with syntax errors"
    exit 1
fi
```

---

## Comparison with Other Tools

### vs. Roslyn

- **BSharp:** Fast, standalone, JSON output
- **Roslyn:** Full compiler, .NET required, complex API

### vs. Tree-sitter

- **BSharp:** C#-specific, complete AST
- **Tree-sitter:** Multi-language, syntax tree only

---

## Implementation

**Location:** `src/bsharp_cli/src/commands/parse.rs`

```rust
pub fn execute(
    input: PathBuf,
    output: Option<PathBuf>,
    errors_json: bool,
    no_color: bool,
    lenient: bool,
) -> Result<()> {
    // Read file, choose strict/lenient, parse, and write <input>.json by default
    // See the source file for detailed behavior and error formatting.
    # Ok(())
}
```

---

## Related Documentation

- [CLI Overview](./overview.md) - General CLI usage
- [Tree Visualization](./tree.md) - Visualize parsed AST
- [AST Structure](../parser/ast-structure.md) - AST node reference
- [Error Handling](../parser/error-handling.md) - Parse error details

---

## References

- **Implementation:** `src/bsharp_cli/src/commands/parse.rs`
- **Parser:** `src/bsharp_parser/src/`
- **AST Definitions:** `src/bsharp_syntax/src/`
