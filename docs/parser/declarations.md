
# Declaration Parsing

BSharp implements comprehensive parsing for all C# declaration types, from simple variables to complex generic types with constraints.

## Declaration Categories

### 1. Namespace Declarations

#### Traditional Namespace
```csharp
namespace MyCompany.MyProject
{
    // namespace members
}
```

#### File-Scoped Namespace (C# 10+)
```csharp
namespace MyCompany.MyProject;

// All following declarations belong to this namespace
```

#### Nested Namespaces
```csharp
namespace Outer
{
    namespace Inner
    {
        // nested namespace content
    }
}
```

### 2. Type Declarations

#### Class Declarations
```csharp
public class MyClass : BaseClass, IInterface1, IInterface2
{
    // class members
}

public abstract class AbstractClass
{
    public abstract void AbstractMethod();
}

public sealed class SealedClass
{
    // cannot be inherited
}
```

#### Interface Declarations
```csharp
public interface IMyInterface : IBaseInterface
{
    void Method();
    int Property { get; set; }
    event Action SomeEvent;
}

public interface IGeneric<T> where T : class
{
    T GenericMethod<U>(U parameter) where U : struct;
}
```

#### Struct Declarations
```csharp
public struct Point
{
    public int X { get; set; }
    public int Y { get; set; }
    
    public Point(int x, int y)
    {
        X = x;
        Y = y;
    }
}

public readonly struct ReadOnlyPoint
{
    public readonly int X;
    public readonly int Y;
    
    public ReadOnlyPoint(int x, int y)
    {
        X = x;
        Y = y;
    }
}
```

#### Record Declarations
```csharp
public record Person(string FirstName, string LastName);

public record class Employee(string FirstName, string LastName, string Department)
    : Person(FirstName, LastName);

public record struct Point(int X, int Y);
```

#### Enum Declarations
```csharp
public enum Color
{
    Red,
    Green,
    Blue
}

[Flags]
public enum FileAccess : byte
{
    None = 0,
    Read = 1,
    Write = 2,
    Execute = 4,
    All = Read | Write | Execute
}
```

#### Delegate Declarations
```csharp
public delegate void EventHandler(object sender, EventArgs e);
public delegate T GenericDelegate<T, U>(U parameter) where T : class;
```

### 3. Member Declarations

#### Field Declarations
```csharp
private int field;
public readonly string ReadOnlyField;
public const double PI = 3.14159;
private static readonly List<string> StaticField = new();
```

#### Property Declarations
```csharp
// Auto-implemented properties
public string Name { get; set; }
public int Age { get; private set; }
public bool IsValid { get; init; }

// Properties with backing fields
private string _description;
public string Description
{
    get => _description;
    set => _description = value?.Trim();
}

// Expression-bodied properties
public string FullName => $"{FirstName} {LastName}";

// Indexer properties
public string this[int index]
{
    get => items[index];
    set => items[index] = value;
}
```

#### Method Declarations
```csharp
public void VoidMethod() { }
public int MethodWithReturnType() => 42;
public static T GenericMethod<T>(T parameter) where T : new() => new T();

// Async methods
public async Task<string> AsyncMethod()
{
    await Task.Delay(1000);
    return "result";
}

// Extension methods
public static class Extensions
{
    public static bool IsEmpty(this string str) => string.IsNullOrEmpty(str);
}
```

#### Constructor Declarations
```csharp
public class MyClass
{
    public MyClass() { }                    // Default constructor
    public MyClass(string name) : this()   // Constructor chaining
    {
        Name = name;
    }
    
    static MyClass()                        // Static constructor
    {
        // Static initialization
    }
}
```

#### Destructor Declarations
```csharp
public class Resource
{
    ~Resource()
    {
        // Cleanup code
    }
}
```

Note: In the AST, `DestructorDeclaration.body` is `Option<Statement>`:

```rust
// Some(Block(...)) for `{ ... }`, None for extern (i.e., `;` only)
pub struct DestructorDeclaration {
    pub name: Identifier,
    pub body: Option<Statement>,
}
```

#### Event Declarations
```csharp
public event Action<string> SomethingHappened;

public event EventHandler<CustomEventArgs> CustomEvent
{
    add { customEvent += value; }
    remove { customEvent -= value; }
}
```

#### Operator Declarations
```csharp
public static Point operator +(Point a, Point b)
{
    return new Point(a.X + b.X, a.Y + b.Y);
}

public static implicit operator string(Point p)
{
    return $"({p.X}, {p.Y})";
}
```

### 4. Generic Constraints

#### Type Parameter Constraints
```csharp
public class Container<T> where T : class, IDisposable, new()
{
    // T must be a reference type, implement IDisposable, and have a parameterless constructor
}

public void Method<T, U>()
    where T : class
    where U : struct, IComparable<U>
{
    // Multiple constraint clauses
}
```

AST mapping for constraints:

```rust
// On type declarations (class/struct/interface/record)
pub struct ClassDeclaration {
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}

// On methods
pub struct MethodDeclaration {
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}
```

### 5. Modifiers and Attributes

#### Access Modifiers
- `public` - Accessible everywhere
- `private` - Accessible only within the same class
- `protected` - Accessible within class and derived classes
- `internal` - Accessible within the same assembly
- `protected internal` - Accessible within assembly or derived classes
- `private protected` - Accessible within derived classes in the same assembly

#### Other Modifiers
- `static` - Belongs to the type rather than instance
- `abstract` - Must be overridden in derived classes
- `virtual` - Can be overridden in derived classes
- `override` - Overrides a virtual/abstract member
- `sealed` - Cannot be overridden further
- `readonly` - Can only be assigned during initialization
- `const` - Compile-time constant
- `async` - Asynchronous method
- `unsafe` - Contains unsafe code
- `extern` - Implemented externally

#### Attributes
```csharp
[Obsolete("Use NewMethod instead")]
public void OldMethod() { }

[DllImport("kernel32.dll")]
public static extern bool SetConsoleTitle(string title);

[AttributeUsage(AttributeTargets.Class | AttributeTargets.Method)]
public class CustomAttribute : Attribute
{
    public string Description { get; set; }
}
```

### 6. Using Directives

```csharp
using System;                           // Namespace using
using System.Collections.Generic;
using static System.Math;               // Static using
using Project = MyCompany.MyProject;    // Alias directive
global using System.Text;              // Global using (C# 10+)
```

Note: `global using` directives are stored at the compilation unit level in `CompilationUnit.global_using_directives`.

## Declaration Parsing Implementation

The declaration parser uses a multi-stage approach:

1. **Modifier Parsing**: Parse access modifiers and other keywords
2. **Declaration Type Detection**: Determine what kind of declaration
3. **Specific Parser Dispatch**: Route to specialized parser
4. **Member Collection**: Gather all declaration components

```rust
fn parse_type_declaration(input: &str) -> BResult<&str, TypeDeclaration> {
    let (input, attributes) = many0(parse_attribute)(input.into())?;
    let (input, modifiers) = parse_modifiers(input.into())?;
    let (input, declaration) = alt((
        parse_class_declaration,
        parse_interface_declaration,
        parse_struct_declaration,
        parse_enum_declaration,
        parse_delegate_declaration,
        parse_record_declaration,
    ))(input.into())?;
    
    Ok((input, TypeDeclaration {
        attributes,
        modifiers,
        declaration,
    }))
}
```

## Error Handling

The declaration parser provides comprehensive error reporting:

- **Modifier conflicts**: Detecting incompatible modifier combinations
- **Constraint validation**: Ensuring generic constraints are valid
- **Accessibility consistency**: Verifying access level consistency
- **Syntax validation**: Catching malformed declarations

### Recovery for Malformed Members

When a member inside a type body fails to parse, the parser uses a scoped recovery strategy to skip to the next safe boundary without crossing the enclosing type's closing brace. See the dedicated section in Error Handling for details on `skip_to_member_boundary_top_level()` and its contract:

- docs: `docs/parser/error-handling.md` (Declaration Error Recovery subsection)

## XML Documentation

The parser handles XML documentation comments:

```csharp
/// <summary>
/// Calculates the area of a rectangle.
/// </summary>
/// <param name="width">The width of the rectangle.</param>
/// <param name="height">The height of the rectangle.</param>
/// <returns>The area of the rectangle.</returns>
public double CalculateArea(double width, double height)
{
    return width * height;
}
```

The declaration parser is designed to handle the full complexity of C# type system while maintaining performance and providing detailed error diagnostics.
