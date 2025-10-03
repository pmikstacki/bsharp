// Invalid-input diagnostics tests to lock in pretty error formatting

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::errors::format_error_tree;
use nom::Err as NomErr;

#[test]
fn invalid_incomplete_ternary_pretty() {
    let input = "cond ? a";
    match parse_expression(input) {
        Ok((_rest, _expr)) => panic!("expected error, got Ok"),
        Err(NomErr::Error(tree)) | Err(NomErr::Failure(tree)) => {
            let msg = format_error_tree(input, &tree);
            assert!(msg.contains("at "), "expected line/col header in diagnostics, got:\n{}", msg);
            assert!(msg.contains("^"), "expected caret to mark column, got:\n{}", msg);
        }
        Err(NomErr::Incomplete(_)) => panic!("unexpected Incomplete"),
    }
}

#[test]
fn invalid_dangling_null_conditional_pretty() {
    let input = "a?.";
    match parse_expression(input) {
        Ok((_rest, _expr)) => panic!("expected error, got Ok"),
        Err(NomErr::Error(tree)) | Err(NomErr::Failure(tree)) => {
            let msg = format_error_tree(input, &tree);
            assert!(msg.contains("at "), "expected line/col header in diagnostics, got:\n{}", msg);
            assert!(msg.contains("^"), "expected caret to mark column, got:\n{}", msg);
        }
        Err(NomErr::Incomplete(_)) => panic!("unexpected Incomplete"),
    }
}

#[test]
fn invalid_missing_close_paren_in_call_pretty() {
    let input = "f(x";
    match parse_expression(input) {
        Ok((_rest, _expr)) => panic!("expected error, got Ok"),
        Err(NomErr::Error(tree)) | Err(NomErr::Failure(tree)) => {
            let msg = format_error_tree(input, &tree);
            assert!(msg.contains("at "), "expected line/col header in diagnostics, got:\n{}", msg);
            assert!(msg.contains("^"), "expected caret to mark column, got:\n{}", msg);
        }
        Err(NomErr::Incomplete(_)) => panic!("unexpected Incomplete"),
    }
}

#[test]
fn invalid_member_trailing_dot_pretty() {
    let input = "a.";
    match parse_expression(input) {
        Ok((_rest, _expr)) => panic!("expected error, got Ok"),
        Err(NomErr::Error(tree)) | Err(NomErr::Failure(tree)) => {
            let msg = format_error_tree(input, &tree);
            assert!(msg.contains("at "), "expected line/col header in diagnostics, got:\n{}", msg);
            assert!(msg.contains("^"), "expected caret to mark column, got:\n{}", msg);
        }
        Err(NomErr::Incomplete(_)) => panic!("unexpected Incomplete"),
    }
}
