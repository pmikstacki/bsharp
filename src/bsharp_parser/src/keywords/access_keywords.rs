//! Access keywords: public, private, protected, internal, file
//!
//! Provide small parser functions per keyword: `kw_public()`, `peek_public()`, etc.
//! Integration: use these in modifier and declaration parsers instead of `keyword("...")`.

define_keyword_pair!(kw_public, peek_public, "public");
define_keyword_pair!(kw_private, peek_private, "private");
define_keyword_pair!(kw_protected, peek_protected, "protected");
define_keyword_pair!(kw_internal, peek_internal, "internal");
define_keyword_pair!(kw_file, peek_file, "file");
