use super::{CatchClause, FinallyClause};
use crate::parser::nodes::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TryStatement {
    // The block of statements to try
    pub try_block: Box<Statement>, // Must be Statement::Block
    pub catches: Vec<CatchClause>,
    pub finally_clause: Option<FinallyClause>,
}
