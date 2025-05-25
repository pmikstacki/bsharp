use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryExpression {
    pub from: FromClause,
    pub body: Vec<QueryClause>,
    pub select_or_group: QuerySelectOrGroup,
    pub continuation: Option<QueryContinuation>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FromClause {
    pub type_annotation: Option<Identifier>, 
    pub identifier: Identifier,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum QueryClause {
    From(FromClause),
    Let(LetClause),
    Where(QueryWhereClause),
    Join(JoinClause),
    OrderBy(QueryOrderByClause),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LetClause {
    pub identifier: Identifier,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryWhereClause {
    pub condition: Expression,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct JoinClause {
    pub type_annotation: Option<Identifier>,
    pub identifier: Identifier,
    pub in_expression: Expression,
    pub on_expression: Expression,
    pub equals_expression: Expression,
    pub into_identifier: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryOrderByClause {
    pub orderings: Vec<OrderByOrdering>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderByOrdering {
    pub expression: Expression,
    pub direction: Option<OrderingDirection>,
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderingDirection {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum QuerySelectOrGroup {
    Select(Expression),
    Group { element: Expression, by: Expression },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryContinuation {
    pub identifier: Identifier,
    pub body: Vec<QueryClause>,
    pub select_or_group: QuerySelectOrGroup,
}
