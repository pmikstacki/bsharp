use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use super::BinaryOperator; 

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
