# Parse Command

The `parse` command parses C# source code and outputs a JSON representation of the Abstract Syntax Tree (AST).

---

## Usage

```bash
bsharp parse <INPUT> [--output <FILE>]
```

### Arguments

**`<INPUT>`** (required)
- Path to C# source file
- Must have `.cs` extension
- File must exist and be readable

### Options

**`--output, -o <FILE>`** (optional)
- Output file path for JSON
- Default: `<input>.json`
- Creates parent directories if needed

---

## Examples

### Basic Parsing

```bash
# Parse and output to default file (Program.cs.json)
bsharp parse Program.cs

# Parse and specify output file
bsharp parse Program.cs --output ast.json

# Parse and output to stdout (if no --output specified and stdout is not a TTY)
bsharp parse Program.cs
```

### Batch Parsing

```bash
# Parse all C# files in directory
for file in src/**/*.cs; do
    bsharp parse "$file" --output "parsed/$(basename $file .cs).json"
done
```

---

## Output Format

### JSON Structure

```json
{
  "global_attributes": [],
  "using_directives": [
    {
      "namespace": "System"
    }
  ],
  "declarations": [
    {
      "Class": {
        "attributes": [],
        "modifiers": ["Public"],
        "identifier": {
          "name": "Program"
        },
        "type_parameters": null,
        "base_types": [],
        "body_declarations": [
          {
            "Method": {
              "attributes": [],
              "modifiers": ["Public", "Static"],
              "return_type": {
                "Primitive": "Void"
              },
              "identifier": {
                "name": "Main"
              },
              "parameters": [],
              "body": {
                "Block": []
              }
            }
          }
        ]
      }
    }
  ],
  "file_scoped_namespace": null,
  "top_level_statements": []
}
```

### AST Node Structure

All AST nodes follow the naming convention:
- **PascalCase** names
- **No 'Syntax' suffix**
- Descriptive names indicating C# construct

Examples:
- `ClassDeclaration` (not `ClassDeclarationSyntax`)
- `MethodDeclaration` (not `MethodDeclarationSyntax`)
- `IfStatement` (not `IfStatementSyntax`)

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
jq '.declarations[0].Class.identifier.name' ast.json
```

### 3. Code Generation Input

```bash
# Parse C# and generate documentation
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

**Location:** `src/cli/commands/parse.rs`

```rust
pub fn execute(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    // 1. Read source file
    let source = fs::read_to_string(&input)?;
    
    // 2. Parse with BSharp parser
    let parser = Parser::new();
    let cu = parser.parse(&source)
        .map_err(|e| anyhow!("Parse failed: {}", e))?;
    
    // 3. Serialize to JSON
    let json = serde_json::to_string_pretty(&cu)?;
    
    // 4. Write output
    let output_path = output.unwrap_or_else(|| {
        let mut p = input.clone();
        p.set_extension("cs.json");
        p
    });
    
    fs::write(&output_path, json)?;
    
    println!("Parsed successfully: {}", output_path.display());
    Ok(())
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

- **Implementation:** `src/cli/commands/parse.rs`
- **Parser:** `src/parser/`
- **AST Definitions:** `src/syntax/nodes/`
