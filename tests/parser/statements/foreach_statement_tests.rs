// Integration tests for foreach_statement_parser.rs
// Content moved from src/parser/statements/foreach_statement_parser.rs

use bsharp::parser::expressions::statements::foreach_statement_parser::parse_foreach_statement;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::statements::statement::Statement;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};
use bsharp::syntax::test_helpers::parse_all;

#[test]
fn test_parse_foreach_statement() {
    let input = "foreach (var item in myList) Console.WriteLine(item);";
    let result = parse_all(parse_foreach_statement, input);
    assert!(result.is_ok());
    // Detailed assertions
    if let Ok((_, Statement::ForEach(stmt))) = result {
        assert_eq!(stmt.var_type, Type::Var);
        assert_eq!(stmt.var_name.name, "item");
        assert!(matches!(*stmt.collection, Expression::Variable(_))); // Check collection is a variable
        assert!(matches!(*stmt.body, Statement::Expression(_))); // Check body is an expression statement
    } else {
        panic!("Expected ForEach statement, got {:?}", result);
    }

    let input_explicit_type = "foreach (int number in numbers) { sum += number; }";
    let result_explicit_type = parse_all(parse_foreach_statement, input_explicit_type);
    assert!(result_explicit_type.is_ok());
    if let Ok((_, Statement::ForEach(stmt))) = result_explicit_type {
        assert!(matches!(stmt.var_type, Type::Primitive(PrimitiveType::Int)));
        assert_eq!(stmt.var_name.name, "number");
        assert!(matches!(*stmt.collection, Expression::Variable(_))); // Check collection is a variable
        assert!(matches!(*stmt.body, Statement::Block(_))); // Check body is a block statement
    } else {
        panic!("Expected ForEach statement, got {:?}", result_explicit_type);
    }
}
