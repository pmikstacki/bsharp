pub mod expression_parser;
pub mod literal_parser;
pub mod primary_parser;
pub mod lambda_expression_parser;
pub mod await_expression_parser;
pub mod query_expression_parser;
pub mod pattern_parser;
pub mod switch_expression_parser;

pub use expression_parser::parse_expression;
// Re-export key parser functions
pub use literal_parser::parse_literal;
pub use primary_parser::parse_primary_expression;
pub use await_expression_parser::parse_await_expression;
// Added for primary expressions
pub use lambda_expression_parser::{parse_anonymous_method_expression, parse_lambda_expression, parse_lambda_or_anonymous_method};
