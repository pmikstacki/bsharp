use crate::syntax::nodes::declarations::LocalVariableDeclaration;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForStatement {
    // Initializer can be a declaration or a list of expression statements
    pub initializer: Option<ForInitializer>,
    // Loop condition
    pub condition: Option<Expression>,
    // Iterator statements (executed after each loop iteration)
    pub iterator: Vec<Expression>,
    // Loop body
    pub body: Box<Statement>, // Expecting Statement::Block usually
}

// Define what can be in the initializer part of a for loop
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ForInitializer {
    Declaration(LocalVariableDeclaration),
    Expressions(Vec<Expression>),
}
