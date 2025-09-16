use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::anonymous_object_creation_expression::{AnonymousObjectCreationExpression, AnonymousObjectMember};
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::new_expression::NewExpression;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};
use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;

use nom::{
    branch::alt,
    combinator::{cut, map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
};

#[derive(Debug, Clone)]
enum InitializerKind {
    Object(Vec<(String, Expression)>),
    Collection(Vec<Expression>),
}

/// Enhanced new expression syntax using robust Nom combinators
/// Handles complex patterns like new User { Name = "John", Email = email }
pub(crate) fn parse_new_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "new expression",
        alt((
            // Try anonymous object creation first (new { ... })
            map(
                preceded(
                    keyword("new"),
                    context(
                        "anonymous object creation",
                        delimited(
                            bws(bchar('{')),
                            separated_list0(bws(bchar(',')), bws(parse_anonymous_object_member)),
                            cut(bws(bchar('}')))
                        ),
                    )
                ),
                |members| Expression::AnonymousObject(AnonymousObjectCreationExpression {
                    initializers: members,
                })
            ),
            // Then try new with type and initializer
            enhanced_new_with_type_and_initializer,
            // Simple new expression as fallback
            simple_new_expression,
        ))
    )(input)
}

/// Enhanced new expression with type and initializer
fn enhanced_new_with_type_and_initializer(input: &str) -> BResult<&str, Expression> {
    map(
        tuple((
            keyword("new"),
            cut(bws(parse_type_expression)),
            opt(delimited(
                bws(bchar('(')),
                separated_list0(bws(bchar(',')), bws(parse_expression)),
                cut(bws(bchar(')')))
            )),
            opt(bws(enhanced_initializer)),
        )),
        |(_new_kw, ty, arguments, initializer)| {
            let (object_initializer, collection_initializer) = match initializer {
                Some(InitializerKind::Object(obj)) => (Some(obj), None),
                Some(InitializerKind::Collection(coll)) => (None, Some(coll)),
                None => (None, None),
            };
            
            Expression::New(Box::new(NewExpression {
                ty,
                arguments: arguments.unwrap_or_default(),
                object_initializer,
                collection_initializer,
            }))
        },
    )(input)
}

/// Simple new expression as fallback
fn simple_new_expression(input: &str) -> BResult<&str, Expression> {
    map(
        tuple((
            keyword("new"),
            bws(parse_type_expression),
            opt(delimited(
                bws(bchar('(')),
                separated_list0(bws(bchar(',')), bws(parse_expression)),
                cut(bws(bchar(')')))
            )),
        )),
        |(_new_kw, ty, arguments)| {
            Expression::New(Box::new(NewExpression {
                ty,
                arguments: arguments.unwrap_or_default(),
                object_initializer: None,
                collection_initializer: None,
            }))
        },
    )(input)
}

/// Enhanced initializer syntax with better error recovery
fn enhanced_initializer(input: &str) -> BResult<&str, InitializerKind> {
    delimited(
        bws(bchar('{')),
        alt((
            enhanced_object_initializer,
        	enhanced_collection_initializer,
        )),
        cut(bws(bchar('}')))
    )(input)
}

/// Enhanced object initializer with graceful fallback
fn enhanced_object_initializer(input: &str) -> BResult<&str, InitializerKind> {
    map(
        separated_list1(
            bws(bchar(',')),
            alt((
                enhanced_property_assignment,
                fallback_property_assignment,
            ))
        ),
        InitializerKind::Object
    )(input)
}

/// Enhanced property assignment parsing
fn enhanced_property_assignment(input: &str) -> BResult<&str, (String, Expression)> {
    map(
        tuple((
            bws(parse_identifier),
            cut(bws(bchar('='))),
            cut(bws(parse_expression)),
        )),
        |(id, _, expr)| (id.name, expr)
    )(input)
}

/// Fallback property assignment for simple cases
fn fallback_property_assignment(input: &str) -> BResult<&str, (String, Expression)> {
    use crate::parser::identifier_parser::parse_identifier;
    
    map(
        tuple((
            bws(parse_identifier),
            bws(bchar('=')),
            bws(map(parse_identifier, |id| Expression::Variable(id))),
        )),
        |(id, _, expr)| (id.name, expr)
    )(input)
}

/// Enhanced collection initializer
fn enhanced_collection_initializer(input: &str) -> BResult<&str, InitializerKind> {
    map(
        separated_list0(bws(bchar(',')), bws(parse_expression)),
        InitializerKind::Collection
    )(input)
}

fn parse_anonymous_object_member(input: &str) -> BResult<&str, AnonymousObjectMember> {
    // Handle both explicit (Name = value) and implicit (expression) initializers
    alt((
        // Explicit initializer: Name = value
        map(
            tuple((
                bws(parse_identifier),
                bws(bchar('=')),
                bws(parse_expression),
            )),
            |(name, _, value)| AnonymousObjectMember {
                name: Some(name),
                value,
            }
        ),
        // Implicit initializer: just an expression (for projection)
        map(
            bws(parse_expression),
            |value| AnonymousObjectMember {
                name: None,
                value,
            }
        ),
    ))(input)
}
