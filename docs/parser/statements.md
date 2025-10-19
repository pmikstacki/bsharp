
# Statement Parsing

BSharp provides comprehensive parsing for all C# statement types, from simple expressions to complex control flow constructs.

## Statement Categories

### 1. Declaration Statements

#### Local Variable Declarations
```csharp
int x = 5;
var name = "John";
const double PI = 3.14159;
```

#### Local Function Declarations
```csharp
void LocalFunction(int parameter)
{
    // function body
}

T GenericLocalFunction<T>(T value) where T : class
{
    return value;
}
```

### 2. Expression Statements

Any expression followed by a semicolon:
```csharp
x++;                    // Increment
Method();              // Method call
obj.Property = value;  // Assignment
```

### 3. Control Flow Statements

#### Conditional Statements

**If Statements**
```csharp
if (condition)
    statement;

if (condition)
{
    // block
}
else if (otherCondition)
{
    // else if block
}
else
{
    // else block
}
```

**Switch Statements**
```csharp
switch (expression)
{
    case constant1:
        statements;
        break;
    case constant2 when condition:
        statements;
        goto case constant1;
    default:
        statements;
        break;
}
```

#### Loop Statements

**For Loops**
```csharp
for (int i = 0; i < 10; i++)
{
    // loop body
}

for (;;)  // infinite loop
{
    // body
}
```

**Foreach Loops**
```csharp
foreach (var item in collection)
{
    // process item
}

foreach ((string key, int value) in dictionary)
{
    // deconstruction in foreach
}
```

**While Loops**
```csharp
while (condition)
{
    // loop body
}
```

**Do-While Loops**
```csharp
do
{
    // loop body
} while (condition);
```

#### Jump Statements

```csharp
break;              // Break from loop/switch
continue;           // Continue to next iteration
return;             // Return from method
return value;       // Return with value
goto label;         // Jump to label
goto case 5;        // Jump to switch case
goto default;       // Jump to switch default
```

### 4. Exception Handling

#### Try-Catch-Finally
```csharp
try
{
    // risky code
}
catch (SpecificException ex) when (ex.Code == 123)
{
    // specific exception handling
}
catch (Exception ex)
{
    // general exception handling
}
finally
{
    // cleanup code
}
```

#### Throw Statements
```csharp
throw;                           // Rethrow current exception
throw new InvalidOperationException();
throw new CustomException("message");
```

### 5. Resource Management

#### Using Statements
```csharp
using (var resource = new DisposableResource())
{
    // use resource
}

using var resource = new DisposableResource();
// resource disposed at end of scope
```

#### Lock Statements
```csharp
lock (syncObject)
{
    // synchronized code
}
```

#### Fixed Statements
```csharp
unsafe
{
    fixed (byte* ptr = array)
    {
        // work with fixed pointer
    }
}
```

### 6. Special Statements

#### Yield Statements
```csharp
yield return value;     // Return value in iterator
yield break;           // End iterator
```

#### Checked/Unchecked Statements
```csharp
checked
{
    // arithmetic overflow checking enabled
}

unchecked
{
    // arithmetic overflow checking disabled
}
```

#### Unsafe Statements
```csharp
unsafe
{
    // unsafe code block
}
```

## Statement Parsing Implementation

The statement parser uses a dispatch mechanism based on the first token:

```rust
fn parse_statement(input: &str) -> BResult<&str, Statement> {
    alt((
        parse_block_statement,
        parse_if_statement,
        parse_while_statement,
        parse_for_statement,
        parse_foreach_statement,
        parse_do_while_statement,
        parse_switch_statement,
        parse_try_statement,
        parse_using_statement,
        parse_lock_statement,
        parse_return_statement,
        parse_throw_statement,
        parse_break_statement,
        parse_continue_statement,
        parse_goto_statement,
        parse_label_statement,
        parse_yield_statement,
        parse_local_declaration_statement,
        parse_local_function_statement,
        parse_expression_statement,
        parse_empty_statement,
    ))(input.into())
}
```

## Block Statements

Block statements group multiple statements:

```csharp
{
    int x = 5;
    Console.WriteLine(x);
    if (x > 0)
    {
        Console.WriteLine("Positive");
    }
}
```

## Error Recovery

The statement parser implements robust error recovery:

1. **Statement-level recovery**: Skip to next statement boundary (semicolon or brace)
2. **Block-level recovery**: Skip to matching brace
3. **Context preservation**: Maintain parsing context across errors

## Statement Attributes

Statements can have attributes applied:

```csharp
[Obsolete("Use NewMethod instead")]
void OldMethod() { }

[ConditionalAttribute("DEBUG")]
static void DebugMethod() { }
```

## Top-Level Statements

Support for C# 9+ top-level statements:

```csharp
// Program.cs
using System;

Console.WriteLine("Hello World!");
return 0;
```

The statement parser is designed to handle the full complexity of C# control flow while providing clear error messages and robust error recovery.
