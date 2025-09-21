//! Selection/switch keywords: if, else, switch, case, default, when
//!
//! Used in selection and switch statement parsers.

use crate::define_keyword_pair;

define_keyword_pair!(kw_if, peek_if, "if");
define_keyword_pair!(kw_else, peek_else, "else");
define_keyword_pair!(kw_switch, peek_switch, "switch");
define_keyword_pair!(kw_case, peek_case, "case");
define_keyword_pair!(kw_default, peek_default, "default");
define_keyword_pair!(kw_when, peek_when, "when");
