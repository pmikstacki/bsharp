use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::declarations::LocalVariableDeclaration;
use crate::parser::nodes::statements::statement::Statement;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForStatement<'a> {
    // Initializer can be a declaration or a list of expression statements
    pub initializer: Option<ForInitializer<'a>>,
    // Loop condition
    pub condition: Option<Expression<'a>>,
    // Iterator statements (executed after each loop iteration)
    pub iterator: Vec<Expression<'a>>,
    // Loop body
    pub body: Box<Statement<'a>>, // Expecting Statement::Block usually
}

// Define what can be in the initializer part of a for loop
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ForInitializer<'a> {
    Declaration(LocalVariableDeclaration<'a>),
    Expressions(Vec<Expression<'a>>),
}
