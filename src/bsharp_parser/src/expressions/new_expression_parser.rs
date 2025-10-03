use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_new;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::anonymous_object_creation_expression::{
    AnonymousObjectCreationExpression, AnonymousObjectMember,
};
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::member_access_expression::MemberAccessExpression;
use crate::syntax::nodes::expressions::new_expression::{NewExpression, ObjectInitializerEntry};
use crate::syntax::parser_helpers::{bchar, bws, context, parse_delimited_list0, peek_bchar};

use nom::{
    branch::alt,
    combinator::{cut, map, opt, peek},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
};

#[derive(Debug, Clone)]
enum InitializerKind {
    Object(Vec<ObjectInitializerEntry>),
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
                    kw_new(),
                    context(
                        "anonymous object creation",
                        delimited(
                            bws(bchar('{')),
                            separated_list0(bws(bchar(',')), bws(parse_anonymous_object_member)),
                            bws(bchar('}')),
                        ),
                    ),
                ),
                |members| {
                    Expression::AnonymousObject(AnonymousObjectCreationExpression {
                        initializers: members,
                    })
                },
            ),
            // Target-typed new (no type) must be tried before typed-with-cut
            target_typed_new_expression,
            // Then try new with type and optional initializer (guarded so it won't consume `new()`)
            enhanced_new_with_type_and_initializer,
            // Simple new expression as fallback (with type)
            simple_new_expression,
        )),
    )(input)
}

/// Enhanced new expression with type and initializer
fn enhanced_new_with_type_and_initializer(input: &str) -> BResult<&str, Expression> {
    map(
        tuple((
            kw_new(),
            cut(bws(parse_type_expression)),
            opt(parse_delimited_list0::<
                _,
                _,
                _,
                _,
                char,
                Expression,
                char,
                char,
                Expression,
            >(
                bchar('('),
                parse_expression,
                bchar(','),
                bchar(')'),
                false, // no trailing comma by default
                true,  // cut on close
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
                target_type: Some(ty),
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
            kw_new(),
            bws(parse_type_expression),
            opt(parse_delimited_list0::<
                _,
                _,
                _,
                _,
                char,
                Expression,
                char,
                char,
                Expression,
            >(
                bchar('('),
                parse_expression,
                bchar(','),
                bchar(')'),
                false, // no trailing comma by default
                true,  // cut on close
            )),
        )),
        |(_new_kw, ty, arguments)| {
            Expression::New(Box::new(NewExpression {
                target_type: Some(ty),
                arguments: arguments.unwrap_or_default(),
                object_initializer: None,
                collection_initializer: None,
            }))
        },
    )(input)
}

/// Target-typed new: new() [initializer]
fn target_typed_new_expression(input: &str) -> BResult<&str, Expression> {
    map(
        tuple((
            kw_new(),
            // Ensure the next token is '(' to disambiguate from typed-new
            bws(peek_bchar('(')),
            // Parse the argument list
            parse_delimited_list0::<_, _, _, _, char, Expression, char, char, Expression>(
                bchar('('),
                parse_expression,
                bchar(','),
                bchar(')'),
                false,
                true,
            ),
            opt(bws(enhanced_initializer)),
        )),
        |(_new_kw, _peek_paren, arguments, initializer)| {
            let (object_initializer, collection_initializer) = match initializer {
                Some(InitializerKind::Object(obj)) => (Some(obj), None),
                Some(InitializerKind::Collection(coll)) => (None, Some(coll)),
                None => (None, None),
            };
            Expression::New(Box::new(NewExpression {
                target_type: None,
                arguments,
                object_initializer,
                collection_initializer,
            }))
        },
    )(input)
}

/// Enhanced initializer syntax with better error recovery
fn enhanced_initializer(input: &str) -> BResult<&str, InitializerKind> {
    delimited(
        bws(bchar('{')),
        alt((
            // If it looks like an object initializer (identifier '=' or '['), commit to object initializer
            preceded(
                peek(object_initializer_guard),
                cut(enhanced_object_initializer),
            ),
            // Otherwise treat as collection initializer
            enhanced_collection_initializer,
        )),
        cut(bws(bchar('}'))),
    )(input)
}

/// Enhanced object initializer with graceful fallback
fn enhanced_object_initializer(input: &str) -> BResult<&str, InitializerKind> {
    map(
        separated_list1(
            bws(bchar(',')),
            alt((
                parse_indexer_assignment,
                enhanced_property_assignment,
                fallback_property_assignment,
            )),
        ),
        InitializerKind::Object,
    )(input)
}

/// Lookahead guard to decide if we are parsing an object initializer.
/// Matches starts like: Identifier '=' ... or '[' ...
fn object_initializer_guard(input: &str) -> BResult<&str, ()> {
    use nom::combinator::map;
    use nom::sequence::tuple;
    map(
        alt((
            map(tuple((bws(parse_identifier), bws(bchar('=')))), |_| ()),
            map(bws(bchar('[')), |_| ()),
        )),
        |_| (),
    )(input)
}

/// Enhanced property assignment parsing
fn enhanced_property_assignment(input: &str) -> BResult<&str, ObjectInitializerEntry> {
    map(
        tuple((
            bws(parse_identifier),
            cut(bws(bchar('='))),
            cut(bws(parse_expression)),
        )),
        |(id, _, expr)| ObjectInitializerEntry::Property {
            name: id.name,
            value: expr,
        },
    )(input)
}

/// Fallback property assignment for simple cases
fn fallback_property_assignment(input: &str) -> BResult<&str, ObjectInitializerEntry> {
    use crate::parser::identifier_parser::parse_identifier;

    map(
        tuple((
            bws(parse_identifier),
            bws(bchar('=')),
            bws(map(parse_identifier, Expression::Variable)),
        )),
        |(id, _, expr)| ObjectInitializerEntry::Property {
            name: id.name,
            value: expr,
        },
    )(input)
}

/// Enhanced collection initializer
fn enhanced_collection_initializer(input: &str) -> BResult<&str, InitializerKind> {
    map(
        separated_list0(bws(bchar(',')), bws(parse_expression)),
        InitializerKind::Collection,
    )(input)
}

/// Indexer assignment: [expr (, expr)* ] = expr
fn parse_indexer_assignment(input: &str) -> BResult<&str, ObjectInitializerEntry> {
    map(
        tuple((
            bws(bchar('[')),
            separated_list1(bws(bchar(',')), bws(parse_expression)),
            cut(bws(bchar(']'))),
            cut(bws(bchar('='))),
            cut(bws(parse_expression)),
        )),
        |(_, indices, _, _, value)| ObjectInitializerEntry::Indexer { indices, value },
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
            },
        ),
        // Implicit initializer: only identifiers allowed (e.g., new { Name, Age })
        parse_implicit_anon_member,
    ))(input)
}

/// Implicit anonymous object member accepts only an identifier (C# inference).
fn parse_implicit_anon_member(input: &str) -> BResult<&str, AnonymousObjectMember> {
    map(parse_dotted_member_expression, |value| {
        AnonymousObjectMember { name: None, value }
    })(input)
}

/// Parse a dotted member expression like `x.Name` or `x.y.z` limited to identifiers only.
fn parse_dotted_member_expression(input: &str) -> BResult<&str, Expression> {
    let (input, first) = bws(parse_identifier)(input)?;
    let (input, rest) = nom::multi::many0(preceded(bws(bchar('.')), bws(parse_identifier)))(input)?;
    if rest.is_empty() {
        Ok((input, Expression::Variable(first)))
    } else {
        let mut expr = Expression::Variable(first);
        for id in rest {
            expr = Expression::MemberAccess(Box::new(MemberAccessExpression {
                object: Box::new(expr),
                member: id,
            }));
        }
        Ok((input, expr))
    }
}
