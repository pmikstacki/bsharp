use crate::expressions::Expression;
use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ObjectInitializerEntry {
    Property {
        name: String,
        value: Expression,
    },
    Indexer {
        indices: Vec<Expression>,
        value: Expression,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    pub target_type: Option<Type>,
    pub arguments: Vec<Expression>,
    pub object_initializer: Option<Vec<ObjectInitializerEntry>>,
    pub collection_initializer: Option<Vec<Expression>>,
}
