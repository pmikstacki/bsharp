# Type Analysis

The type analysis system provides insights into type usage, inheritance hierarchies, and type-related patterns in C# code.

---

## Overview

**Location:** `src/analysis/types/`

Type analysis tracks:
- Type definitions and their relationships
- Inheritance hierarchies
- Interface implementations
- Generic type usage
- Type references and dependencies

---

## Type Information

### Type Categories

**Value Types:**
- Primitives (`int`, `bool`, `double`, etc.)
- Structs
- Enums

**Reference Types:**
- Classes
- Interfaces
- Delegates
- Arrays

**Special Types:**
- Generic type parameters
- Nullable types
- Tuple types
- Anonymous types

---

## Inheritance Analysis

### Class Hierarchies

**Tracking Inheritance:**
```csharp
public class Animal { }
public class Mammal : Animal { }
public class Dog : Mammal { }
```

**Hierarchy Representation:**
```
Animal
└── Mammal
    └── Dog
```

**Analysis:**
```rust
pub struct InheritanceHierarchy {
    // Type -> Base Type
    base_types: HashMap<TypeId, TypeId>,
    // Type -> Derived Types
    derived_types: HashMap<TypeId, Vec<TypeId>>,
}

impl InheritanceHierarchy {
    pub fn get_base_type(&self, type_id: TypeId) -> Option<TypeId>;
    pub fn get_derived_types(&self, type_id: TypeId) -> &[TypeId];
    pub fn get_all_ancestors(&self, type_id: TypeId) -> Vec<TypeId>;
    pub fn get_all_descendants(&self, type_id: TypeId) -> Vec<TypeId>;
    pub fn inheritance_depth(&self, type_id: TypeId) -> usize;
}
```

### Interface Implementation

**Tracking Implementations:**
```csharp
public interface IRepository { }
public interface IUserRepository : IRepository { }
public class UserRepository : IUserRepository { }
```

**Analysis:**
```rust
pub struct InterfaceImplementations {
    // Type -> Interfaces it implements
    implementations: HashMap<TypeId, Vec<TypeId>>,
    // Interface -> Types that implement it
    implementers: HashMap<TypeId, Vec<TypeId>>,
}
```

---

## Generic Type Analysis

### Type Parameters

**Tracking Generic Definitions:**
```csharp
public class Container<T> where T : class { }
public class Repository<TEntity, TKey> where TEntity : class { }
```

**Analysis:**
```rust
pub struct GenericTypeInfo {
    pub type_parameters: Vec<TypeParameter>,
    pub constraints: Vec<TypeConstraint>,
}

pub struct TypeParameter {
    pub name: String,
    pub variance: Option<Variance>,  // in, out
}

pub struct TypeConstraint {
    pub parameter: String,
    pub kind: ConstraintKind,
}

pub enum ConstraintKind {
    Class,              // where T : class
    Struct,             // where T : struct
    New,                // where T : new()
    BaseType(TypeId),   // where T : BaseClass
    Interface(TypeId),  // where T : IInterface
}
```

### Generic Type Usage

**Tracking Instantiations:**
```csharp
var list = new List<int>();
var dict = new Dictionary<string, User>();
```

**Analysis:**
```rust
pub struct GenericInstantiation {
    pub generic_type: TypeId,
    pub type_arguments: Vec<TypeId>,
}

pub fn find_generic_instantiations(cu: &CompilationUnit) -> Vec<GenericInstantiation>;
```

---

## Type Usage Patterns

### Frequency Analysis

**Most Used Types:**
```rust
pub struct TypeUsageStats {
    pub type_references: HashMap<TypeId, usize>,
}

impl TypeUsageStats {
    pub fn most_used_types(&self, limit: usize) -> Vec<(TypeId, usize)>;
    pub fn usage_count(&self, type_id: TypeId) -> usize;
}
```

### Type Categories Distribution

```rust
pub struct TypeDistribution {
    pub class_count: usize,
    pub interface_count: usize,
    pub struct_count: usize,
    pub enum_count: usize,
    pub delegate_count: usize,
}
```

---

## Type Metrics

### Depth of Inheritance Tree (DIT)

**Definition:** Maximum depth from type to root of hierarchy

**Example:**
```csharp
class A { }              // DIT = 0 (or 1 from Object)
class B : A { }          // DIT = 1 (or 2 from Object)
class C : B { }          // DIT = 2 (or 3 from Object)
```

**Interpretation:**
- **Low DIT (0-2):** Simple hierarchy, easy to understand
- **Medium DIT (3-4):** Moderate complexity
- **High DIT (5+):** Complex hierarchy, may indicate over-engineering

### Number of Children (NOC)

**Definition:** Number of immediate subclasses

**Example:**
```csharp
class Animal { }
class Dog : Animal { }
class Cat : Animal { }
class Bird : Animal { }
// Animal has NOC = 3
```

**Interpretation:**
- **High NOC:** Type is heavily reused (good abstraction or god class)
- **Low NOC:** Specialized type or leaf in hierarchy

### Lack of Cohesion of Methods (LCOM)

**Definition:** Measure of how well methods in a class are related

**Simplified Calculation:**
- Count pairs of methods that don't share instance variables
- High LCOM suggests class should be split

---

## Type Compatibility Analysis

### Assignability

**Checking Compatibility:**
```rust
pub fn is_assignable_to(from: &Type, to: &Type, context: &TypeContext) -> bool {
    // Check if 'from' type can be assigned to 'to' type
    // Considers inheritance, interface implementation, variance, etc.
}
```

**Rules:**
- Derived type assignable to base type
- Type assignable to implemented interface
- Covariant/contravariant generic types
- Nullable value types
- Implicit conversions

### Type Conversions

**Tracking Conversions:**
```csharp
int x = 42;
long y = x;              // Implicit conversion
string s = x.ToString(); // Explicit conversion
```

**Analysis:**
```rust
pub enum ConversionKind {
    Implicit,
    Explicit,
    UserDefined,
}

pub struct TypeConversion {
    pub from: TypeId,
    pub to: TypeId,
    pub kind: ConversionKind,
}
```

---

## Nullable Reference Types Analysis

### Nullability Tracking

**C# 8+ Nullable Annotations:**
```csharp
string? nullable = null;      // Nullable reference
string nonNull = "value";     // Non-nullable reference
```

**Analysis:**
```rust
pub struct NullabilityInfo {
    pub is_nullable: bool,
    pub nullability_context: NullabilityContext,
}

pub enum NullabilityContext {
    Enabled,
    Disabled,
    Warnings,
}
```

### Null Safety Diagnostics

**Potential Null Reference:**
```
warning[TYPE001]: Possible null reference
  --> src/UserService.cs:15:9
   |
15 |     user.Name = "John";
   |     ^^^^ 'user' may be null here
   |
   = help: Add null check or use null-conditional operator
```

---

## Type Analysis in Pipeline

### Integration

**Phase:** Semantic (after symbol indexing)

```rust
impl AnalyzerPass for TypeAnalysisPass {
    fn id(&self) -> &'static str { "type_analysis" }
    fn phase(&self) -> Phase { Phase::Semantic }
    
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let type_info = analyze_types(cu, &session.artifacts.symbols);
        session.artifacts.type_info = Some(type_info);
    }
}
```

---

## Programmatic Usage

### Analyzing Type Hierarchy

```rust
use bsharp::analysis::types::InheritanceHierarchy;

let hierarchy = InheritanceHierarchy::build(cu);

// Get base type
if let Some(base) = hierarchy.get_base_type(type_id) {
    println!("Base type: {:?}", base);
}

// Get all derived types
let derived = hierarchy.get_derived_types(type_id);
println!("Derived types: {:?}", derived);

// Calculate inheritance depth
let depth = hierarchy.inheritance_depth(type_id);
println!("Inheritance depth: {}", depth);
```

### Finding Generic Instantiations

```rust
use bsharp::analysis::types::find_generic_instantiations;

let instantiations = find_generic_instantiations(cu);

for inst in instantiations {
    println!("Generic type: {:?}", inst.generic_type);
    println!("Type arguments: {:?}", inst.type_arguments);
}
```

---

## Future Enhancements

### Planned Features

1. **Type Inference Tracking**
   - Track `var` usage and inferred types
   - Analyze type inference patterns

2. **Variance Analysis**
   - Detect variance violations
   - Suggest covariant/contravariant annotations

3. **Type Safety Metrics**
   - Measure use of `dynamic`
   - Track unsafe casts
   - Nullable reference type coverage

4. **Design Pattern Detection**
   - Identify common patterns (Factory, Strategy, etc.)
   - Detect anti-patterns

---

## Implementation Status

**Current State:**
- Basic type tracking infrastructure in place
- Type analysis module integrated with analysis framework
- Foundation for inheritance and generic analysis established

**In Progress:**
- Full inheritance hierarchy analysis
- Generic type instantiation tracking
- Type usage statistics collection
- Comprehensive test coverage

**Planned:**
- Variance analysis
- Type safety metrics
- Design pattern detection based on type relationships

---

## Related Documentation

- [Analysis Pipeline](./pipeline.md) - Pipeline integration
- [Dependency Analysis](./dependencies.md) - Type dependencies
- [Metrics Collection](./metrics.md) - Type-related metrics
- [AST Structure](../parser/ast-structure.md) - Type representations

---

## References

- **Implementation:** `src/analysis/types/`
- **Tests:** `tests/analysis/types/`
- **Type System:** `src/syntax/nodes/types/`
