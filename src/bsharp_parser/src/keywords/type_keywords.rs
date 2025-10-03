//! Type keywords: void, bool, byte, sbyte, short, ushort, int, uint, long, ulong, char, decimal, double, float, string, object, nint, nuint
//!
//! Used in type parser.

define_keyword_pair!(kw_void, peek_void, "void");
define_keyword_pair!(kw_bool, peek_bool, "bool");

define_keyword_pair!(kw_byte, peek_byte, "byte");
define_keyword_pair!(kw_sbyte, peek_sbyte, "sbyte");
define_keyword_pair!(kw_short, peek_short, "short");
define_keyword_pair!(kw_ushort, peek_ushort, "ushort");
define_keyword_pair!(kw_int, peek_int, "int");
define_keyword_pair!(kw_uint, peek_uint, "uint");
define_keyword_pair!(kw_long, peek_long, "long");
define_keyword_pair!(kw_ulong, peek_ulong, "ulong");

define_keyword_pair!(kw_char, peek_char, "char");
define_keyword_pair!(kw_decimal, peek_decimal, "decimal");
define_keyword_pair!(kw_double, peek_double, "double");
define_keyword_pair!(kw_float, peek_float, "float");

define_keyword_pair!(kw_string, peek_string, "string");
define_keyword_pair!(kw_object, peek_object, "object");

// Newer C# intrinsic integer pointer-sized types
define_keyword_pair!(kw_nint, peek_nint, "nint");
define_keyword_pair!(kw_nuint, peek_nuint, "nuint");
