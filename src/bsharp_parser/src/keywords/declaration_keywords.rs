//! Declaration keywords: class, struct, interface, enum, delegate, record, namespace, using, operator, event
//!
//! These will be used across various declaration parsers.

define_keyword_pair!(kw_class, peek_class, "class");
define_keyword_pair!(kw_struct, peek_struct, "struct");
define_keyword_pair!(kw_interface, peek_interface, "interface");
define_keyword_pair!(kw_enum, peek_enum, "enum");
define_keyword_pair!(kw_delegate, peek_delegate, "delegate");
define_keyword_pair!(kw_record, peek_record, "record");

define_keyword_pair!(kw_namespace, peek_namespace, "namespace");
define_keyword_pair!(kw_using, peek_using, "using");
define_keyword_pair!(kw_event, peek_event, "event");

define_keyword_pair!(kw_operator, peek_operator, "operator");
define_keyword_pair!(kw_explicit, peek_explicit, "explicit");
define_keyword_pair!(kw_implicit, peek_implicit, "implicit");
