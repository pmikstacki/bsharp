use bsharp::parsers::statements::block_statement_parser::parse_block_statement;
use bsharp::parsers::statement_parser::parse_statement_ws;

fn main() {
    let block_with_comments = "{ /* body */ }";
    println!("Testing block with comments: {}", block_with_comments);
    
    // Try parsing with the block statement parser directly
    match parse_block_statement(block_with_comments) {
        Ok((rest, stmt)) => {
            println!("✅ Block statement parser succeeded!");
            println!("   Remaining: {:?}", rest);
            println!("   Result: {:?}", stmt);
        },
        Err(e) => {
            println!("❌ Block statement parser failed: {:?}", e);
        }
    }
    
    // Try parsing with the statement parser with whitespace handling
    match parse_statement_ws(block_with_comments) {
        Ok((rest, stmt)) => {
            println!("✅ Statement parser with whitespace succeeded!");
            println!("   Remaining: {:?}", rest);
            println!("   Result: {:?}", stmt);
        },
        Err(e) => {
            println!("❌ Statement parser with whitespace failed: {:?}", e);
        }
    }
    
    // Try the test case from the for statement test
    let test_case = "for (int k = 0; k < 5; k++, DoSomething()) { /* body */ }";
    
    // Focus on just the body part
    let body_part = "{ /* body */ }";
    println!("\nTesting just the body part: {}", body_part);
    
    match parse_statement_ws(body_part) {
        Ok((rest, stmt)) => {
            println!("✅ Statement parser succeeded on body part!");
            println!("   Remaining: {:?}", rest);
            println!("   Result: {:?}", stmt);
        },
        Err(e) => {
            println!("❌ Statement parser failed on body part: {:?}", e);
        }
    }
}
