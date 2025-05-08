use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForEachStatement<'a> {
    // TypeSyntax of the iteration variable (can be 'var')
    pub var_type: Type<'a>, // Or maybe a special 'Var' type?
    // Name of the iteration variable
    pub var_name: Identifier,
    // The collection expression being iterated over
    pub collection: Box<Expression<'a>>,
    // Loop body
    pub body: Box<Statement<'a>>, // Expecting Statement::Block usually
}
