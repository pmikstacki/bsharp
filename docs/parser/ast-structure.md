
# AST Structure

The BSharp AST (Abstract Syntax Tree) provides a complete, structured representation of C# source code. This document explains the organization and relationships between different AST node types.

## AST Hierarchy

### Root Node: CompilationUnit

Every parsed C# file results in a `CompilationUnit`, which serves as the root of the AST:

```rust
pub struct CompilationUnit {
    pub global_attributes: Vec<GlobalAttribute>,        // [assembly: ...] attributes
    pub using_directives: Vec<UsingDirective>,          // using statements
    pub global_using_directives: Vec<GlobalUsingDirective>, // C# 10+ global using
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
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub primary_constructor_parameters: Option<Vec<Parameter>>, // C# 12
    pub base_types: Vec<Type>,
    pub body_declarations: Vec<ClassBodyDeclaration>,
    pub documentation: Option<XmlDocumentationComment>,
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
}
```

#### MethodDeclaration
```rust
pub struct MethodDeclaration {
    pub modifiers: Vec<Modifier>,
    pub return_type: Type,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub parameters: Vec<Parameter>,
    pub body: Option<Statement>,                 // None for abstract/interface methods
    pub constraints: Option<Vec<TypeParameterConstraintClause>>,
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
    // Primary and names
    Literal(Literal),
    Variable(Identifier),

    // Object and member operations
    New(Box<NewExpression>),
    MemberAccess(Box<MemberAccessExpression>),
    Invocation(Box<InvocationExpression>),
    Indexing(Box<IndexingExpression>),
    Index(Box<IndexExpression>),
    Range(Box<RangeExpression>),

    // Lambda and anonymous methods
    Lambda(Box<LambdaExpression>),
    AnonymousMethod(Box<AnonymousMethodExpression>),

    // Keywords
    This,
    Base,

    // Operators
    Unary { op: UnaryOperator, expr: Box<Expression> },
    Binary { left: Box<Expression>, op: BinaryOperator, right: Box<Expression> },
    PostfixUnary { op: UnaryOperator, expr: Box<Expression> },
    Assignment(Box<AssignmentExpression>),

    // Patterns and type ops
    Pattern(Box<Pattern>),
    IsPattern { expression: Box<Expression>, pattern: Box<Pattern> },
    As { expression: Box<Expression>, target_type: Type },
    Cast { expression: Box<Expression>, target_type: Type },

    // Misc language features
    Conditional(Box<ConditionalExpression>),
    Query(Box<QueryExpression>),
    Await(Box<AwaitExpression>),
    Throw(Box<ThrowExpression>),
    Nameof(Box<NameofExpression>),
    Typeof(Box<TypeofExpression>),
    Sizeof(Box<SizeofExpression>),
    Default(Box<DefaultExpression>),
    StackAlloc(Box<StackAllocExpression>),
    Ref(Box<Expression>),
    Checked(Box<CheckedExpression>),
    Unchecked(Box<UncheckedExpression>),

    // With/collection expressions
    With { target: Box<Expression>, initializers: Vec<WithInitializerEntry> },
    Collection(Vec<CollectionElement>),

    // Composite forms
    AnonymousObject(AnonymousObjectCreationExpression),
    Tuple(TupleExpression),
    SwitchExpression(Box<SwitchExpression>),
}
```

Key helper structs:

```rust
pub struct SwitchExpression {
    pub expression: Expression,
    pub arms: Vec<SwitchExpressionArm>,
}

pub enum WithInitializerEntry {
    Property { name: String, value: Expression },
    Indexer { indices: Vec<Expression>, value: Expression },
}

pub enum CollectionElement {
    Expr(Expression),
    Spread(Expression),
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
