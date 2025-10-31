# Parse Errors JSON Output

When `bsharp parse` is run with `--errors-json`, parse failures are emitted as a single JSON object to stdout and the process exits with a non-zero code.

---

## Schema

```json
{
  "error": {
    "kind": "parse_error",
    "file": "<path>",
    "line": 0,
    "column": 0,
    "expected": "",
    "found": "",
    "line_text": "",
    "message": "<pretty formatted message>",
    "spans": {
      "abs": { "start": 0, "end": 1 },
      "rel": {
        "start": { "line": 0, "column": 0 },
        "end": { "line": 0, "column": 1 }
      }
    }
  }
}
```

- `kind` – always `parse_error` for parse failures.
- `file` – path of the file being parsed.
- `line`, `column` – 1-based location of the deepest error span.
- `expected`, `found` – reserved fields (currently empty strings).
- `line_text` – the full source line at the error location.
- `message` – multi-line pretty message formatted from the parser's error tree.
- `spans` – present only when `--emit-spans` is provided; includes absolute byte range and relative line/column positions.

---

## Example

```bash
bsharp parse Invalid.cs --errors-json | jq
```

```json
{
  "error": {
    "kind": "parse_error",
    "file": "Invalid.cs",
    "line": 7,
    "column": 12,
    "expected": "",
    "found": "",
    "line_text": "public clas Program { }",
    "message": "0: at 7:12: expected keyword \"class\"\n  public clas Program { }\n           ^\nContexts:\n  - class declaration\n"
  }
}
```

---

## Notes

- In pretty (non-JSON) mode, errors are sent to stderr with optional ANSI colors (disable via `--no-color` or `NO_COLOR=1`).
- `--errors-json` disables pretty errors and always prints the JSON object.
