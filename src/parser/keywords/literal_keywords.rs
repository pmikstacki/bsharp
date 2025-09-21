//! Literal keywords: true, false, null
//!
//! Used in literal parsing contexts.

define_keyword_pair!(kw_true, peek_true, "true");
define_keyword_pair!(kw_false, peek_false, "false");
define_keyword_pair!(kw_null, peek_null, "null");
