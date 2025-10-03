//! Contextual miscellaneous keywords: this, base, global, var, dynamic
//!
//! Provide small parser functions per keyword where context decides meaning.

use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{keyword, peek_keyword};

pub fn kw_this() -> impl Fn(&str) -> BResult<&str, &str> {
    keyword("this")
}
pub fn peek_this() -> impl Fn(&str) -> BResult<&str, &str> {
    peek_keyword("this")
}

pub fn kw_base() -> impl Fn(&str) -> BResult<&str, &str> {
    keyword("base")
}
pub fn peek_base() -> impl Fn(&str) -> BResult<&str, &str> {
    peek_keyword("base")
}

pub fn kw_global() -> impl Fn(&str) -> BResult<&str, &str> {
    keyword("global")
}
pub fn peek_global() -> impl Fn(&str) -> BResult<&str, &str> {
    peek_keyword("global")
}

pub fn kw_var() -> impl Fn(&str) -> BResult<&str, &str> {
    keyword("var")
}
pub fn peek_var() -> impl Fn(&str) -> BResult<&str, &str> {
    peek_keyword("var")
}

pub fn kw_dynamic() -> impl Fn(&str) -> BResult<&str, &str> {
    keyword("dynamic")
}
pub fn peek_dynamic() -> impl Fn(&str) -> BResult<&str, &str> {
    peek_keyword("dynamic")
}
