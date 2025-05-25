---
description: "Concept and steps for implementing AST navigation to parse XML documentation comments in bsharp."
---

# AST Navigation for XML Documentation Comments in bsharp

## 1. Goal

To define a clear strategy and implementation steps for navigating the `bsharp` Abstract Syntax Tree (AST) to locate, parse, and extract structured information from C# XML documentation comments. Specifically, this will enable tests like those in `name_attribute_value_parsing_tests.rs` to verify the parsing of elements like `<param name="...">`.

## 2. Understanding `bsharp`'s Current AST and Parsing (Assumptions & Investigation Points)

- **AST Root**: Parsing starts with `parse_csharp_source` and yields a `CompilationUnit` (from `bsharp::parser::ast`).
- **Top-Level Members**: `CompilationUnit` contains `members` (a `Vec<TopLevelMember>`), which can be `ClassDeclaration`, `StructDeclaration`, etc.
- **Method/Property Declarations**: These are typically members within `ClassDeclaration` or `StructDeclaration`.
- **Trivia and Doc Comments**: A key investigation is how trivia (whitespace, comments), and specifically XML documentation comments, are currently handled by the `bsharp` parser and represented in the AST.
    - Are they attached to tokens?
    - Are they stored as a list on each AST node (e.g., `node.leading_trivia`, `node.trailing_trivia`)?
    - Is there a specific `DocumentationComment` AST node type, or are they just generic comment strings?

## 3. Inspiration: Roslyn's Approach

Roslyn provides a robust model for handling trivia:
- Trivia is attached to `SyntaxToken`s (leading and trailing).
- XML documentation comments are a form of `StructuredTriviaSyntax`.
- Specific nodes exist for XML elements within these comments (e.g., `XmlElementSyntax`, `XmlNameAttributeSyntax`).
- Navigation involves: `SyntaxNode` -> `GetLeadingTrivia` -> Find `DocumentationCommentTriviaSyntax` -> Access its structured XML content.

## 4. Proposed `bsharp` Approach for XML Doc Comments

### 4.1. Trivia and Doc Comment Representation in AST

- **Decision Point**: How should `bsharp` store doc comments?
    - **Option A (Simple String)**: Store raw doc comment text as part of a node's trivia. Parsing the XML would be a secondary step on this string.
    - **Option B (Structured Node)**: Introduce specific AST nodes for XML documentation comments during the primary parsing phase.
        - E.g., `struct XmlDocumentationComment { elements: Vec<XmlElementNode> }`
        - This would likely involve integrating an XML parser into the C# parser's logic for doc comments.

### 4.2. XML Parsing Strategy

- If Option B (Structured Node) is chosen, or if XML parsing is deferred (Option A), an XML parsing strategy is needed.
- **Internal Mini-Parser**: Develop a small, focused XML parser for the subset of XML used in doc comments.
- **External Crate**: Utilize an existing Rust XML parsing crate (e.g., `quick-xml`, `xml-rs`). This is generally recommended for robustness.

### 4.3. AST Nodes for XML Content

If XML is parsed into a structured representation, define corresponding AST nodes:
- `enum XmlNode { Element(XmlElement), Text(String), CData(String) }`
- `struct XmlElement { name: Identifier, attributes: Vec<XmlAttribute>, children: Vec<XmlNode> }`
- `struct XmlAttribute { name: Identifier, value: String }`

### 4.4. Attaching to the Main AST

The C# parser needs to be modified:
- To recognize and collect lines forming XML documentation comments.
- To associate these comments (either as raw strings or parsed structures) with the relevant AST nodes (e.g., `MethodDeclaration`, `ClassDeclaration`).

## 5. Designing Helper Functions for Navigation

To support tests and potential future uses (like code analysis or documentation generation):
- **Accessing Doc Comments**: `ast_node.get_documentation_comment() -> Option<ParsedXmlDoc>` (or `Option<String>` if raw).
- **Querying XML Structure**: If parsed, methods to query the XML structure:
    - `parsed_xml_doc.find_elements_by_name("param") -> Vec<XmlElementNode>`
    - `xml_element_node.get_attribute_value("name") -> Option<String>`

This would allow the `try_parse_xml_name_attribute_identifier` helper in tests to be implemented by:
1. Parsing a C# snippet containing a method with a doc comment.
2. Navigating to the `MethodDeclaration` node.
3. Retrieving its associated `XmlDocumentationComment`.
4. Searching within the XML structure for a `<param>` tag.
5. Extracting the value of its `name` attribute.

## 6. Implementation Steps

1.  **Decision & Design (Trivia/XML)**: Finalize how XML doc comments will be represented in the AST (raw string vs. structured nodes) and the XML parsing strategy.
2.  **AST Node Definition**: If creating new AST nodes for XML, define them clearly.
3.  **Parser Modification (C# Parser)**:
    *   Update the lexer/parser to identify XML doc comment blocks.
    *   Store this information in the AST (as per step 1's design).
    *   If parsing XML directly, integrate the XML parsing logic.
4.  **Navigation API Development**: Implement helper methods or traits for easy access to documentation comment data from relevant AST nodes (e.g., `MethodDeclaration`, `ClassDeclaration`, `PropertyDeclaration`).
5.  **Testing**: Update `name_attribute_value_parsing_tests.rs` to use the new navigation API and remove placeholder logic. Add more comprehensive tests for various XML doc comment structures.
