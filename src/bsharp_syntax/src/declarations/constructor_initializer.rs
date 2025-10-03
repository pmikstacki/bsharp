use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConstructorInitializer {
    Base(Vec<Expression>),
    This(Vec<Expression>),
}
