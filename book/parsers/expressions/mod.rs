pub mod expression_parser;
pub mod literal_parser;
pub mod primary_parser;
pub mod lambda_expression_parser;
pub mod await_expression_parser;
pub mod query_expression_parser;
pub mod pattern_parser;
pub mod switch_expression_parser;
pub mod tuple_expression_parser;
pub mod throw_expression_parser;
pub mod nameof_expression_parser;
pub mod typeof_expression_parser;
pub mod sizeof_expression_parser;
pub mod default_expression_parser;
pub mod stackalloc_expression_parser;
pub mod ref_expression_parser;
pub mod deconstruction_expression_parser;

pub use expression_parser::parse_expression;
// Re-export key parser functions
pub use literal_parser::parse_literal;
pub use primary_parser::parse_primary_expression;
pub use await_expression_parser::parse_await_expression;
// Added for primary expressions
pub use lambda_expression_parser::{parse_anonymous_method_expression, parse_lambda_expression, parse_lambda_or_anonymous_method};
pub use throw_expression_parser::parse_throw_expression;
pub use nameof_expression_parser::parse_nameof_expression;
pub use typeof_expression_parser::parse_typeof_expression;
pub use sizeof_expression_parser::parse_sizeof_expression;
pub use default_expression_parser::parse_default_expression;
