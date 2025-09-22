// Tests for expression traversal in navigation::implementations

use bsharp::syntax::{AstNavigate, Parser};
use bsharp::syntax::nodes::expressions::expression::Expression;

#[test]
fn test_collect_expressions_basic() {
    let parser = Parser::new();
    let source = r#"
namespace N {
  public class C {
    public void M() {
      int x = 1 + 2;
      Console.WriteLine(x);
      if (x > 0) { x = x - 1; }
      for (int i = 0; i < 3; i++) { x += i; }
    }
  }
}
"#;

    let ast = parser.parse(source).expect("parse failed");

    // All expressions
    let all = ast.find_expressions(|_| true);
    assert!(all.len() >= 10, "expected at least 10 expressions, got {}", all.len());

    // Only invocations
    let invocations = ast.find_expressions(|e| matches!(e, Expression::Invocation(_)));
    assert_eq!(invocations.len(), 1, "should find 1 invocation expression");

    // Only binary operators
    let binaries = ast.find_expressions(|e| matches!(e, Expression::Binary{..}));
    assert!(binaries.len() >= 2, "expected at least 2 binary expressions (1+2, x>0), got {}", binaries.len());

    // Assignments (x = 1+2, x = x - 1, x += i)
    let assignments = ast.find_expressions(|e| matches!(e, Expression::Assignment(_)));
    assert!(assignments.len() >= 2, "expected at least 2 assignments, got {}", assignments.len());
}
