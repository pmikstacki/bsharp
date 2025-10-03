//! Exception and safety keywords: try, catch, finally, throw, checked, unchecked, fixed, lock
//!
//! Used in statement parsers and safety contexts.

define_keyword_pair!(kw_try, peek_try, "try");
define_keyword_pair!(kw_catch, peek_catch, "catch");
define_keyword_pair!(kw_finally, peek_finally, "finally");
define_keyword_pair!(kw_throw, peek_throw, "throw");

define_keyword_pair!(kw_checked, peek_checked, "checked");
define_keyword_pair!(kw_unchecked, peek_unchecked, "unchecked");
define_keyword_pair!(kw_fixed, peek_fixed, "fixed");
define_keyword_pair!(kw_lock, peek_lock, "lock");
