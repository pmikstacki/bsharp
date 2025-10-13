use parser::expressions::statements::parse_try_statement;
use parser::expressions::statements::try_catch_finally_parser::{
    parse_catch_clause, parse_finally_clause,
};
use parser::syntax::test_helpers::parse_all;
use parser::syntax::test_helpers::parse_input_unwrap;
use syntax::identifier::Identifier;
use syntax::statements::statement::Statement;
use syntax::statements::FinallyClause;
use syntax::types::Type;

#[test]
fn test_parse_specific_catch_clause() {
    let input = "catch (Exception e) { } CATCH_SPECIFIC_BODY";
    let (remaining_input, catch_clause) = parse_input_unwrap(parse_catch_clause(input));
    // With proper whitespace handling, there's no leading space anymore
    assert_eq!(remaining_input, "CATCH_SPECIFIC_BODY");
    assert_eq!(
        catch_clause.exception_type,
        Some(Type::Reference(Identifier {
            name: "Exception".to_string()
        }))
    );
    assert_eq!(
        catch_clause.exception_variable,
        Some(Identifier {
            name: "e".to_string()
        })
    );
    match *catch_clause.block {
        Statement::Block(ref block_statement) => assert!(block_statement.is_empty()),
        _ => panic!("Expected BlockStatement"),
    }
}

#[test]
fn test_parse_general_catch_clause() {
    let input = "catch { } CATCH_GENERAL_BODY";
    let (remaining_input, catch_clause) = parse_input_unwrap(parse_catch_clause(input));
    // With proper whitespace handling, there's no leading space anymore
    assert_eq!(remaining_input, "CATCH_GENERAL_BODY");
    assert!(catch_clause.exception_type.is_none());
    assert!(catch_clause.exception_variable.is_none());
    match *catch_clause.block {
        Statement::Block(ref block_statement) => assert!(block_statement.is_empty()),
        _ => panic!("Expected BlockStatement"),
    }
}

#[test]
fn test_parse_catch_clause_no_identifier() {
    let input = "catch (System.Exception) { } CATCH_NO_IDENT_BODY";
    let (remaining_input, catch_clause) = parse_input_unwrap(parse_catch_clause(input));
    // With proper whitespace handling, there's no leading space anymore
    assert_eq!(remaining_input, "CATCH_NO_IDENT_BODY");
    assert_eq!(
        catch_clause.exception_type,
        Some(Type::Reference(Identifier {
            name: "System.Exception".to_string()
        }))
    );
    assert!(catch_clause.exception_variable.is_none());
}

#[test]
fn test_parse_finally_clause() {
    let input = "finally { } FINALLY_BODY";
    let (remaining_input, finally_clause) = parse_input_unwrap(parse_finally_clause(input));
    // With proper whitespace handling, there's no leading space anymore
    assert_eq!(remaining_input, "FINALLY_BODY");
    match *finally_clause.block {
        Statement::Block(ref block_statement) => assert!(block_statement.is_empty()),
        _ => panic!("Expected BlockStatement for finally clause"),
    }
}

#[test]
fn test_parse_try_catch_statement() {
    let input = "try { } catch (Exception e) { }";
    let result = parse_input_unwrap(parse_try_statement(input)).1;
    match result {
        Statement::Try(ts) => {
            assert_eq!(ts.catches.len(), 1, "Expected 1 catch clause");
            assert!(ts.finally_clause.is_none(), "Expected no finally clause");
        }
        _ => panic!("Expected Try statement"),
    }
}

#[test]
fn test_parse_try_catch_finally() {
    let input_try_catch = "try { DoSomething(); } catch (Exception e) { Log(e); }";
    let result_try_catch = parse_all(parse_try_statement, input_try_catch);
    assert!(result_try_catch.is_ok());
    match result_try_catch.unwrap().1 {
        Statement::Try(ts) => {
            assert!(matches!(*ts.try_block, Statement::Block(_)));
            assert_eq!(ts.catches.len(), 1);
            assert!(ts.finally_clause.is_none());
            let catch_clause = &ts.catches[0];
            assert_eq!(
                catch_clause.exception_type,
                Some(Type::Reference(Identifier {
                    name: "Exception".to_string()
                }))
            );
            assert_eq!(
                catch_clause.exception_variable,
                Some(Identifier {
                    name: "e".to_string()
                })
            );
            assert!(matches!(*catch_clause.block, Statement::Block(_)));
        }
        _ => panic!("Expected Try statement"),
    }

    // Note: The syntax was updated to allow try-finally without catch.
    let input_try_finally = "try { x = 1; } finally { CleanUp(); }";
    let result_try_finally = parse_all(parse_try_statement, input_try_finally);
    assert!(result_try_finally.is_ok());
    match result_try_finally.unwrap().1 {
        Statement::Try(ts) => {
            assert!(matches!(*ts.try_block, Statement::Block(_)));
            assert!(ts.catches.is_empty());
            assert!(ts.finally_clause.is_some());
            match ts.finally_clause {
                Some(FinallyClause { block }) => assert!(matches!(*block, Statement::Block(_))),
                _ => panic!("Expected Finally clause"),
            }
        }
        _ => panic!("Expected Try statement"),
    }

    let input_try_catch_finally = "try { /*...*/ } catch (IOException ex) { } finally { /*...*/ }";
    let result_try_catch_finally = parse_all(parse_try_statement, input_try_catch_finally);
    assert!(result_try_catch_finally.is_ok());
    match result_try_catch_finally.unwrap().1 {
        Statement::Try(ts) => {
            assert!(matches!(*ts.try_block, Statement::Block(_)));
            assert_eq!(ts.catches.len(), 1);
            assert!(ts.finally_clause.is_some());
            let catch_clause = &ts.catches[0];
            assert_eq!(
                catch_clause.exception_type,
                Some(Type::Reference(Identifier {
                    name: "IOException".to_string()
                }))
            );
            assert_eq!(
                catch_clause.exception_variable,
                Some(Identifier {
                    name: "ex".to_string()
                })
            );
            match ts.finally_clause {
                Some(FinallyClause { block }) => assert!(matches!(*block, Statement::Block(_))),
                _ => panic!("Expected Finally clause"),
            }
        }
        _ => panic!("Expected Try statement"),
    }

    let input_try_multiple_catch = "try { } catch (ArgumentException a) { } catch { }";
    let result_try_multiple_catch = parse_all(parse_try_statement, input_try_multiple_catch);
    assert!(result_try_multiple_catch.is_ok());
    match result_try_multiple_catch.unwrap().1 {
        Statement::Try(ts) => {
            assert!(matches!(*ts.try_block, Statement::Block(_)));
            assert_eq!(ts.catches.len(), 2);
            assert!(ts.finally_clause.is_none());
            // First catch has a specific type and identifier
            assert!(ts.catches[0].exception_type.is_some());
            assert!(ts.catches[0].exception_variable.is_some());
            // Second catch is a general catch block (no type or identifier)
            assert!(ts.catches[1].exception_type.is_none());
            assert!(ts.catches[1].exception_variable.is_none());
        }
        _ => panic!("Expected Try statement"),
    }
}
