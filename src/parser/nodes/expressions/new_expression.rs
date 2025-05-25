use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewExpression {
    pub ty: Type,
    pub arguments: Vec<Expression>,
    pub object_initializer: Option<Vec<(String, Expression)>>,
    pub collection_initializer: Option<Vec<Expression>>,
}
