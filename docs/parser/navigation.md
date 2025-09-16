
# AST Navigation

The BSharp parser provides powerful navigation capabilities through a set of traits that allow traversing and searching the AST efficiently. This system enables complex code analysis scenarios.

## Navigation Traits

### AstNavigate Trait

The primary trait for finding specific statement types within AST nodes:

```rust
pub trait AstNavigate {
    fn find_if_statements(&self) -> Vec<&Statement>;
    fn find_for_loops(&self) -> Vec<&Statement>;
    fn find_while_loops(&self) -> Vec<&Statement>;
    fn find_switch_statements(&self) -> Vec<&Statement>;
}
```

This trait is implemented for all major AST node types, providing a consistent interface for statement searching.

### FindDeclarations Trait

Specialized trait for finding declaration types:

```rust
pub trait FindDeclarations {
    fn find_classes(&self) -> Vec<&ClassDeclaration>;
    fn find_methods(&self) -> Vec<&MethodDeclaration>;
}
```

This trait focuses on structural elements like classes and methods, which are commonly needed for code analysis.

## Implementation Hierarchy

### CompilationUnit Navigation

The root-level implementation provides top-down traversal:

```rust
impl AstNavigate for CompilationUnit {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for class in self.find_classes() {
            results.extend(class.find_if_statements());
        }
        results
    }
    // Similar implementations for other statement types
}
```

This implementation:
- Traverses all top-level declarations
- Delegates to class-level navigation
- Aggregates results from all classes

### Class-Level Navigation

Class navigation focuses on methods and their contents:

```rust
impl AstNavigate for ClassDeclaration {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        for method in self.find_methods() {
            results.extend(method.find_if_statements());
        }
        results
    }
}
```

Key features:
- Iterates through all class body declarations
- Filters for method declarations
- Delegates statement searching to methods

### Method-Level Navigation

Methods provide direct access to their statement body:

```rust
impl AstNavigate for MethodDeclaration {
    fn find_if_statements(&self) -> Vec<&Statement> {
        if let Some(body) = &self.body {
            body.find_if_statements()
        } else {
            Vec::new()
        }
    }
}
```

Handles both:
- Methods with bodies (concrete methods)
- Methods without bodies (abstract/interface methods)

### Statement-Level Navigation

The most complex navigation occurs at the statement level:

```rust
impl AstNavigate for Statement {
    fn find_if_statements(&self) -> Vec<&Statement> {
        let mut results = Vec::new();
        collect_if_statements(self, &mut results);
        results
    }
}
```

Uses recursive helper functions to traverse nested statement structures.

## Recursive Collection Algorithms

### If Statement Collection

Specialized algorithm for finding if statements:

```rust
fn collect_if_statements<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::If(if_stmt) => {
            results.push(stmt);                              // Add this if statement
            collect_if_statements(&if_stmt.consequence, results);  // Search consequence
            if let Some(alt) = &if_stmt.alternative {
                collect_if_statements(alt, results);         // Search alternative
            }
        }
        Statement::Block(statements) => {
            for s in statements {
                collect_if_statements(s, results);           // Search each statement in block
            }
        }
        // Handle other statement types that can contain nested statements
        Statement::For(for_stmt) => collect_if_statements(&for_stmt.body, results),
        Statement::While(while_stmt) => collect_if_statements(&while_stmt.body, results),
        // ... other cases
        _ => {}
    }
}
```

This algorithm:
- Identifies target statements and adds them to results
- Recursively searches nested statement structures
- Handles all statement types that can contain other statements

### Loop Collection

Similar algorithms exist for different loop types:

#### For Loop Collection
```rust
fn collect_for_loops<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::For(for_stmt) => {
            results.push(stmt);
            collect_for_loops(&for_stmt.body, results);
        }
        // ... search other statement types for nested for loops
    }
}
```

#### While Loop Collection
```rust
fn collect_while_loops<'a>(stmt: &'a Statement, results: &mut Vec<&'a Statement>) {
    match stmt {
        Statement::While(_) | Statement::DoWhile(_) => {
            results.push(stmt);
            match stmt {
                Statement::While(while_stmt) => collect_while_loops(&while_stmt.body, results),
                Statement::DoWhile(do_while_stmt) => collect_while_loops(&do_while_stmt.body, results),
                _ => unreachable!(),
            }
        }
        // ... search other statement types
    }
}
```

## Declaration Finding

### Class Discovery

Finding classes across namespace and top-level boundaries:

```rust
impl FindDeclarations for CompilationUnit {
    fn find_classes(&self) -> Vec<&ClassDeclaration> {
        let mut classes = Vec::new();
        for member in &self.declarations {
            match member {
                TopLevelDeclaration::Namespace(ns) => {
                    for ns_member in &ns.declarations {
                        if let NamespaceBodyDeclaration::Class(class) = ns_member {
                            classes.push(class);
                        }
                    }
                }
                TopLevelDeclaration::Class(class) => {
                    classes.push(class);
                }
                _ => {}
            }
        }
        classes
    }
}
```

Handles:
- Classes declared at the top level
- Classes declared within namespaces
- Nested class scenarios

### Method Discovery

Finding methods within class bodies:

```rust
impl FindDeclarations for ClassDeclaration {
    fn find_methods(&self) -> Vec<&MethodDeclaration> {
        let mut methods = Vec::new();
        for member in &self.body_declarations {
            if let ClassBodyDeclaration::Method(method) = member {
                methods.push(method);
            }
        }
        methods
    }
}
```

Features:
- Iterates through all class body declarations
- Filters for method declarations
- Returns references to avoid cloning

## Usage Patterns

### Basic Statement Search

```rust
// Find all if statements in a compilation unit
let if_statements = compilation_unit.find_if_statements();
for if_stmt in if_statements {
    // Analyze each if statement
}
```

### Complex Analysis

```rust
// Find methods with complex control flow
let methods = compilation_unit.find_methods();
for method in methods {
    let if_count = method.find_if_statements().len();
    let loop_count = method.find_for_loops().len() + method.find_while_loops().len();
    let switch_count = method.find_switch_statements().len();
    
    if if_count + loop_count + switch_count > 10 {
        println!("Complex method: {:?}", method.identifier);
    }
}
```

### Nested Structure Analysis

```rust
// Analyze nesting depth of control structures
fn analyze_nesting_depth(stmt: &Statement) -> usize {
    match stmt {
        Statement::If(if_stmt) => {
            let consequence_depth = analyze_nesting_depth(&if_stmt.consequence);
            let alternative_depth = if_stmt.alternative
                .as_ref()
                .map(|alt| analyze_nesting_depth(alt))
                .unwrap_or(0);
            1 + std::cmp::max(consequence_depth, alternative_depth)
        }
        Statement::Block(statements) => {
            statements.iter()
                .map(analyze_nesting_depth)
                .max()
                .unwrap_or(0)
        }
        // ... handle other statement types
        _ => 0,
    }
}
```

## Performance Considerations

### Memory Efficiency

- Uses references (`&`) instead of cloning nodes
- Minimal allocation during traversal
- Lazy evaluation where possible

### Time Complexity

- Linear traversal: O(n) where n is the number of AST nodes
- No redundant visits to nodes
- Early termination for targeted searches

### Extensibility

The navigation system is designed for extension:

- Add new search methods to existing traits
- Implement traits for new AST node types
- Create specialized navigation traits for specific analysis needs

This navigation system provides the foundation for sophisticated code analysis while maintaining performance and usability.
