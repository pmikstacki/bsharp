# Phase 2: Control Flow + Exception Handling (EH)

## Goals
- Add richer branching (`switch`, `leave`) and structured EH (try/catch/finally) to the VM and validate emission paths.

## Detailed tasks
- **[switch]** Implement `switch`
  - Decode switch table (N targets + base) and compute absolute branch targets from relative offsets.
  - Update IP by index lookup; validate index bounds; fall-through to next instruction on no match (per IL semantics, `switch` jumps or falls through?).
- **[leave]** Implement `leave`/`leave.s`
  - Transfer control out of current protected region to target; ensure pending `finally` handlers execute.
- **[eh-parse]** Parse EH clauses from dotscope
  - For each method body, use dotscope to obtain EH sections (try start/len, handler start/len, flags: catch/filter/finally/fault, catch type token).
  - Store clauses in runtime metadata for fast lookup by offset.
- **[eh-runtime]** Unwinding and handlers
  - On `throw` and on `leave`, search current method’s EH table for matching handler: order per ECMA-335.
  - Implement catch (typed and any), finally (always run on exit of protected region), fault (run on exceptional exit only).
  - Implement filter (phase 2.5 optional): evaluate filter method body region and branch accordingly.
- **[emission-demo]** Emission with `MethodBodyBuilder`
  - Author sample try/catch/finally using label-based helpers (`catch_handler`, `finally_handler`, `*_with_labels`).
  - Validate VM executes handlers in correct order (finally runs on both normal and exceptional paths).
- **[diagnostics]** Error reporting
  - On unhandled exception, produce a stack trace of frames with IL offsets; include method tokens and optionally sequence points (future).

## Deliverables
- VM support for `switch`, `leave`, `throw`, catch/finally EH.
- Sample emitted methods with EH and a test suite for normal and exceptional flows.

## Acceptance criteria
- Unit tests cover: multi-branch switch; nested try/catch; try/finally; `leave` jumping out through finally; rethrow behavior.
- Emitted EH via dotscope decodes and runs identically in the VM.

## References
- dotscope method body + EH helpers: `MethodBodyBuilder`.
- ECMA-335 Partition III: `switch`, `leave`, EH table semantics.
- Microsoft Learn: Exception handling clauses (`ExceptionHandlingClauseOptions`), `OpCodes.Throw`.
