### Wrapper Expression Variants

For clarity, several operations are modeled as distinct expression variants in the AST:

- `New(NewExpression)` for object creation
- `MemberAccess(MemberAccessExpression)` for `obj.Member`
- `Invocation(InvocationExpression)` for calls `expr(args)`
- `Indexing(IndexingExpression)` and `Index(IndexExpression)`
- `Range(RangeExpression)` for `start..end`
- `With { target, initializers }` for record-like with-expressions
- `Collection(Vec<CollectionElement>)` for collection expressions

# Expression Parsing

BSharp implements a complete expression parser that handles all C# expression types with proper operator precedence and associativity.

## Expression Hierarchy

The expression parser follows C#'s operator precedence rules:

1. **Primary Expressions** (`x`, `x.y`, `x[y]`, `x()`, etc.)
2. **Unary Expressions** (`+x`, `-x`, `!x`, `~x`, `++x`, `--x`)
3. **Multiplicative** (`*`, `/`, `%`)
4. **Additive** (`+`, `-`)
5. **Shift** (`<<`, `>>`)
6. **Relational** (`<`, `>`, `<=`, `>=`, `is`, `as`)
7. **Equality** (`==`, `!=`)
8. **Logical AND** (`&`)
9. **Logical XOR** (`^`)
10. **Logical OR** (`|`)
11. **Conditional AND** (`&&`)
12. **Conditional OR** (`||`)
13. **Null Coalescing** (`??`)
14. **Conditional** (`?:`)
15. **Assignment** (`=`, `+=`, `-=`, etc.)

## Expression Types

### Primary Expressions

#### Literals
- **Numeric**: `42`, `3.14`, `0x1A`
- **String**: `"hello"`, `@"verbatim"`, `$"interpolated {value}"`
- **Character**: `'a'`, `'\n'`
- **Boolean**: `true`, `false`
- **Null**: `null`

#### Identifiers and Member Access
```csharp
variable          // Simple identifier
obj.property      // Member access
obj.method()      // Method invocation
obj[index]        // Indexer access
```

Note: In the AST, simple identifiers are represented by the `Expression::Variable(Identifier)` variant. Member access, invocation, and indexing are represented by dedicated wrapper variants (`MemberAccess`, `Invocation`, `Indexing`).

#### Object Creation
```csharp
new MyClass()                    // Constructor
new MyClass { Prop = value }     // Object initializer
new[] { 1, 2, 3 }               // Array initializer
new { Name = "John", Age = 30 }  // Anonymous object
```

### Lambda Expressions

The parser supports various lambda syntax forms:

```csharp
x => x * 2                      // Single parameter
(x, y) => x + y                 // Multiple parameters
() => DoSomething()             // No parameters
(int x, string y) => Process(x, y)  // Typed parameters
x => { return x * 2; }          // Block body
async x => await ProcessAsync(x) // Async lambda
```

### Query Expressions (LINQ)

Complete LINQ query syntax support:

```csharp
from item in collection
where item.IsValid
orderby item.Name
select item.Value
```

Supported clauses:
- `from` - Data source
- `where` - Filtering
- `select` - Projection
- `orderby` - Sorting
- `group by` - Grouping
- `join` - Joining
- `let` - Variable introduction
- `into` - Query continuation

### Pattern Expressions

Modern C# pattern matching:

```csharp
obj is int value           // Type pattern
obj is not null           // Negation pattern
obj is > 0 and < 100     // Relational patterns
obj is var x             // Var pattern
```

### Switch Expressions

```csharp
value switch
{
    1 => "one",
    2 => "two",
    _ => "other"
}
```

## Operator Precedence Implementation

The expression entrypoint is spanned-first. Callers can unwrap the `Spanned<Expression>` when they do not need spans:

```rust
use bsharp_parser::parser::expressions::primary_expression_parser::parse_expression_spanned;
use bsharp_syntax::span::Span;

let result = parse_expression_spanned(Span::new(input))
    .map(|(rest, s)| (rest, s.node));
```

## Error Handling in Expressions

The expression parser provides detailed error messages:

- **Operator precedence conflicts**
- **Missing operands**
- **Invalid syntax combinations**
- **Type compatibility issues**

## Advanced Features

### Null-Conditional Operators
```csharp
obj?.Property        // Null-conditional member access
obj?[index]         // Null-conditional element access
obj?.Method()       // Null-conditional invocation
```

### Throw Expressions
```csharp
value ?? throw new ArgumentNullException()
```

### Range and Index Expressions
```csharp
array[^1]           // Index from end
array[1..5]         // Range
array[..^1]         // Range to index from end
```

### With Expressions (Records)
```csharp
person with { Name = "Updated" }
```

The expression parser is designed to be extensible, allowing for easy addition of new expression types as the C# language evolves.

---

## See Also

- [Keywords and Tokens](./keywords-and-tokens.md) – keyword helpers, word boundaries, trivia handling for tokens used in expressions
