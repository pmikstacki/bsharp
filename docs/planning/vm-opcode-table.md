# BSharp VM – Opcode Table

This document defines the concrete bytecode instruction set for the BSharp VM. It is the primary reference for mapping between IR instructions and VM opcodes.

The VM uses a **32-bit fixed-width opcode word** with several layouts inspired by Lua-style ABC/ABx encodings:

*   **Common fields**
    *   Bits `0..=7`: opcode ID (0–255).
*   **ABC format** (three 8-bit operands)
    *   Bits `8..=15`: operand `A`.
    *   Bits `16..=23`: operand `B`.
    *   Bits `24..=31`: operand `C`.
*   **ABx format** (one 8-bit operand, one 16-bit operand)
    *   Bits `8..=15`: operand `A`.
    *   Bits `16..=31`: operand `Bx` (unsigned 16-bit immediate).
*   **AsBx format** (signed 16-bit immediate)
    *   Same physical layout as ABx, but `Bx` is interpreted as a **signed** `sBx` using a fixed bias.

Unless stated otherwise:

*   `A`, `B`, `C` are **register indices** (0–255).
*   `Bx` / `sBx` are immediates (e.g., constant pool index, jump offset, closure metadata index).
*   All register operands refer into the **global register stack**, with meaning scoped by the current call frame’s base pointer.

## Opcode reference

The table below lists each opcode, its encoding format, operands, and semantics. IR–opcode mapping is described in the Stage 2 and Stage 4 planning documents.

| Opcode | Format | Operands | Semantics |
| --- | --- | --- | --- |
| `NOP` | ABC | – | No operation. Reserved for padding or future use. |
| `LOAD_CONST` | ABx | `A` = destination register, `Bx` = const index | `R[A] = CONST[Bx]`. Loads a value from the constant pool into a register. |
| `MOVE` | ABC | `A` = dst, `B` = src | `R[A] = R[B]`. Simple register-to-register move. |
| `LOAD_GLOBAL` | ABx | `A` = dst, `Bx` = global index | `R[A] = GLOBAL[Bx]`. Loads a global value. |
| `STORE_GLOBAL` | ABx | `A` = global index, `Bx` = src register | `GLOBAL[A] = R[Bx_low]`. Uses low 8 bits of `Bx` as register index; reserved high bits for future expansion. |
| `ADD_INT` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = as_int(R[B]) + as_int(R[C])`. On non-integer operands, raises `VmError::TypeMismatch`. |
| `SUB_INT` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = as_int(R[B]) - as_int(R[C])`. |
| `MUL_INT` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = as_int(R[B]) * as_int(R[C])`. |
| `DIV_INT` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = as_int(R[B]) / as_int(R[C])`. On division by zero, raises `VmError::DivideByZero`. |
| `CMP_EQ` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = (R[B] == R[C])` (value equality for supported pairs). |
| `CMP_NE` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = (R[B] != R[C])`. |
| `CMP_LT` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = as_int(R[B]) < as_int(R[C])`. |
| `CMP_LE` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = as_int(R[B]) <= as_int(R[C])`. |
| `CMP_GT` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = as_int(R[B]) > as_int(R[C])`. |
| `CMP_GE` | ABC | `A` = dst, `B` = lhs, `C` = rhs | `R[A] = as_int(R[B]) >= as_int(R[C])`. |
| `LOGICAL_NOT` | ABx | `A` = dst, `Bx` (low 8 bits) = src register | `R[A] = !as_bool(R[src])`. Uses low 8 bits of `Bx` as register index. |
| `JUMP` | AsBx | `sBx` = signed relative offset | `ip = ip + 1 + sBx`. Unconditional PC-relative jump. |
| `JUMP_IF_TRUE` | AsBx | `A` = cond register, `sBx` = offset | If `as_bool(R[A])` is true: `ip = ip + 1 + sBx`, else `ip += 1`. |
| `JUMP_IF_FALSE` | AsBx | `A` = cond register, `sBx` = offset | If `as_bool(R[A])` is false: `ip = ip + 1 + sBx`, else `ip += 1`. |
| `CALL` | ABC | `A` = base register, `B` = arg count, `C` = ret count | Calls a **direct function** determined by the current instruction’s associated function ID. Arguments live in `R[A..A+B)`. A single return value is written back into `R[A]` when `C == 1`. |
| `CALL_INTRINSIC` | ABC | `A` = base register, `B` = arg count, `C` = intrinsic ID (0–255) | Calls a built-in intrinsic. Semantics are defined by the runtime; arguments and return convention match `CALL`. |
| `MAKE_CLOSURE` | ABx | `A` = dst, `Bx` = closure metadata index | Allocates a closure object in the heap using metadata (`function id` and capture layout) at `Bx`. Writes `Value::ObjectRef` (closure) to `R[A]`. |
| `GET_UPVALUE` | ABx | `A` = dst register, `Bx` = upvalue index | Reads an upvalue from the current closure’s environment and writes it to `R[A]`. |
| `SET_UPVALUE` | ABx | `A` = src register, `Bx` = upvalue index | Writes `R[A]` into the upvalue slot in the current closure’s environment. |
| `CALL_DYNAMIC` | ABC | `A` = base register, `B` = arg count, `C` = ret count | `R[A]` contains the callee value (either `FunctionId` or a heap-allocated closure). Dispatches accordingly using the same calling convention as `CALL`. |
| `RETURN` | ABx | `A` = src register (or sentinel when void), `Bx` unused | Returns from the current function. If a value is returned, it is taken from `R[A]` and written into the caller’s designated return slot. |

### Notes on closure support

*   Closure metadata (indexed by `Bx` in `MAKE_CLOSURE`) is generated at compile time during IR → bytecode lowering. It encodes:
    *   The target `FunctionId`.
    *   The number of captured variables and whether they are locals or upvalues.
*   `GET_UPVALUE` and `SET_UPVALUE` operate on the closure’s environment. The runtime is free to implement environments as arrays, cells, or other heap structures as long as semantics are preserved.

### Error behavior

Unless otherwise specified, incorrect operand types or out-of-range indices result in a `VmError` being raised by the interpreter. The VM **must not** panic on user-level errors; all such failures are reported through structured error values.