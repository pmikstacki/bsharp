//! Iteration keywords: for, foreach, while, do
//!
//! Used in statement parsers.

define_keyword_pair!(kw_for, peek_for, "for");
define_keyword_pair!(kw_foreach, peek_foreach, "foreach");
define_keyword_pair!(kw_while, peek_while, "while");
define_keyword_pair!(kw_do, peek_do, "do");
