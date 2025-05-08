use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::statements::statement::Statement;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CatchClause<'a> {
    // Optional: Specific exception type to catch (e.g., System.Exception)
    // If None, it's a general catch block (catch { ... })
    pub exception_type: Option<Type<'a>>,
    // Optional: Variable name for the caught exception (e.g., ex)
    pub exception_variable: Option<Identifier>,
    // The block of statements to execute when the exception is caught
    pub block: Box<Statement<'a>>, // Must be Statement::Block
}
