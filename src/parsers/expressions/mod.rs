pub mod literal_parser;
pub mod expression_parser;
pub mod primary_parser; // Added for primary expressions
pub mod lambda_expression_parser; // Added for lambda expressions
pub mod await_expression_parser; // Added for await expressions

pub use expression_parser::parse_expression;
// Re-export key parser functions
pub use literal_parser::parse_literal;
pub use primary_parser::parse_primary_expression;
pub use await_expression_parser::parse_await_expression;
// Added for primary expressions
pub use lambda_expression_parser::{parse_anonymous_method_expression, parse_lambda_expression, parse_lambda_or_anonymous_method};
