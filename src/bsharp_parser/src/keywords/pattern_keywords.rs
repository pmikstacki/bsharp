//! Pattern keywords: and, or, not
//!
//! Used in pattern matching grammar.

define_keyword_pair!(kw_and, peek_and, "and");
define_keyword_pair!(kw_or, peek_or, "or");
define_keyword_pair!(kw_not, peek_not, "not");
