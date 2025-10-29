use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_new;
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;

use crate::syntax::list_parser::parse_delimited_list0;
use nom::Parser;
use nom::character::complete::char as nom_char;
use nom::{
    branch::alt,
    combinator::{cut, map, opt, peek},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded},
};
use nom_supreme::ParserExt;
use syntax::Identifier;
use syntax::expressions::new_expression::ObjectInitializerEntry;
use syntax::expressions::{
    AnonymousObjectCreationExpression, AnonymousObjectMember, Expression, MemberAccessExpression,
    NewExpression,
};

#[derive(Debug, Clone)]
enum InitializerKind {
    Object(Vec<ObjectInitializerEntry>),
    Collection(Vec<Expression>),
}

use syntax::span::Span;

use crate::tokens::assignment::tok_assign;
use crate::tokens::delimiters::{
    tok_l_brace, tok_l_brack, tok_l_paren, tok_r_brace, tok_r_brack, tok_r_paren,
};
use crate::tokens::separators::tok_comma;

/// Enhanced new expression syntax using robust Nom combinators
/// Handles complex patterns like new User { Name = "John", Email = email }
pub(crate) fn parse_new_expression(input: Span) -> BResult<Expression> {
    map(
        alt((
            // Try anonymous object creation first (new { ... })
            map(
                preceded(
                    kw_new(),
                    delimited(
                        delimited(ws, tok_l_brace(), ws),
                        separated_list0(
                            delimited(ws, tok_comma(), ws),
                            delimited(ws, parse_anonymous_object_member, ws),
                        ),
                        delimited(ws, tok_r_brace(), ws),
                    )
                    .context("anonymous object creation"),
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
        |v| v,
    )
    .context("new expression")
    .parse(input)
}

/// Enhanced new expression with type and initializer
fn enhanced_new_with_type_and_initializer(input: Span) -> BResult<Expression> {
    fn parse_arg_list(i: Span) -> BResult<Vec<Expression>> {
        parse_delimited_list0::<_, _, _, _, char, char, char, Expression>(
            |i| delimited(ws, tok_l_paren(), ws).parse(i),
            |i| {
                let (r, s) = delimited(ws, parse_expression_spanned, ws).parse(i)?;
                Ok((r, s.node))
            },
            |i| delimited(ws, tok_comma(), ws).parse(i),
            |i| delimited(ws, tok_r_paren(), ws).parse(i),
            false,
            true,
        )
        .parse(i)
    }
    map(
        (
            kw_new(),
            cut(delimited(ws, parse_type_expression, ws)),
            // Only proceed as typed-new if next significant token starts an arg list or initializer
            peek(alt((delimited(ws, tok_l_paren(), ws), delimited(ws, tok_l_brace(), ws)))),
            opt(parse_arg_list),
            opt(delimited(ws, enhanced_initializer, ws)),
        ),
        |(_new_kw, ty, _guard, arguments, initializer)| {
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
    )
    .parse(input)
}

/// Simple new expression as fallback
fn simple_new_expression(input: Span) -> BResult<Expression> {
    fn parse_arg_list(i: Span) -> BResult<Vec<Expression>> {
        parse_delimited_list0::<_, _, _, _, char, char, char, Expression>(
            |i| delimited(ws, tok_l_paren(), ws).parse(i),
            |i| {
                let (r, s) = delimited(ws, parse_expression_spanned, ws).parse(i)?;
                Ok((r, s.node))
            },
            |i| delimited(ws, tok_comma(), ws).parse(i),
            |i| delimited(ws, tok_r_paren(), ws).parse(i),
            false,
            true,
        )
        .parse(i)
    }
    map(
        (
            kw_new(),
            delimited(ws, parse_type_expression, ws),
            // Require there to be an argument list to consider this a simple typed-new
            peek(delimited(ws, tok_l_paren(), ws)),
            opt(parse_arg_list),
        ),
        |(_new_kw, ty, _guard, arguments)| {
            Expression::New(Box::new(NewExpression {
                target_type: Some(ty),
                arguments: arguments.unwrap_or_default(),
                object_initializer: None,
                collection_initializer: None,
            }))
        },
    )
    .parse(input)
}

/// Target-typed new: new() [initializer]
fn target_typed_new_expression(input: Span) -> BResult<Expression> {
    fn parse_arg_list(i: Span) -> BResult<Vec<Expression>> {
        parse_delimited_list0::<_, _, _, _, char, char, char, Expression>(
            |i| delimited(ws, tok_l_paren(), ws).parse(i),
            |i| {
                let (r, s) = delimited(ws, parse_expression_spanned, ws).parse(i)?;
                Ok((r, s.node))
            },
            |i| delimited(ws, tok_comma(), ws).parse(i),
            |i| delimited(ws, tok_r_paren(), ws).parse(i),
            false,
            true,
        )
        .parse(i)
    }
    map(
        (
            kw_new(),
            // Ensure the next token is '(' to disambiguate from typed-new
            peek(delimited(ws, tok_l_paren(), ws)),
            // Parse the argument list
            parse_arg_list,
            opt(delimited(ws, enhanced_initializer, ws)),
        ),
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
    )
    .parse(input)
}

/// Enhanced initializer syntax with better error recovery
fn enhanced_initializer(input: Span) -> BResult<InitializerKind> {
    delimited(
        delimited(ws, tok_l_brace(), ws),
        alt((
            preceded(
                peek(object_initializer_guard),
                cut(enhanced_object_initializer),
            ),
            enhanced_collection_initializer,
        )),
        cut(delimited(ws, tok_r_brace(), ws)),
    )
    .parse(input)
}

/// Enhanced object initializer with graceful fallback
fn enhanced_object_initializer(input: Span) -> BResult<InitializerKind> {
    map(
        separated_list1(
            delimited(ws, tok_comma(), ws),
            alt((
                parse_indexer_assignment,
                enhanced_property_assignment,
                fallback_property_assignment,
            )),
        ),
        InitializerKind::Object,
    )
    .parse(input)
}

/// Lookahead guard to decide if we are parsing an object initializer.
/// Matches starts like: Identifier '=' ... or '[' ...
fn object_initializer_guard(input: Span) -> BResult<()> {
    use nom::combinator::map;
    map(
        alt((
            map(
                (
                    delimited(ws, parse_identifier, ws),
                    delimited(ws, tok_assign(), ws),
                ),
                |_| (),
            ),
            map(delimited(ws, tok_l_brack(), ws), |_| ()),
        )),
        |_| (),
    )
    .parse(input)
}

/// Enhanced property assignment parsing
fn enhanced_property_assignment(input: Span) -> BResult<ObjectInitializerEntry> {
    map(
        (
            delimited(ws, parse_identifier, ws),
            delimited(ws, tok_assign(), ws),
            delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
        ),
        |(id, _, expr)| {
            let name = match id {
                Identifier::Simple(s) => s,
                Identifier::QualifiedIdentifier(segs) => segs.join("."),
                Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
            };
            ObjectInitializerEntry::Property { name, value: expr }
        },
    )
    .parse(input)
}

/// Fallback property assignment for simple cases
fn fallback_property_assignment(input: Span) -> BResult<ObjectInitializerEntry> {
    use crate::parser::identifier_parser::parse_identifier;

    map(
        (
            delimited(ws, parse_identifier, ws),
            delimited(ws, tok_assign(), ws),
            delimited(ws, map(parse_identifier, Expression::Variable), ws),
        ),
        |(id, _, expr)| {
            let name = match id {
                Identifier::Simple(s) => s,
                Identifier::QualifiedIdentifier(segs) => segs.join("."),
                Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
            };
            ObjectInitializerEntry::Property { name, value: expr }
        },
    )
    .parse(input)
}

/// Enhanced collection initializer
fn enhanced_collection_initializer(input: Span) -> BResult<InitializerKind> {
    map(
        separated_list0(
            delimited(ws, tok_comma(), ws),
            delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
        ),
        InitializerKind::Collection,
    )
    .parse(input)
}

/// Indexer assignment: [expr (, expr)* ] = expr
fn parse_indexer_assignment(input: Span) -> BResult<ObjectInitializerEntry> {
    map(
        (
            delimited(ws, tok_l_brack(), ws),
            separated_list1(
                delimited(ws, tok_comma(), ws),
                delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
            ),
            cut(delimited(ws, tok_r_brack(), ws)),
            cut(delimited(ws, tok_assign(), ws)),
            cut(delimited(ws, parse_expression_spanned, ws).map(|s| s.node)),
        ),
        |(_, indices, _, _, value)| ObjectInitializerEntry::Indexer { indices, value },
    )
    .parse(input)
}

fn parse_anonymous_object_member(input: Span) -> BResult<AnonymousObjectMember> {
    // Handle both explicit (Name = value) and implicit (expression) initializers
    alt((
        // Explicit initializer: Name = value
        map(
            (
                delimited(ws, parse_identifier, ws),
                delimited(ws, tok_assign(), ws),
                delimited(ws, parse_expression_spanned, ws).map(|s| s.node),
            ),
            |(name, _, value)| AnonymousObjectMember {
                name: Some(name),
                value,
            },
        ),
        // Implicit initializer: only identifiers allowed (e.g., new { Name, Age })
        parse_implicit_anon_member,
    ))
    .parse(input)
}

/// Implicit anonymous object member accepts only an identifier (C# inference).
fn parse_implicit_anon_member(input: Span) -> BResult<AnonymousObjectMember> {
    map(parse_dotted_member_expression, |value| {
        AnonymousObjectMember { name: None, value }
    })
    .parse(input)
}

/// Parse a dotted member expression like `x.Name` or `x.y.z` limited to identifiers only.
fn parse_dotted_member_expression(input: Span) -> BResult<Expression> {
    let (input, first) = delimited(ws, parse_identifier, ws).parse(input)?;
    let (input, rest) = nom::multi::many0(preceded(
        delimited(ws, nom_char('.'), ws),
        delimited(ws, parse_identifier, ws),
    ))
    .parse(input)?;
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
