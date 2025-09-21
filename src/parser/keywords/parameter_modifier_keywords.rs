//! Parameter modifier keywords: ref, out, in, params, this
//!
//! Used in parameter and argument modifier parsing.

define_keyword_pair!(kw_ref, peek_ref, "ref");
define_keyword_pair!(kw_out, peek_out, "out");
define_keyword_pair!(kw_in, peek_in, "in");
define_keyword_pair!(kw_params, peek_params, "params");
define_keyword_pair!(kw_this, peek_this, "this");
