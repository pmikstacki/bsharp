use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::stackalloc_expression::StackAllocExpression;
use crate::syntax::nodes::types::Type;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword, parse_delimited_list0};

use nom::combinator::cut;
use nom::{branch::alt, combinator::map, sequence::{tuple, delimited}};

// Simple collection initializer syntax for stackalloc
fn parse_collection_initializer(input: &str) -> BResult<&str, Vec<Expression>> {
    parse_delimited_list0::<_, _, _, _, char, Expression, char, char, Expression>(
        bchar('{'),
        parse_expression,
        bchar(','),
        bchar('}'),
        false,
        true,
    )(input)
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
pub fn parse_stackalloc_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "stackalloc expression",
        map(
            tuple((
                keyword("stackalloc"),
                bws(alt((
                    // stackalloc[] { ... } (implicitly typed) - try this first to avoid type parsing
                    map(
                        tuple((
                            bchar('['),
                            bws(bchar(']')),
                            bws(parse_collection_initializer),
                        )),
                        |(_, _, initializer)| StackAllocExpression {
                            ty: None,
                            count: None,
                            initializer: Some(initializer),
                        },
                    ),
                    // Try to parse type and then check what follows
                    map(
                        tuple((
                            parse_type_expression,
                            alt((
                                // Case 1: type already parsed as array type (e.g., int[]) - expect initializer
                                map(bws(parse_collection_initializer), |initializer| {
                                    (None, Some(initializer))
                                }),
                                // Case 2: type is element type, expect [size]
                                map(
                                    delimited(
                                        bws(bchar('[')),
                                        bws(parse_expression),
                                        cut(bws(bchar(']'))),
                                    ),
                                    |count| (Some(count), None),
                                ),
                                // Case 3: type is element type, expect [] { ... }
                                map(
                                    tuple((
                                        bws(bchar('[')),
                                        bws(bchar(']')),
                                        bws(parse_collection_initializer),
                                    )),
                                    |(_, _, initializer)| (None, Some(initializer)),
                                ),
                            )),
                        )),
                        |(ty, (count, initializer))| {
                            // Extract element type if it's an array type
                            let (element_type, _is_array_type) = match &ty {
                                Type::Array { element_type, .. } => {
                                    (element_type.as_ref().clone(), true)
                                }
                                _ => (ty, false),
                            };

                            StackAllocExpression {
                                ty: Some(element_type),
                                count,
                                initializer,
                            }
                        },
                    ),
                ))),
            )),
            |(_, stackalloc)| Expression::StackAlloc(Box::new(stackalloc)),
        ),
    )(input)
}
