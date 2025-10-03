//! Expression keywords: new, typeof, sizeof, nameof, stackalloc, as, is, await, default
//!
//! Used in expression parser modules.

define_keyword_pair!(kw_new, peek_new, "new");
define_keyword_pair!(kw_typeof, peek_typeof, "typeof");
define_keyword_pair!(kw_sizeof, peek_sizeof, "sizeof");
define_keyword_pair!(kw_nameof, peek_nameof, "nameof");
define_keyword_pair!(kw_stackalloc, peek_stackalloc, "stackalloc");
define_keyword_pair!(kw_as, peek_as, "as");
define_keyword_pair!(kw_is, peek_is, "is");
define_keyword_pair!(kw_await, peek_await, "await");
define_keyword_pair!(kw_default, peek_default, "default");
define_keyword_pair!(kw_with, peek_with, "with");
