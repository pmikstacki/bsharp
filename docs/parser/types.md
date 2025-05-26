
# Type System

BSharp implements a comprehensive type system that accurately represents all C# type constructs, from primitive types to complex generic types with constraints.

## Type Categories

### 1. Primitive Types

#### Built-in Value Types
```csharp
bool        // Boolean type
byte        // 8-bit unsigned integer
sbyte       // 8-bit signed integer
short       // 16-bit signed integer
ushort      // 16-bit unsigned integer
int         // 32-bit signed integer
uint        // 32-bit unsigned integer
long        // 64-bit signed integer
ulong       // 64-bit unsigned integer
char        // 16-bit Unicode character
float       // 32-bit floating point
double      // 64-bit floating point
decimal     // 128-bit decimal
```

#### Special Types
```csharp
object      // Base type of all types
string      // Immutable string type
void        // Absence of type (method returns)
dynamic     // Dynamic type
var         // Implicitly typed variable
```

### 2. Reference Types

#### Class Types
```csharp
MyClass                 // Simple class reference
System.Collections.List<int>  // Generic class
```

#### Interface Types
```csharp
IEnumerable<T>         // Generic interface
IDisposable            // Non-generic interface
```

#### Array Types
```csharp
int[]                  // Single-dimensional array
int[,]                 // Multi-dimensional array
int[][]                // Jagged array
int[,,]                // Three-dimensional array
```

#### Delegate Types
```csharp
Action                 // Parameterless action
Action<int>            // Action with parameter
Func<int, string>      // Function with return type
EventHandler<T>        // Event handler
```

### 3. Nullable Types

#### Nullable Value Types
```csharp
int?                   // Nullable integer
DateTime?              // Nullable DateTime
bool?                  // Nullable boolean
```

#### Nullable Reference Types (C# 8+)
```csharp
string?                // Nullable string
List<int>?             // Nullable list
MyClass?               // Nullable custom class
```

### 4. Generic Types

#### Type Parameters
```csharp
T                      // Simple type parameter
TKey, TValue           // Multiple type parameters
```

#### Constructed Generic Types
```csharp
List<int>              // Generic list of integers
Dictionary<string, object>  // Generic dictionary
```

#### Generic Constraints
```csharp
T where T : class                    // Reference type constraint
T where T : struct                   // Value type constraint
T where T : new()                    // Constructor constraint
T where T : BaseClass                // Base class constraint
T where T : IInterface               // Interface constraint
T where T : class, IDisposable, new() // Multiple constraints
```

### 5. Tuple Types

#### Named Tuples
```csharp
(int x, int y)         // Named tuple elements
(string name, int age) // Different element types
```

#### Unnamed Tuples
```csharp
(int, string)          // Unnamed tuple elements
```

#### Nested Tuples
```csharp
(int, (string, bool))  // Nested tuple structure
```

### 6. Pointer Types (Unsafe Context)

```csharp
int*                   // Pointer to integer
char**                 // Pointer to pointer to char
void*                  // Void pointer
```

### 7. Function Pointer Types (C# 9+)

```csharp
delegate*<int, string>              // Function pointer
delegate* managed<int, void>        // Managed function pointer
delegate* unmanaged<int, void>      // Unmanaged function pointer
```

## Type Syntax Parsing

### Basic Type Parsing

The type parser handles various syntactic forms:

```rust
fn parse_type(input: &str) -> BResult<&str, Type> {
    alt((
        parse_tuple_type,
        parse_function_pointer_type,
        parse_named_type,
        parse_primitive_type,
    ))(input)
}
```

### Array Type Parsing

Array types have specific syntax rules:

```csharp
int[]                  // T[]
int[,]                 // T[,]
int[,,]                // T[,,]
int[][]                // T[][] (jagged)
```

### Generic Type Parsing

Generic types require careful parsing of type arguments:

```csharp
List<int>              // Simple generic
Dictionary<string, List<int>>  // Nested generics
```

### Nullable Type Parsing

Nullable types use special syntax:

```csharp
int?                   // Nullable<int>
string?                // string with nullable annotation
```

## Type Resolution

### Qualified Names

Types can be fully qualified:

```csharp
System.Collections.Generic.List<int>
MyNamespace.MyClass
```

### Type Aliases

Using directives create type aliases:

```csharp
using StringList = System.Collections.Generic.List<string>;
```

### Global Type References

Global namespace references:

```csharp
global::System.String  // Fully qualified from global namespace
```

## Type Constraints

### Constraint Types

1. **Reference Type**: `where T : class`
2. **Value Type**: `where T : struct`
3. **Constructor**: `where T : new()`
4. **Base Class**: `where T : BaseClass`
5. **Interface**: `where T : IInterface`
6. **Type Parameter**: `where T : U`

### Constraint Combinations

Multiple constraints can be combined:

```csharp
where T : class, IDisposable, new()
```

### Constraint Validation

The parser validates constraint combinations:

- `class` and `struct` are mutually exclusive
- `new()` constraint must come last
- Base class constraint must come before interface constraints

## Type Variance

### Covariance and Contravariance

```csharp
interface ICovariant<out T> { }     // Covariant
interface IContravariant<in T> { }  // Contravariant
interface IInvariant<T> { }         // Invariant
```

## Advanced Type Features

### Record Types

```csharp
record Person(string Name, int Age);
record class Employee(string Name, int Age, string Department);
record struct Point(int X, int Y);
```

### Pattern Types

Types used in pattern matching:

```csharp
obj is string str          // Type pattern
obj is not null           // Negation pattern
obj is > 0 and < 100     // Relational pattern
```

## Type System Implementation

The type system is implemented with a hierarchical structure:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    Primitive(PrimitiveType),
    Named {
        name: Identifier,
        type_arguments: Option<Vec<Type>>,
    },
    Array {
        element_type: Box<Type>,
        dimensions: u32,
    },
    Nullable(Box<Type>),
    Tuple(Vec<(Option<Identifier>, Type)>),
    Pointer(Box<Type>),
    FunctionPointer {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}
```

## Error Handling

The type parser provides detailed error messages for:

- **Invalid type syntax**
- **Constraint violations**
- **Generic parameter mismatches**
- **Nullable context errors**
- **Variance violations**

## Type Inference

While the parser doesn't perform type inference (that's the compiler's job), it correctly parses:

- `var` declarations
- Anonymous types
- Implicitly typed arrays
- Lambda parameter types

The type system parser is designed to accurately represent the full complexity of C#'s type system while maintaining performance and providing clear error diagnostics.
