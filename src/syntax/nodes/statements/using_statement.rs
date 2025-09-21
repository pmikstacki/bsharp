use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::declarations::LocalVariableDeclaration;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UsingStatement {
    pub is_await: bool,
    // One of these will be populated depending on the form
    pub resource: Option<Expression>,
    pub declaration: Option<LocalVariableDeclaration>,
    // Body is present for using-statement form, absent for using-declaration
    pub body: Option<Box<Statement>>,
}
