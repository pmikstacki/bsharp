//! Tests for keyword parsers defined in `src/parser/keywords/`.
//! Focus on:
//! - Word boundary behavior (should not match suffixes like `int_`, `publicity`).
//! - Peek non-consumption behavior (should not advance input).

use bsharp::parser::keywords::access_keywords::*;
use bsharp::parser::keywords::accessor_keywords::*;
use bsharp::parser::keywords::contextual_misc_keywords::*;
use bsharp::parser::keywords::expression_keywords::{kw_await, kw_default};
use bsharp::parser::keywords::linq_query_keywords::*;
use bsharp::parser::keywords::literal_keywords::*;
use bsharp::parser::keywords::selection_and_switch_keywords::*;
use bsharp::parser::keywords::type_keywords::*;
use bsharp::syntax::parser_helpers::bws;

#[test]
fn kw_public_matches_and_respects_boundaries() {
    let (rest, out) = kw_public()("public ").expect("ok");
    assert_eq!(out, "public");
    assert_eq!(rest, " ");

    // Should not match when followed by identifier characters
    assert!(kw_public()("public1").is_err());
    assert!(kw_public()("public_").is_err());
    assert!(kw_public()("publicity").is_err());
}

#[test]
fn kw_this_matches_and_respects_boundaries() {
    let (rest, out) = kw_this()("this;").expect("ok");
    assert_eq!(out, "this");
    assert_eq!(rest, ";");

    assert!(kw_this()("this_foo").is_err());
}

#[test]
fn peek_get_is_non_consuming_and_kw_get_consumes_with_bws() {
    let src = "   get;";

    // Peek must not consume input
    let (rest, matched) = peek_get()(src).expect("peek ok");
    assert_eq!(rest, src, "peek must not consume input");
    assert_eq!(matched, "get");

    // kw_get() alone expects exact position; bws(kw_get()) tolerates leading WS/comments
    let (rest2, matched2) = bws(kw_get())(src).expect("kw ok");
    assert_eq!(matched2, "get");
    assert_eq!(rest2, ";");
}

#[test]
fn type_keywords_basic_and_boundaries() {
    let (rest, out) = kw_int()("int [").expect("ok");
    assert_eq!(out, "int");
    assert_eq!(rest, " [");

    // Boundaries
    assert!(kw_int()("integer").is_err());
    assert!(kw_int()("int_").is_err());

    // Ensure no accidental prefix match: `short` must not match `ushort`
    assert!(kw_short()("ushort").is_err());
    let (rest_u, out_u) = kw_ushort()("ushort,").expect("ok");
    assert_eq!(out_u, "ushort");
    assert_eq!(rest_u, ",");
}

#[test]
fn selection_keywords_and_peek() {
    let src = " if(x)";

    // peek_if does not consume
    let (rest, matched) = peek_if()(src).expect("peek ok");
    assert_eq!(rest, src);
    assert_eq!(matched, "if");

    // kw_else boundaries
    assert!(kw_else()("elsewhere").is_err());
    let (rest2, out2) = kw_else()("else ").expect("ok");
    assert_eq!(out2, "else");
    assert_eq!(rest2, " ");
}

#[test]
fn await_and_default_expression_keywords() {
    let (rest, out) = kw_await()("await expr").expect("ok");
    assert_eq!(out, "await");
    assert_eq!(rest, " expr");

    // default should respect boundaries
    assert!(kw_default()("defaulted").is_err());
    let (r2, o2) = kw_default()("default(").expect("ok");
    assert_eq!(o2, "default");
    assert_eq!(r2, "(");
}

#[test]
fn linq_keywords_basic_and_peek() {
    let src = "from x in xs select x";
    let (rest, out) = kw_from()(src).expect("ok");
    assert_eq!(out, "from");
    assert!(rest.starts_with(" x in xs"));

    // `where` boundaries
    assert!(kw_where()("wherever").is_err());

    // peek_orderby non-consuming
    let src2 = "  orderby x";
    let (rest2, out2) = peek_orderby()(src2).expect("peek ok");
    assert_eq!(rest2, src2);
    assert_eq!(out2, "orderby");
}

#[test]
fn literal_keywords_basic() {
    let (rest_t, out_t) = kw_true()("true;").expect("ok");
    assert_eq!(out_t, "true");
    assert_eq!(rest_t, ";");

    // Boundaries
    assert!(kw_false()("falsey").is_err());
    assert!(kw_null()("nullable").is_err());
}
