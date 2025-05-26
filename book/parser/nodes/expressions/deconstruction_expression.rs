use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::Type;
use serde::{Deserialize, Serialize};

/// Represents a deconstruction expression in C# 
/// Examples: (var x, var y) = tuple; (int a, string b) = GetTuple(); var (x, y) = tuple;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeconstructionExpression {
    /// The variables being assigned to
    pub targets: Vec<DeconstructionTarget>,
    /// The expression being deconstructed
    pub value: Box<Expression>,
}

/// Represents a single target in a deconstruction
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DeconstructionTarget {
    /// Variable declaration: var x, int y, string z
    Declaration {
        /// Type of the variable (can be inferred with 'var')
        variable_type: Option<Type>,
        /// Name of the variable
        name: Identifier,
        /// Whether this uses 'var' for type inference
        is_var: bool,
    },
    /// Existing variable: existingVar
    Variable(Identifier),
    /// Discard: _
    Discard,
    /// Nested deconstruction: (var a, var b)
    Nested(Vec<DeconstructionTarget>),
}
