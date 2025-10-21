# Format Command

The `format` command formats C# code using the built-in formatter and syntax emitters.

---

## Usage

```bash
bsharp format <INPUT> [--write <BOOL>] [--newline-mode lf|crlf] \
  [--max-consecutive-blank-lines <N>] [--blank-line-between-members <BOOL>] \
  [--trim-trailing-whitespace <BOOL>] [--emit-trace] [--emit-trace-file <FILE>]
```

### Arguments

**`<INPUT>`** (required)
- Path to `.cs` file or directory
- When a directory is given, formats all `.cs` files recursively
- Hidden directories and `bin/`, `obj/`, `target/` are skipped

### Options

**`--write, -w <BOOL>`**
- Write changes to files in-place
- Default: `true`
- When `false` and `<INPUT>` is a single file, the formatted content is printed to stdout
- When `false` and formatting differences are found for multiple files, exits with code `2`

**`--newline-mode <MODE>`**
- Newline mode: `lf` (default) or `crlf`

**`--max-consecutive-blank-lines <N>`**
- Maximum consecutive blank lines to keep (default: `1`)

**`--blank-line-between-members <BOOL>`**
- Insert a blank line between type members (default: `true`)

**`--trim-trailing-whitespace <BOOL>`**
- Trim trailing whitespace (default: `true`)

**`--emit-trace`**
- Enable emission tracing (JSONL) for debugging formatter behavior
- Can also be enabled via environment variable `BSHARP_EMIT_TRACE=1`

**`--emit-trace-file <FILE>`**
- Path to write the trace JSONL (defaults to stdout when omitted)

---

## Examples

```bash
# Format a single file in-place
bsharp format Program.cs

# Print formatted output to stdout (do not write)
bsharp format Program.cs --write false

# Format a directory recursively
bsharp format src/

# Use CRLF newlines and avoid extra blank lines
bsharp format Program.cs --newline-mode crlf --max-consecutive-blank-lines 1

# Enable emission tracing to a file
bsharp format Program.cs --emit-trace --emit-trace-file format_trace.jsonl
```

---

## Implementation

- **Command:** `src/bsharp_cli/src/commands/format.rs`
- **Formatter:** `bsharp_syntax::Formatter` with `FormatOptions`
- Emission tracing is controlled by CLI flags or `BSHARP_EMIT_TRACE` and recorded as JSONL.

---

## Related Documentation

- [CLI Overview](./overview.md)
- [AST Structure](../parser/ast-structure.md)
- [Formatter Design](../syntax/formatter.md)
