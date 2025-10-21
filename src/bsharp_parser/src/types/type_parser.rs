use crate::keywords::declaration_keywords::kw_delegate;
use crate::keywords::modifier_keywords::{kw_managed, kw_unmanaged};
use crate::parser::identifier_parser::parse_qualified_name;
use crate::parser::keywords::contextual_misc_keywords::{kw_dynamic, kw_var};
use crate::parser::keywords::modifier_keywords::kw_readonly;
use crate::parser::keywords::parameter_modifier_keywords::kw_ref;
use crate::parser::keywords::type_keywords::{
    kw_bool, kw_byte, kw_char, kw_decimal, kw_double, kw_float, kw_int, kw_long, kw_nint, kw_nuint,
    kw_object, kw_sbyte, kw_short, kw_string, kw_uint, kw_ulong, kw_ushort, kw_void,
};
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::list_parser::parse_delimited_list0;
use crate::syntax::span::Span;
use crate::tokens::arithmetic::tok_multiply;
use crate::tokens::delimiters::{tok_l_brack, tok_r_brack};
use crate::tokens::relational::{tok_gt, tok_lt};
use crate::tokens::separators::tok_comma;
use nom::bytes::complete::take_while;
use nom::combinator::cut;
use nom::combinator::{map, opt};
use nom::sequence::{delimited, pair};
use nom::{Parser, branch::alt, character::complete::char as nom_char, combinator::value};
use nom_supreme::ParserExt;
use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
use syntax::Identifier;
use syntax::types::{CallingConvention, PrimitiveType, Type};

// Parse primitive types like int, bool, string
fn parse_primitive_type(input: Span) -> BResult<Type> {
    (|i| {
        let mut p = alt((
            // Void type
            map(kw_void(), |_| Type::Primitive(PrimitiveType::Void)),
            // Boolean type
            map(kw_bool(), |_| Type::Primitive(PrimitiveType::Bool)),
            // Integral types - order matters! Put longer keywords first to avoid partial matches
            map(kw_ushort(), |_| Type::Primitive(PrimitiveType::UShort)),
            map(kw_uint(), |_| Type::Primitive(PrimitiveType::UInt)),
            map(kw_ulong(), |_| Type::Primitive(PrimitiveType::ULong)),
            map(kw_sbyte(), |_| Type::Primitive(PrimitiveType::SByte)),
            map(kw_short(), |_| Type::Primitive(PrimitiveType::Short)),
            map(kw_byte(), |_| Type::Primitive(PrimitiveType::Byte)),
            map(kw_int(), |_| Type::Primitive(PrimitiveType::Int)),
            map(kw_long(), |_| Type::Primitive(PrimitiveType::Long)),
            map(kw_nint(), |_| Type::Primitive(PrimitiveType::NInt)),
            map(kw_nuint(), |_| Type::Primitive(PrimitiveType::NUInt)),
            // Floating-point types
            map(kw_double(), |_| Type::Primitive(PrimitiveType::Double)),
            map(kw_decimal(), |_| Type::Primitive(PrimitiveType::Decimal)),
            map(kw_float(), |_| Type::Primitive(PrimitiveType::Float)),
            // Character and string types
            map(kw_string(), |_| Type::Primitive(PrimitiveType::String)),
            map(kw_object(), |_| Type::Primitive(PrimitiveType::Object)),
            map(kw_char(), |_| Type::Primitive(PrimitiveType::Char)),
            // Special types
            map(kw_dynamic(), |_| Type::Dynamic),
            map(kw_var(), |_| Type::Var),
        ));
        p.parse(i)
    })
    .context("primitive type (expected built-in C# type like 'int', 'string', 'bool', etc.)")
    .parse(input)
}

// Parse a ref readonly return type (ref readonly Type)
fn parse_ref_readonly_return_type(input: Span) -> BResult<Type> {
    (|input| {
        let (input, _) = delimited(ws, kw_ref(), ws).parse(input)?;
        let (input, _) = delimited(ws, kw_readonly(), ws).parse(input)?;
        let (input, inner_type) = parse_type_expression(input)?;
        Ok((input, Type::RefReadOnlyReturn(Box::new(inner_type))))
    })
    .context("ref readonly return type (expected 'ref readonly' followed by a type)")
    .parse(input)
}

// Parse function pointer type: delegate*<int, string, void> or delegate* managed<int, void>
fn parse_function_pointer_type(input: Span) -> BResult<Type> {
    (|input| {
        let (input, _) = pair(kw_delegate(), tok_multiply()).parse(input)?;

        // Parse optional calling convention using standard nom combinators
        let mut opt_cc = opt(delimited(
            ws,
            |i| {
                let mut cc = alt((
                    value(CallingConvention::Managed, kw_managed()),
                    value(CallingConvention::Unmanaged, kw_unmanaged()),
                ));
                cc.parse(i)
            },
            ws,
        ));
        let (input, calling_convention) = opt_cc.parse(input)?;

        // Parse type parameters: <param1, param2, ..., return_type>
        let (input, types) = parse_delimited_list0::<_, _, _, _, char, char, char, Type>(
            tok_lt(),
            parse_type_expression,
            tok_comma(),
            tok_gt(),
            false,
            true,
        )
        .parse(input)?;

        if types.is_empty() {
            let error_tree = ErrorTree::Base {
                location: input,
                kind: BaseErrorKind::Expected(Expectation::Tag(
                    "function pointer type parameters (expected at least one type)",
                )),
            };
            return Err(nom::Err::Error(error_tree));
        }

        // Last type is the return type, rest are parameter types
        let return_type = Box::new(types.last().unwrap().clone());
        let parameter_types = types[..types.len().saturating_sub(1)].to_vec();

        Ok((
            input,
            Type::FunctionPointer {
                calling_convention,
                parameter_types,
                return_type,
            },
        ))
    })
    .context("function pointer type")
    .parse(input)
}

// Parse an identifier node (qualified or simple)
fn parse_identifier_node(input: Span) -> BResult<Identifier> {
    let (input, parts) = parse_qualified_name(input)?;
    let segs: Vec<String> = parts
        .into_iter()
        .map(|id| match id {
            Identifier::Simple(s) => s,
            Identifier::QualifiedIdentifier(v) => v.join("."),
            Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
        })
        .collect();
    let ident = if segs.len() == 1 {
        Identifier::Simple(segs.into_iter().next().unwrap())
    } else {
        Identifier::QualifiedIdentifier(segs)
    };
    Ok((input, ident))
}

// Parse a generic type: Identifier<type1, type2, ...>
fn parse_generic_type(input: Span) -> BResult<Type> {
    (|input| {
        let (input, base) = parse_identifier_node(input)?;
        let (input, opt_generics) =
            opt(parse_delimited_list0::<_, _, _, _, char, char, char, Type>(
                tok_lt(),
                parse_type_expression,
                tok_comma(),
                tok_gt(),
                false,
                true,
            ))
            .parse(input)?;
        if let Some(args) = opt_generics {
            if args.is_empty() {
                let error_tree = ErrorTree::Base {
                    location: input,
                    kind: BaseErrorKind::Expected(Expectation::Tag("generic type arguments ")),
                };
                return Err(nom::Err::Error(error_tree));
            }
            Ok((input, Type::Generic { base, args }))
        } else {
            Ok((input, Type::Reference(base)))
        }
    })
    .context("generic or reference type")
    .parse(input)
}

fn parse_identifier_type(input: Span) -> BResult<Type> {
    parse_generic_type(input)
}

// Parse array type suffix like [], [,] etc.
fn parse_array_suffix(input: Span) -> BResult<usize> {
    let (input, _) = delimited(ws, tok_l_brack(), ws).parse(input)?;
    // Count the number of commas between brackets
    let (input, inner) = take_while(|c: char| c == ',' || c.is_whitespace()).parse(input)?;
    let (input, _) = cut(delimited(ws, tok_r_brack(), ws)).parse(input)?;
    let rank = inner.fragment().chars().filter(|&c| c == ',').count() + 1;
    Ok((input, rank))
}

// Parse a potentially nullable type (e.g., int?)
fn parse_nullable_suffix(input: Span) -> BResult<()> {
    value((), nom_char('?')).parse(input)
}

// Parse pointer suffix (e.g., int* or char**)
fn parse_pointer_suffix(input: Span) -> BResult<()> {
    value((), nom_char('*')).parse(input)
}

// Helper function for parsing ref keyword with word boundary
fn parse_ref_keyword(input: Span<'_>) -> BResult<'_, &str> {
    use nom::Parser as _;
    kw_ref().parse(input)
}

// Parse a ref return type (ref Type)
fn parse_ref_return_type(input: Span) -> BResult<Type> {
    (|input| {
        let (input, _) = delimited(ws, parse_ref_keyword, ws).parse(input)?;
        let (input, inner_type) = parse_type_expression(input)?;
        Ok((input, Type::RefReturn(Box::new(inner_type))))
    })
    .context("ref return type")
    .parse(input)
}

// Main type syntax: handles primitives, identifiers, arrays, nullables, pointers, function pointers
pub fn parse_type_expression(input: Span) -> BResult<Type> {
    (|input| {
        // Try function pointer first, then ref return, then primitive, then identifier
        let mut p = alt((
            parse_function_pointer_type,
            parse_ref_readonly_return_type,
            parse_ref_return_type,
            parse_primitive_type,
            parse_identifier_type,
        ));
        let (input, ty) = p.parse(input)?;
        let (input, _) = ws(input)?;

        // Define a helper function to parse and apply suffixes recursively
        fn parse_suffixes(input: Span, ty: Type) -> BResult<Type> {
            // Try pointer suffix first (higher precedence than array)
            if let Ok((next_input, _)) = parse_pointer_suffix(input) {
                let pointer_type = Type::Pointer(Box::new(ty));
                return parse_suffixes(next_input, pointer_type);
            }

            // Try array suffix
            if let Ok((next_input, rank)) = parse_array_suffix(input) {
                let array_type = Type::Array {
                    element_type: Box::new(ty),
                    rank,
                };
                return parse_suffixes(next_input, array_type);
            }

            // Try nullable suffix
            if let Ok((next_input, _)) = parse_nullable_suffix(input) {
                // Use Nullable for all nullable types to match existing test expectations
                let nullable_type = Type::Nullable(Box::new(ty));
                return parse_suffixes(next_input, nullable_type);
            }

            // No more suffixes found, return the current type
            Ok((input, ty))
        }

        // Apply the helper function to handle all suffixes
        let (input, ty) = parse_suffixes(input, ty)?;
        let (input, _) = ws(input)?;
        Ok((input, ty))
    })
    .context("type expression")
    .parse(input)
}
