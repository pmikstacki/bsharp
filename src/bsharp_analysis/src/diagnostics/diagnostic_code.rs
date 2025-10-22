use crate::DiagnosticSeverity;
use crate::diagnostics::diagnostic_category::DiagnosticCategory;
use serde::{Deserialize, Serialize};

/// B# Diagnostic Error/Warning Codes
/// Format: BSE[XXXXX] for errors, BSW[XXXXX] for warnings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticCode {
    // Semantic Errors (BSE01000-BSE01999)
    BSE01001, // Async constructor not allowed
    BSE01002, // Constructor cannot have return type
    BSE01003, // Constructor cannot be virtual/abstract
    BSE01004, // Constructor cannot be static and instance
    BSE01005, // Constructor name must match class name
    BSE01006, // Multiple constructors with same signature
    BSE01007, // Base constructor call invalid
    BSE01008, // Circular constructor dependency
    BSE01009, // Constructor cannot override
    BSE01010, // Interface cannot have constructor

    // Method Semantic Errors (BSE02000-BSE02999)
    BSE02001, // Abstract method cannot have body
    BSE02002, // Non-abstract method must have body
    BSE02003, // Virtual method in sealed class
    BSE02004, // Override method has wrong signature
    BSE02005, // Method cannot be both virtual and static
    BSE02006, // Static method cannot override
    BSE02007, // Method not found for override
    BSE02008, // Interface method cannot have body
    BSE02009, // Async method must return Task or Task<T>
    BSE02010, // Method parameter name conflicts

    // Type System Errors (BSE03000-BSE03999)
    BSE03001, // Type not found
    BSE03002, // Type cannot be instantiated
    BSE03003, // Type argument mismatch
    BSE03004, // Constraints not satisfied
    BSE03005, // Circular type dependency
    BSE03006, // Interface cannot inherit from class
    BSE03007, // Class cannot inherit from interface
    BSE03008, // Struct cannot inherit
    BSE03009, // Type visibility inconsistency
    BSE03010, // Generic type parameter name conflict

    // Access Modifier Errors (BSE04000-BSE04999)
    BSE04001, // Member not accessible
    BSE04002, // Private member in interface
    BSE04003, // Protected member in struct
    BSE04004, // Inconsistent accessibility
    BSE04005, // Static constructor cannot have modifiers
    BSE04006, // Abstract member cannot be private
    BSE04007, // Virtual member cannot be private
    BSE04008, // Override member visibility mismatch
    BSE04009, // Sealed member not virtual
    BSE04010, // Abstract member in non-abstract class

    // Maintainability Warnings (BSW01000-BSW01999)
    BSW01001, // Method too complex (cyclomatic complexity)
    BSW01002, // Method too long (line count)
    BSW01003, // Too many parameters
    BSW01004, // Missing documentation
    BSW01005, // Deep nesting detected
    BSW01006, // Large class detected
    BSW01007, // High coupling detected
    BSW01008, // Low cohesion detected
    BSW01009, // God class anti-pattern
    BSW01010, // Feature envy detected

    // Style Warnings (BSW02000-BSW02999)
    BSW02001, // Naming convention violation
    BSW02002, // PascalCase expected
    BSW02003, // camelCase expected
    BSW02004, // UPPER_CASE expected
    BSW02005, // Unused variable
    BSW02006, // Unused parameter
    BSW02007, // Redundant assignment
    BSW02008, // Magic number detected
    BSW02009, // String literal duplication
    BSW02010, // Empty block statement

    // Performance Warnings (BSW03000-BSW03999)
    BSW03001, // Boxing/unboxing detected
    BSW03002, // String concatenation in loop
    BSW03003, // LINQ performance concern
    BSW03004, // Unnecessary allocation
    BSW03005, // Synchronous call in async method
    BSW03006, // Exception for control flow
    BSW03007, // Inefficient collection usage
    BSW03008, // Closure allocation in loop
    BSW03009, // Large object heap allocation
    BSW03010, // Database query in loop

    // Security Warnings (BSW04000-BSW04999)
    BSW04001, // SQL injection risk
    BSW04002, // XSS vulnerability risk
    BSW04003, // Hardcoded credential
    BSW04004, // Weak cryptography
    BSW04005, // Path traversal risk
    BSW04006, // Insecure random generation
    BSW04007, // Missing input validation
    BSW04008, // Sensitive data in log
    BSW04009, // Unsafe deserialization
    BSW04010, // Missing authentication check
}

impl DiagnosticCode {
    pub fn severity(&self) -> DiagnosticSeverity {
        match self {
            // All BSE codes are errors
            DiagnosticCode::BSE01001
            | DiagnosticCode::BSE01002
            | DiagnosticCode::BSE01003
            | DiagnosticCode::BSE01004
            | DiagnosticCode::BSE01005
            | DiagnosticCode::BSE01006
            | DiagnosticCode::BSE01007
            | DiagnosticCode::BSE01008
            | DiagnosticCode::BSE01009
            | DiagnosticCode::BSE01010
            | DiagnosticCode::BSE02001
            | DiagnosticCode::BSE02002
            | DiagnosticCode::BSE02003
            | DiagnosticCode::BSE02004
            | DiagnosticCode::BSE02005
            | DiagnosticCode::BSE02006
            | DiagnosticCode::BSE02007
            | DiagnosticCode::BSE02008
            | DiagnosticCode::BSE02009
            | DiagnosticCode::BSE02010
            | DiagnosticCode::BSE03001
            | DiagnosticCode::BSE03002
            | DiagnosticCode::BSE03003
            | DiagnosticCode::BSE03004
            | DiagnosticCode::BSE03005
            | DiagnosticCode::BSE03006
            | DiagnosticCode::BSE03007
            | DiagnosticCode::BSE03008
            | DiagnosticCode::BSE03009
            | DiagnosticCode::BSE03010
            | DiagnosticCode::BSE04001
            | DiagnosticCode::BSE04002
            | DiagnosticCode::BSE04003
            | DiagnosticCode::BSE04004
            | DiagnosticCode::BSE04005
            | DiagnosticCode::BSE04006
            | DiagnosticCode::BSE04007
            | DiagnosticCode::BSE04008
            | DiagnosticCode::BSE04009
            | DiagnosticCode::BSE04010 => DiagnosticSeverity::Error,

            // All BSW codes are warnings
            _ => DiagnosticSeverity::Warning,
        }
    }

    pub fn category(&self) -> DiagnosticCategory {
        match self {
            // Constructor and method errors
            DiagnosticCode::BSE01001
            | DiagnosticCode::BSE01002
            | DiagnosticCode::BSE01003
            | DiagnosticCode::BSE01004
            | DiagnosticCode::BSE01005
            | DiagnosticCode::BSE01006
            | DiagnosticCode::BSE01007
            | DiagnosticCode::BSE01008
            | DiagnosticCode::BSE01009
            | DiagnosticCode::BSE01010
            | DiagnosticCode::BSE02001
            | DiagnosticCode::BSE02002
            | DiagnosticCode::BSE02003
            | DiagnosticCode::BSE02004
            | DiagnosticCode::BSE02005
            | DiagnosticCode::BSE02006
            | DiagnosticCode::BSE02007
            | DiagnosticCode::BSE02008
            | DiagnosticCode::BSE02009
            | DiagnosticCode::BSE02010 => DiagnosticCategory::Semantic,

            // Type errors
            DiagnosticCode::BSE03001
            | DiagnosticCode::BSE03002
            | DiagnosticCode::BSE03003
            | DiagnosticCode::BSE03004
            | DiagnosticCode::BSE03005
            | DiagnosticCode::BSE03006
            | DiagnosticCode::BSE03007
            | DiagnosticCode::BSE03008
            | DiagnosticCode::BSE03009
            | DiagnosticCode::BSE03010 => DiagnosticCategory::Type,

            // Access modifier errors
            DiagnosticCode::BSE04001
            | DiagnosticCode::BSE04002
            | DiagnosticCode::BSE04003
            | DiagnosticCode::BSE04004
            | DiagnosticCode::BSE04005
            | DiagnosticCode::BSE04006
            | DiagnosticCode::BSE04007
            | DiagnosticCode::BSE04008
            | DiagnosticCode::BSE04009
            | DiagnosticCode::BSE04010 => DiagnosticCategory::Semantic,

            // Maintainability warnings
            DiagnosticCode::BSW01001
            | DiagnosticCode::BSW01002
            | DiagnosticCode::BSW01003
            | DiagnosticCode::BSW01004
            | DiagnosticCode::BSW01005
            | DiagnosticCode::BSW01006
            | DiagnosticCode::BSW01007
            | DiagnosticCode::BSW01008
            | DiagnosticCode::BSW01009
            | DiagnosticCode::BSW01010 => DiagnosticCategory::Maintainability,

            // Style warnings
            DiagnosticCode::BSW02001
            | DiagnosticCode::BSW02002
            | DiagnosticCode::BSW02003
            | DiagnosticCode::BSW02004
            | DiagnosticCode::BSW02005
            | DiagnosticCode::BSW02006
            | DiagnosticCode::BSW02007
            | DiagnosticCode::BSW02008
            | DiagnosticCode::BSW02009
            | DiagnosticCode::BSW02010 => DiagnosticCategory::Style,

            // Performance warnings
            DiagnosticCode::BSW03001
            | DiagnosticCode::BSW03002
            | DiagnosticCode::BSW03003
            | DiagnosticCode::BSW03004
            | DiagnosticCode::BSW03005
            | DiagnosticCode::BSW03006
            | DiagnosticCode::BSW03007
            | DiagnosticCode::BSW03008
            | DiagnosticCode::BSW03009
            | DiagnosticCode::BSW03010 => DiagnosticCategory::Performance,

            // Security warnings
            DiagnosticCode::BSW04001
            | DiagnosticCode::BSW04002
            | DiagnosticCode::BSW04003
            | DiagnosticCode::BSW04004
            | DiagnosticCode::BSW04005
            | DiagnosticCode::BSW04006
            | DiagnosticCode::BSW04007
            | DiagnosticCode::BSW04008
            | DiagnosticCode::BSW04009
            | DiagnosticCode::BSW04010 => DiagnosticCategory::Security,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            // Constructor errors
            DiagnosticCode::BSE01001 => "BSE01001",
            DiagnosticCode::BSE01002 => "BSE01002",
            DiagnosticCode::BSE01003 => "BSE01003",
            DiagnosticCode::BSE01004 => "BSE01004",
            DiagnosticCode::BSE01005 => "BSE01005",
            DiagnosticCode::BSE01006 => "BSE01006",
            DiagnosticCode::BSE01007 => "BSE01007",
            DiagnosticCode::BSE01008 => "BSE01008",
            DiagnosticCode::BSE01009 => "BSE01009",
            DiagnosticCode::BSE01010 => "BSE01010",

            // Method errors
            DiagnosticCode::BSE02001 => "BSE02001",
            DiagnosticCode::BSE02002 => "BSE02002",
            DiagnosticCode::BSE02003 => "BSE02003",
            DiagnosticCode::BSE02004 => "BSE02004",
            DiagnosticCode::BSE02005 => "BSE02005",
            DiagnosticCode::BSE02006 => "BSE02006",
            DiagnosticCode::BSE02007 => "BSE02007",
            DiagnosticCode::BSE02008 => "BSE02008",
            DiagnosticCode::BSE02009 => "BSE02009",
            DiagnosticCode::BSE02010 => "BSE02010",

            // Type errors
            DiagnosticCode::BSE03001 => "BSE03001",
            DiagnosticCode::BSE03002 => "BSE03002",
            DiagnosticCode::BSE03003 => "BSE03003",
            DiagnosticCode::BSE03004 => "BSE03004",
            DiagnosticCode::BSE03005 => "BSE03005",
            DiagnosticCode::BSE03006 => "BSE03006",
            DiagnosticCode::BSE03007 => "BSE03007",
            DiagnosticCode::BSE03008 => "BSE03008",
            DiagnosticCode::BSE03009 => "BSE03009",
            DiagnosticCode::BSE03010 => "BSE03010",

            // Access modifier errors
            DiagnosticCode::BSE04001 => "BSE04001",
            DiagnosticCode::BSE04002 => "BSE04002",
            DiagnosticCode::BSE04003 => "BSE04003",
            DiagnosticCode::BSE04004 => "BSE04004",
            DiagnosticCode::BSE04005 => "BSE04005",
            DiagnosticCode::BSE04006 => "BSE04006",
            DiagnosticCode::BSE04007 => "BSE04007",
            DiagnosticCode::BSE04008 => "BSE04008",
            DiagnosticCode::BSE04009 => "BSE04009",
            DiagnosticCode::BSE04010 => "BSE04010",

            // Maintainability warnings
            DiagnosticCode::BSW01001 => "BSW01001",
            DiagnosticCode::BSW01002 => "BSW01002",
            DiagnosticCode::BSW01003 => "BSW01003",
            DiagnosticCode::BSW01004 => "BSW01004",
            DiagnosticCode::BSW01005 => "BSW01005",
            DiagnosticCode::BSW01006 => "BSW01006",
            DiagnosticCode::BSW01007 => "BSW01007",
            DiagnosticCode::BSW01008 => "BSW01008",
            DiagnosticCode::BSW01009 => "BSW01009",
            DiagnosticCode::BSW01010 => "BSW01010",

            // Style warnings
            DiagnosticCode::BSW02001 => "BSW02001",
            DiagnosticCode::BSW02002 => "BSW02002",
            DiagnosticCode::BSW02003 => "BSW02003",
            DiagnosticCode::BSW02004 => "BSW02004",
            DiagnosticCode::BSW02005 => "BSW02005",
            DiagnosticCode::BSW02006 => "BSW02006",
            DiagnosticCode::BSW02007 => "BSW02007",
            DiagnosticCode::BSW02008 => "BSW02008",
            DiagnosticCode::BSW02009 => "BSW02009",
            DiagnosticCode::BSW02010 => "BSW02010",

            // Performance warnings
            DiagnosticCode::BSW03001 => "BSW03001",
            DiagnosticCode::BSW03002 => "BSW03002",
            DiagnosticCode::BSW03003 => "BSW03003",
            DiagnosticCode::BSW03004 => "BSW03004",
            DiagnosticCode::BSW03005 => "BSW03005",
            DiagnosticCode::BSW03006 => "BSW03006",
            DiagnosticCode::BSW03007 => "BSW03007",
            DiagnosticCode::BSW03008 => "BSW03008",
            DiagnosticCode::BSW03009 => "BSW03009",
            DiagnosticCode::BSW03010 => "BSW03010",

            // Security warnings
            DiagnosticCode::BSW04001 => "BSW04001",
            DiagnosticCode::BSW04002 => "BSW04002",
            DiagnosticCode::BSW04003 => "BSW04003",
            DiagnosticCode::BSW04004 => "BSW04004",
            DiagnosticCode::BSW04005 => "BSW04005",
            DiagnosticCode::BSW04006 => "BSW04006",
            DiagnosticCode::BSW04007 => "BSW04007",
            DiagnosticCode::BSW04008 => "BSW04008",
            DiagnosticCode::BSW04009 => "BSW04009",
            DiagnosticCode::BSW04010 => "BSW04010",
        }
    }

    pub fn default_message(&self) -> &'static str {
        match self {
            // Constructor errors
            DiagnosticCode::BSE01001 => "Constructors cannot be declared async",
            DiagnosticCode::BSE01002 => "Constructors cannot have an explicit return type",
            DiagnosticCode::BSE01003 => "Constructors cannot be virtual or abstract",
            DiagnosticCode::BSE01004 => "Constructor cannot be both static and instance",
            DiagnosticCode::BSE01005 => "Constructor name must match the containing class name",
            DiagnosticCode::BSE01006 => "Multiple constructors with the same signature",
            DiagnosticCode::BSE01007 => "Invalid base constructor call",
            DiagnosticCode::BSE01008 => "Circular constructor dependency detected",
            DiagnosticCode::BSE01009 => "Constructors cannot override other constructors",
            DiagnosticCode::BSE01010 => "Interfaces cannot contain constructors",

            // Method errors
            DiagnosticCode::BSE02001 => "Abstract methods cannot have a body",
            DiagnosticCode::BSE02002 => "Non-abstract methods must have a body",
            DiagnosticCode::BSE02003 => "Virtual methods cannot be declared in sealed classes",
            DiagnosticCode::BSE02004 => "Override method signature does not match base method",
            DiagnosticCode::BSE02005 => "Methods cannot be both virtual and static",
            DiagnosticCode::BSE02006 => "Static methods cannot override other methods",
            DiagnosticCode::BSE02007 => "No suitable method found to override",
            DiagnosticCode::BSE02008 => "Interface methods cannot have a body",
            DiagnosticCode::BSE02009 => "Async methods must return Task or Task<T>",
            DiagnosticCode::BSE02010 => "Method parameter names must be unique",

            // Type errors
            DiagnosticCode::BSE03001 => "Type could not be found",
            DiagnosticCode::BSE03002 => "Cannot create an instance of this type",
            DiagnosticCode::BSE03003 => "Type argument does not match constraint",
            DiagnosticCode::BSE03004 => "Generic constraints are not satisfied",
            DiagnosticCode::BSE03005 => "Circular type dependency detected",
            DiagnosticCode::BSE03006 => "Interfaces cannot inherit from classes",
            DiagnosticCode::BSE03007 => "Classes cannot inherit from interfaces (use : instead)",
            DiagnosticCode::BSE03008 => "Structs cannot inherit from other types",
            DiagnosticCode::BSE03009 => "Inconsistent type visibility",
            DiagnosticCode::BSE03010 => "Generic type parameter names must be unique",

            // Access modifier errors
            DiagnosticCode::BSE04001 => "Member is not accessible in this context",
            DiagnosticCode::BSE04002 => "Interface members cannot be private",
            DiagnosticCode::BSE04003 => "Struct members cannot be protected",
            DiagnosticCode::BSE04004 => "Inconsistent accessibility between types",
            DiagnosticCode::BSE04005 => "Static constructors cannot have access modifiers",
            DiagnosticCode::BSE04006 => "Abstract members cannot be private",
            DiagnosticCode::BSE04007 => "Virtual members cannot be private",
            DiagnosticCode::BSE04008 => "Override member visibility does not match base",
            DiagnosticCode::BSE04009 => "Sealed modifier can only be used on overriding members",
            DiagnosticCode::BSE04010 => "Abstract members cannot exist in non-abstract classes",

            // Maintainability warnings
            DiagnosticCode::BSW01001 => "Method has high cyclomatic complexity",
            DiagnosticCode::BSW01002 => "Method is too long",
            DiagnosticCode::BSW01003 => "Method has too many parameters",
            DiagnosticCode::BSW01004 => "Missing XML documentation",
            DiagnosticCode::BSW01005 => "Deep nesting detected",
            DiagnosticCode::BSW01006 => "Class is too large",
            DiagnosticCode::BSW01007 => "High coupling detected",
            DiagnosticCode::BSW01008 => "Low cohesion detected",
            DiagnosticCode::BSW01009 => "God class anti-pattern detected",
            DiagnosticCode::BSW01010 => "Feature envy anti-pattern detected",

            // Style warnings
            DiagnosticCode::BSW02001 => "Naming convention violation",
            DiagnosticCode::BSW02002 => "PascalCase naming expected",
            DiagnosticCode::BSW02003 => "camelCase naming expected",
            DiagnosticCode::BSW02004 => "UPPER_CASE naming expected",
            DiagnosticCode::BSW02005 => "Variable is declared but never used",
            DiagnosticCode::BSW02006 => "Parameter is declared but never used",
            DiagnosticCode::BSW02007 => "Redundant assignment detected",
            DiagnosticCode::BSW02008 => "Magic number should be a named constant",
            DiagnosticCode::BSW02009 => "String literal duplication",
            DiagnosticCode::BSW02010 => "Empty block statement",

            // Performance warnings
            DiagnosticCode::BSW03001 => "Boxing/unboxing operation detected",
            DiagnosticCode::BSW03002 => "String concatenation in loop",
            DiagnosticCode::BSW03003 => "LINQ performance concern",
            DiagnosticCode::BSW03004 => "Unnecessary object allocation",
            DiagnosticCode::BSW03005 => "Synchronous call in async method",
            DiagnosticCode::BSW03006 => "Using exceptions for control flow",
            DiagnosticCode::BSW03007 => "Inefficient collection usage",
            DiagnosticCode::BSW03008 => "Closure allocation in loop",
            DiagnosticCode::BSW03009 => "Large object heap allocation",
            DiagnosticCode::BSW03010 => "Database query in loop",

            // Security warnings
            DiagnosticCode::BSW04001 => "Potential SQL injection vulnerability",
            DiagnosticCode::BSW04002 => "Potential XSS vulnerability",
            DiagnosticCode::BSW04003 => "Hardcoded credential detected",
            DiagnosticCode::BSW04004 => "Weak cryptographic algorithm",
            DiagnosticCode::BSW04005 => "Potential path traversal vulnerability",
            DiagnosticCode::BSW04006 => "Insecure random number generation",
            DiagnosticCode::BSW04007 => "Missing input validation",
            DiagnosticCode::BSW04008 => "Sensitive data in log statement",
            DiagnosticCode::BSW04009 => "Unsafe deserialization",
            DiagnosticCode::BSW04010 => "Missing authentication check",
        }
    }
}
