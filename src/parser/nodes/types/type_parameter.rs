use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Variance {
    None, // Default
    In,   // contravariant
    Out,  // covariant
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeParameter {
    pub name: Identifier,
    pub variance: Variance,
    // pub constraints: Vec<Constraint>, // Placeholder for constraints
}
