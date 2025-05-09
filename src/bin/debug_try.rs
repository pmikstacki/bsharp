use bsharp::parsers::statements::parse_try_statement;

fn main() {
    let input_try_catch_finally = "try { /*...*/ } catch (IOException ex) { } finally { /*...*/ }";
    let result = parse_try_statement(input_try_catch_finally);
    
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
