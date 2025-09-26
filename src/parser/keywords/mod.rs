//! Keyword parsers, split by C# categories.
//!
//! Each module defines small functions per keyword, e.g. `kw_public()` and `peek_public()`.
//! These are thin wrappers around `syntax::parser_helpers::{keyword, peek_keyword}` that enforce
//! word boundaries and integrate with our error/context handling.
//!
//! Integration guidance:
//! - Replace direct calls to `keyword("...")` with the appropriate `kw_*()` function.
//! - Replace direct calls to `peek_keyword("...")` with the appropriate `peek_*()` function.
//! - Keep `bws(...)` usage at call sites to control whitespace/comment consumption.
//!
//! Note: We intentionally do NOT glob re-export all functions to avoid name collisions for
//! tokens that appear in multiple grammar contexts (e.g., `default`, `new`, `using`, `this`).
//! Import modules explicitly at usage sites, e.g.:
//! `use crate::parser::keywords::access_keywords::kw_public;`

// Local helper macro to define a keyword parser pair without boilerplate.
// Usage: define_keyword_pair!(kw_public, peek_public, "public");
#[macro_export]
macro_rules! define_keyword_pair {
    ($kw_fn:ident, $peek_fn:ident, $lit:literal) => {
        pub fn $kw_fn() -> impl Fn(&str) -> $crate::syntax::errors::BResult<&str, &str> {
            $crate::syntax::parser_helpers::keyword($lit)
        }
        pub fn $peek_fn() -> impl Fn(&str) -> $crate::syntax::errors::BResult<&str, &str> {
            $crate::syntax::parser_helpers::peek_keyword($lit)
        }
    };
}

pub mod access_keywords;
pub mod accessor_keywords;
pub mod declaration_keywords;
pub mod exception_and_safety_keywords;
pub mod expression_keywords;
pub mod flow_control_keywords;
pub mod iteration_keywords;
pub mod linq_query_keywords;
pub mod modifier_keywords;
pub mod parameter_modifier_keywords;
pub mod selection_and_switch_keywords;
pub mod type_keywords;
pub mod contextual_misc_keywords;
pub mod literal_keywords;
pub mod pattern_keywords;
