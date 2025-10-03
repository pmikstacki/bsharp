//! LINQ query keywords: from, where, select, group, into, join, let, orderby, ascending, descending, on, equals, by
//!
//! Used in query expression parser.

define_keyword_pair!(kw_from, peek_from, "from");
define_keyword_pair!(kw_where, peek_where, "where");
define_keyword_pair!(kw_select, peek_select, "select");
define_keyword_pair!(kw_group, peek_group, "group");
define_keyword_pair!(kw_into, peek_into, "into");
define_keyword_pair!(kw_join, peek_join, "join");
define_keyword_pair!(kw_let, peek_let, "let");
define_keyword_pair!(kw_orderby, peek_orderby, "orderby");
define_keyword_pair!(kw_ascending, peek_ascending, "ascending");
define_keyword_pair!(kw_descending, peek_descending, "descending");
define_keyword_pair!(kw_on, peek_on, "on");
define_keyword_pair!(kw_equals, peek_equals, "equals");
define_keyword_pair!(kw_by, peek_by, "by");
