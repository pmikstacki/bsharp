use crate::syntax::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConstructorInitializer {
    Base(Vec<Expression>),
    This(Vec<Expression>),
}
