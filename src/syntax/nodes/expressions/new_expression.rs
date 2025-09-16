use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    pub ty: Type,
    pub arguments: Vec<Expression>,
    pub object_initializer: Option<Vec<(String, Expression)>>,
    pub collection_initializer: Option<Vec<Expression>>,
}
