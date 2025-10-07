# Roadmap

- Phase 0: Dotscope spike (read/emit).
- Phase 1: VM MVP (primitives, branches, static calls).
- Phase 2: Control flow + EH.
- Phase 3: Object model + arrays.
- Phase 4: Back-compat polish (tokens across modules, P/Invoke, strings).
- Phase 5: AST→IL emitter and dual-run conformance.

## Risks
- IL surface area creep; mitigate via tight MVP subset.
- Metadata correctness; rely on dotscope builders and validation.
- Performance; consider Cranelift backend later.
