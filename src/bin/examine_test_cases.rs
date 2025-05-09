use bsharp::parsers::statements::for_statement_parser::parse_for_statement;
use bsharp::parsers::statements::parse_try_statement;

fn main() {
    // Test case 1: For statement with multiple iterators
    let for_input = "for (int k = 0; k < 5; k++, DoSomething()) { /* body */ }";
    println!("FOR TEST CASE: {}", for_input);
    match parse_for_statement(for_input) {
        Ok(result) => println!("✅ FOR TEST PASSED! Remaining: '{}'", result.0),
        Err(e) => println!("❌ FOR TEST FAILED: {:?}", e),
    }
    
    // Test case 2: Try statement with comments
    let try_input = "try { /*...*/ } catch (IOException ex) { } finally { /*...*/ }";
    println!("\nTRY TEST CASE: {}", try_input);
    match parse_try_statement(try_input) {
        Ok(result) => println!("✅ TRY TEST PASSED! Remaining: '{}'", result.0),
        Err(e) => println!("❌ TRY TEST FAILED: {:?}", e),
    }
}
