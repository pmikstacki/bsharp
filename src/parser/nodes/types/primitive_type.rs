use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PrimitiveType {
    Void,
    Int,
    Bool,
    String, 
}
