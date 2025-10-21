# Command Line Interface

The BSharp CLI provides command-line tools for parsing, analyzing, and visualizing C# code.

---

## Installation

### From Source

```bash
git clone https://github.com/mikserek/bsharp.git
cd bsharp
cargo build --release
```

The binary will be available at `target/release/bsharp`.

### Add to PATH

```bash
# Linux/macOS
export PATH="$PATH:/path/to/bsharp/target/release"

# Windows
# Add to System Environment Variables
```

---

## Command Structure

```bash
bsharp <COMMAND> [OPTIONS] <INPUT>
```

### Global Options

```bash
--help, -h      Show help information
--version, -V   Show version information
```

### Argument Files (@file)

All commands support argument files via `@file` syntax. Example:

```bash
bsharp @args.txt
```

Where `args.txt` contains one argument per line (comments and quoting follow standard shell parsing rules).

---

## Available Commands

### parse

Parse C# source code and print a textual AST tree to stdout.

```bash
bsharp parse <INPUT>
```

**See:** [Parse Command](./parse.md)

### tree

Generate a visualization of the Abstract Syntax Tree.

```bash
bsharp tree <INPUT> [--output <FILE>] [--format mermaid|dot]
```

Notes:
- Default format is `mermaid`; output defaults to `<input>.mmd`.
- For DOT/Graphviz, use `--format dot` (or `graphviz`); output defaults to `<input>.dot`.

**See:** [Tree Visualization](./tree.md)

<!-- compile command intentionally omitted (experimental/disabled) -->

### analyze

Analyze C# code and generate comprehensive analysis report.

```bash
bsharp analyze <INPUT> [OPTIONS]
```

**See:** [Analysis Command](./analyze.md)

### format

Format C# files using the built-in formatter and syntax emitters.

```bash
bsharp format <INPUT> [--write] [--newline-mode lf|crlf] [--max-consecutive-blank-lines <N>] \
  [--blank-line-between-members <BOOL>] [--trim-trailing-whitespace <BOOL>] \
  [--emit-trace] [--emit-trace-file <FILE>]
```

Notes:
- `<INPUT>` can be a file or directory (recursively formats .cs files; skips hidden/bin/obj/target).
- `--write` defaults to true; when false and a single file is given, the formatted output is printed to stdout.
- Emission tracing can be enabled by `--emit-trace` or environment variable `BSHARP_EMIT_TRACE=1`.

**See:** [Format Command](./format.md)

---

## Common Usage Patterns

### Quick Parse Check

```bash
# Check if file parses successfully
bsharp parse MyFile.cs
```

### Generate AST for Inspection

```bash
# Pretty-printed JSON
bsharp parse MyFile.cs --output ast.json
```

### Visualize Code Structure

```bash
# Generate Mermaid diagram (default), writes MyClass.mmd
bsharp tree MyClass.cs

# Generate Graphviz DOT diagram
bsharp tree MyClass.cs --format dot --output diagram.dot
```

### Analyze Project Quality

```bash
# Full analysis with report
bsharp analyze MyProject.csproj --out report.json --format pretty-json
```

### Analyze Solution

```bash
# Analyze entire solution
bsharp analyze MySolution.sln --follow-refs true
```

---

## Input Types

### Single File

```bash
bsharp parse Program.cs
```

### Project File (.csproj)

```bash
bsharp analyze MyProject.csproj
```

### Solution File (.sln)

```bash
bsharp analyze MySolution.sln
```

### Directory

```bash
bsharp analyze ./src
```

---

## Output Formats

### JSON (Compact)

```bash
bsharp analyze MyFile.cs --format json
```

**Output:** Single-line JSON, optimized for machine consumption

### Pretty JSON

```bash
bsharp analyze MyFile.cs --format pretty-json
```

**Output:** Indented JSON, human-readable

### Mermaid/DOT (Tree Command)

```bash
# Mermaid (default)
bsharp tree MyFile.cs --output diagram.mmd

# Graphviz DOT
bsharp tree MyFile.cs --format dot --output diagram.dot
```

**Output:** Mermaid (.mmd) or Graphviz DOT (.dot)

---

## Error Handling

### Parse Errors

```bash
$ bsharp parse InvalidSyntax.cs
Error: Parse failed at line 5, column 12
Expected ';' but found 'class'

public class MyClass
            ^
```

### File Not Found

```bash
$ bsharp parse NonExistent.cs
Error: File not found: NonExistent.cs
```

### Invalid Project

```bash
$ bsharp analyze Invalid.csproj
Error: Failed to parse project file: Invalid XML
```

---

## Environment Variables

### RUST_LOG

Control logging verbosity:

```bash
# Show all logs
RUST_LOG=debug bsharp parse MyFile.cs

# Show only warnings and errors
RUST_LOG=warn bsharp analyze MyProject.csproj

# Show specific module logs
RUST_LOG=bsharp::parser=debug bsharp parse MyFile.cs
```

### RUST_BACKTRACE

Enable stack traces on panic:

```bash
RUST_BACKTRACE=1 bsharp parse MyFile.cs
```

---

## Performance Considerations

### Large Files

For large files (> 10,000 lines), parsing may take several seconds:

```bash
# Monitor progress with debug logging
RUST_LOG=info bsharp parse LargeFile.cs
```

### Large Solutions

For solutions with many projects, use parallel analysis:

```bash
# Requires parallel_analysis feature
cargo build --release --features parallel_analysis
bsharp analyze LargeSolution.sln
```

### Memory Usage

Memory usage scales with AST size. For very large codebases:

```bash
# Analyze incrementally by project
for proj in **/*.csproj; do
    bsharp analyze "$proj" --out "$(basename $proj .csproj).json"
done
```

---

## Integration with Other Tools

### CI/CD Pipeline

```yaml
# GitHub Actions example
- name: Analyze Code Quality
  run: |
    bsharp analyze MySolution.sln --out analysis.json
    # Upload analysis.json as artifact
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

changed_files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.cs$')

for file in $changed_files; do
    if ! bsharp parse "$file" > /dev/null 2>&1; then
        echo "Parse error in $file"
        exit 1
    fi
done
```

### Editor Integration

```json
// VS Code tasks.json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Analyze Current File",
            "type": "shell",
            "command": "bsharp",
            "args": [
                "analyze",
                "${file}",
                "--out",
                "${file}.analysis.json"
            ]
        }
    ]
}
```

---

## Troubleshooting

### Command Not Found

```bash
$ bsharp: command not found
```

**Solution:** Add bsharp to PATH or use full path:
```bash
/path/to/bsharp/target/release/bsharp parse MyFile.cs
```

### Permission Denied

```bash
$ bsharp parse MyFile.cs
Permission denied
```

**Solution:** Make binary executable:
```bash
chmod +x /path/to/bsharp
```

### Out of Memory

```bash
$ bsharp analyze HugeSolution.sln
Error: memory allocation failed
```

**Solution:** Analyze smaller subsets or increase system memory

---

## Configuration Files

### Analysis Configuration

Create `.bsharp.toml` in project root:

```toml
[analysis]
max_cyclomatic_complexity = 10
max_method_length = 50

[analysis.quality]
long_method = "warning"
god_class = "error"

[workspace]
follow_refs = true
include = ["**/*.cs"]
exclude = ["**/obj/**", "**/bin/**"]
```

**Usage:**
```bash
# Automatically loads .bsharp.toml from current directory
bsharp analyze MyProject.csproj
```

---

## Shell Completion

Shell completion generation is currently not available in the CLI.

---

## Examples

### Example 1: Quick Syntax Check

```bash
# Check if all C# files in directory parse correctly
find . -name "*.cs" -exec bsharp parse {} \; 2>&1 | grep -i error
```

### Example 2: Generate Documentation

```bash
# Parse all files and extract class/method names
for file in src/**/*.cs; do
    bsharp parse "$file" --output "${file}.json"
done

# Process JSON to generate documentation
# (custom script)
```

### Example 3: Code Quality Gate

```bash
#!/bin/bash
# quality-gate.sh

bsharp analyze MyProject.csproj --out report.json --format json

# Extract error count
errors=$(jq '.diagnostics | map(select(.severity == "error")) | length' report.json)

if [ "$errors" -gt 0 ]; then
    echo "Quality gate failed: $errors errors found"
    exit 1
fi

echo "Quality gate passed"
```

### Example 4: Complexity Report

```bash
# Generate complexity report for all methods
bsharp analyze MySolution.sln --out complexity.json

# Extract high-complexity methods
jq '.diagnostics | map(select(.code == "MET001"))' complexity.json
```

---

## CLI Architecture

### Implementation

**Location:** `src/bsharp_cli/`

```
src/bsharp_cli/
├── src/
│   ├── main.rs         # CLI entry point, clap definitions
│   └── commands/
│       ├── mod.rs      # Command module exports
│       ├── parse.rs    # Parse command implementation
│       ├── tree.rs     # AST visualization command (Mermaid/DOT)
│       └── analyze.rs  # Analysis command
└── Cargo.toml
```

### Command Pattern

Each command follows this pattern:

```rust
pub fn execute(input: PathBuf, /* other args */) -> Result<()> {
    // 1. Validate input
    // 2. Load/parse files
    // 3. Perform operation
    // 4. Generate output
    // 5. Handle errors
    Ok(())
}
```

---

## Future Enhancements

### Planned Features

1. **Interactive Mode**
   - REPL for exploring AST
   - Interactive analysis

2. **Watch Mode**
   - Monitor files for changes
   - Re-analyze on save

3. **Language Server**
   - LSP implementation
   - IDE integration

4. **Web Interface**
   - Browser-based visualization
   - Interactive reports

---

## Related Documentation

- [Parse Command](./parse.md) - Detailed parse command documentation
- [Tree Visualization](./tree.md) - AST visualization
- [Analysis Pipeline](../analysis/pipeline.md) - Analysis internals

---

## References

- **Implementation:** `src/bsharp_cli/`
- **Commands:** `src/bsharp_cli/src/commands/`
- **Clap Documentation:** https://docs.rs/clap/
