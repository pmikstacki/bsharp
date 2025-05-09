use bsharp::parser::nodes::statements::statement::Statement;
use bsharp::parser::parser_helpers::{bchar, bws, keyword};
use bsharp::parsers::expressions::expression_parser::parse_expression;
use bsharp::parsers::statement_parser::parse_statement_ws;
use bsharp::parsers::statements::for_statement_parser::parse_for_statement;
use nom::multi::separated_list0;

fn main() {
    let input_multiple_iter = "for (int k = 0; k < 5; k++, DoSomething()) { /* body */ }";
    println!("Testing: {}", input_multiple_iter);
    
    // Step-by-step debugging
    println!("\nStep-by-step debug:");
    println!("------------------");
    
    // Initial parsing steps
    let mut input = input_multiple_iter;
    
    println!("1. Parsing 'for' keyword");
    let keyword_result = keyword("for")(input);
    if let Ok((rest, _)) = keyword_result {
        println!("✅ Parsed 'for' keyword");
        println!("   Remaining: {:?}", rest);
        input = rest;
    } else {
        println!("❌ Failed to parse 'for' keyword: {:?}", keyword_result);
        return;
    }
    
    println!("\n2. Parsing opening parenthesis");
    let open_paren_result = bws(bchar('('))(input);
    if let Ok((rest, _)) = open_paren_result {
        println!("✅ Parsed opening parenthesis");
        println!("   Remaining: {:?}", rest);
        input = rest;
    } else {
        println!("❌ Failed to parse opening parenthesis: {:?}", open_paren_result);
        return;
    }
    
    // Skip initializer and condition parsing for brevity
    println!("\n3. Skipping initializer and condition parsing...");
    
    // Try to find the semicolon after condition
    let mut found_second_semicolon = false;
    for i in 0..input.len() {
        if input[i..].starts_with(";") && i > 0 {
            // Find the second semicolon
            for j in (i+1)..input.len() {
                if input[j..].starts_with(";") {
                    println!("✅ Found second semicolon at position {}", j);
                    input = &input[j+1..]; // Skip to after the second semicolon
                    found_second_semicolon = true;
                    println!("   Remaining for iterator parsing: {:?}", input);
                    break;
                }
            }
            if found_second_semicolon {
                break;
            }
        }
    }
    
    if !found_second_semicolon {
        println!("❌ Could not find the second semicolon");
        return;
    }
    
    println!("\n4. Parsing iterator expressions");
    let iterator_parsing = separated_list0(bws(bchar(',')), bws(parse_expression))(input);
    if let Ok((rest, expressions)) = iterator_parsing {
        println!("✅ Parsed {} iterator expressions", expressions.len());
        for (i, expr) in expressions.iter().enumerate() {
            println!("   Expression {}: {:?}", i+1, expr);
        }
        println!("   Remaining: {:?}", rest);
        input = rest;
    } else {
        println!("❌ Failed to parse iterator expressions: {:?}", iterator_parsing);
        
        // Try parsing just the first expression to see if that works
        println!("\n4a. Trying to parse just the first iterator expression");
        let first_expr_result = bws(parse_expression)(input);
        if let Ok((rest, expr)) = first_expr_result {
            println!("✅ Parsed first expression: {:?}", expr);
            println!("   Remaining: {:?}", rest);
            
            // Try parsing the comma
            println!("\n4b. Trying to parse the comma");
            let comma_result = bws(bchar(','))(rest);
            if let Ok((after_comma, _)) = comma_result {
                println!("✅ Parsed comma");
                println!("   Remaining: {:?}", after_comma);
                
                // Try parsing the second expression
                println!("\n4c. Trying to parse the second expression");
                let second_expr_result = bws(parse_expression)(after_comma);
                if let Ok((final_rest, second_expr)) = second_expr_result {
                    println!("✅ Parsed second expression: {:?}", second_expr);
                    println!("   Remaining: {:?}", final_rest);
                } else {
                    println!("❌ Failed to parse second expression: {:?}", second_expr_result);
                }
            } else {
                println!("❌ Failed to parse comma: {:?}", comma_result);
            }
        } else {
            println!("❌ Failed to parse even first iterator expression: {:?}", first_expr_result);
        }
        return;
    }
    
    println!("\n5. Parsing closing parenthesis");
    let close_paren_result = bws(bchar(')'))(input);
    if let Ok((rest, _)) = close_paren_result {
        println!("✅ Parsed closing parenthesis");
        println!("   Remaining: {:?}", rest);
        input = rest;
    } else {
        println!("❌ Failed to parse closing parenthesis: {:?}", close_paren_result);
        return;
    }
    
    println!("\n6. Parsing statement body");
    let body_result = bws(parse_statement_ws)(input);
    if let Ok((rest, body)) = body_result {
        println!("✅ Parsed body: {:?}", body);
        println!("   Remaining: {:?}", rest);
    } else {
        println!("❌ Failed to parse body: {:?}", body_result);
        return;
    }
    
    // Finally, try the whole thing
    println!("\nTrying the full parser:");
    match parse_for_statement(input_multiple_iter) {
        Ok((rest, stmt)) => {
            println!("✅ Full parse successful!");
            println!("   Remaining: {:?}", rest);
            if let Statement::For(for_stmt) = stmt {
                println!("   Iterator expressions: {}", for_stmt.iterator.len());
            }
        },
        Err(e) => {
            println!("❌ Full parse failed: {:?}", e);
        }
    }
}
