use bsharp::parsers::statements::for_statement_parser::parse_for_statement;

fn main() {
    let input_multiple_iter = "for (int k = 0; k < 5; k++, DoSomething()) { /* body */ }";
    let result = parse_for_statement(input_multiple_iter);
    
    match result {
        Ok((remaining, stmt)) => {
            println!("SUCCESS!");
            println!("Remaining: '{}'", remaining);
            println!("Statement: {:?}", stmt);
        }
        Err(e) => {
            println!("ERROR! Failed to parse: {:?}", e);
        }
    }
}
