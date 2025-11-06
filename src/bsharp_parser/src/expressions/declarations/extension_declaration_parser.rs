use crate::errors::BResult;
use crate::parser::expressions::declarations::type_declaration_parser::parse_class_body_span;
use crate::parser::expressions::declarations::type_declaration_parser::parse_class_member_for_spans as parse_class_member;
use crate::parser::keywords::contextual_misc_keywords::kw_extension;
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use nom::sequence::delimited;
use nom::Parser;
use syntax::declarations::ExtensionDeclaration;
use syntax::span::Span;

pub fn parse_extension_declaration(input: Span) -> BResult<ExtensionDeclaration> {
    let (input, _) = delimited(ws, kw_extension(), ws).parse(input)?;
    let (input, receiver) = delimited(ws, parse_type_expression, ws).parse(input)?;
    let (input, members) = parse_class_body_span(input, parse_class_member)?;
    Ok((input, ExtensionDeclaration { receiver, members }))
}
