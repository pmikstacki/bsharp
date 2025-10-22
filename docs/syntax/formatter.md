# Formatter and Emitters

This page describes the formatting architecture in BSharp, implemented in the `bsharp_syntax` crate.

---

## Overview

The formatter is an AST-driven emitter that produces the final C# text directly. There is no post-processing pass (no normalize_text): the output is exactly what emitters write.

- Core types:
  - `Formatter`
  - `FormatOptions`
- Emission is instrumentable via a JSONL trace for debugging and profiling.

---

## FormatOptions

```rust
pub struct FormatOptions {
    pub indent_width: usize,                      // default: 4 spaces
    pub newline: &'static str,                    // "\n" or "\r\n"
    pub max_consecutive_blank_lines: u8,          // default: 1
    pub blank_line_between_members: bool,         // default: true
    pub ensure_final_newline: bool,               // default: true (emit one final newline if any content)
    pub trim_trailing_whitespace: bool,           // default: true
    pub instrument_emission: bool,                // default: false
    pub trace_file: Option<std::path::PathBuf>,   // optional JSONL output
    pub current_file: Option<std::path::PathBuf>, // helpful in messages
}
```

- Newline mode is controlled by CLI `--newline-mode` or defaults to LF.
- Emission tracing can be toggled via CLI `--emit-trace` or `BSHARP_EMIT_TRACE=1`.

---

## Brace Style and Spacing Policy

- **Brace style:** All containers and headers use Allman style
  - Header ends the line (e.g., `namespace X`, `class C`, `void M()`)
  - Next line is an opening `{`, indented body, then closing `}` on its own line.

- **Spacing is centralized** in simple policy helpers (see `src/bsharp_syntax/src/emitters/policy.rs`):
  - `between_header_and_body_of_file` → blank line between file header (e.g., file-scoped ns) and body
  - `after_file_scoped_namespace_header` → blank line after `namespace X.Y;`
  - `between_using_blocks_and_declarations` → blank line after using block before first declaration
  - `between_top_level_declarations` → single separator newline between top-level declarations
  - `between_members` → single separator newline between adjacent type members
  - `between_block_items` → optional extra newline inside a block when a control-flow block (if/for/while/do/switch/inner block) is followed by a declaration

Notes:
- Policies are invoked from emitters; emitters themselves keep logic minimal and do not hardcode extra blank lines.
- Interfaces, classes, structs, and records call `between_members` between members; the boolean `blank_line_between_members` toggles this globally.

---

## End-of-file Newline

- The `CompilationUnit` emitter ensures at most one final newline at EOF.
- There are no per-statement trailing newlines at the root; separation is handled by policy functions.

---

## Usage

```rust
use bsharp_syntax::{Formatter, FormatOptions};

let mut opts = FormatOptions::default();
opts.newline = "\n";
opts.max_consecutive_blank_lines = 1;
opts.blank_line_between_members = true;
opts.trim_trailing_whitespace = true;

let fmt = Formatter::new(opts);
let output = fmt.format_compilation_unit(&cu)?; // cu: CompilationUnit
```

---

## Emission Trace (JSONL)

When instrumentation is enabled, the formatter emits a stream of JSON objects describing emission steps.

- CLI integration:
  - `--emit-trace` to enable
  - `--emit-trace-file <FILE>` to write to a file (stdout by default)
  - Env var `BSHARP_EMIT_TRACE=1` acts as a default toggle

The trace can be useful to:
- Diagnose spacing/blank line decisions (look for `action: "policy"` with names like `between_members`, `between_top_level_declarations`, `between_block_items`)
- Identify costly emission paths
- Reproduce formatting anomalies

Typical actions include: `enter_node`, `open_brace`, `close_brace`, `newline`, `space`, `token`, and `policy`.

---

## Integration with CLI

- See `bsharp format` in `docs/cli/format.md` for options mapping to `FormatOptions`.
- Files that fail to parse are skipped; a summary is printed.
- With `--write false` on a single file input, the formatted output is printed to stdout.

---

## Design Notes

- Emitters are AST-driven to preserve structure while normalizing whitespace and layout based on policies.
- The formatter avoids changing semantics and focuses on consistent style.
- Options default to safe, conservative values and can be tuned via CLI.
