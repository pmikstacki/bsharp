# bsharp_tests Overview

Structure and conventions for the test crate.

---

## Location

- All tests live under `src/bsharp_tests/src/`.
- Organize by domain:
  - `parser/` for parsing-related tests
  - `analysis/` for analysis pipeline tests

---

## Running Tests

```bash
cargo test -p bsharp_tests
```

---

## Conventions

- Prefer descriptive file names and test names.
- Keep fixtures small and focused.
- Use `Parser::parse_with_spans` and `AnalyzerPipeline::run_with_defaults` in integration-style tests.
