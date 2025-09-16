use super::BinaryOperator;
use crate::syntax::nodes::expressions::expression::Expression;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssignmentExpression {
    // Left-hand side (e.g., Variable, MemberAccess)
    // Using Expression for now, might need refinement later (e.g., LValue trait)
    pub target: Box<Expression>,
    // Operator
    pub op: BinaryOperator,
    // Right-hand side
    pub value: Box<Expression>,
}
