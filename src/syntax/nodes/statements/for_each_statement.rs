use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::statements::statement::Statement;
use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForEachStatement {
    // Whether this is an 'await foreach'
    pub is_await: bool,
    // TypeSyntax of the iteration variable (can be 'var')
    pub var_type: Type, // Or maybe a special 'Var' type?
    // Name of the iteration variable
    pub var_name: Identifier,
    // The collection expression being iterated over
    pub collection: Box<Expression>,
    // Loop body
    pub body: Box<Statement>, // Expecting Statement::Block usually
}
