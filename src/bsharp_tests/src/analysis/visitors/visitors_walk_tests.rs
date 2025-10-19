#![cfg(test)]

use analysis::visitors::walk_statements;
use parser::statement_parser::parse_statement_ws;
use syntax::statements::statement::Statement;

fn count_kind(root: &Statement, predicate: fn(&Statement) -> bool) -> usize {
    let mut count = 0;
    walk_statements(root, &mut |s| {
        if predicate(s) {
            count += 1;
        }
    });
    count
}

#[test]
fn walk_lock_and_yield_and_label_and_local_function() {
    let code = r#"
{
    lock(obj) { yield return 5; }
    label: for (int i = 0; i < 1; i++) { }
    void Local() { }
}
"#;
    let (_, stmt) = parse_statement_ws(code.into()).expect("should parse");

    assert!(matches!(stmt, Statement::Block(_)));

    let lock_count = count_kind(&stmt, |s| matches!(s, Statement::Lock(_)));
    let yield_count = count_kind(&stmt, |s| matches!(s, Statement::Yield(_)));
    let label_count = count_kind(&stmt, |s| matches!(s, Statement::Label(_)));
    let local_fn_count = count_kind(&stmt, |s| matches!(s, Statement::LocalFunction(_)));

    assert_eq!(lock_count, 1);
    assert_eq!(yield_count, 1);
    assert_eq!(label_count, 1);
    assert_eq!(local_fn_count, 1);
}

#[test]
fn walk_fixed_and_using_and_unsafe() {
    let code = r#"
{
    fixed (int* p = &x) { }
    using (var d = factory()) { }
    unsafe { fixed (int* q = &x) { } }
}
"#;
    let (_, stmt) = parse_statement_ws(code.into()).expect("should parse");

    let fixed_count = count_kind(&stmt, |s| matches!(s, Statement::Fixed(_)));
    let using_count = count_kind(&stmt, |s| matches!(s, Statement::Using(_)));
    let unsafe_count = count_kind(&stmt, |s| matches!(s, Statement::Unsafe(_)));

    assert_eq!(fixed_count, 2);
    assert_eq!(using_count, 1);
    assert_eq!(unsafe_count, 1);
}
