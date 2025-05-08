use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    Integer(i64),
    Float(f64), // Added Float variant
    Boolean(bool),
    String(String),
    Char(char), // Added char literal
}
