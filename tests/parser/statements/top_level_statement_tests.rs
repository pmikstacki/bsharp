// Tests for parsing top-level statements (C# 9+ feature)

use bsharp::parser::expressions::statements::top_level_statement_parser::{
    parse_top_level_statement, parse_top_level_statements,
};
use bsharp::syntax::nodes::statements::statement::Statement;

fn parse_top_level_statements_helper(code: &str) -> Result<Vec<Statement>, String> {
    match parse_top_level_statements(code) {
        Ok((remaining, statements)) => {
            if remaining.trim().is_empty() {
                Ok(statements)
            } else {
                Err(format!("Unexpected remaining input: '{}'", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_top_level_statement_helper(code: &str) -> Result<Statement, String> {
    match parse_top_level_statement(code) {
        Ok((remaining, statement)) => {
            if remaining.trim().is_empty() {
                Ok(statement)
            } else {
                Err(format!("Unexpected remaining input: '{}'", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_top_level_statement() {
    let code = r#"Console.WriteLine("Hello, World!");"#;

    let result = parse_top_level_statement_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse simple top-level statement: {:?}",
        result
    );

    let statement = result.unwrap();
    match statement {
        Statement::Expression(_) => {
            // Expected: expression statement
        }
        _ => panic!("Expected expression statement, got {:?}", statement),
    }
}

#[test]
fn test_parse_multiple_top_level_statements() {
    let code = r#"Console.WriteLine("Hello, World!");
var name = "Alice";
Console.WriteLine($"Hello, {name}!");"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse multiple top-level statements: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 3);

    // First statement: Console.WriteLine
    match &statements[0] {
        Statement::Expression(_) => {
            // Expected: expression statement
        }
        _ => panic!("Expected expression statement, got {:?}", statements[0]),
    }

    // Second statement: var name = "Alice"
    match &statements[1] {
        Statement::Declaration(_) => {
            // Expected: local variable declaration
        }
        _ => panic!("Expected declaration statement, got {:?}", statements[1]),
    }

    // Third statement: Console.WriteLine with interpolation
    match &statements[2] {
        Statement::Expression(_) => {
            // Expected: expression statement
        }
        _ => panic!("Expected expression statement, got {:?}", statements[2]),
    }
}

#[test]
fn test_parse_top_level_variable_declarations() {
    let code = r#"int x = 42;
string message = "Hello";
var list = new List<int>();"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level variable declarations: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 3);

    // All should be declaration statements
    for (i, statement) in statements.iter().enumerate() {
        match statement {
            Statement::Declaration(_) => {
                // Expected: local variable declaration
            }
            _ => panic!(
                "Expected declaration statement at index {}, got {:?}",
                i, statement
            ),
        }
    }
}

#[test]
fn test_parse_top_level_control_flow() {
    let code = r#"int value = 10;

if (value > 5) {
    Console.WriteLine("Value is greater than 5");
}

for (int i = 0; i < 3; i++) {
    Console.WriteLine($"Iteration {i}");
}"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level control flow: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 3);

    // First: variable declaration
    match &statements[0] {
        Statement::Declaration(_) => {
            // Expected: int value = 10;
        }
        _ => panic!("Expected declaration statement, got {:?}", statements[0]),
    }

    // Second: if statement
    match &statements[1] {
        Statement::If(_) => {
            // Expected: if statement
        }
        _ => panic!("Expected if statement, got {:?}", statements[1]),
    }

    // Third: for loop
    match &statements[2] {
        Statement::For(_) => {
            // Expected: for statement
        }
        _ => panic!("Expected for statement, got {:?}", statements[2]),
    }
}

#[test]
fn test_parse_top_level_using_statements() {
    let code = r#"using (fileStream) {
    fileStream.Read(buffer, 0, buffer.Length);
}"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level using statements: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 1);

    // Should be a using statement
    match &statements[0] {
        Statement::Using(_) => {
            // Expected: using statement
        }
        _ => panic!("Expected using statement, got {:?}", statements[0]),
    }
}

#[test]
fn test_parse_top_level_try_catch() {
    let code = r#"try {
    Console.WriteLine("test");
} catch {
    Console.WriteLine("error");
}"#;

    let result = parse_top_level_statement_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level try-catch: {:?}",
        result
    );

    let statement = result.unwrap();
    match statement {
        Statement::Try(_) => {
            // Expected: try statement
        }
        _ => panic!("Expected try statement, got {:?}", statement),
    }
}

#[test]
fn test_parse_top_level_switch_statement() {
    let code = r#"int day = 3;

switch (day) {
    case 1:
        Console.WriteLine("Monday");
        break;
    case 2:
        Console.WriteLine("Tuesday");
        break;
    default:
        Console.WriteLine("Other day");
        break;
}"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level switch statement: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 2);

    // First: variable declaration
    match &statements[0] {
        Statement::Declaration(_) => {
            // Expected: int day = 3;
        }
        _ => panic!("Expected declaration statement, got {:?}", statements[0]),
    }

    // Second: switch statement
    match &statements[1] {
        Statement::Switch(_) => {
            // Expected: switch statement
        }
        _ => panic!("Expected switch statement, got {:?}", statements[1]),
    }
}

#[test]
fn test_parse_top_level_while_loops() {
    let code = r#"int counter = 0;

while (counter < 3) {
    Console.WriteLine($"Counter: {counter}");
    counter++;
}

do {
    Console.WriteLine("At least once");
    counter--;
} while (counter > 0);"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level while loops: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 3);

    // First: variable declaration
    match &statements[0] {
        Statement::Declaration(_) => {
            // Expected: int counter = 0;
        }
        _ => panic!("Expected declaration statement, got {:?}", statements[0]),
    }

    // Second: while loop
    match &statements[1] {
        Statement::While(_) => {
            // Expected: while statement
        }
        _ => panic!("Expected while statement, got {:?}", statements[1]),
    }

    // Third: do-while loop
    match &statements[2] {
        Statement::DoWhile(_) => {
            // Expected: do-while statement
        }
        _ => panic!("Expected do-while statement, got {:?}", statements[2]),
    }
}

#[test]
fn test_parse_top_level_foreach_loop() {
    let code = r#"var numbers = new int[] { 1, 2, 3, 4, 5 };

foreach (var number in numbers) {
    Console.WriteLine(number);
}"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level foreach loop: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 2);

    // First: variable declaration
    match &statements[0] {
        Statement::Declaration(_) => {
            // Expected: var numbers = ...
        }
        _ => panic!("Expected declaration statement, got {:?}", statements[0]),
    }

    // Second: foreach loop
    match &statements[1] {
        Statement::ForEach(_) => {
            // Expected: foreach statement
        }
        _ => panic!("Expected foreach statement, got {:?}", statements[1]),
    }
}

#[test]
fn test_parse_top_level_local_functions() {
    let code = r#"int Add(int a, int b) {
    return a + b;
}

static void PrintGreeting(string name) {
    Console.WriteLine($"Hello, {name}!");
}

int result = Add(5, 3);
PrintGreeting("World");"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level local functions: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 4);

    // First two: local function declarations
    match &statements[0] {
        Statement::LocalFunction(_) => {
            // Expected: local function
        }
        _ => panic!("Expected local function statement, got {:?}", statements[0]),
    }

    match &statements[1] {
        Statement::LocalFunction(_) => {
            // Expected: local function
        }
        _ => panic!("Expected local function statement, got {:?}", statements[1]),
    }

    // Third: variable declaration
    match &statements[2] {
        Statement::Declaration(_) => {
            // Expected: int result = Add(5, 3);
        }
        _ => panic!("Expected declaration statement, got {:?}", statements[2]),
    }

    // Fourth: function call
    match &statements[3] {
        Statement::Expression(_) => {
            // Expected: PrintGreeting("World");
        }
        _ => panic!("Expected expression statement, got {:?}", statements[3]),
    }
}

#[test]
fn test_parse_top_level_complex_example() {
    let code = r#"var numbers = new List<int> { 1, 2, 3, 4, 5 };

int CalculateSum(List<int> list) {
    int sum = 0;
    foreach (var item in list) {
        sum += item;
    }
    return sum;
}

var sum = CalculateSum(numbers);

if (sum > 10) {
    Console.WriteLine($"Sum is {sum}, which is greater than 10");
} else {
    Console.WriteLine($"Sum is {sum}, which is 10 or less");
}

for (int i = 0; i < numbers.Count; i++) {
    if (numbers[i] % 2 == 0) {
        Console.WriteLine($"Even number at index {i}: {numbers[i]}");
    }
}"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse complex top-level program: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 5); // var numbers, CalculateSum function, var sum, if statement, for loop
}

#[test]
fn test_parse_empty_top_level_statements() {
    let code = "";

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse empty top-level statements: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 0);
}

#[test]
fn test_parse_top_level_statements_with_whitespace() {
    let code = r#"

    Console.WriteLine("Hello");

    int x = 42;

    Console.WriteLine(x);

"#;

    let result = parse_top_level_statements_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse top-level statements with whitespace: {:?}",
        result
    );

    let statements = result.unwrap();
    assert_eq!(statements.len(), 3);
}

#[test]
fn test_parse_top_level_statements_error_recovery() {
    // Test that syntax can handle some invalid statements gracefully
    let code = r#"Console.WriteLine("Valid");
invalid parser here!
Console.WriteLine("Another valid");"#;

    let result = parse_top_level_statements_helper(code);
    // This should either parse what it can or fail gracefully
    // The exact behavior depends on error recovery implementation
    match result {
        Ok(statements) => {
            assert!(
                statements.len() >= 1,
                "Should parse at least one valid statement"
            );
        }
        Err(_) => {
            // Error is also acceptable if error recovery isn't implemented
        }
    }
}
