
# AST Structure

The BSharp AST (Abstract Syntax Tree) provides a complete, structured representation of C# source code. This document explains the organization and relationships between different AST node types.

## AST Hierarchy

### Root Node: CompilationUnit

Every parsed C# file results in a `CompilationUnit`, which serves as the root of the AST:

```rust
pub struct CompilationUnit {
    pub global_attributes: Vec<GlobalAttribute>,        // [assembly: ...] attributes
    pub using_directives: Vec<UsingDirective>,          // using statements
    pub declarations: Vec<TopLevelDeclaration>,         // namespaces, types
    pub file_scoped_namespace: Option<FileScopedNamespaceDeclaration>, // C# 10+
    pub top_level_statements: Vec<Statement>,           // C# 9+ top-level code
}
```

This structure supports both traditional C# files and modern features like file-scoped namespaces and top-level statements.

## Declaration Hierarchy

### Top-Level Declarations

Top-level declarations represent constructs that can appear at the file or namespace level:

```rust
pub enum TopLevelDeclaration {
    Namespace(NamespaceDeclaration),
    FileScopedNamespace(FileScopedNamespaceDeclaration),
    Class(ClassDeclaration),
    Struct(StructDeclaration),
    Record(RecordDeclaration),
    Interface(InterfaceDeclaration),
    Enum(EnumDeclaration),
    Delegate(DelegateDeclaration),
    GlobalAttribute(GlobalAttribute),
}
```

### Type Declarations

Each type declaration contains comprehensive information about the type:

#### ClassDeclaration
```rust
pub struct ClassDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub identifier: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>,                    // base class and interfaces
    pub body_declarations: Vec<ClassBodyDeclaration>,
    pub type_parameter_constraints: Vec<TypeParameterConstraint>,
}
```

#### MethodDeclaration
```rust
pub struct MethodDeclaration {
    pub attributes: Vec<Attribute>,
    pub modifiers: Vec<Modifier>,
    pub return_type: Option<Type>,               // None for constructors
    pub identifier: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,                 // None for abstract/interface methods
    pub type_parameter_constraints: Vec<TypeParameterConstraint>,
}
```

### Member Declarations

Class body declarations represent all possible class members:

```rust
pub enum ClassBodyDeclaration {
    Method(MethodDeclaration),
    Constructor(ConstructorDeclaration),
    Destructor(DestructorDeclaration),
    Property(PropertyDeclaration),
    Field(FieldDeclaration),
    Event(EventDeclaration),
    Indexer(IndexerDeclaration),
    Operator(OperatorDeclaration),
    NestedClass(ClassDeclaration),
    NestedStruct(StructDeclaration),
    NestedInterface(InterfaceDeclaration),
    NestedEnum(EnumDeclaration),
    NestedDelegate(DelegateDeclaration),
}
```

## Expression Hierarchy

### Expression Types

The expression system covers all C# expression types with proper precedence:

```rust
pub enum Expression {
    // Primary expressions
    Literal(Literal),
    Identifier(Identifier),
    Parenthesized(Box<Expression>),
    
    // Member access
    MemberAccess { object: Box<Expression>, member: Identifier },
    ElementAccess { object: Box<Expression>, arguments: Vec<Expression> },
    
    // Method calls
    Invocation { expression: Box<Expression>, arguments: Vec<Expression> },
    
    // Operators
    Unary { operator: UnaryOperator, operand: Box<Expression> },
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
    
    // Assignment
    Assignment { left: Box<Expression>, operator: AssignmentOperator, right: Box<Expression> },
    
    // Object creation
    ObjectCreation { type_: Type, arguments: Vec<Expression> },
    ArrayCreation { type_: Type, dimensions: Vec<Expression> },
    
    // Modern features
    Lambda(LambdaExpression),
    Query(QueryExpression),
    Await(Box<AwaitExpression>),
    Switch(SwitchExpression),
    
    // Type operations
    Cast { expression: Box<Expression>, target_type: Type },
    TypeCheck { expression: Box<Expression>, target_type: Type },
    
    // And many more...
}
```

### Literal Types

Comprehensive support for C# literals:

```rust
pub enum Literal {
    Boolean(bool),
    Integer(String),          // Preserves original format
    FloatingPoint(String),    // Preserves original format
    Character(char),
    String(String),
    InterpolatedString(InterpolatedStringLiteral),
    Null,
    Default,
}
```

## Statement Hierarchy

### Statement Types

Complete coverage of C# statement types:

```rust
pub enum Statement {
    // Control flow
    If(IfStatement),
    Switch(SwitchStatement),
    For(ForStatement),
    ForEach(ForEachStatement),
    While(WhileStatement),
    DoWhile(DoWhileStatement),
    
    // Jump statements
    Break(BreakStatement),
    Continue(ContinueStatement),
    Return(ReturnStatement),
    Throw(ThrowStatement),
    Goto(GotoStatement),
    
    // Exception handling
    Try(TryStatement),
    
    // Resource management
    Using(UsingStatement),
    Lock(LockStatement),
    
    // Declarations and expressions
    LocalVariableDeclaration(LocalVariableDeclaration),
    ExpressionStatement(Expression),
    Block(Vec<Statement>),
    Empty,
    
    // Modern features
    LocalFunction(LocalFunctionStatement),
}
```

### Control Flow Statements

Complex control flow statements contain nested structures:

#### IfStatement
```rust
pub struct IfStatement {
    pub condition: Expression,
    pub consequence: Box<Statement>,
    pub alternative: Option<Box<Statement>>,
}
```

#### TryStatement
```rust
pub struct TryStatement {
    pub body: Box<Statement>,
    pub catch_clauses: Vec<CatchClause>,
    pub finally_clause: Option<FinallyClause>,
}
```

## Type System

### Type Representation

The type system models all C# type constructs:

```rust
pub enum Type {
    // Primitive types
    Primitive(PrimitiveType),
    
    // Named types
    Named { name: Identifier, type_arguments: Vec<Type> },
    
    // Array types
    Array { element_type: Box<Type>, rank: usize },
    
    // Pointer types
    Pointer(Box<Type>),
    
    // Nullable types
    Nullable(Box<Type>),
    
    // Generic type parameters
    TypeParameter(Identifier),
    
    // Tuple types
    Tuple(Vec<Type>),
}
```

### Generic Support

Full support for C# generics:

#### TypeParameter
```rust
pub struct TypeParameter {
    pub attributes: Vec<Attribute>,
    pub variance: Option<Variance>,      // in, out
    pub identifier: Identifier,
}
```

#### TypeParameterConstraint
```rust
pub enum TypeParameterConstraint {
    TypeConstraint { parameter: Identifier, constraint_type: Type },
    ConstructorConstraint(Identifier),    // new()
    ClassConstraint(Identifier),          // class
    StructConstraint(Identifier),         // struct
    UnmanagedConstraint(Identifier),      // unmanaged
}
```

## AST Metadata

### Attributes

Comprehensive attribute support:

```rust
pub struct Attribute {
    pub name: Identifier,
    pub arguments: Vec<AttributeArgument>,
}

pub enum AttributeArgument {
    Positional(Expression),
    Named { name: Identifier, value: Expression },
}
```

### Modifiers

All C# modifiers are represented:

```rust
pub enum Modifier {
    // Access modifiers
    Public, Private, Protected, Internal, ProtectedInternal, PrivateProtected,
    
    // Other modifiers
    Static, Abstract, Virtual, Override, Sealed, New,
    Async, Unsafe, Volatile, Readonly, Const,
    Partial, Extern,
}
```

## Navigation and Relationships

The AST maintains clear parent-child relationships while providing navigation capabilities through traits:

- **Ownership**: Parent nodes own their children
- **Navigation**: Traits provide methods to traverse and search the AST
- **Context**: Nodes can access their containing context when needed

This structure provides a complete, navigable representation of C# code that supports both analysis and transformation scenarios.
