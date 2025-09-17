mod assignment_expression_parser;
pub mod await_expression_parser;
mod conditional_expression_parser;
pub mod declarations;
pub mod deconstruction_expression_parser;
pub mod default_expression_parser;
pub mod lambda_expression_parser;
pub mod literal_parser;
mod logical_expression_parser;
pub mod nameof_expression_parser;
pub mod new_expression_parser;
mod null_coalescing_expression_parser;
pub mod paren_tuple_primary_parser;
pub mod pattern_parser;
pub mod postfix_expression_parser;
mod precedence;
pub mod primary_expression_parser;
pub mod query_expression_parser;
pub mod range_expression_parser;
pub mod ref_expression_parser;
pub mod sizeof_expression_parser;
pub mod stackalloc_expression_parser;
pub mod statements;
pub mod switch_expression_parser;
pub mod throw_expression_parser;
pub mod tuple_expression_parser;
pub mod typeof_expression_parser;
pub mod unary_expression_parser;

// Re-export key syntax functions
pub use await_expression_parser::parse_await_expression;
// Added for primary expressions
pub use default_expression_parser::parse_default_expression;
pub use lambda_expression_parser::{
    parse_anonymous_method_expression, parse_lambda_expression, parse_lambda_or_anonymous_method,
};
pub use literal_parser::parse_literal;
pub use nameof_expression_parser::parse_nameof_expression;
pub use primary_expression_parser::parse_expression;
pub use primary_expression_parser::parse_primary_expression;
pub use sizeof_expression_parser::parse_sizeof_expression;
pub use throw_expression_parser::parse_throw_expression;
pub use typeof_expression_parser::parse_typeof_expression;
