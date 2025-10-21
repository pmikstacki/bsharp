# Tree Visualization Command

The `tree` command generates a visualization of the Abstract Syntax Tree (AST) from C# source code in Mermaid or Graphviz DOT format.

---

## Usage

```bash
bsharp tree <INPUT> [--output <FILE>] [--format mermaid|dot]
```

### Arguments

**`<INPUT>`** (required)
- Path to C# source file
- Must have `.cs` extension

### Options

**`--output, -o <FILE>`** (optional)
- Output file path
- Default: `<input>.mmd` for Mermaid, `<input>.dot` for DOT

**`--format <FORMAT>`** (optional)
- One of: `mermaid` (default), `dot` (alias: `graphviz`)

---

## Examples

### Basic Visualization

```bash
# Generate Mermaid diagram (default)
bsharp tree Program.cs              # writes Program.mmd

# Generate Graphviz DOT diagram
bsharp tree Program.cs --format dot # writes Program.dot

# Specify output file
bsharp tree Program.cs --format dot --output ast-diagram.dot
```

### View/Render

```bash
# Mermaid preview (e.g., VS Code Mermaid extension) or CLI renderer
# Graphviz render to PNG
dot -Tpng Program.dot -o Program.png
```

---

## Output Formats

### Mermaid

Outputs a simple top-level graph in Mermaid syntax (`.mmd`).

```text
graph TD
n0["CompilationUnit\\nUsings: 1\\nDecls: 1"]
u0["Using using System;"]
n0 --> u0
d0["Class: Program"]
n0 --> d0
```

### Graphviz DOT

Outputs a simple top-level graph in DOT syntax (`.dot`).

```text
digraph AST {
  node [shape=box, fontname="Courier New"];
  n0 [label="CompilationUnit\\nUsings: 1\\nDecls: 1"];
  u0 [label="Using using System;"];
  n0 -> u0;
  d0 [label="Class: Program"];
  n0 -> d0;
}
```

### Color Scheme

- **Gray** - Root nodes (CompilationUnit)
- **Blue** - Type declarations (Class, Interface, Struct)
- **Green** - Member declarations (Method, Property, Field)
- **Yellow** - Statements (If, For, While)
- **Orange** - Expressions (Binary, Invocation)
- **Purple** - Types (Primitive, Named, Generic)

---

## Visualization Features

### Node Information

Each node displays:
- **Node Type** - AST node type name
- **Identifier** - Name (for named nodes)
- **Additional Info** - Modifiers, types, etc.

### Tree Layout

- **Top-down** - Root at top, leaves at bottom
- **Hierarchical** - Parent-child relationships clear
- **Balanced** - Nodes distributed evenly
- **Scalable** - Adjusts to tree size

---

## Use Cases

### 1. Understanding Code Structure

```bash
# Visualize complex class
bsharp tree ComplexClass.cs --output structure.svg
```

### 2. Teaching/Documentation

```bash
# Generate diagrams for documentation
bsharp tree Example.cs --output docs/ast-example.svg
```

### 3. Debugging Parser

```bash
# Verify parser output
bsharp tree TestCase.cs --output debug.svg
```

### 4. Code Review

```bash
# Visualize changes
bsharp tree NewFeature.cs --output review.svg
```

---

## Limitations

### Large Files

- Files > 1000 lines may produce very large SVGs
- Consider visualizing specific classes/methods only

### Complex Nesting

- Deeply nested structures may be hard to read
- SVG may require horizontal scrolling

### Performance

- Generation time increases with AST size
- Large files (> 5000 lines) may take several seconds

---

## Advanced Usage

### Selective Visualization

```bash
# Extract specific class and visualize
# (requires custom script to extract class)
extract-class.sh MyFile.cs MyClass > temp.cs
bsharp tree temp.cs --output MyClass-ast.svg
rm temp.cs
```

### Batch Generation

```bash
# Generate visualizations for all files
for file in src/**/*.cs; do
    output="diagrams/$(basename $file .cs).svg"
    bsharp tree "$file" --output "$output"
done
```

### Integration with Documentation

```markdown
# MyClass Documentation

## AST Structure

![AST Diagram](./diagrams/MyClass.svg)

The class structure shows...
```

---

## Implementation

**Location:** `src/bsharp_cli/src/commands/tree.rs`

```rust
pub fn execute(args: Box<TreeArgs>) -> Result<()> {
    // Parses input in lenient mode, then writes Mermaid (.mmd) or DOT (.dot)
    // using bsharp_syntax::node::render::{to_mermaid, to_dot}.
    # Ok(())
}
```

Renderer functions live in `src/bsharp_syntax/src/node/render.rs`:

```rust
to_mermaid(&ast);
to_dot(&ast);
```

---

## Customization

### Future Enhancements

1. **Interactive SVG**
   - Click to expand/collapse nodes
   - Hover for details
   - Search functionality

2. **Export Formats**
   - PNG/PDF export
   - DOT format for Graphviz
   - PlantUML format

3. **Filtering**
   - Show only specific node types
   - Hide implementation details
   - Focus on structure

4. **Styling**
   - Custom color schemes
   - Font customization
   - Layout options

---

## Troubleshooting

### SVG Too Large

**Problem:** Generated SVG is too large to view

**Solution:**
- Visualize smaller code sections
- Use SVG viewer with zoom/pan
- Export to PDF for printing

### Overlapping Nodes

**Problem:** Nodes overlap in complex trees

**Solution:**
- Increase SVG dimensions
- Simplify code structure
- Use horizontal layout (future feature)

### Missing Nodes

**Problem:** Some AST nodes not shown

**Solution:**
- Check parser output with `parse` command
- Report issue if nodes are missing

---

## Related Documentation

- [CLI Overview](./overview.md) - General CLI usage
- [Parse Command](./parse.md) - Parse textual AST tree
- [AST Structure](../parser/ast-structure.md) - AST node reference

---

## References

- **Implementation:** `src/bsharp_cli/src/commands/tree.rs`
- **Formats:** Mermaid or Graphviz DOT
