pub mod block_statement_parser;
pub mod break_statement_parser;
pub mod continue_statement_parser;
pub mod deconstruction_statement_parser;
pub mod do_while_statement_parser;
pub mod empty_statement_parser;
pub mod expression_statement_parser;
pub mod for_statement_parser;
pub mod foreach_statement_parser;
pub mod if_statement_parser;
pub mod local_function_statement_parser;
pub mod return_statement_parser;
pub mod switch_statement_parser;
pub mod throw_statement_parser;
pub mod top_level_statement_parser;
pub mod try_catch_finally_parser;
pub mod using_statement_parser;
pub mod while_statement_parser;

// New statement parser
pub mod checked_statement_parser;
pub mod fixed_statement_parser;
pub mod goto_case_statement_parser;
pub mod goto_statement_parser;
pub mod label_statement_parser;
pub mod lock_statement_parser;
pub mod unsafe_statement_parser;
pub mod yield_statement_parser;

pub use block_statement_parser::parse_block_statement;
pub use break_statement_parser::parse_break_statement;
pub use continue_statement_parser::parse_continue_statement;
pub use do_while_statement_parser::parse_do_while_statement;
pub use empty_statement_parser::parse_empty_statement;
pub use expression_statement_parser::parse_expression_statement;
pub use for_statement_parser::parse_for_statement;
pub use foreach_statement_parser::parse_foreach_statement;
pub use if_statement_parser::parse_if_statement;
pub use local_function_statement_parser::parse_local_function_statement;
pub use return_statement_parser::parse_return_statement;
pub use switch_statement_parser::parse_switch_statement;
pub use throw_statement_parser::parse_throw_statement;
pub use top_level_statement_parser::parse_top_level_statements;
pub use try_catch_finally_parser::parse_try_statement;
pub use using_statement_parser::parse_using_statement;
pub use while_statement_parser::parse_while_statement;

// New statement syntax exports
pub use checked_statement_parser::{
    parse_checked_statement, parse_checked_unchecked_statement, parse_unchecked_statement,
};
pub use fixed_statement_parser::parse_fixed_statement;
pub use goto_case_statement_parser::parse_goto_case_statement;
pub use goto_statement_parser::parse_goto_statement;
pub use label_statement_parser::parse_label_statement;
pub use lock_statement_parser::parse_lock_statement;
pub use unsafe_statement_parser::parse_unsafe_statement;
pub use yield_statement_parser::parse_yield_statement;
// Re-export key node types if needed for convenience, though often handled in src/syntax/nodes/mod.rs

pub use syntax::declarations::*;
pub use syntax::expressions::*;
pub use syntax::trivia::*;
