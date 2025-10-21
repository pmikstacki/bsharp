# Formatter and Emitters

This page describes the formatting architecture in BSharp, implemented in the `bsharp_syntax` crate.

---

## Overview

The formatter combines AST-driven emitters with normalization passes to produce consistent, readable C# source code.

- Core types:
  - `Formatter`
  - `FormatOptions`
- Emission is instrumentable via a JSONL trace for debugging and profiling.

---

## FormatOptions

```rust
pub struct FormatOptions {
    pub newline: &'static str,                    // "\n" or "\r\n"
    pub max_consecutive_blank_lines: u8,          // default: 1
    pub blank_line_between_members: bool,         // default: true
    pub trim_trailing_whitespace: bool,           // default: true
    pub instrument_emission: bool,                // default: false
    pub trace_file: Option<std::path::PathBuf>,   // optional JSONL output
    pub current_file: Option<std::path::PathBuf>, // helpful in messages
}
```

- Newline mode is controlled by CLI `--newline-mode` or defaults to LF.
- Emission tracing can be toggled via CLI `--emit-trace` or `BSHARP_EMIT_TRACE=1`.

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
- Diagnose spacing/blank line decisions
- Identify costly emission paths
- Reproduce formatting anomalies

---

## Integration with CLI

- See `bsharp format` in `docs/cli/format.md` for options mapping to `FormatOptions`.
- Files that fail to parse are skipped; a summary is printed.
- With `--write false` on a single file input, the formatted output is printed to stdout.

---

## Design Notes

- Emitters are AST-driven to preserve structure while normalizing whitespace and layout.
- The formatter avoids changing semantics and focuses on consistent style.
- Options default to safe, conservative values and can be tuned via CLI.
