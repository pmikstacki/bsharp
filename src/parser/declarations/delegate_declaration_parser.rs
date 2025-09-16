use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::DelegateDeclaration;
use crate::syntax::parser_helpers::{context, bws, keyword, bchar};
use crate::parser::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parser::declarations::parameter_parser::parse_parameter_list;
use crate::parser::declarations::type_parameter_parser::{opt_parse_type_parameter_list, parse_type_parameter_constraints_clauses};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use nom::{
    combinator::opt,
};

/// Parse a delegate declaration
/// Example: public delegate void MyDelegate(int x, string y);
/// Example: public delegate T MyGenericDelegate<T>(T input) where T : class;
pub fn parse_delegate_declaration(input: &str) -> BResult<&str, DelegateDeclaration> {
    context(
        "delegate declaration (expected optional attributes, modifiers, 'delegate' keyword, return type, name, optional type parameters, parameter list, optional constraints, and semicolon)",
        |input| {
            // Parse optional attributes
            let (input, attributes) = bws(parse_attribute_lists)(input)?;
            
            // Parse modifiers
            let (input, modifiers) = parse_modifiers_for_decl_type(input, "delegate")?;
            
            // Parse 'delegate' keyword
            let (input, _) = bws(keyword("delegate"))(input)?;
            
            // Parse return type
            let (input, return_type) = context(
                "delegate return type (expected valid C# type)",
                bws(parse_type_expression),
            )(input)?;
            
            // Parse delegate name
            let (input, name) = context(
                "delegate name (expected valid identifier)",
                bws(parse_identifier),
            )(input)?;
            
            // Parse optional type parameters
            let (input, type_parameters) = bws(opt_parse_type_parameter_list)(input)?;
            
            // Parse parameter list
            let (input, parameters) = context(
                "delegate parameter list (expected '(' followed by parameters and ')')",
                bws(parse_parameter_list),
            )(input)?;
            
            // Parse optional constraints
            let (input, constraints) = opt(bws(parse_type_parameter_constraints_clauses))(input)?;
            
            // Parse semicolon
            let (input, _) = context(
                "semicolon (expected ';' to end delegate declaration)",
                bws(bchar(';')),
            )(input)?;

            // Convert Some([]) to None for constraints and type_parameters
            let constraints = match constraints {
                Some(constraints_vec) if constraints_vec.is_empty() => None,
                other => other,
            };

            let type_parameters = match type_parameters {
                Some(type_params_vec) if type_params_vec.is_empty() => None,
                other => other,
            };

            Ok((
                input,
                DelegateDeclaration {
                    attributes,
                    modifiers,
                    return_type,
                    name,
                    type_parameters: type_parameters.unwrap_or_default(),
                    parameters,
                    constraints,
                },
            ))
        },
    )(input)
} 