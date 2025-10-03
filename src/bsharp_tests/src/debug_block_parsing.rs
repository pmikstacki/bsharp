use parser::expressions::statements::block_statement_parser::parse_block_statement;
use syntax::test_helpers::parse_all;

#[test]
fn debug_block_parsing() {
    // Test 1: Empty block
    println!("Test 1: Empty block");
    let result = parse_all(parse_block_statement, "{}");
    println!("Result: {:?}\n", result);
    assert!(result.is_ok(), "Empty block failed");

    // Test 2: Simple expression statement
    println!("Test 2: Simple expression statement");
    let result = parse_all(parse_block_statement, "{ 42; }");
    println!("Result: {:?}\n", result);

    // Test 3: Variable expression
    println!("Test 3: Variable expression");
    let result = parse_all(parse_block_statement, "{ foo; }");
    println!("Result: {:?}\n", result);

    // Test 4: Multiple simple statements
    println!("Test 4: Multiple simple statements");
    let result = parse_all(parse_block_statement, "{ foo; 42; }");
    println!("Result: {:?}\n", result);

    // Let's trace the actual failure from the original test
    println!("Original failing test:");
    let result = parse_all(parse_block_statement, "{ foo; 42; }");
    match result {
        Ok((remaining, statement)) => {
            println!(
                "✅ SUCCESS - remaining: {:?}, statement: {:?}",
                remaining, statement
            );
        }
        Err(e) => {
            println!("❌ FAILED - {:?}", e);
        }
    }
}
