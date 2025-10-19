//! Modifier keywords: abstract, sealed, static, virtual, override, readonly, volatile, unsafe, extern, partial, new, async, required
//!
//! Used in declaration and member modifier parsers.

use crate::define_keyword_pair;

define_keyword_pair!(kw_abstract, peek_abstract, "abstract");
define_keyword_pair!(kw_sealed, peek_sealed, "sealed");
define_keyword_pair!(kw_static, peek_static, "static");
define_keyword_pair!(kw_virtual, peek_virtual, "virtual");
define_keyword_pair!(kw_override, peek_override, "override");
define_keyword_pair!(kw_readonly, peek_readonly, "readonly");
define_keyword_pair!(kw_volatile, peek_volatile, "volatile");
define_keyword_pair!(kw_unsafe, peek_unsafe, "unsafe");
define_keyword_pair!(kw_extern, peek_extern, "extern");
define_keyword_pair!(kw_partial, peek_partial, "partial");
define_keyword_pair!(kw_new, peek_new, "new");
define_keyword_pair!(kw_async, peek_async, "async");
define_keyword_pair!(kw_required, peek_required, "required");
define_keyword_pair!(kw_const, peek_const, "const");
define_keyword_pair!(kw_managed, peek_managed, "managed");
define_keyword_pair!(kw_unmanaged, peek_unmanaged, "unmanaged");
