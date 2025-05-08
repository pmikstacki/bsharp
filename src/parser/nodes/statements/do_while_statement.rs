use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DoWhileStatement<'a> {
    // Loop body (executed at least once)
    pub body: Box<Statement<'a>>, // Expecting Statement::Block usually
    // Loop condition (checked after the body executes)
    pub condition: Expression<'a>,
}
