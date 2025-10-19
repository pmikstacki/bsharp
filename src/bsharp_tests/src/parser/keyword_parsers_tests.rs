//! Tests for keyword parsers defined in `src/parser/keywords/`.
//! Focus on:
//! - Word boundary behavior (should not match suffixes like `int_`, `publicity`).
//! - Peek non-consumption behavior (should not advance input).

use parser::keywords::access_keywords::*;
use parser::keywords::accessor_keywords::*;
use parser::keywords::contextual_misc_keywords::*;
use parser::keywords::expression_keywords::{kw_await, kw_default};
use parser::keywords::linq_query_keywords::*;
use parser::keywords::literal_keywords::*;
use parser::keywords::selection_and_switch_keywords::*;
use parser::keywords::type_keywords::*;
// Removed parser_helpers::bws import; using direct keyword parsers instead

#[test]
fn kw_public_matches_and_respects_boundaries() {
    let (rest, out) = kw_public()("public ".into()).expect("ok");
    assert_eq!(out, "public");
    assert_eq!(*rest.fragment(), " ");

    // Should not match when followed by identifier characters
    assert!(kw_public()("public1".into()).is_err());
    assert!(kw_public()("public_".into()).is_err());
    assert!(kw_public()("publicity".into()).is_err());
}

#[test]
fn kw_this_matches_and_respects_boundaries() {
    let (rest, out) = kw_this()("this;".into()).expect("ok");
    assert_eq!(out, "this");
    assert_eq!(*rest.fragment(), ";");

    assert!(kw_this()("this_foo".into()).is_err());
}

#[test]
fn peek_get_is_non_consuming_and_kw_get_consumes_with_bws() {
    let src = "get;";

    // Peek must not consume input
    let (rest, matched) = peek_get()(src.into()).expect("peek ok");
    assert_eq!(*rest.fragment(), src, "peek must not consume input");
    assert_eq!(matched, "get");

    // kw_get() consumes the keyword at current position
    let (rest2, matched2) = kw_get()(src.into()).expect("kw ok");
    assert_eq!(matched2, "get");
    assert_eq!(*rest2.fragment(), ";");
}

#[test]
fn type_keywords_basic_and_boundaries() {
    let (rest, out) = kw_int()("int [".into()).expect("ok");
    assert_eq!(out, "int");
    assert_eq!(*rest.fragment(), " [");

    // Boundaries
    assert!(kw_int()("integer".into()).is_err());
    assert!(kw_int()("int_".into()).is_err());

    // Ensure no accidental prefix match: `short` must not match `ushort`
    assert!(kw_short()("ushort".into()).is_err());
    let (rest_u, out_u) = kw_ushort()("ushort,".into()).expect("ok");
    assert_eq!(out_u, "ushort");
    assert_eq!(*rest_u.fragment(), ",");
}

#[test]
fn selection_keywords_and_peek() {
    let src = " if(x)";

    // peek_if does not consume
    let (rest, matched) = peek_if()(src.into()).expect("peek ok");
    assert_eq!(*rest.fragment(), src);
    assert_eq!(matched, "if");

    // kw_else boundaries
    assert!(kw_else()("elsewhere".into()).is_err());
    let (rest2, out2) = kw_else()("else ".into()).expect("ok");
    assert_eq!(out2, "else");
    assert_eq!(*rest2.fragment(), " ");
}

#[test]
fn await_and_default_expression_keywords() {
    let (rest, out) = kw_await()("await expr".into()).expect("ok");
    assert_eq!(out, "await");
    assert_eq!(*rest.fragment(), " expr");

    // default should respect boundaries
    assert!(kw_default()("defaulted".into()).is_err());
    let (r2, o2) = kw_default()("default(".into()).expect("ok");
    assert_eq!(o2, "default");
    assert_eq!(*r2.fragment(), "(");
}

#[test]
fn linq_keywords_basic_and_peek() {
    let src = "from x in xs select x";
    let (rest, out) = kw_from()(src.into()).expect("ok");
    assert_eq!(out, "from");
    assert!((*rest.fragment()).starts_with(" x in xs"));

    // `where` boundaries
    assert!(kw_where()("wherever".into()).is_err());

    // peek_orderby non-consuming
    let src2 = "  orderby x";
    let (rest2, out2) = peek_orderby()(src2.into()).expect("peek ok");
    assert_eq!(*rest2.fragment(), src2);
    assert_eq!(out2, "orderby");
}

#[test]
fn literal_keywords_basic() {
    let (rest_t, out_t) = kw_true()("true;".into()).expect("ok");
    assert_eq!(out_t, "true");
    assert_eq!(*rest_t.fragment(), ";");

    // Boundaries
    assert!(kw_false()("falsey".into()).is_err());
    assert!(kw_null()("nullable".into()).is_err());
}
