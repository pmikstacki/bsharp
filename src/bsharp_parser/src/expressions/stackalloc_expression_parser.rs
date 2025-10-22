use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::expression_keywords::kw_stackalloc;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::list_parser::parse_delimited_list0;
use nom::Parser;
use nom::combinator::cut;
use nom::{branch::alt, combinator::map, sequence::delimited};
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, StackAllocExpression};
use syntax::types::Type;

// Simple collection initializer syntax for stackalloc
fn parse_collection_initializer(input: Span) -> BResult<Vec<Expression>> {
    parse_delimited_list0::<_, _, _, _, char, char, char, Expression>(
        |i| delimited(ws, tok_l_brace(), ws).parse(i),
        |i| delimited(ws, parse_expression, ws).parse(i),
        |i| delimited(ws, tok_comma(), ws).parse(i),
        |i| delimited(ws, tok_r_brace(), ws).parse(i),
        false,
        true,
    )
    .parse(input)
}

/// Parse a stackalloc expression
///
/// Examples:
/// ```csharp
/// stackalloc int[10]
/// stackalloc byte[size]
/// stackalloc int[] { 1, 2, 3, 4 }
/// stackalloc[] { 1, 2, 3 }  // implicitly typed
/// ```
pub fn parse_stackalloc_expression(input: Span) -> BResult<Expression> {
    map(
        (
            kw_stackalloc().context("stackalloc keyword"),
            alt((
                // stackalloc[] { ... } (implicitly typed)
                map(
                    (
                        delimited(ws, tok_l_brack(), ws),
                        delimited(ws, tok_r_brack(), ws),
                        delimited(ws, parse_collection_initializer, ws),
                    ),
                    |(_, _, initializer)| StackAllocExpression {
                        target_type: None,
                        count: None,
                        initializer: Some(initializer),
                    },
                ),
                // Try to parse type and then check what follows
                map(
                    (
                        delimited(ws, parse_type_expression, ws),
                        alt((
                            // Case 1: initializer after type[]
                            map(
                                delimited(ws, parse_collection_initializer, ws),
                                |initializer| (None, Some(initializer)),
                            ),
                            // Case 2: [size]
                            map(
                                delimited(
                                    delimited(ws, tok_l_brack(), ws),
                                    delimited(ws, parse_expression, ws),
                                    cut(delimited(ws, tok_r_brack(), ws)),
                                ),
                                |count| (Some(count), None),
                            ),
                            // Case 3: [] { ... }
                            map(
                                (
                                    delimited(ws, tok_l_brack(), ws),
                                    delimited(ws, tok_r_brack(), ws),
                                    delimited(ws, parse_collection_initializer, ws),
                                ),
                                |(_, _, initializer)| (None, Some(initializer)),
                            ),
                        )),
                    ),
                    |(ty, (count, initializer))| {
                        // Extract element type if it's an array type
                        let (element_type, _is_array_type) = match &ty {
                            Type::Array { element_type, .. } => {
                                (element_type.as_ref().clone(), true)
                            }
                            _ => (ty, false),
                        };

                        StackAllocExpression {
                            target_type: Some(element_type),
                            count,
                            initializer,
                        }
                    },
                ),
            )),
        ),
        |(_, stackalloc)| Expression::StackAlloc(Box::new(stackalloc)),
    )
    .context("stackalloc expression")
    .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::delimiters::{tok_l_brace, tok_l_brack, tok_r_brace, tok_r_brack};
use crate::tokens::separators::tok_comma;
