# Tree Visualization Command

The `tree` command generates an SVG visualization of the Abstract Syntax Tree (AST) from C# source code.

---

## Usage

```bash
bsharp tree <INPUT> [--output <FILE>]
```

### Arguments

**`<INPUT>`** (required)
- Path to C# source file
- Must have `.cs` extension

### Options

**`--output, -o <FILE>`** (optional)
- Output SVG file path
- Default: `<input>.svg`

---

## Examples

### Basic Visualization

```bash
# Generate AST visualization
bsharp tree Program.cs

# Specify output file
bsharp tree Program.cs --output ast-diagram.svg
```

### View in Browser

```bash
# Generate and open in browser
bsharp tree MyClass.cs --output diagram.svg
open diagram.svg  # macOS
xdg-open diagram.svg  # Linux
start diagram.svg  # Windows
```

---

## Output Format

### SVG Structure

The generated SVG contains:
- **Tree layout** - Hierarchical node arrangement
- **Node boxes** - AST node types and names
- **Connecting lines** - Parent-child relationships
- **Color coding** - Different colors for node types

### Example Output

```svg
<svg width="800" height="600">
  <!-- CompilationUnit at root -->
  <rect x="400" y="20" width="120" height="40" fill="#e0e0e0"/>
  <text x="460" y="45">CompilationUnit</text>
  
  <!-- ClassDeclaration -->
  <line x1="460" y1="60" x2="460" y2="100"/>
  <rect x="400" y="100" width="120" height="40" fill="#90caf9"/>
  <text x="460" y="125">ClassDeclaration</text>
  <text x="460" y="140" font-size="10">Program</text>
  
  <!-- MethodDeclaration -->
  <line x1="460" y1="140" x2="460" y2="180"/>
  <rect x="400" y="180" width="120" height="40" fill="#a5d6a7"/>
  <text x="460" y="205">MethodDeclaration</text>
  <text x="460" y="220" font-size="10">Main</text>
</svg>
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

**Location:** `src/cli/commands/tree.rs`

```rust
pub fn execute(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    // 1. Parse source file
    let source = fs::read_to_string(&input)?;
    let parser = Parser::new();
    let cu = parser.parse(&source)?;
    
    // 2. Generate SVG from AST
    let svg = generate_svg_tree(&cu)?;
    
    // 3. Write output
    let output_path = output.unwrap_or_else(|| {
        let mut p = input.clone();
        p.set_extension("svg");
        p
    });
    
    fs::write(&output_path, svg)?;
    
    println!("Generated tree: {}", output_path.display());
    Ok(())
}

fn generate_svg_tree(cu: &CompilationUnit) -> Result<String> {
    let mut builder = SvgBuilder::new();
    
    // Build tree structure
    let root = build_tree_node(cu);
    
    // Calculate layout
    let layout = calculate_layout(&root);
    
    // Render to SVG
    builder.render_tree(&root, &layout);
    
    Ok(builder.to_string())
}
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
- [Parse Command](./parse.md) - Parse to JSON
- [AST Structure](../parser/ast-structure.md) - AST node reference

---

## References

- **Implementation:** `src/cli/commands/tree.rs`
- **SVG Generation:** Uses `svg` crate
- **Layout Algorithm:** Tree layout algorithm
