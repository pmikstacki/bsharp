use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DoWhileStatement {
    // Loop body (executed at least once)
    pub body: Box<Statement>, // Expecting Statement::Block usually
    // Loop condition (checked after the body executes)
    pub condition: Expression,
}
