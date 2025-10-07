use crate::expressions::Expression;
use crate::types::Type;
use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    // Basic patterns
    Declaration {
        target_type: Type,
        name: Identifier,
    },
    Constant(Expression),
    Var(Identifier),
    Discard, // _

    // Type patterns
    Type {
        target_type: Type,
        designation: Option<PatternDesignation>,
    },

    // Property patterns
    Property {
        type_name: Option<Type>,
        subpatterns: Vec<PropertySubpattern>,
    },

    // Positional patterns (for tuples, records, deconstructable types)
    Positional {
        type_name: Option<Type>,
        subpatterns: Vec<Pattern>,
    },

    // Tuple patterns
    Tuple(Vec<Pattern>),

    // List patterns (C# 11+)
    List {
        patterns: Vec<ListPatternElement>,
    },

    // Slice pattern (..)
    Slice {
        pattern: Option<Box<Pattern>>, // Optional pattern for capturing the slice
    },

    // Relational patterns (> 5, <= 10, etc.)
    Relational {
        op: RelationalOperator,
        value: Expression,
    },

    // Logical patterns
    LogicalAnd(Box<Pattern>, Box<Pattern>), // pattern1 and pattern2
    LogicalOr(Box<Pattern>, Box<Pattern>),  // pattern1 or pattern2
    Not(Box<Pattern>),                      // not pattern

    // Parenthesized pattern
    Parenthesized(Box<Pattern>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PatternDesignation {
    Variable(Identifier),
    Discard, // _
    Parenthesized(Box<PatternDesignation>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PropertySubpattern {
    pub member_name: Identifier,
    pub pattern: Pattern,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ListPatternElement {
    Pattern(Pattern),
    Slice(Option<Pattern>), // .. or ..pattern
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum RelationalOperator {
    LessThan,           // <
    LessThanOrEqual,    // <=
    GreaterThan,        // >
    GreaterThanOrEqual, // >=
    Equal,              // == (rarely used in patterns but possible)
    NotEqual,           // != (rarely used in patterns but possible)
}

// For switch statements and expressions
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PatternCase {
    pub pattern: Pattern,
    pub when_clause: Option<Expression>,
    pub body: Vec<Expression>, // For switch statements this would be statements
}

// Switch expression specific
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwitchExpressionArm {
    pub pattern: Pattern,
    pub when_clause: Option<Expression>,
    pub expression: Expression,
}
