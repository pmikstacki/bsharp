use nom::{
    combinator::{map, opt},
    sequence::{preceded, tuple},
};

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::local_variable_declaration::{
    LocalVariableDeclaration, VariableDeclarator,
};
use crate::syntax::parser_helpers::{bchar, bseparated_list1, bws, context, keyword};

/// Parse a variable declarator (name with optional initializer)
/// Example: "x = 5" or just "x"
pub fn parse_variable_declarator(input: &str) -> BResult<&str, VariableDeclarator> {
    map(
        tuple((
            context(
                "variable name (expected valid identifier)",
                bws(parse_identifier),
            ),
            opt(preceded(
                context(
                    "variable initializer (expected '=' followed by expression)",
                    bws(bchar('=')),
                ),
                context(
                    "variable initializer expression (expected valid C# expression)",
                    bws(parse_expression),
                ),
            )),
        )),
        |(name, initializer)| VariableDeclarator { name, initializer },
    )(input)
}

/// Parse a variable declaration
/// Examples: "int x = 5", "var y", "string name, address", "const double PI = 3.14"
pub fn parse_variable_declaration(input: &str) -> BResult<&str, LocalVariableDeclaration> {
    // Parse optional const modifier
    let (input, is_const) = context(
        "optional const modifier",
        nom::combinator::map(nom::combinator::opt(bws(keyword("const"))), |opt| {
            opt.is_some()
        }),
    )(input)?;

    // Note: For variable declarations, we start with a type
    let (input, variable_type) = context(
        "variable type (expected valid C# type or 'var')",
        bws(parse_type_expression),
    )(input)?;

    // Parse one or more variable declarators separated by commas
    let (input, declarators) = context(
        "variable declarators (expected one or more variable names with optional initializers)",
        bseparated_list1(bws(bchar(',')), bws(parse_variable_declarator)),
    )(input)?;

    Ok((
        input,
        LocalVariableDeclaration {
            declaration_type: variable_type,
            declarators,
            is_const,
            is_ref: false,
        },
    ))
}

/// Parse a local variable declaration statement (with semicolon)
/// Example: "int x = 5;"
pub fn parse_local_variable_declaration(input: &str) -> BResult<&str, LocalVariableDeclaration> {
    let (input, declaration) = parse_variable_declaration(input)?;

    let (input, _) = context(
        "variable declaration terminator (expected ';')",
        bws(bchar(';')),
    )(input)?;

    Ok((input, declaration))
}

/// Wrapper function to use in statement parsing
pub fn parse_local_variable_declaration_statement(
    input: &str,
) -> BResult<&str, crate::syntax::nodes::statements::statement::Statement> {
    use crate::syntax::nodes::statements::statement::Statement;
    map(parse_local_variable_declaration, Statement::Declaration)(input)
}
