use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewExpression<'a> {
    pub ty: Type<'a>,
    pub arguments: Vec<Expression<'a>>,
    pub object_initializer: Option<Vec<(String, Expression<'a>)>>,
    pub collection_initializer: Option<Vec<Expression<'a>>>,
}
