//! Flow-control keywords: break, continue, goto, return, yield
//!
//! Used in statement parsers.

define_keyword_pair!(kw_break, peek_break, "break");
define_keyword_pair!(kw_continue, peek_continue, "continue");
define_keyword_pair!(kw_goto, peek_goto, "goto");
define_keyword_pair!(kw_return, peek_return, "return");
define_keyword_pair!(kw_yield, peek_yield, "yield");
