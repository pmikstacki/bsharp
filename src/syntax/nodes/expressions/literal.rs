use crate::syntax::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64), // Added Float variant
    Boolean(bool),
    String(String),
    Char(char),                                    // Added char literal
    Null,                                          // null literal
    InterpolatedString(InterpolatedStringLiteral), // String interpolation: $"Hello {name}"
    VerbatimString(String),                        // Verbatim string: @"path\to\file"
    RawString(String),                             // Raw string literal: """text"""
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterpolatedStringLiteral {
    pub parts: Vec<InterpolatedStringPart>,
    pub is_verbatim: bool, // true for $@"..." or @$"..."
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterpolatedStringPart {
    Text(String),
    Interpolation {
        expression: Expression,
        alignment: Option<Expression>, // For format specifiers like {value,10}
        format_string: Option<String>, // For format specifiers like {value:F2}
    },
}
