# Testing & Conformance

## Unit tests
- Hand-authored IL via `MethodBodyBuilder` for arithmetic/branching/stack behavior.
- VM result assertions; negative tests for stack underflow/type errors.

## End-to-end tests
- Parse C# → emit IL → run on VM; assert outputs.
- Optional: run emitted DLL on .NET and compare results.

## CFG fidelity
- Mirror shapes produced by `build_cfg()` from `control_flow_graph` to ensure consistent semantics.

## Golden tests
- Snapshot IL bytes and metadata tables for regression protection.
