
# Comment Parsing

BSharp implements comprehensive comment parsing for both regular comments and XML documentation comments, preserving them as part of the AST for documentation generation and analysis tools.

## Comment Types

### 1. Single-Line Comments

Standard C++ style comments:
```csharp
// This is a single-line comment
int x = 5; // End-of-line comment
```

### 2. Multi-Line Comments

Traditional C-style block comments:
```csharp
/*
 * This is a multi-line comment
 * that spans several lines
 */
int y = 10; /* Inline block comment */
```

### 3. XML Documentation Comments

#### Single-Line XML Comments
```csharp
/// <summary>
/// This method calculates the sum of two integers.
/// </summary>
/// <param name="a">The first integer.</param>
/// <param name="b">The second integer.</param>
/// <returns>The sum of a and b.</returns>
public int Add(int a, int b)
{
    return a + b;
}
```

#### Multi-Line XML Comments
```csharp
/**
 * <summary>
 * This is a multi-line XML documentation comment.
 * It provides detailed information about the method.
 * </summary>
 * <param name="value">The input value to process.</param>
 * <returns>The processed result.</returns>
 */
public string ProcessValue(string value) { }
```

## XML Documentation Structure

### Standard XML Tags

#### Summary and Description
```xml
<summary>
Brief description of the member.
</summary>

<remarks>
Detailed remarks and additional information.
</remarks>
```

#### Parameters and Returns
```xml
<param name="parameterName">Description of the parameter.</param>
<returns>Description of the return value.</returns>
```

#### Exceptions
```xml
<exception cref="ArgumentNullException">
Thrown when the parameter is null.
</exception>
```

#### Examples
```xml
<example>
This example shows how to use the method:
<code>
var result = MyMethod("input");
Console.WriteLine(result);
</code>
</example>
```

#### See References
```xml
<see cref="RelatedMethod"/>
<seealso cref="AnotherClass"/>
```

#### Generic Type Parameters
```xml
<typeparam name="T">The type parameter.</typeparam>
<typeparamref name="T"/>
```

### Custom XML Tags

The parser supports custom XML tags:
```xml
<custom attribute="value">
Custom content with <nested>elements</nested>.
</custom>
```

## XML Documentation Parsing

### XML Element Structure

The parser represents XML elements with:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlElement {
    pub name: Identifier,
    pub attributes: Vec<XmlAttribute>,
    pub children: Vec<XmlNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlAttribute {
    pub name: Identifier,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum XmlNode {
    Element(XmlElement),
    Text(String),
    CData(String),
    Comment(String),
}
```

### XML Documentation Comment

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XmlDocumentationComment {
    pub elements: Vec<XmlNode>,
}
```

### Parsing XML Attributes

The parser handles XML attributes with various syntaxes:

```xml
<param name="value">Description</param>
<see cref="MyClass.MyMethod(int, string)"/>
<exception cref="System.ArgumentException">Error description</exception>
```

### XML Content Parsing

The parser processes mixed content:

```xml
<summary>
This method processes <paramref name="input"/> and returns
<see cref="ProcessResult"/> containing the result.
</summary>
```

## Comment Association

### Declaration Comments

Comments are associated with their following declarations:

```csharp
/// <summary>Class documentation</summary>
public class MyClass
{
    /// <summary>Method documentation</summary>
    public void MyMethod() { }
}
```

### Member Comments

Each declaration can have associated documentation:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodDeclaration {
    pub documentation: Option<XmlDocumentationComment>,
    // ... other fields
}
```

## Advanced XML Features

### CDATA Sections

The parser handles CDATA sections for literal content:

```xml
<example>
<![CDATA[
if (x < y && y > z)
{
    Console.WriteLine("Complex condition");
}
]]>
</example>
```

### Nested XML Elements

Complex nested structures are supported:

```xml
<summary>
This method handles <see cref="List{T}"/> where T is
<typeparamref name="T"/> and implements <see cref="IComparable{T}"/>.
</summary>
```

### XML Namespaces

The parser can handle XML namespaces in documentation:

```xml
<doc:summary xmlns:doc="http://schemas.microsoft.com/developer/documentation">
Namespaced documentation content.
</doc:summary>
```

## Comment Preservation

### Comment Tokens

Comments are preserved as tokens in the AST:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentToken {
    SingleLine(String),
    MultiLine(String),
    XmlDocumentation(XmlDocumentationComment),
}
```

### Position Information

Comments maintain position information:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionedComment {
    pub comment: CommentToken,
    pub line: usize,
    pub column: usize,
}
```

## Error Handling

### XML Validation

The parser validates XML structure:

- **Well-formed XML**: Proper opening and closing tags
- **Attribute syntax**: Valid attribute name-value pairs
- **Nesting rules**: Correct element nesting
- **Character escaping**: Proper XML character escaping

### Error Recovery

When XML is malformed, the parser attempts recovery:

- **Skip malformed elements**: Continue parsing after errors
- **Preserve content**: Keep as much content as possible
- **Error reporting**: Provide detailed error locations

## Integration with Analysis

### Documentation Analysis

Comments are available for analysis tools:

```rust
impl XmlDocumentationComment {
    pub fn find_elements_by_name(&self, name: &str) -> Vec<&XmlElement> {
        // Find all elements with the given tag name
    }
    
    pub fn get_summary(&self) -> Option<String> {
        // Extract summary text
    }
    
    pub fn get_parameters(&self) -> Vec<(String, String)> {
        // Extract parameter documentation
    }
}
```

### Documentation Generation

The parsed XML documentation can be used for:

- **API documentation generation**
- **IntelliSense information**
- **Code analysis and quality checks**
- **Documentation coverage reports**

## Performance Considerations

### Lazy Parsing

XML documentation can be parsed lazily when needed:

```rust
#[derive(Debug, Clone)]
pub enum DocumentationState {
    Unparsed(String),
    Parsed(XmlDocumentationComment),
    Invalid(String, ParseError),
}
```

### Memory Optimization

The parser optimizes memory usage by:

- **String interning**: Reusing common XML tag names
- **Structured storage**: Efficient representation of XML structure
- **On-demand parsing**: Parse XML only when accessed

The comment parsing system ensures that all documentation and comments are preserved and available for analysis, while maintaining the performance characteristics needed for large codebases.
