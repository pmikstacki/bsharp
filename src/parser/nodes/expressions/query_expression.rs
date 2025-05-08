use serde::{Serialize, Deserialize};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryExpression<'a> {
    pub from: FromClause<'a>,
    pub body: Vec<QueryClause<'a>>,
    pub select_or_group: QuerySelectOrGroup<'a>,
    pub continuation: Option<QueryContinuation<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FromClause<'a> {
    pub type_annotation: Option<Identifier>, 
    pub identifier: Identifier,
    pub expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum QueryClause<'a> {
    From(FromClause<'a>),
    Let(LetClause<'a>),
    Where(QueryWhereClause<'a>),
    Join(JoinClause<'a>),
    OrderBy(QueryOrderByClause<'a>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LetClause<'a> {
    pub identifier: Identifier,
    pub expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryWhereClause<'a> {
    pub condition: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct JoinClause<'a> {
    pub type_annotation: Option<Identifier>,
    pub identifier: Identifier,
    pub in_expression: Expression<'a>,
    pub on_expression: Expression<'a>,
    pub equals_expression: Expression<'a>,
    pub into_identifier: Option<Identifier>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryOrderByClause<'a> {
    pub orderings: Vec<OrderByOrdering<'a>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderByOrdering<'a> {
    pub expression: Expression<'a>,
    pub direction: Option<OrderingDirection>,
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderingDirection {
    Ascending,
    Descending,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum QuerySelectOrGroup<'a> {
    Select(Expression<'a>),
    Group { element: Expression<'a>, by: Expression<'a> },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryContinuation<'a> {
    pub identifier: Identifier,
    pub body: Vec<QueryClause<'a>>,
    pub select_or_group: QuerySelectOrGroup<'a>,
}
