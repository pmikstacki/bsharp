# Emitter Design (AST→IL)

## Inputs
- AST from `bsharp_parser`.
- Analysis CFG from `bsharp_analysis/src/artifacts/control_flow_graph/` to structure branches.

## Strategy
- Expressions lower to evaluation stack IL.
- Statements map to labeled blocks and branches; use `MethodBodyBuilder`.
- Locals declared via `.local(name, TypeSignature)`.

## Control-flow mapping
- If/Else → diamond with `brtrue/brfalse` and join.
- While/For → head/test blocks, back-edge with `br`.
- Switch → `switch` table + join.
- Try/Catch/Finally → EH helpers (label-based ranges).

## Metadata
- Use builders to create `Assembly`, `TypeDef`, `MethodDef` entries.
- Link method bodies with local sig tokens returned by `MethodBodyBuilder::build`.
