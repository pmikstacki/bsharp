use crate::expressions::expression::Expression;
use crate::identifier::Identifier;
use crate::statements::statement::Statement;
use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CatchClause {
    // Optional: Specific exception type to catch (e.g., System.Exception)
    // If None, it's a general catch block (catch { ... })
    pub exception_type: Option<Type>,
    // Optional: Variable name for the caught exception (e.g., ex)
    pub exception_variable: Option<Identifier>,
    // The block of statements to execute when the exception is caught
    pub block: Box<Statement>, // Must be Statement::Block
    // Optional: catch filter (C# 'when (expr)')
    pub when_clause: Option<Expression>,
}
