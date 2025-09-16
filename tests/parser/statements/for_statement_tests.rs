// Tests for parsing for statements

// Tests originally from src/parser/statements/for_statement_parser.rs
#[test]
fn test_parse_for_statement_logic() { 
    use bsharp::syntax::nodes::expressions::expression::Expression;
    
    
    use bsharp::syntax::nodes::statements::{ForInitializer, statement::Statement};
    use bsharp::syntax::nodes::declarations::LocalVariableDeclaration;
    
    
    use bsharp::parser::statements::for_statement_parser::parse_for_statement;

    let input_simple = "for (int i = 0; i < 10; i++) { Write(i); }";
    let result_simple = parse_for_statement(input_simple);
    assert!(result_simple.is_ok());
    let (remaining_simple, stmt_simple) = result_simple.unwrap();
    assert!(remaining_simple.is_empty(), "Input not fully consumed: {}", remaining_simple);
    match stmt_simple {
        Statement::For(boxed_for_stmt) => {
            let for_stmt = &*boxed_for_stmt;
            assert!(matches!(for_stmt.initializer, Some(ForInitializer::Declaration(_))));
            assert!(for_stmt.condition.is_some());
            assert!(!for_stmt.iterator.is_empty());
            assert!(matches!(*for_stmt.body, Statement::Block(_)));
        }
        _ => panic!("Expected For statement, got {:?}", stmt_simple),
    }

    let input_multiple_init = "for (int i = 0, j = 1; i < 10; i++) Print(i);";
    let result_multiple_init = parse_for_statement(input_multiple_init);
    assert!(result_multiple_init.is_ok());
    let (remaining_multiple_init, stmt_multiple_init) = result_multiple_init.unwrap();
    assert!(remaining_multiple_init.is_empty(), "Input not fully consumed: {}", remaining_multiple_init);
    match stmt_multiple_init {
        Statement::For(boxed_for_stmt) => {
            let for_stmt = &*boxed_for_stmt;
            match &for_stmt.initializer {
                Some(ForInitializer::Declaration(LocalVariableDeclaration { declarators, .. })) => {
                    assert_eq!(declarators.len(), 2);
                }
                _ => panic!("Expected Declaration initializer"),
            }
        }
        _ => panic!("Expected For statement, got {:?}", stmt_multiple_init),
    }

    let input_multiple_iter = "for (int k = 0; k < 5; k++, DoSomething()) { /* body */ }";
    let result_multiple_iter = parse_for_statement(input_multiple_iter);
    assert!(result_multiple_iter.is_ok());
    let (remaining_multiple_iter, stmt_multiple_iter) = result_multiple_iter.unwrap();
    assert!(remaining_multiple_iter.is_empty(), "Input not fully consumed: {}", remaining_multiple_iter);
    match stmt_multiple_iter {
        Statement::For(boxed_for_stmt) => {
            let for_stmt = &*boxed_for_stmt;
            assert_eq!(for_stmt.iterator.len(), 2);
            assert!(matches!(for_stmt.iterator[0], Expression::PostfixUnary { .. }));
            assert!(matches!(for_stmt.iterator[1], Expression::Invocation(_)));
        }
        _ => panic!("Expected For statement, got {:?}", stmt_multiple_iter),
    }

    let input_no_init = "for (; i < 10; i++) Write(i);";
    let result_no_init = parse_for_statement(input_no_init);
    assert!(result_no_init.is_ok());
    let (remaining_no_init, stmt_no_init) = result_no_init.unwrap();
    assert!(remaining_no_init.is_empty(), "Input not fully consumed: {}", remaining_no_init);
    match stmt_no_init {
        Statement::For(boxed_for_stmt) => {
            let for_stmt = &*boxed_for_stmt;
            assert!(for_stmt.initializer.is_none());
        }
        _ => panic!("Expected For statement, got {:?}", stmt_no_init),
    }

    let input_no_cond = "for (int i = 0; ; i++) { if (i > 10) break; }";
    let result_no_cond = parse_for_statement(input_no_cond);
    assert!(result_no_cond.is_ok());
    let (remaining_no_cond, stmt_no_cond) = result_no_cond.unwrap();
    assert!(remaining_no_cond.is_empty(), "Input not fully consumed: {}", remaining_no_cond);
    match stmt_no_cond {
        Statement::For(boxed_for_stmt) => {
            let for_stmt = &*boxed_for_stmt;
            assert!(for_stmt.condition.is_none());
        }
        _ => panic!("Expected For statement, got {:?}", stmt_no_cond),
    }

    let input_no_iter = "for (int i = 0; i < 10;) { i++; }";
    let result_no_iter = parse_for_statement(input_no_iter);
    assert!(result_no_iter.is_ok());
    let (remaining_no_iter, stmt_no_iter) = result_no_iter.unwrap();
    assert!(remaining_no_iter.is_empty(), "Input not fully consumed: {}", remaining_no_iter);
    match stmt_no_iter {
        Statement::For(boxed_for_stmt) => {
            let for_stmt = &*boxed_for_stmt;
            assert!(for_stmt.iterator.is_empty());
        }
        _ => panic!("Expected For statement, got {:?}", stmt_no_iter),
    }

    let input_empty = "for (;;) { /* infinite loop */ }";
    let result_empty = parse_for_statement(input_empty);
    assert!(result_empty.is_ok());
    let (remaining_empty, stmt_empty) = result_empty.unwrap();
    assert!(remaining_empty.is_empty(), "Input not fully consumed: {}", remaining_empty);
    match stmt_empty {
        Statement::For(boxed_for_stmt) => {
            let for_stmt = &*boxed_for_stmt;
            assert!(for_stmt.initializer.is_none());
            assert!(for_stmt.condition.is_none());
            assert!(for_stmt.iterator.is_empty());
        }
        _ => panic!("Expected For statement, got {:?}", stmt_empty),
    }

    let input_expr_init = "for (i = 0, j = 1; i < 10; i++) { /* ... */ }";
    let result_expr_init = parse_for_statement(input_expr_init);
    assert!(result_expr_init.is_ok());
    let (remaining_expr_init, stmt_expr_init) = result_expr_init.unwrap();
    assert!(remaining_expr_init.is_empty(), "Input not fully consumed: {}", remaining_expr_init);
    match stmt_expr_init {
        Statement::For(boxed_for_stmt) => {
            let for_stmt = &*boxed_for_stmt;
            match &for_stmt.initializer { 
                Some(ForInitializer::Expressions(exprs)) => { 
                    assert_eq!(exprs.len(), 2);
                    assert!(matches!(exprs[0], Expression::Assignment { .. }));
                    assert!(matches!(exprs[1], Expression::Assignment { .. }));
                }
                _ => panic!("Expected Expressions initializer"),
            }
        }
        _ => panic!("Expected For statement, got {:?}", stmt_expr_init),
    }
}
