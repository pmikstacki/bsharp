use bsharp::parsers::statements::for_statement_parser::parse_for_statement;

fn main() {
    let input_multiple_iter = "for (int k = 0; k < 5; k++, DoSomething()) { /* body */ }";
    println!("Testing: {}", input_multiple_iter);
    
    println!("\nDebugging each component separately:");
    println!("-------------------------------------");
    
    // Debug specific parts individually
    println!("\nTrying to parse with a simpler for statement:");
    let simple_for = "for (int i = 0; i < 10; i++) { }";
    match parse_for_statement(simple_for) {
        Ok(_) => println!("✅ Simple for statement parsed successfully"),
        Err(e) => println!("❌ Simple for failed: {:?}", e),
    }
    
    println!("\nTrying to parse with a for statement with multiple initializers:");
    let multiple_init = "for (int i = 0, j = 1; i < 10; i++) { }";
    match parse_for_statement(multiple_init) {
        Ok(_) => println!("✅ Multiple initializers parsed successfully"),
        Err(e) => println!("❌ Multiple initializers failed: {:?}", e),
    }
    
    println!("\nTrying to parse with a for statement with multiple iterators:");
    match parse_for_statement(input_multiple_iter) {
        Ok(_) => println!("✅ Multiple iterators parsed successfully"),
        Err(e) => println!("❌ Multiple iterators failed: {:?}", e),
    }
}
