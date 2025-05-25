use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::constructor_declaration::ConstructorDeclaration;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parsers::declarations::parameter_parser::parse_parameter_list;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::statements::block_statement_parser::parse_block_statement;
use nom::combinator::opt;
// Assuming constructors can have modifiers

pub fn parse_constructor_declaration(input: &str) -> BResult<&str, ConstructorDeclaration> {
    println!("[DEBUG] parse_constructor_declaration: input = {:?}", input.chars().take(60).collect::<String>());

    // Parse modifiers (e.g., public, private)
    let (input, modifiers) = bws(nom_to_bs(|i| parse_modifiers_for_decl_type(i, "constructor")))(input)?;

    // Check if the next token after modifiers looks like a return type (void, int, string, etc.)
    // If so, this is likely a method, not a constructor
    let trimmed_after_modifiers = input.trim_start();
    println!("[DEBUG] parse_constructor_declaration: after modifiers = {:?}", trimmed_after_modifiers.chars().take(30).collect::<String>());
    
    // Common return type keywords that indicate this is a method, not a constructor
    let return_type_keywords = ["void", "int", "string", "bool", "double", "float", "char", "byte", "long", "short", "decimal", "object"];
    
    for keyword in &return_type_keywords {
        if trimmed_after_modifiers.starts_with(keyword) {
            // Check if it's followed by whitespace or identifier (not part of a larger identifier)
            let after_keyword = &trimmed_after_modifiers[keyword.len()..];
            if after_keyword.is_empty() || after_keyword.chars().next().unwrap().is_whitespace() {
                // This looks like a method with a return type, not a constructor
                println!("[DEBUG] parse_constructor_declaration: detected return type '{}', rejecting as constructor", keyword);
                return Err(nom::Err::Error(crate::parser::errors::BSharpParseError::new(
                    input,
                    crate::parser::errors::CustomErrorKind::Expected("constructor (no return type)")
                )));
            }
        }
    }

    // Parse constructor name (must be same as class name, validation later)
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    println!("[DEBUG] parse_constructor_declaration: name = {:?}", name);

    // Parse parameter list
    let (input, parameters) = bws(nom_to_bs(parse_parameter_list))(input)?;
    println!("[DEBUG] parse_constructor_declaration: parameters = {:?}", parameters);
    
    // Optional: Parse constructor initializer (e.g., : base(args), : this(args))
    // For now, we'll skip this for simplicity.
    // let (input, initializer) = opt(parse_constructor_initializer)(input)?;

    // Parse constructor body
    let (input, body) = bws(nom_to_bs(opt(parse_block_statement)))(input)?;
    println!("[DEBUG] parse_constructor_declaration: body parsed, remaining input = {:?}", input.chars().take(60).collect::<String>());
    
    Ok((input, ConstructorDeclaration {
        modifiers,
        name,
        parameters,
        body,
        // initializer,
    }))
}

// TODO: Later, add parse_constructor_initializer if needed.
// fn parse_constructor_initializer(input: &str) -> BResult<&str, ConstructorInitializer> { ... } 