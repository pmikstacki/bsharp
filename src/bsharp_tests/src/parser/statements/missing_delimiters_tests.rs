use parser::expressions::statements::break_statement_parser::parse_break_statement;
use parser::expressions::statements::continue_statement_parser::parse_continue_statement;
use parser::expressions::statements::do_while_statement_parser::parse_do_while_statement;
use parser::expressions::statements::expression_statement_parser::parse_expression_statement;
use parser::expressions::statements::goto_case_statement_parser::parse_goto_case_statement;
use parser::expressions::statements::goto_statement_parser::parse_goto_statement;
use parser::expressions::statements::throw_statement_parser::parse_throw_statement;
use parser::expressions::statements::yield_statement_parser::parse_yield_statement;
use parser::expressions::statements::switch_statement_parser::parse_switch_statement;
use syntax::errors::BResult;

fn assert_failure<T>(res: BResult<&str, T>) {
    match res {
        Err(nom::Err::Failure(_)) => {}
        Err(other) => panic!("Expected Failure due to cut(), got: {:?}", other),
        Ok(ok) => panic!("Expected Failure due to cut(), got Ok: {:?}", ok),
    }
}

#[test]
fn break_missing_semicolon() {
    assert_failure(parse_break_statement("break"));
}

#[test]
fn continue_missing_semicolon() {
    assert_failure(parse_continue_statement("continue"));
}

#[test]
fn return_missing_semicolon() {
    assert_failure(parse_expression_statement("return 1"));
}

#[test]
fn throw_missing_semicolon() {
    assert_failure(parse_throw_statement("throw ex"));
}

#[test]
fn yield_return_missing_semicolon() {
    assert_failure(parse_yield_statement("yield return x"));
}

#[test]
fn goto_missing_semicolon() {
    assert_failure(parse_goto_statement("goto label"));
}

#[test]
fn goto_case_missing_semicolon() {
    assert_failure(parse_goto_case_statement("goto case x"));
    assert_failure(parse_goto_case_statement("goto default"));
}

#[test]
fn expression_stmt_missing_semicolon() {
    assert_failure(parse_expression_statement("DoSomething()"));
}

#[test]
fn do_while_missing_closing_paren() {
    // Missing ')'
    assert_failure(parse_do_while_statement("do {} while(1;"));
}

#[test]
fn do_while_missing_semicolon() {
    assert_failure(parse_do_while_statement("do {} while(1)"));
}

#[test]
fn switch_missing_closing_brace() {
    assert_failure(parse_switch_statement("switch(x) { case 1: break;"));
}
