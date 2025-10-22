//! Contextual miscellaneous keywords: this, base, global, var, dynamic
//!
//! Provide small parser functions per keyword where context decides meaning.

use crate::define_keyword_pair;

define_keyword_pair!(kw_this, peek_this, "this");
define_keyword_pair!(kw_base, peek_base, "base");
define_keyword_pair!(kw_global, peek_global, "global");
define_keyword_pair!(kw_var, peek_var, "var");
define_keyword_pair!(kw_dynamic, peek_dynamic, "dynamic");
